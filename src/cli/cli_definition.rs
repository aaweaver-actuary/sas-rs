//! Top-level command-line contract for the `sasrs` binary.
//!
//! This leaf module exists so the public CLI type can keep focused docs while
//! the parent [`crate::cli`] module stays small.

use clap::Parser;

/// Parsed top-level CLI contract for the `sasrs` binary.
///
/// The CLI stays intentionally small: it exists to turn process arguments into
/// a strongly typed transform request.
///
/// # Examples
///
/// ```
/// use clap::Parser;
/// use sas_rs::cli::Cli;
///
/// let parsed = Cli::try_parse_from([
///     "sasrs",
///     "transform",
///     "input.sas7bdat",
///     "output.parquet",
/// ]);
///
/// assert!(parsed.is_ok());
/// ```
#[derive(Debug, Parser)]
#[command(name = "sasrs")]
#[command(about = "Transform SAS datasets through a reviewable pipeline contract")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: super::Command,
}
