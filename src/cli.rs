//! Command-line entrypoints for the reviewable transform pipeline.
//!
//! This module keeps CLI parsing thin and delegates the actual work to the
//! transform service interfaces in [`crate::transform`].

use std::path::PathBuf;

use clap::{Args, Subcommand};

mod cli_definition;
mod cli_error;
mod command_outcome;
mod run;
mod run_with_service;

/// Parsed top-level command-line contract for `sasrs`.
pub use cli_definition::Cli;
/// Error type returned by the public CLI entrypoints.
pub use cli_error::CliError;
/// Result of executing a parsed CLI command.
pub use command_outcome::CommandOutcome;
/// Run the default filesystem-backed transform service from process-style args.
pub use run::run;
/// Run the CLI against a caller-provided transform service implementation.
pub use run_with_service::run_with_service;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Transform(TransformArgs),
}

#[derive(Debug, Clone, Args)]
struct TransformArgs {
    input: PathBuf,
    output: PathBuf,
    #[arg(long, value_delimiter = ',')]
    select: Vec<String>,
    #[arg(long)]
    filter: Option<String>,
    #[arg(long = "batch-size-rows", default_value_t = 131_072)]
    batch_size_rows: usize,
    #[arg(long = "max-rows-in-memory")]
    max_rows_in_memory: Option<usize>,
    #[arg(long = "worker-threads")]
    worker_threads: Option<usize>,
}
