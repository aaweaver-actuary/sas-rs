use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::parser::SupportedSas7bdatParser;
use crate::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use crate::transform::pipeline::{
    FileSystemSourceLoader, ParserTransformService, TransformReport, TransformService,
    TransformServiceError,
};
use crate::transform::sink::LocalParquetSink;

#[derive(Debug, Parser)]
#[command(name = "sasrs")]
#[command(about = "Transform SAS datasets through a reviewable pipeline contract")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOutcome {
    Transform(TransformReport),
}

impl Display for CommandOutcome {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transform(report) => report.fmt(formatter),
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    Parse(clap::Error),
    Transform(TransformServiceError),
}

impl CliError {
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
