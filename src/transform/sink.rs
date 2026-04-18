use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use arrow_array::{
    ArrayRef, Date32Array, DurationMicrosecondArray, Float64Array, RecordBatch, StringArray,
    Time64MicrosecondArray, TimestampMicrosecondArray,
};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::parser::contracts::{
    ColumnKind, Endianness, NumericValue, ParsedRow, ParsedSas7bdat, ParsedValue, RowBatch,
    SasColumn, SasMetadata, SasMissingTag, SemanticTypeHint,
};

use super::contracts::{ExecutionModel, SinkFormat, TransformRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParquetSinkPlan {
    pub output_path: PathBuf,
    pub row_group_rows: usize,
    pub format: SinkFormat,
}

impl ParquetSinkPlan {
    pub fn from_request(request: &TransformRequest) -> Self {
        let row_group_rows = match request.transform.execution {
            ExecutionModel::Streaming => request.transform.tuning.batch_size_rows,
            ExecutionModel::BoundedMemory { max_rows_in_memory } => request
                .transform
                .tuning
                .batch_size_rows
                .min(max_rows_in_memory),
        };

        Self {
            output_path: request.sink.path.clone(),
            row_group_rows,
            format: request.sink.format.clone(),
        }
    }
}

pub trait ParquetSink {
    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError>;
}

pub trait StreamingParquetSink: ParquetSink {
    fn stage_batches(
        &self,
        plan: ParquetSinkPlan,
        execution: &TransformExecution,
        dataset: &mut ParsedSas7bdat,
    ) -> Result<ParquetSinkReport, ParquetSinkError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParquetSinkReport {
    pub plan: ParquetSinkPlan,
    pub status: ParquetSinkStatus,
    pub staged_row_count: usize,
    pub staged_batch_count: usize,
    pub output_size_bytes: u64,
    pub parallel_batch_count: usize,
    pub transform_threads_used: usize,
}

impl ParquetSinkReport {
    pub fn skeleton(plan: ParquetSinkPlan) -> Self {
        Self {
            plan,
            status: ParquetSinkStatus::SkeletonReady,
            staged_row_count: 0,
            staged_batch_count: 0,
            output_size_bytes: 0,
            parallel_batch_count: 0,
            transform_threads_used: 1,
        }
    }

    pub fn decoded_rows_staged(
        plan: ParquetSinkPlan,
        staged_row_count: usize,
        staged_batch_count: usize,
        parallel_batch_count: usize,
        transform_threads_used: usize,
    ) -> Self {
        Self {
            plan,
            status: ParquetSinkStatus::DecodedRowsStaged,
            staged_row_count,
            staged_batch_count,
            output_size_bytes: 0,
            parallel_batch_count,
            transform_threads_used: transform_threads_used.max(1),
        }
    }

    pub fn parquet_written(
        plan: ParquetSinkPlan,
        staged_row_count: usize,
        staged_batch_count: usize,
        output_size_bytes: u64,
        parallel_batch_count: usize,
        transform_threads_used: usize,
    ) -> Self {
        Self {
            plan,
            status: ParquetSinkStatus::ParquetWritten,
            staged_row_count,
            staged_batch_count,
            output_size_bytes,
            parallel_batch_count,
            transform_threads_used: transform_threads_used.max(1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParquetSinkStatus {
    SkeletonReady,
    DecodedRowsStaged,
    ParquetWritten,
}

impl ParquetSinkStatus {
    pub fn label(&self) -> &str {
        match self {
            Self::SkeletonReady => "parquet-skeleton",
            Self::DecodedRowsStaged => "decoded-rows-staged",
            Self::ParquetWritten => "parquet-written",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParquetSinkError {
    message: String,
}

impl ParquetSinkError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for ParquetSinkError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ParquetSinkError {}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformExecution {
    projected_columns: Vec<ProjectionColumn>,
    filter: Option<FilterPredicate>,
    selection_applied: bool,
    parallelism: BatchParallelism,
    source_endianness: Endianness,
}

impl TransformExecution {
    pub fn from_request(
        request: &TransformRequest,
        metadata: &SasMetadata,
    ) -> Result<Self, TransformExecutionError> {
        let projected_columns = if request.transform.selection.is_empty() {
            metadata
                .columns
                .iter()
                .enumerate()
                .map(|(index, column)| ProjectionColumn::from_source(index, column))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect()
        } else {
            let mut projected_columns = Vec::with_capacity(request.transform.selection.len() * 2);
            for name in &request.transform.selection {
                let (index, column) = metadata
                    .columns
                    .iter()
                    .enumerate()
                    .find(|(_, column)| column.name == *name)
                    .ok_or_else(|| {
                        TransformExecutionError::new(format!("unknown selected column: {name}"))
                    })?;
                if projected_columns
                    .iter()
                    .any(|column: &ProjectionColumn| column.name == *name)
                {
                    return Err(TransformExecutionError::new(format!(
                        "duplicate selected column: {name}"
                    )));
                }
                projected_columns.extend(ProjectionColumn::from_source(index, column)?);
            }
            projected_columns
        };

        let filter = request
            .transform
            .filter
            .as_deref()
            .map(|expression| FilterPredicate::parse(expression, metadata))
            .transpose()?;

        Ok(Self {
            projected_columns,
            filter,
            selection_applied: !request.transform.selection.is_empty(),
            parallelism: BatchParallelism::from_request(request),
            source_endianness: metadata.subset.endianness,
        })
    }

    pub fn output_column_count(&self) -> usize {
        self.projected_columns.len()
    }

    pub fn selection_applied(&self) -> bool {
        self.selection_applied
    }

    pub fn filter_applied(&self) -> bool {
        self.filter.is_some()
    }

    fn apply(&self, batch: RowBatch) -> Result<ExecutedBatch, TransformExecutionError> {
        let threads_used = self.parallelism.threads_for(batch.rows.len());
        let batch = if threads_used > 1 {
            self.apply_parallel(batch, threads_used)?
        } else {
            self.apply_serial(batch)?
        };

        Ok(ExecutedBatch {
            batch,
            threads_used,
        })
    }

    fn apply_serial(&self, batch: RowBatch) -> Result<TypedBatch, TransformExecutionError> {
        let chunk = self.apply_rows(&batch.rows)?;
        Ok(TypedBatch {
            row_index_start: batch.row_index_start,
            row_count: chunk.row_count,
            columns: chunk.columns,
        })
    }

    fn apply_parallel(
        &self,
        batch: RowBatch,
        threads_used: usize,
    ) -> Result<TypedBatch, TransformExecutionError> {
        let row_index_start = batch.row_index_start;
        let chunk_size = batch.rows.len().div_ceil(threads_used);
        let pool = transform_thread_pool(threads_used)?;
        let chunks = pool.install(|| {
            batch
                .rows
                .par_chunks(chunk_size)
                .map(|rows| self.apply_rows(rows))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(TypedBatch::from_chunks(
            row_index_start,
            chunks,
            &self.projected_columns,
        ))
    }

    fn apply_rows(&self, rows: &[ParsedRow]) -> Result<TypedBatchChunk, TransformExecutionError> {
        let mut columns = self
            .projected_columns
            .iter()
            .map(|column| TypedColumn::with_capacity(column.kind.clone(), rows.len()))
            .collect::<Vec<_>>();
        let mut row_count = 0;

        for row in rows {
            if !self.row_matches(row)? {
                continue;
            }

            for (typed, projected) in columns.iter_mut().zip(&self.projected_columns) {
                let value = row.values.get(projected.source_index).ok_or_else(|| {
                    TransformExecutionError::new(format!(
                        "column {} is out of bounds for the parsed row",
                        projected.name
                    ))
                })?;
                typed.push(value, &projected.name, self.source_endianness)?;
            }
            row_count += 1;
        }

        Ok(TypedBatchChunk { row_count, columns })
    }

    fn row_matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {
        match &self.filter {
            Some(filter) => filter.matches(row),
            None => Ok(true),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BatchParallelism {
    worker_threads: usize,
    min_rows_per_thread: usize,
}

impl BatchParallelism {
    fn from_request(request: &TransformRequest) -> Self {
        let worker_threads = request
            .transform
            .tuning
            .worker_threads
            .map(|value| value.max(1))
            .unwrap_or_else(default_worker_threads);
        Self {
            worker_threads,
            min_rows_per_thread: 4_096,
        }
    }

    fn threads_for(&self, row_count: usize) -> usize {
        if self.worker_threads <= 1 {
            return 1;
        }
        if row_count < self.min_rows_per_thread.saturating_mul(self.worker_threads) {
            return 1;
        }

        self.worker_threads
            .min(row_count.div_ceil(self.min_rows_per_thread))
            .max(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectionColumn {
    source_index: usize,
    name: String,
    kind: ProjectionKind,
    nullable: bool,
    metadata: HashMap<String, String>,
}

impl ProjectionColumn {
    fn from_source(
        source_index: usize,
        column: &SasColumn,
    ) -> Result<Vec<Self>, TransformExecutionError> {
        let primary_kind = ProjectionKind::from_source_column(column);
        let mut projected_columns = vec![Self {
            source_index,
            name: column.name.clone(),
            nullable: primary_kind.is_nullable(),
            metadata: primary_field_metadata(column),
            kind: primary_kind,
        }];

        if column.kind == ColumnKind::Numeric {
            projected_columns.push(Self {
                source_index,
                name: missing_tag_column_name(&column.name),
                kind: ProjectionKind::MissingTagUtf8,
                nullable: true,
                metadata: missing_tag_field_metadata(column),
            });
        }

        Ok(projected_columns)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectionKind {
    Float64,
    Utf8,
    Date32,
    Time64Microsecond,
    TimestampMicrosecond,
    DurationMicrosecond,
    MissingTagUtf8,
}

impl ProjectionKind {
    fn from_source_column(column: &SasColumn) -> Self {
        match column.kind {
            ColumnKind::String => Self::Utf8,
            ColumnKind::Numeric => match column.semantic_type {
                SemanticTypeHint::Deferred => Self::Float64,
                SemanticTypeHint::Date => Self::Date32,
                SemanticTypeHint::Time => Self::Time64Microsecond,
                SemanticTypeHint::DateTime => Self::TimestampMicrosecond,
                SemanticTypeHint::Duration => Self::DurationMicrosecond,
            },
        }
    }

    fn data_type(&self) -> DataType {
        match self {
            Self::Float64 => DataType::Float64,
            Self::Utf8 | Self::MissingTagUtf8 => DataType::Utf8,
            Self::Date32 => DataType::Date32,
            Self::Time64Microsecond => DataType::Time64(TimeUnit::Microsecond),
            Self::TimestampMicrosecond => DataType::Timestamp(TimeUnit::Microsecond, None),
            Self::DurationMicrosecond => DataType::Duration(TimeUnit::Microsecond),
        }
    }

    fn is_nullable(&self) -> bool {
        !matches!(self, Self::Utf8)
    }
}

fn missing_tag_column_name(column_name: &str) -> String {
    format!("{column_name}__sas_missing_tag")
}

fn primary_field_metadata(column: &SasColumn) -> HashMap<String, String> {
    let mut metadata = HashMap::from([
        (
            "sas.kind".to_string(),
            match column.kind {
                ColumnKind::Numeric => "numeric",
                ColumnKind::String => "string",
            }
            .to_string(),
        ),
        (
            "sas.semantic_type".to_string(),
            column.semantic_type.label().to_string(),
        ),
    ]);
    if let Some(label) = &column.metadata.label {
        metadata.insert("sas.label".to_string(), label.clone());
    }
    if let Some(format_name) = &column.metadata.format_name {
        metadata.insert("sas.format_name".to_string(), format_name.clone());
    }
    if let Some(informat_name) = &column.metadata.informat_name {
        metadata.insert("sas.informat_name".to_string(), informat_name.clone());
    }
    if column.kind == ColumnKind::Numeric {
        metadata.insert(
            "sas.missing_tag_column".to_string(),
            missing_tag_column_name(&column.name),
        );
    }
    metadata
}

fn missing_tag_field_metadata(column: &SasColumn) -> HashMap<String, String> {
    HashMap::from([
        ("sas.kind".to_string(), "missing_tag".to_string()),
        ("sas.parent_column".to_string(), column.name.clone()),
        ("sas.tag_domain".to_string(), ". _ A-Z".to_string()),
    ])
}

#[derive(Debug, Clone, PartialEq)]
struct TypedBatch {
    row_index_start: usize,
    row_count: usize,
    columns: Vec<TypedColumn>,
}

impl TypedBatch {
    fn from_chunks(
        row_index_start: usize,
        chunks: Vec<TypedBatchChunk>,
        projected_columns: &[ProjectionColumn],
    ) -> Self {
        let row_count = chunks.iter().map(|chunk| chunk.row_count).sum();
        let mut columns = projected_columns
            .iter()
            .map(|column| TypedColumn::with_capacity(column.kind.clone(), row_count))
            .collect::<Vec<_>>();

        for chunk in chunks {
            for (typed, partial) in columns.iter_mut().zip(chunk.columns) {
                typed.extend(partial);
            }
        }

        Self {
            row_index_start,
            row_count,
            columns,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TypedBatchChunk {
    row_count: usize,
    columns: Vec<TypedColumn>,
}

#[derive(Debug, Clone, PartialEq)]
enum TypedColumn {
    Float64(Vec<Option<f64>>),
    Utf8(Vec<Option<String>>),
    Date32(Vec<Option<i32>>),
    Time64Microsecond(Vec<Option<i64>>),
    TimestampMicrosecond(Vec<Option<i64>>),
    DurationMicrosecond(Vec<Option<i64>>),
}

impl TypedColumn {
    fn with_capacity(kind: ProjectionKind, capacity: usize) -> Self {
        match kind {
            ProjectionKind::Float64 => Self::Float64(Vec::with_capacity(capacity)),
            ProjectionKind::Utf8 | ProjectionKind::MissingTagUtf8 => {
                Self::Utf8(Vec::with_capacity(capacity))
            }
            ProjectionKind::Date32 => Self::Date32(Vec::with_capacity(capacity)),
            ProjectionKind::Time64Microsecond => {
                Self::Time64Microsecond(Vec::with_capacity(capacity))
            }
            ProjectionKind::TimestampMicrosecond => {
                Self::TimestampMicrosecond(Vec::with_capacity(capacity))
            }
            ProjectionKind::DurationMicrosecond => {
                Self::DurationMicrosecond(Vec::with_capacity(capacity))
            }
        }
    }

    fn push(
        &mut self,
        value: &ParsedValue,
        column_name: &str,
        endianness: Endianness,
    ) -> Result<(), TransformExecutionError> {
        match (self, value) {
            (Self::Float64(values), ParsedValue::Numeric(numeric)) => {
                values.push(materialized_float64(numeric, column_name, endianness)?);
                Ok(())
            }
            (Self::Date32(values), ParsedValue::Numeric(numeric)) => {
                values.push(materialized_date32(numeric, column_name, endianness)?);
                Ok(())
            }
            (Self::Time64Microsecond(values), ParsedValue::Numeric(numeric)) => {
                values.push(materialized_time64_micros(numeric, column_name, endianness)?);
                Ok(())
            }
            (Self::TimestampMicrosecond(values), ParsedValue::Numeric(numeric)) => {
                values.push(materialized_timestamp_micros(numeric, column_name, endianness)?);
                Ok(())
            }
            (Self::DurationMicrosecond(values), ParsedValue::Numeric(numeric)) => {
                values.push(materialized_duration_micros(numeric, column_name, endianness)?);
                Ok(())
            }
            (Self::Utf8(values), ParsedValue::String(value)) => {
                values.push(Some(value.clone()));
                Ok(())
            }
            (Self::Utf8(values), ParsedValue::Numeric(numeric)) => {
                let (_, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
                values.push(missing_tag.map(|tag| tag.code().to_string()));
                Ok(())
            }
            (_, ParsedValue::String(_)) => Err(TransformExecutionError::new(format!(
                "column {column_name} expected a numeric value"
            ))),
        }
    }

    fn extend(&mut self, other: Self) {
        match (self, other) {
            (Self::Float64(values), Self::Float64(mut other_values)) => {
                values.append(&mut other_values)
            }
            (Self::Utf8(values), Self::Utf8(mut other_values)) => values.append(&mut other_values),
            (Self::Date32(values), Self::Date32(mut other_values)) => {
                values.append(&mut other_values)
            }
            (Self::Time64Microsecond(values), Self::Time64Microsecond(mut other_values)) => {
                values.append(&mut other_values)
            }
            (Self::TimestampMicrosecond(values), Self::TimestampMicrosecond(mut other_values)) => {
                values.append(&mut other_values)
            }
            (Self::DurationMicrosecond(values), Self::DurationMicrosecond(mut other_values)) => {
                values.append(&mut other_values)
            }
            _ => unreachable!("typed chunk columns always follow the execution schema"),
        }
    }
}

const SAS_EPOCH_DAYS_TO_UNIX_EPOCH: i32 = 3_653;
const MICROS_PER_SECOND: f64 = 1_000_000.0;
const SECONDS_PER_DAY: f64 = 86_400.0;

fn materialized_float64(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<Option<f64>, TransformExecutionError> {
    let (value, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
    Ok(if missing_tag.is_some() {
        None
    } else {
        Some(value)
    })
}

fn materialized_date32(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<Option<i32>, TransformExecutionError> {
    let (value, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
    if missing_tag.is_some() {
        return Ok(None);
    }
    let whole_days = expect_whole_number(value, column_name)?;
    Ok(Some(
        (whole_days as i64 - SAS_EPOCH_DAYS_TO_UNIX_EPOCH as i64) as i32,
    ))
}

fn materialized_time64_micros(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<Option<i64>, TransformExecutionError> {
    let (value, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
    if missing_tag.is_some() {
        return Ok(None);
    }
    Ok(Some((value * MICROS_PER_SECOND).round() as i64))
}

fn materialized_timestamp_micros(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<Option<i64>, TransformExecutionError> {
    let (value, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
    if missing_tag.is_some() {
        return Ok(None);
    }
    Ok(Some(
        ((value - SAS_EPOCH_DAYS_TO_UNIX_EPOCH as f64 * SECONDS_PER_DAY) * MICROS_PER_SECOND)
            .round() as i64,
    ))
}

fn materialized_duration_micros(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<Option<i64>, TransformExecutionError> {
    let (value, missing_tag) = materialized_numeric_parts(numeric, column_name, endianness)?;
    if missing_tag.is_some() {
        return Ok(None);
    }
    Ok(Some((value * MICROS_PER_SECOND).round() as i64))
}

fn materialized_numeric_parts(
    numeric: &NumericValue,
    column_name: &str,
    endianness: Endianness,
) -> Result<(f64, Option<SasMissingTag>), TransformExecutionError> {
    match numeric {
        NumericValue::Float64 {
            value, missing_tag, ..
        } => Ok((*value, *missing_tag)),
        NumericValue::DeferredBytes {
            width_bytes,
            raw_bytes,
        } => decode_deferred_numeric(raw_bytes, *width_bytes, endianness, column_name),
    }
}

fn decode_deferred_numeric(
    raw_bytes: &[u8],
    width_bytes: usize,
    endianness: Endianness,
    column_name: &str,
) -> Result<(f64, Option<SasMissingTag>), TransformExecutionError> {
    if raw_bytes.len() != width_bytes || !(1..=7).contains(&width_bytes) {
        return Err(TransformExecutionError::new(format!(
            "column {column_name} has an invalid deferred numeric width of {width_bytes} bytes"
        )));
    }

    let mut raw_bits = 0_u64;
    match endianness {
        Endianness::Little => {
            for byte in raw_bytes.iter().rev() {
                raw_bits = (raw_bits << 8) | u64::from(*byte);
            }
        }
        Endianness::Big => {
            for byte in raw_bytes {
                raw_bits = (raw_bits << 8) | u64::from(*byte);
            }
        }
    }
    raw_bits <<= (8 - width_bytes) * 8;

    let value = f64::from_bits(raw_bits);
    Ok((value, decode_materialized_missing_tag(value, raw_bits)))
}

fn decode_materialized_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {
    if !value.is_nan() {
        return None;
    }

    let tag = !((raw_bits >> 40) & 0xFF) as u8;
    match tag {
        0 => Some(SasMissingTag::Underscore),
        2..=27 => Some(SasMissingTag::Letter((b'A' + (tag - 2)) as char)),
        b'_' => Some(SasMissingTag::Underscore),
        b'A'..=b'Z' => Some(SasMissingTag::Letter(tag as char)),
        _ => Some(SasMissingTag::Dot),
    }
}

fn expect_whole_number(value: f64, column_name: &str) -> Result<i32, TransformExecutionError> {
    let rounded = value.round();
    if (value - rounded).abs() > 1e-9 {
        return Err(TransformExecutionError::new(format!(
            "column {column_name} requires whole-number day values for date materialization"
        )));
    }
    Ok(rounded as i32)
}

#[derive(Debug, Clone, PartialEq)]
struct FilterPredicate {
    source_index: usize,
    column_name: String,
    operator: FilterOperator,
    literal: FilterLiteral,
    source_endianness: Endianness,
}

impl FilterPredicate {
    fn parse(expression: &str, metadata: &SasMetadata) -> Result<Self, TransformExecutionError> {
        let tokens = expression.split_whitespace().collect::<Vec<_>>();
        if tokens.len() != 3 {
            return Err(TransformExecutionError::new(format!(
                "unsupported filter expression: {expression}"
            )));
        }

        let column_name = tokens[0];
        let operator = FilterOperator::parse(tokens[1], expression)?;
        let (source_index, column) = metadata
            .columns
            .iter()
            .enumerate()
            .find(|(_, column)| column.name == column_name)
            .ok_or_else(|| {
                TransformExecutionError::new(format!("unknown filter column: {column_name}"))
            })?;
        let literal = FilterLiteral::parse(tokens[2], &column.kind, column_name, &operator)?;

        Ok(Self {
            source_index,
            column_name: column_name.to_string(),
            operator,
            literal,
            source_endianness: metadata.subset.endianness,
        })
    }

    fn matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {
        let value = row.values.get(self.source_index).ok_or_else(|| {
            TransformExecutionError::new(format!(
                "filter column {} is out of bounds for the parsed row",
                self.column_name
            ))
        })?;
        match (&self.literal, value) {
            (FilterLiteral::Numeric(expected), ParsedValue::Numeric(numeric)) => {
                let (actual, _) = materialized_numeric_parts(
                    numeric,
                    &self.column_name,
                    self.source_endianness,
                )?;
                Ok(self.operator.apply_numeric(actual, *expected))
            }
            (FilterLiteral::Utf8(expected), ParsedValue::String(actual)) => self
                .operator
                .apply_string(actual, expected, &self.column_name),
            (FilterLiteral::Numeric(_), ParsedValue::String(_)) => {
                Err(TransformExecutionError::new(format!(
                    "filter column {} resolved to a string unexpectedly",
                    self.column_name
                )))
            }
            (FilterLiteral::Utf8(_), ParsedValue::Numeric(_)) => {
                Err(TransformExecutionError::new(format!(
                    "filter column {} resolved to a numeric unexpectedly",
                    self.column_name
                )))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl FilterOperator {
    fn parse(token: &str, expression: &str) -> Result<Self, TransformExecutionError> {
        match token {
            "=" | "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            ">" => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterThanOrEqual),
            "<" => Ok(Self::LessThan),
            "<=" => Ok(Self::LessThanOrEqual),
            _ => Err(TransformExecutionError::new(format!(
                "unsupported filter expression: {expression}"
            ))),
        }
    }

    fn apply_numeric(&self, actual: f64, expected: f64) -> bool {
        match self {
            Self::Equal => actual == expected,
            Self::NotEqual => actual != expected,
            Self::GreaterThan => actual > expected,
            Self::GreaterThanOrEqual => actual >= expected,
            Self::LessThan => actual < expected,
            Self::LessThanOrEqual => actual <= expected,
        }
    }

    fn apply_string(
        &self,
        actual: &str,
        expected: &str,
        column_name: &str,
    ) -> Result<bool, TransformExecutionError> {
        match self {
            Self::Equal => Ok(actual == expected),
            Self::NotEqual => Ok(actual != expected),
            Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => Err(TransformExecutionError::new(format!(
                "string filters only support = and != for column {column_name}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FilterLiteral {
    Numeric(f64),
    Utf8(String),
}

impl FilterLiteral {
    fn parse(
        token: &str,
        column_kind: &ColumnKind,
        column_name: &str,
        operator: &FilterOperator,
    ) -> Result<Self, TransformExecutionError> {
        match column_kind {
            ColumnKind::Numeric => token.parse::<f64>().map(Self::Numeric).map_err(|_| {
                TransformExecutionError::new(format!(
                    "filter literal {token} is not a valid numeric value for column {column_name}"
                ))
            }),
            ColumnKind::String => {
                if matches!(
                    operator,
                    FilterOperator::GreaterThan
                        | FilterOperator::GreaterThanOrEqual
                        | FilterOperator::LessThan
                        | FilterOperator::LessThanOrEqual
                ) {
                    return Err(TransformExecutionError::new(format!(
                        "string filters only support = and != for column {column_name}"
                    )));
                }
                Ok(Self::Utf8(strip_quotes(token).to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformExecutionError {
    message: String,
}

impl TransformExecutionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TransformExecutionError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TransformExecutionError {}

#[derive(Debug, Clone, PartialEq)]
struct ExecutedBatch {
    batch: TypedBatch,
    threads_used: usize,
}

fn default_worker_threads() -> usize {
    std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
}

fn transform_thread_pool(
    worker_threads: usize,
) -> Result<Arc<ThreadPool>, TransformExecutionError> {
    static TRANSFORM_THREAD_POOLS: OnceLock<Mutex<HashMap<usize, Arc<ThreadPool>>>> =
        OnceLock::new();
    let cache = TRANSFORM_THREAD_POOLS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut pools = cache
        .lock()
        .map_err(|_| TransformExecutionError::new("transform thread pool cache is unavailable"))?;
    if let Some(pool) = pools.get(&worker_threads) {
        return Ok(pool.clone());
    }

    let pool = Arc::new(
        ThreadPoolBuilder::new()
            .num_threads(worker_threads)
            .build()
            .map_err(|error| TransformExecutionError::new(error.to_string()))?,
    );
    pools.insert(worker_threads, pool.clone());
    Ok(pool)
}

#[derive(Debug, Default)]
pub struct StubParquetSink;

impl ParquetSink for StubParquetSink {
    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {
        Ok(ParquetSinkReport::skeleton(plan))
    }
}

impl StreamingParquetSink for StubParquetSink {
    fn stage_batches(
        &self,
        plan: ParquetSinkPlan,
        execution: &TransformExecution,
        dataset: &mut ParsedSas7bdat,
    ) -> Result<ParquetSinkReport, ParquetSinkError> {
        let mut staged_row_count = 0;
        let mut staged_batch_count = 0;
        let mut parallel_batch_count = 0;
        let mut transform_threads_used = 1;

        while let Some(batch) = dataset
            .next_batch(plan.row_group_rows)
            .map_err(|error| ParquetSinkError::new(error.to_string()))?
        {
            let executed = execution
                .apply(batch)
                .map_err(|error| ParquetSinkError::new(error.to_string()))?;
            if executed.threads_used > 1 {
                parallel_batch_count += 1;
                transform_threads_used = transform_threads_used.max(executed.threads_used);
            }
            let batch = executed.batch;
            if batch.row_count == 0 {
                continue;
            }
            staged_row_count += batch.row_count;
            staged_batch_count += 1;
        }

        Ok(ParquetSinkReport::decoded_rows_staged(
            plan,
            staged_row_count,
            staged_batch_count,
            parallel_batch_count,
            transform_threads_used,
        ))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LocalParquetSink;

impl ParquetSink for LocalParquetSink {
    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {
        Ok(ParquetSinkReport::skeleton(plan))
    }
}

impl StreamingParquetSink for LocalParquetSink {
    #[allow(clippy::collapsible_if)]
    fn stage_batches(
        &self,
        plan: ParquetSinkPlan,
        execution: &TransformExecution,
        dataset: &mut ParsedSas7bdat,
    ) -> Result<ParquetSinkReport, ParquetSinkError> {
        match plan.output_path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                fs::create_dir_all(parent)
                    .map_err(|error| ParquetSinkError::new(error.to_string()))?;
            }
            _ => {}
        }

        let schema = Arc::new(build_arrow_schema(execution, &dataset.metadata));
        let file = File::create(&plan.output_path)
            .map_err(|error| ParquetSinkError::new(error.to_string()))?;
        let properties = WriterProperties::builder()
            .set_max_row_group_row_count(Some(plan.row_group_rows))
            .build();
        let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(properties))
            .map_err(|error| ParquetSinkError::new(error.to_string()))?;
        let mut staged_row_count = 0;
        let mut staged_batch_count = 0;
        let mut parallel_batch_count = 0;
        let mut transform_threads_used = 1;

        while let Some(batch) = dataset
            .next_batch(plan.row_group_rows)
            .map_err(|error| ParquetSinkError::new(error.to_string()))?
        {
            let executed = execution
                .apply(batch)
                .map_err(|error| ParquetSinkError::new(error.to_string()))?;
            if executed.threads_used > 1 {
                parallel_batch_count += 1;
                transform_threads_used = transform_threads_used.max(executed.threads_used);
            }
            let batch = executed.batch;
            if batch.row_count == 0 {
                continue;
            }
            let row_count = batch.row_count;
            let record_batch = typed_batch_to_record_batch(batch, schema.clone());
            writer
                .write(&record_batch)
                .map_err(|error| ParquetSinkError::new(error.to_string()))?;
            staged_row_count += row_count;
            staged_batch_count += 1;
        }

        writer
            .close()
            .map_err(|error| ParquetSinkError::new(error.to_string()))?;
        let output_size_bytes = fs::metadata(&plan.output_path)
            .map_err(|error| ParquetSinkError::new(error.to_string()))?
            .len();
        Ok(ParquetSinkReport::parquet_written(
            plan,
            staged_row_count,
            staged_batch_count,
            output_size_bytes,
            parallel_batch_count,
            transform_threads_used,
        ))
    }
}

fn build_arrow_schema(execution: &TransformExecution, metadata: &SasMetadata) -> Schema {
    let fields = execution
        .projected_columns
        .iter()
        .map(|column| {
            Field::new(
                column.name.clone(),
                column.kind.data_type(),
                column.nullable,
            )
            .with_metadata(column.metadata.clone())
        })
        .collect::<Vec<_>>();
    Schema::new_with_metadata(
        fields,
        HashMap::from([
            ("sas.table_name".to_string(), metadata.table_name.clone()),
            ("sas.file_label".to_string(), metadata.file_label.clone()),
            (
                "sas.subset_name".to_string(),
                metadata.subset.name.to_string(),
            ),
        ]),
    )
}

fn typed_batch_to_record_batch(batch: TypedBatch, schema: Arc<Schema>) -> RecordBatch {
    let arrays = batch
        .columns
        .into_iter()
        .map(|column| match column {
            TypedColumn::Float64(values) => Arc::new(Float64Array::from(values)) as ArrayRef,
            TypedColumn::Utf8(values) => Arc::new(StringArray::from(values)) as ArrayRef,
            TypedColumn::Date32(values) => Arc::new(Date32Array::from(values)) as ArrayRef,
            TypedColumn::Time64Microsecond(values) => {
                Arc::new(Time64MicrosecondArray::from(values)) as ArrayRef
            }
            TypedColumn::TimestampMicrosecond(values) => {
                Arc::new(TimestampMicrosecondArray::from(values)) as ArrayRef
            }
            TypedColumn::DurationMicrosecond(values) => {
                Arc::new(DurationMicrosecondArray::from(values)) as ArrayRef
            }
        })
        .collect::<Vec<_>>();
    RecordBatch::try_new(schema, arrays)
        .expect("typed parquet batches should always match the derived schema")
}

fn strip_quotes(token: &str) -> &str {
    let bytes = token.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == 34 || first == 39) && first == last {
            return &token[1..token.len() - 1];
        }
    }
    token
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::SUPPORTED_SUBSET;
    use crate::transform::contracts::{
        DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
        SourceFormat, TransformContract, TransformRequest, TransformTuning,
    };

    #[test]
    fn arrow_schema_preserves_parser_metadata_including_informats() {
        let metadata = SasMetadata {
            subset: SUPPORTED_SUBSET,
            table_name: "DATASET".to_string(),
            file_label: "labelled dataset".to_string(),
            row_count: 1,
            row_length: 8,
            page_size: 4096,
            page_count: 1,
            columns: vec![SasColumn {
                name: "event_dt".to_string(),
                kind: ColumnKind::Numeric,
                offset: 0,
                width: 8,
                semantic_type: SemanticTypeHint::DateTime,
                metadata: crate::parser::contracts::ColumnMetadata {
                    label: Some("event timestamp".to_string()),
                    format_name: Some("DATETIME".to_string()),
                    informat_name: Some("ANYDTDTM".to_string()),
                },
            }],
        };
        let request = TransformRequest {
            source: SourceContract {
                path: "fixtures/example.sas7bdat".into(),
                format: SourceFormat::Sas7bdat,
            },
            decoder: DecoderContract {
                mode: DecodeMode::StreamingPages,
            },
            transform: TransformContract {
                selection: vec!["event_dt".to_string()],
                filter: None,
                execution: ExecutionModel::Streaming,
                tuning: TransformTuning {
                    batch_size_rows: 64,
                    worker_threads: Some(1),
                },
            },
            sink: SinkContract {
                path: "fixtures/example.parquet".into(),
                format: SinkFormat::Parquet,
            },
        };

        let execution = TransformExecution::from_request(&request, &metadata)
            .expect("metadata-bearing schema planning should succeed");
        let schema = build_arrow_schema(&execution, &metadata);
        let field = schema
            .field_with_name("event_dt")
            .expect("field should exist");
        let missing_tag_field = schema
            .field_with_name("event_dt__sas_missing_tag")
            .expect("missing-tag sidecar should exist");

        assert_eq!(
            field.metadata().get("sas.label"),
            Some(&"event timestamp".to_string())
        );
        assert_eq!(
            field.metadata().get("sas.format_name"),
            Some(&"DATETIME".to_string())
        );
        assert_eq!(
            field.metadata().get("sas.informat_name"),
            Some(&"ANYDTDTM".to_string())
        );
        assert_eq!(
            field.metadata().get("sas.semantic_type"),
            Some(&"datetime".to_string())
        );
        assert_eq!(
            field.metadata().get("sas.missing_tag_column"),
            Some(&"event_dt__sas_missing_tag".to_string())
        );
        assert_eq!(
            missing_tag_field.metadata().get("sas.parent_column"),
            Some(&"event_dt".to_string())
        );
        assert_eq!(
            schema.metadata().get("sas.table_name"),
            Some(&"DATASET".to_string())
        );
    }
}
