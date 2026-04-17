use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use arrow_array::{ArrayRef, Float64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::parser::contracts::{
    ColumnKind, ParsedRow, ParsedSas7bdat, ParsedValue, RowBatch, SasMetadata,
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
                .map(|(index, column)| ProjectionColumn::new(index, column))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            let mut projected_columns = Vec::with_capacity(request.transform.selection.len());
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
                projected_columns.push(ProjectionColumn::new(index, column)?);
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
                typed.push(value, &projected.name)?;
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
}

impl ProjectionColumn {
    fn new(
        source_index: usize,
        column: &crate::parser::contracts::SasColumn,
    ) -> Result<Self, TransformExecutionError> {
        let kind = match column.kind {
            ColumnKind::Numeric64 => ProjectionKind::Float64,
            ColumnKind::String => ProjectionKind::Utf8,
        };
        Ok(Self {
            source_index,
            name: column.name.clone(),
            kind,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectionKind {
    Float64,
    Utf8,
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
    Float64(Vec<f64>),
    Utf8(Vec<String>),
}

impl TypedColumn {
    fn with_capacity(kind: ProjectionKind, capacity: usize) -> Self {
        match kind {
            ProjectionKind::Float64 => Self::Float64(Vec::with_capacity(capacity)),
            ProjectionKind::Utf8 => Self::Utf8(Vec::with_capacity(capacity)),
        }
    }

    fn push(
        &mut self,
        value: &ParsedValue,
        column_name: &str,
    ) -> Result<(), TransformExecutionError> {
        match (self, value) {
            (Self::Float64(values), ParsedValue::Numeric(value)) => {
                values.push(*value);
                Ok(())
            }
            (Self::Utf8(values), ParsedValue::String(value)) => {
                values.push(value.clone());
                Ok(())
            }
            (Self::Float64(_), ParsedValue::String(_)) => Err(TransformExecutionError::new(
                format!("column {column_name} expected a numeric value"),
            )),
            (Self::Utf8(_), ParsedValue::Numeric(_)) => Err(TransformExecutionError::new(format!(
                "column {column_name} expected a string value"
            ))),
        }
    }

    fn extend(&mut self, other: Self) {
        match (self, other) {
            (Self::Float64(values), Self::Float64(mut other_values)) => {
                values.append(&mut other_values)
            }
            (Self::Utf8(values), Self::Utf8(mut other_values)) => values.append(&mut other_values),
            (Self::Float64(_), Self::Utf8(_)) | (Self::Utf8(_), Self::Float64(_)) => {
                unreachable!("typed chunk columns always follow the execution schema")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FilterPredicate {
    source_index: usize,
    column_name: String,
    operator: FilterOperator,
    literal: FilterLiteral,
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
            (FilterLiteral::Numeric(expected), ParsedValue::Numeric(actual)) => {
                Ok(self.operator.apply_numeric(*actual, *expected))
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
            ColumnKind::Numeric64 => token.parse::<f64>().map(Self::Numeric).map_err(|_| {
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

        let schema = Arc::new(build_arrow_schema(execution));
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

fn build_arrow_schema(execution: &TransformExecution) -> Schema {
    let fields = execution
        .projected_columns
        .iter()
        .map(|column| {
            let data_type = match column.kind {
                ProjectionKind::Float64 => DataType::Float64,
                ProjectionKind::Utf8 => DataType::Utf8,
            };
            Field::new(column.name.clone(), data_type, false)
        })
        .collect::<Vec<_>>();
    Schema::new(fields)
}

fn typed_batch_to_record_batch(batch: TypedBatch, schema: Arc<Schema>) -> RecordBatch {
    let arrays = batch
        .columns
        .into_iter()
        .map(|column| match column {
            TypedColumn::Float64(values) => Arc::new(Float64Array::from(values)) as ArrayRef,
            TypedColumn::Utf8(values) => Arc::new(StringArray::from(values)) as ArrayRef,
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
