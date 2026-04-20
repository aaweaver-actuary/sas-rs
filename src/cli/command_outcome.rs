//! Public command result enum returned by CLI entrypoints.

//! Public command result enum returned by CLI entrypoints.

/// Outcome produced by a public CLI command.
///
/// The enum exists so the CLI surface can grow without changing the signature
/// of [`crate::cli::run`] or [`crate::cli::run_with_service`].
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use sas_rs::cli::CommandOutcome;
/// use sas_rs::transform::contracts::{
///     DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
///     SourceFormat, TransformContract, TransformRequest, TransformTuning,
/// };
/// use sas_rs::transform::pipeline::TransformReport;
///
/// let request = TransformRequest {
///     source: SourceContract {
///         path: PathBuf::from("input.sas7bdat"),
///         format: SourceFormat::Sas7bdat,
///     },
///     decoder: DecoderContract {
///         mode: DecodeMode::StreamingPages,
///     },
///     transform: TransformContract {
///         selection: vec!["id".to_string()],
///         filter: None,
///         execution: ExecutionModel::Streaming,
///         tuning: TransformTuning {
///             batch_size_rows: 128,
///             worker_threads: None,
///         },
///     },
///     sink: SinkContract {
///         path: PathBuf::from("output.parquet"),
///         format: SinkFormat::Parquet,
///     },
/// };
///
/// let outcome = CommandOutcome::Transform(TransformReport::not_yet_implemented(request));
///
/// assert!(outcome.to_string().contains("status=not-yet-implemented"));
/// ```

use std::fmt::{self, Display, Formatter};

use crate::transform::pipeline::TransformReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOutcome {
    /// Result of running the `transform` subcommand.
    Transform(TransformReport),
}

impl Display for CommandOutcome {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transform(report) => report.fmt(formatter),
        }
    }
}
