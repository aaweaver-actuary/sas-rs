//! Default CLI runner backed by the filesystem parser and local sink.

/// Run the default filesystem-backed CLI flow from process-style arguments.
///
/// This helper is the library equivalent of invoking the `sasrs` binary. It
/// wires together the standard source loader, parser, and local Parquet sink.
///
/// # Examples
///
/// ```
/// use std::process::ExitCode;
/// use sas_rs::cli::{CliError, run};
///
/// let error = run(["sasrs", "--help"]).unwrap_err();
///
/// assert!(matches!(error, CliError::Parse(_)));
/// assert_eq!(error.exit_code(), ExitCode::from(1));
/// ```
use std::ffi::OsString;

use crate::parser::SupportedSas7bdatParser;
use crate::transform::pipeline::{FileSystemSourceLoader, ParserTransformService};
use crate::transform::sink::LocalParquetSink;

use super::{CliError, CommandOutcome, run_with_service};

/// Run the default CLI flow with the filesystem-backed parser and parquet sink.
pub fn run<I, T>(args: I) -> Result<CommandOutcome, CliError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let service = ParserTransformService::new(
        FileSystemSourceLoader,
        SupportedSas7bdatParser,
        LocalParquetSink,
    );
    run_with_service(args, &service)
}
