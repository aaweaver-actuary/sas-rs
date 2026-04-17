use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;

use crate::parser::{
    BoxedParserDataSource, ParserError, ParserInput, Sas7bdatParser, SupportedSas7bdatParser,
};

use super::contracts::{SourceContract, TransformRequest};
use super::sink::{
    ParquetSink, ParquetSinkError, ParquetSinkPlan, ParquetSinkReport, ParquetSinkStatus,
    StreamingParquetSink, StubParquetSink, TransformExecution, TransformExecutionError,
};

pub trait TransformService {
    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError>;
}

pub trait SourceDataLoader {
    fn open(&self, source: &SourceContract)
    -> Result<BoxedParserDataSource, SourceDataLoaderError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemSourceLoader;

impl SourceDataLoader for FileSystemSourceLoader {
    fn open(
        &self,
        source: &SourceContract,
    ) -> Result<BoxedParserDataSource, SourceDataLoaderError> {
        File::open(&source.path)
            .map(|file| Box::new(file) as BoxedParserDataSource)
            .map_err(SourceDataLoaderError::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDataLoaderError {
    message: String,
}

impl SourceDataLoaderError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for SourceDataLoaderError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for SourceDataLoaderError {}

impl From<std::io::Error> for SourceDataLoaderError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserExecutionReport {
    pub subset_name: String,
    pub row_count: usize,
    pub column_count: usize,
    pub selection_applied: bool,
    pub filter_applied: bool,
}

impl ParserExecutionReport {
    pub fn deferred() -> Self {
        Self {
            subset_name: "deferred".to_string(),
            row_count: 0,
            column_count: 0,
            selection_applied: false,
            filter_applied: false,
        }
    }

    pub fn from_execution(
        dataset: &crate::parser::contracts::ParsedSas7bdat,
        execution: &TransformExecution,
    ) -> Self {
        Self {
            subset_name: dataset.metadata.subset.name.to_string(),
            row_count: dataset.metadata.row_count,
            column_count: execution.output_column_count(),
            selection_applied: execution.selection_applied(),
            filter_applied: execution.filter_applied(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformReport {
    pub request: TransformRequest,
    pub parser: ParserExecutionReport,
    pub sink: ParquetSinkReport,
    pub status: TransformStatus,
}

impl TransformReport {
    pub fn not_yet_implemented(request: TransformRequest) -> Self {
        let sink = ParquetSinkReport::skeleton(ParquetSinkPlan::from_request(&request));
        Self::with_sink(request, sink)
    }

    pub fn with_sink(request: TransformRequest, sink: ParquetSinkReport) -> Self {
        Self {
            request,
            parser: ParserExecutionReport::deferred(),
            sink,
            status: TransformStatus::NotYetImplemented,
        }
    }

    pub fn decoded_rows_staged(
        request: TransformRequest,
        parser: ParserExecutionReport,
        sink: ParquetSinkReport,
    ) -> Self {
        Self {
            request,
            parser,
            sink,
            status: TransformStatus::DecodedRowsStaged,
        }
    }

    pub fn parquet_written(
        request: TransformRequest,
        parser: ParserExecutionReport,
        sink: ParquetSinkReport,
    ) -> Self {
        Self {
            request,
            parser,
            sink,
            status: TransformStatus::ParquetWritten,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "status={} input={} output={} execution={} batch_size_rows={} worker_threads={} parser_subset={} parsed_rows={} parsed_columns={} selection_applied={} filter_applied={} sink_status={} row_group_rows={} staged_rows={} staged_batches={} parallel_batches={} transform_threads_used={} output_size_bytes={}",
            self.status.label(),
            self.request.source.path.display(),
            self.request.sink.path.display(),
            self.request.transform.execution.label(),
            self.request.transform.tuning.batch_size_rows,
            self.request
                .transform
                .tuning
                .worker_threads
                .map(|value| value.to_string())
                .unwrap_or_else(|| "auto".to_string()),
            self.parser.subset_name,
            self.parser.row_count,
            self.parser.column_count,
            bool_label(self.parser.selection_applied),
            bool_label(self.parser.filter_applied),
            self.sink.status.label(),
            self.sink.plan.row_group_rows,
            self.sink.staged_row_count,
            self.sink.staged_batch_count,
            self.sink.parallel_batch_count,
            self.sink.transform_threads_used,
            self.sink.output_size_bytes,
        )
    }
}

impl Display for TransformReport {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&self.summary())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformStatus {
    NotYetImplemented,
    DecodedRowsStaged,
    ParquetWritten,
}

impl TransformStatus {
    pub fn label(&self) -> &str {
        match self {
            Self::NotYetImplemented => "not-yet-implemented",
            Self::DecodedRowsStaged => "decoded-rows-staged",
            Self::ParquetWritten => "parquet-written",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformServiceError {
    message: String,
}

impl TransformServiceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TransformServiceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TransformServiceError {}

impl From<ParquetSinkError> for TransformServiceError {
    fn from(error: ParquetSinkError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<SourceDataLoaderError> for TransformServiceError {
    fn from(error: SourceDataLoaderError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<ParserError> for TransformServiceError {
    fn from(error: ParserError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<TransformExecutionError> for TransformServiceError {
    fn from(error: TransformExecutionError) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Default)]
pub struct StubTransformService<S = StubParquetSink> {
    sink: S,
}

impl<S> StubTransformService<S> {
    pub fn new(sink: S) -> Self {
        Self { sink }
    }
}

impl<S> TransformService for StubTransformService<S>
where
    S: ParquetSink,
{
    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {
        let sink = self.sink.prepare(ParquetSinkPlan::from_request(&request))?;
        Ok(TransformReport::with_sink(request, sink))
    }
}

#[derive(Debug, Default)]
pub struct ParserTransformService<
    L = FileSystemSourceLoader,
    P = SupportedSas7bdatParser,
    S = StubParquetSink,
> {
    loader: L,
    parser: P,
    sink: S,
}

impl<L, P, S> ParserTransformService<L, P, S> {
    pub fn new(loader: L, parser: P, sink: S) -> Self {
        Self {
            loader,
            parser,
            sink,
        }
    }
}

impl<L, P, S> TransformService for ParserTransformService<L, P, S>
where
    L: SourceDataLoader,
    P: Sas7bdatParser,
    S: StreamingParquetSink,
{
    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {
        let source = self.loader.open(&request.source)?;
        let source_name = request.source.path.display().to_string();
        let mut dataset = self.parser.parse(ParserInput::new(&source_name, source))?;
        let execution = TransformExecution::from_request(&request, &dataset.metadata)?;
        let parser = ParserExecutionReport::from_execution(&dataset, &execution);
        let sink = self.sink.stage_batches(
            ParquetSinkPlan::from_request(&request),
            &execution,
            &mut dataset,
        )?;

        Ok(match sink.status {
            ParquetSinkStatus::ParquetWritten => {
                TransformReport::parquet_written(request, parser, sink)
            }
            ParquetSinkStatus::DecodedRowsStaged => {
                TransformReport::decoded_rows_staged(request, parser, sink)
            }
            ParquetSinkStatus::SkeletonReady => TransformReport::with_sink(request, sink),
        })
    }
}

fn bool_label(value: bool) -> String {
    if value {
        "applied".to_string()
    } else {
        "deferred".to_string()
    }
}
