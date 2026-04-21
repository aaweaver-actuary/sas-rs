//! Injectable CLI runner used by tests and embedding callers.
//!
//! The parent [`crate::cli`] module re-exports [`run_with_service`] from this
//! leaf module so the public path stays stable after the one-export split.

use std::ffi::OsString;

use clap::Parser;

use crate::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use crate::transform::pipeline::TransformService;

use super::{Cli, CliError, Command, CommandOutcome, TransformArgs};

impl TransformArgs {
    fn into_request(self) -> TransformRequest {
        let execution = match self.max_rows_in_memory {
            Some(max_rows_in_memory) => ExecutionModel::BoundedMemory { max_rows_in_memory },
            None => ExecutionModel::Streaming,
        };

        TransformRequest {
            source: SourceContract {
                path: self.input,
                format: SourceFormat::Sas7bdat,
            },
            decoder: DecoderContract {
                mode: DecodeMode::StreamingPages,
            },
            transform: TransformContract {
                selection: self.select,
                filter: self.filter,
                execution,
                tuning: TransformTuning {
                    batch_size_rows: self.batch_size_rows,
                    worker_threads: self.worker_threads,
                },
            },
            sink: SinkContract {
                path: self.output,
                format: SinkFormat::Parquet,
            },
        }
    }
}

/// Run the CLI against a caller-provided transform service.
///
/// This is the main seam used by tests and embedding callers that want to keep
/// CLI parsing but replace the downstream execution strategy.
///
/// # Examples
///
/// ```
/// use sas_rs::cli::run_with_service;
/// use sas_rs::transform::pipeline::{StubTransformService, TransformStatus};
/// use sas_rs::transform::sink::StubParquetSink;
///
/// let service = StubTransformService::new(StubParquetSink);
/// let outcome = run_with_service(
///     ["sasrs", "transform", "input.sas7bdat", "output.parquet"],
///     &service,
/// )
/// .expect("stub service should accept a syntactically valid request");
///
/// match outcome {
///     sas_rs::cli::CommandOutcome::Transform(report) => {
///         assert_eq!(report.status, TransformStatus::NotYetImplemented);
///     }
/// }
/// ```
pub fn run_with_service<I, T, S>(args: I, service: &S) -> Result<CommandOutcome, CliError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    S: TransformService,
{
    let cli = Cli::try_parse_from(args).map_err(CliError::Parse)?;

    match cli.command {
        Command::Transform(command) => service
            .run(command.into_request())
            .map(CommandOutcome::Transform)
            .map_err(CliError::Transform),
    }
}
