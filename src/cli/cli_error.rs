//! Public CLI error type and exit-code mapping.

/// Error returned by the public CLI entrypoints.
///
/// `CliError` separates argument-parsing failures from downstream transform
/// failures so callers can preserve the same exit semantics as the binary.
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::process::ExitCode;

use crate::transform::pipeline::TransformServiceError;

/// Public error returned by CLI entrypoints.
#[derive(Debug)]
pub enum CliError {
    /// Clap rejected the supplied command-line arguments.
    Parse(clap::Error),
    /// The transform service rejected an otherwise valid request.
    Transform(TransformServiceError),
}

impl CliError {
    /// Return the process exit code that `sasrs` would use for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::process::ExitCode;
    /// use sas_rs::cli::{CliError, run};
    ///
    /// let error = run(["sasrs", "--definitely-not-a-real-flag"]).unwrap_err();
    ///
    /// assert!(matches!(error, CliError::Parse(_)));
    /// assert_ne!(error.exit_code(), ExitCode::SUCCESS);
    /// ```
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Parse(error) => ExitCode::from(error.exit_code().clamp(1, 255) as u8),
            Self::Transform(_) => ExitCode::from(1),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => error.fmt(formatter),
            Self::Transform(error) => error.fmt(formatter),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Transform(error) => Some(error),
        }
    }
}
