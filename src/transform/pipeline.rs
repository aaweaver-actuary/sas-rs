//! Public transform orchestration traits, reports, and default services.

/// Re-export of the filesystem-backed source loader type.
pub mod file_system_source_loader;
/// Re-export of the parser-stage execution report.
pub mod parser_execution_report;
/// Re-export of the parser-backed transform service.
pub mod parser_transform_service;
/// Re-export of the source loader abstraction.
pub mod source_data_loader;
/// Re-export of source loader failures.
pub mod source_data_loader_error;
/// Re-export of the stub transform service.
pub mod stub_transform_service;
/// Re-export of the end-to-end transform report.
pub mod transform_report;
/// Re-export of the transform service abstraction.
pub mod transform_service;
/// Re-export of transform service failures.
pub mod transform_service_error;
/// Re-export of the top-level transform status enum.
pub mod transform_status;

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

/// Executes a transform request against a configured parser and sink.
pub trait TransformService {
    /// Run the requested transform and return the resulting execution report.
    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError>;
}

/// Opens parser input streams for a declared transform source.
pub trait SourceDataLoader {
    /// Open the source as a boxed parser data source.
    fn open(&self, source: &SourceContract)
    -> Result<BoxedParserDataSource, SourceDataLoaderError>;
}

/// Loads transform sources from the local filesystem.
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

/// Error raised when a transform source cannot be opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDataLoaderError {
    message: String,
}

impl SourceDataLoaderError {
    /// Construct a source-loader error from a human-readable message.
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

/// Summary of the parser work completed for a transform request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserExecutionReport {
    /// Name of the parser subset that decoded the input.
    pub subset_name: String,
    /// Number of rows observed in the parsed dataset metadata.
    pub row_count: usize,
    /// Number of output columns after selection expansion.
    pub column_count: usize,
    /// Whether a column selection changed the default projection.
    pub selection_applied: bool,
    /// Whether a row filter changed the default row stream.
    pub filter_applied: bool,
}

impl ParserExecutionReport {
    /// Return a placeholder report for stubbed transform flows.
    pub fn deferred() -> Self {
        Self {
            subset_name: "deferred".to_string(),
            row_count: 0,
            column_count: 0,
            selection_applied: false,
            filter_applied: false,
        }
    }

    /// Build a parser execution report from a parsed dataset and execution plan.
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

/// End-to-end report for a transform request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformReport {
    /// Original transform request that produced this report.
    pub request: TransformRequest,
    /// Parser-stage summary for the request.
    pub parser: ParserExecutionReport,
    /// Sink-stage summary for the request.
    pub sink: ParquetSinkReport,
    /// Final transform lifecycle status.
    pub status: TransformStatus,
}

impl TransformReport {
    /// Build a report for a request whose execution remains intentionally stubbed.
    pub fn not_yet_implemented(request: TransformRequest) -> Self {
        let sink = ParquetSinkReport::skeleton(ParquetSinkPlan::from_request(&request));
        Self::with_sink(request, sink)
    }

    /// Build a report around an already-computed sink report.
    pub fn with_sink(request: TransformRequest, sink: ParquetSinkReport) -> Self {
        Self {
            request,
            parser: ParserExecutionReport::deferred(),
            sink,
            status: TransformStatus::NotYetImplemented,
        }
    }

    /// Build a report for a request whose rows were decoded and staged.
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

    /// Build a report for a request that produced a parquet file.
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

    /// Render the report as a single-line summary suitable for CLI output.
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

/// High-level lifecycle status for a transform request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformStatus {
    /// The request resolved only to a skeleton execution plan.
    NotYetImplemented,
    /// Rows were decoded and staged but no final parquet file was written.
    DecodedRowsStaged,
    /// A parquet file was written for the request.
    ParquetWritten,
}

impl TransformStatus {
    /// Return the stable machine-readable label for this status.
    pub fn label(&self) -> &str {
        match self {
            Self::NotYetImplemented => "not-yet-implemented",
            Self::DecodedRowsStaged => "decoded-rows-staged",
            Self::ParquetWritten => "parquet-written",
        }
    }
}

/// Error raised by a transform service implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformServiceError {
    message: String,
}

impl TransformServiceError {
    /// Construct a transform-service error from a human-readable message.
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

/// Stub service that only prepares sink plans without running the parser.
#[derive(Debug, Default)]
pub struct StubTransformService<S = StubParquetSink> {
    sink: S,
}

impl<S> StubTransformService<S> {
    /// Build a stub service around the provided sink implementation.
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

/// Default transform service that wires a source loader, parser, and sink together.
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
    /// Build a transform service from explicit loader, parser, and sink dependencies.
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
