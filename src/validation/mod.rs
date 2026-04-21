mod contracts;

pub use contracts::{
    CorpusFixtureStatus, CorpusSweepReport, DifferentialColumnKind, DifferentialColumnSpec,
    DifferentialFixtureResult, DifferentialFixtureSpec, DifferentialReport, DifferentialStatus,
    InvalidFixtureCase, ProbeFailureKind, ProbeOutcome, ProbeResult, ReadableOutcome,
    RegressionCase, RegressionExpectation,
};

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::Command;

use arrow_array::{
    Array, Date32Array, Float64Array, StringArray, Time64MicrosecondArray,
    TimestampMicrosecondArray,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::parser::{ParserError, ParserInput, Sas7bdatParser, SupportedSas7bdatParser};
use crate::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use crate::transform::pipeline::{
    FileSystemSourceLoader, ParserTransformService, TransformService,
};
use crate::transform::sink::LocalParquetSink;

const REGRESSION_TAGS_FTS0003: &[&str] = &["required_baseline", "wide_rows", "many_pages"];
const REGRESSION_TAGS_10REC: &[&str] = &["wide_schema", "many_columns", "big_endian"];
const REGRESSION_TAGS_DATES: &[&str] = &["semantic_dates_times", "string_companion_columns"];
const REGRESSION_TAGS_MISSINGS: &[&str] = &["special_missing_values"];
const REGRESSION_TAGS_LONG_NAMES: &[&str] = &[
    "long_column_names",
    "string_columns",
    "binary_compression_boundary",
];
const REGRESSION_TAGS_UNUSUAL_STRINGS: &[&str] =
    &["unusual_strings", "unsupported_encoding_boundary"];

const REGRESSION_CASES: &[RegressionCase] = &[
    RegressionCase {
        file_name: "fts0003.sas7bdat",
        tags: REGRESSION_TAGS_FTS0003,
        expectation: RegressionExpectation::Readable,
    },
    RegressionCase {
        file_name: "10rec.sas7bdat",
        tags: REGRESSION_TAGS_10REC,
        expectation: RegressionExpectation::Readable,
    },
    RegressionCase {
        file_name: "dates.sas7bdat",
        tags: REGRESSION_TAGS_DATES,
        expectation: RegressionExpectation::Readable,
    },
    RegressionCase {
        file_name: "missing_test.sas7bdat",
        tags: REGRESSION_TAGS_MISSINGS,
        expectation: RegressionExpectation::Readable,
    },
    RegressionCase {
        file_name: "dates_longname_char.sas7bdat",
        tags: REGRESSION_TAGS_LONG_NAMES,
        expectation: RegressionExpectation::Readable,
    },
    RegressionCase {
        file_name: "0x40controlbyte.sas7bdat",
        tags: REGRESSION_TAGS_UNUSUAL_STRINGS,
        expectation: RegressionExpectation::Readable,
    },
];

const DATES_SELECTED_COLUMNS: &[&str] = &["dt", "dates", "times"];
const DATES_DIFFERENTIAL_COLUMNS: &[DifferentialColumnSpec] = &[
    DifferentialColumnSpec {
        source_name: "dt",
        kind: DifferentialColumnKind::TimestampMicros,
    },
    DifferentialColumnSpec {
        source_name: "dates",
        kind: DifferentialColumnKind::Date32,
    },
    DifferentialColumnSpec {
        source_name: "times",
        kind: DifferentialColumnKind::Time64Micros,
    },
];
const MISSING_SELECTED_COLUMNS: &[&str] = &["var1", "var7", "var9"];
const MISSING_DIFFERENTIAL_COLUMNS: &[DifferentialColumnSpec] = &[
    DifferentialColumnSpec {
        source_name: "var1",
        kind: DifferentialColumnKind::NumericWithMissingTag,
    },
    DifferentialColumnSpec {
        source_name: "var7",
        kind: DifferentialColumnKind::NumericWithMissingTag,
    },
    DifferentialColumnSpec {
        source_name: "var9",
        kind: DifferentialColumnKind::NumericWithMissingTag,
    },
];
const DETAIL_MISSING_MAGIC: &str = "missing sas7bdat magic number";
const DETAIL_INVALID_HEADER_PAGE_SIZE: &str =
    "header_size and page_size must both be at least 1024 bytes";
const DETAIL_MISSING_ROW_SIZE: &str = "row size subheader is missing";

const EXPECTED_INVALID_FIXTURES: &[InvalidFixtureCase] = &[
    InvalidFixtureCase {
        file_name: "FileFromJMP.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "corrupt.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_INVALID_HEADER_PAGE_SIZE,
    },
    InvalidFixtureCase {
        file_name: "depress.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "drugprob.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "drugtest.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "environ.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "event1.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "event2.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "event3.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "event4.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "firstsex.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "gpa.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "gss96.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "osteo_analysis_data.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "religion.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "stress.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "yrbscol.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_MAGIC,
    },
    InvalidFixtureCase {
        file_name: "zero_variables.sas7bdat",
        kind: ProbeFailureKind::InvalidFormat,
        detail_contains: DETAIL_MISSING_ROW_SIZE,
    },
];

const DIFFERENTIAL_FIXTURES: &[DifferentialFixtureSpec] = &[
    DifferentialFixtureSpec {
        file_name: "dates.sas7bdat",
        selected_columns: DATES_SELECTED_COLUMNS,
        columns: DATES_DIFFERENTIAL_COLUMNS,
        note: "Compare real datetime/date/time materialization against haven.",
    },
    DifferentialFixtureSpec {
        file_name: "missing_test.sas7bdat",
        selected_columns: MISSING_SELECTED_COLUMNS,
        columns: MISSING_DIFFERENTIAL_COLUMNS,
        note: "Compare tagged missing values that haven exposes on the shared semantic surface.",
    },
];

const HAVEN_DIFF_SCRIPT: &str = r#"
suppressPackageStartupMessages(library(haven))
args <- commandArgs(trailingOnly=TRUE)
path <- args[[1]]
spec_entries <- strsplit(args[[2]], ";", fixed=TRUE)[[1]]
dataset <- read_sas(path)

value_token <- function(value, kind) {
  if (is.na(value)) {
    return("NA")
  }
  if (kind == "timestamp_micros") {
    return(sprintf("%.0f", round(as.numeric(as.POSIXct(value, tz='UTC')) * 1000000)))
  }
  if (kind == "date32") {
    return(as.character(as.integer(value - as.Date('1970-01-01'))))
  }
  if (kind == "time64_micros") {
    return(sprintf("%.0f", round(as.numeric(value) * 1000000)))
  }
  if (kind == "numeric_with_missing_tag") {
    return(format(value, scientific=FALSE, trim=TRUE, digits=17))
  }
  stop(paste("unsupported kind", kind))
}

tag_token <- function(value) {
  if (is_tagged_na(value)) {
    return(toupper(as.character(na_tag(value))))
  }
  return("<none>")
}

writeLines(paste("ROW_COUNT", nrow(dataset), sep='	'))
for (entry in spec_entries) {
  parts <- strsplit(entry, "=", fixed=TRUE)[[1]]
  name <- parts[[1]]
  kind <- parts[[2]]
  column <- dataset[[name]]
  for (index in seq_along(column)) {
    writeLines(
      paste(
        "VAL",
        name,
        index - 1L,
        value_token(column[[index]], kind),
        tag_token(column[[index]]),
        sep='	'
      )
    )
  }
}
"#;

#[derive(Debug)]
struct ProbeFailure {
    kind: ProbeFailureKind,
    stage: &'static str,
    detail: String,
}

impl ProbeFailure {
    fn new(kind: ProbeFailureKind, stage: &'static str, detail: impl Into<String>) -> Self {
        Self {
            kind,
            stage,
            detail: detail.into(),
        }
    }

    fn from_parser(stage: &'static str, error: ParserError) -> Self {
        Self::new(classify_parser_error(&error), stage, error.to_string())
    }
}

#[derive(Debug)]
enum TrustedReaderError {
    Error(String),
    Skipped(String),
}

/// Return the repository path for the sample SAS fixture corpus.
pub fn sample_corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample-sas-datasets")
}

/// Return the named real-file regression cases used by validation sweeps.
pub fn real_regression_cases() -> &'static [RegressionCase] {
    REGRESSION_CASES
}

/// Return the differential-validation fixture contracts.
pub fn differential_fixture_specs() -> &'static [DifferentialFixtureSpec] {
    DIFFERENTIAL_FIXTURES
}

/// Return the catalog of fixtures intentionally classified as invalid.
pub fn expected_invalid_sample_fixtures() -> &'static [InvalidFixtureCase] {
    EXPECTED_INVALID_FIXTURES
}

/// Classify a probe result against the expected-invalid catalog.
pub fn classify_sample_corpus_fixture(result: &ProbeResult) -> CorpusFixtureStatus {
    if result.is_readable() {
        return CorpusFixtureStatus::Readable;
    }

    if expected_invalid_sample_fixtures().iter().any(|fixture| {
        result.file_name == fixture.file_name
            && result.failure_kind() == Some(fixture.kind)
            && result
                .failure_detail()
                .is_some_and(|detail| detail.contains(fixture.detail_contains))
    }) {
        CorpusFixtureStatus::ExpectedInvalid
    } else {
        CorpusFixtureStatus::CompatibilityFailure
    }
}

/// Probe one SAS fixture and summarize whether parsing and row decoding succeeded.
pub fn probe_file(path: &Path, batch_size_rows: usize) -> ProbeResult {
    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_else(|| path.display().to_string());

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return ProbeResult {
                file_name,
                path: path.to_path_buf(),
                outcome: ProbeOutcome::Failure {
                    kind: ProbeFailureKind::Io,
                    stage: "open",
                    detail: error.to_string(),
                },
            };
        }
    };

    let source_name = path.display().to_string();
    let parser = SupportedSas7bdatParser;
    let attempt = catch_unwind(AssertUnwindSafe(
        move || -> Result<ReadableOutcome, ProbeFailure> {
            let mut parsed = parser
                .parse(ParserInput::from_reader(&source_name, file))
                .map_err(|error| ProbeFailure::from_parser("parse", error))?;
            let row_count = parsed.metadata.row_count;
            let mut decoded_rows = 0_usize;

            while let Some(batch) = parsed
                .next_batch(batch_size_rows.max(1))
                .map_err(|error| ProbeFailure::from_parser("decode", error))?
            {
                decoded_rows += batch.rows.len();
            }

            Ok(ReadableOutcome {
                row_count,
                decoded_rows,
            })
        },
    ));

    let outcome = match attempt {
        Ok(Ok(readable)) => ProbeOutcome::Readable(readable),
        Ok(Err(failure)) => ProbeOutcome::Failure {
            kind: failure.kind,
            stage: failure.stage,
            detail: failure.detail,
        },
        Err(payload) => ProbeOutcome::Failure {
            kind: ProbeFailureKind::Panic,
            stage: "panic",
            detail: panic_detail(&payload),
        },
    };

    ProbeResult {
        file_name,
        path: path.to_path_buf(),
        outcome,
    }
}

/// Sweep a sample-corpus directory and aggregate probe results for each `.sas7bdat` file.
pub fn sweep_sample_corpus(
    root: &Path,
    batch_size_rows: usize,
    limit: Option<usize>,
) -> io::Result<CorpusSweepReport> {
    let mut paths = fs::read_dir(root)?
        .filter_map(|entry| entry.ok().map(|value| value.path()))
        .filter(|path| {
            path.extension()
                .and_then(OsStr::to_str)
                .is_some_and(|ext| ext.eq_ignore_ascii_case("sas7bdat"))
        })
        .collect::<Vec<_>>();
    paths.sort();
    if let Some(limit) = limit {
        paths.truncate(limit);
    }

    let results = paths
        .iter()
        .map(|path| probe_file(path, batch_size_rows))
        .collect::<Vec<_>>();
    let readable_files = results.iter().filter(|result| result.is_readable()).count();

    Ok(CorpusSweepReport {
        root: root.to_path_buf(),
        total_files: results.len(),
        readable_files,
        results,
    })
}

/// Run parquet differential validation for each configured fixture.
pub fn run_differential_validation(output_root: &Path) -> DifferentialReport {
    let mut results = Vec::with_capacity(DIFFERENTIAL_FIXTURES.len());
    for spec in DIFFERENTIAL_FIXTURES {
        results.push(run_differential_fixture(spec, output_root));
    }
    DifferentialReport { results }
}

fn run_differential_fixture(
    spec: &DifferentialFixtureSpec,
    output_root: &Path,
) -> DifferentialFixtureResult {
    if let Err(error) = fs::create_dir_all(output_root) {
        return DifferentialFixtureResult {
            file_name: spec.file_name.to_string(),
            status: DifferentialStatus::Error,
            detail: error.to_string(),
        };
    }

    let file_stem = Path::new(spec.file_name)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("fixture");
    let output_path = output_root.join(format!("{file_stem}-differential.parquet"));
    let source_path = sample_corpus_root().join(spec.file_name);

    let result = match run_transform_fixture(spec, &source_path, &output_path) {
        Ok(()) => match canonicalize_parquet_fixture(spec, &output_path) {
            Ok(local_lines) => match canonicalize_haven_fixture(spec, &source_path) {
                Ok(trusted_lines) => {
                    if local_lines == trusted_lines {
                        DifferentialFixtureResult {
                            file_name: spec.file_name.to_string(),
                            status: DifferentialStatus::Matched,
                            detail: format!(
                                "matched {} canonical lines against haven",
                                local_lines.len()
                            ),
                        }
                    } else {
                        DifferentialFixtureResult {
                            file_name: spec.file_name.to_string(),
                            status: DifferentialStatus::Mismatched,
                            detail: first_mismatch_detail(&local_lines, &trusted_lines),
                        }
                    }
                }
                Err(TrustedReaderError::Skipped(detail)) => DifferentialFixtureResult {
                    file_name: spec.file_name.to_string(),
                    status: DifferentialStatus::Skipped,
                    detail,
                },
                Err(TrustedReaderError::Error(detail)) => DifferentialFixtureResult {
                    file_name: spec.file_name.to_string(),
                    status: DifferentialStatus::Error,
                    detail,
                },
            },
            Err(error) => DifferentialFixtureResult {
                file_name: spec.file_name.to_string(),
                status: DifferentialStatus::Error,
                detail: error,
            },
        },
        Err(error) => DifferentialFixtureResult {
            file_name: spec.file_name.to_string(),
            status: DifferentialStatus::Error,
            detail: error,
        },
    };

    let _ = fs::remove_file(&output_path);
    result
}

fn run_transform_fixture(
    spec: &DifferentialFixtureSpec,
    source_path: &Path,
    output_path: &Path,
) -> Result<(), String> {
    let service = ParserTransformService::new(
        FileSystemSourceLoader,
        SupportedSas7bdatParser,
        LocalParquetSink,
    );
    let request = TransformRequest {
        source: SourceContract {
            path: source_path.to_path_buf(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: spec
                .selected_columns
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            filter: None,
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 256,
                worker_threads: Some(1),
            },
        },
        sink: SinkContract {
            path: output_path.to_path_buf(),
            format: SinkFormat::Parquet,
        },
    };

    service
        .run(request)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn canonicalize_parquet_fixture(
    spec: &DifferentialFixtureSpec,
    output_path: &Path,
) -> Result<Vec<String>, String> {
    let mut lines = vec![format!("ROW_COUNT\t{}", read_total_rows(output_path)?)];

    for column in spec.columns {
        match column.kind {
            DifferentialColumnKind::TimestampMicros => {
                let values = read_timestamp_column(output_path, column.source_name)?;
                let tags =
                    read_utf8_column(output_path, &missing_tag_column_name(column.source_name))?;
                extend_value_lines(&mut lines, column.source_name, values, tags, |value| {
                    value.to_string()
                })?;
            }
            DifferentialColumnKind::Date32 => {
                let values = read_date32_column(output_path, column.source_name)?;
                let tags =
                    read_utf8_column(output_path, &missing_tag_column_name(column.source_name))?;
                extend_value_lines(&mut lines, column.source_name, values, tags, |value| {
                    value.to_string()
                })?;
            }
            DifferentialColumnKind::Time64Micros => {
                let values = read_time64_column(output_path, column.source_name)?;
                let tags =
                    read_utf8_column(output_path, &missing_tag_column_name(column.source_name))?;
                extend_value_lines(&mut lines, column.source_name, values, tags, |value| {
                    value.to_string()
                })?;
            }
            DifferentialColumnKind::NumericWithMissingTag => {
                let values = read_float64_column(output_path, column.source_name)?;
                let tags =
                    read_utf8_column(output_path, &missing_tag_column_name(column.source_name))?;
                extend_value_lines(&mut lines, column.source_name, values, tags, normalize_f64)?;
            }
        }
    }

    Ok(lines)
}

fn extend_value_lines<T, F>(
    lines: &mut Vec<String>,
    column_name: &str,
    primary: Vec<Option<T>>,
    tags: Vec<Option<String>>,
    render_value: F,
) -> Result<(), String>
where
    F: Fn(T) -> String,
    T: Copy,
{
    if primary.len() != tags.len() {
        return Err(format!(
            "column {column_name} value/tag length mismatch: {} != {}",
            primary.len(),
            tags.len()
        ));
    }

    for (index, (value, tag)) in primary.into_iter().zip(tags.into_iter()).enumerate() {
        let value_token = value.map(&render_value).unwrap_or_else(|| "NA".to_string());
        lines.push(format!(
            "VAL\t{}\t{}\t{}\t{}",
            column_name,
            index,
            value_token,
            tag_token(tag.as_deref())
        ));
    }

    Ok(())
}

fn canonicalize_haven_fixture(
    spec: &DifferentialFixtureSpec,
    source_path: &Path,
) -> Result<Vec<String>, TrustedReaderError> {
    let spec_arg = spec
        .columns
        .iter()
        .map(|column| format!("{}={}", column.source_name, column.kind.label()))
        .collect::<Vec<_>>()
        .join(";");

    let output = Command::new("Rscript")
        .arg("-e")
        .arg(HAVEN_DIFF_SCRIPT)
        .arg(source_path)
        .arg(spec_arg)
        .output()
        .map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                TrustedReaderError::Skipped(
                    "Rscript is not installed in this environment".to_string(),
                )
            } else {
                TrustedReaderError::Error(error.to_string())
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("there is no package called")
            || stderr.contains("Error in library(haven)")
        {
            return Err(TrustedReaderError::Skipped(if stderr.is_empty() {
                "haven is not available in the active R environment".to_string()
            } else {
                stderr
            }));
        }
        return Err(TrustedReaderError::Error(if stderr.is_empty() {
            format!("Rscript exited with status {}", output.status)
        } else {
            stderr
        }));
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect())
}

fn first_mismatch_detail(local_lines: &[String], trusted_lines: &[String]) -> String {
    let index = local_lines
        .iter()
        .zip(trusted_lines.iter())
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| local_lines.len().min(trusted_lines.len()));
    format!(
        "first mismatch at line {}: local={:?}, trusted={:?}",
        index,
        local_lines.get(index),
        trusted_lines.get(index)
    )
}

fn read_total_rows(path: &Path) -> Result<usize, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    Ok(record_reader
        .map(|batch| {
            batch
                .map(|batch| batch.num_rows())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

fn read_timestamp_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    let mut values = Vec::new();
    for batch in record_reader {
        let batch = batch.map_err(|error| error.to_string())?;
        let index = batch
            .schema()
            .index_of(column_name)
            .map_err(|error| error.to_string())?;
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<TimestampMicrosecondArray>()
            .ok_or_else(|| format!("column {column_name} is not Timestamp(Microsecond)"))?;
        values.extend(
            (0..array.len()).map(|index| (!array.is_null(index)).then(|| array.value(index))),
        );
    }
    Ok(values)
}

fn read_date32_column(path: &Path, column_name: &str) -> Result<Vec<Option<i32>>, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    let mut values = Vec::new();
    for batch in record_reader {
        let batch = batch.map_err(|error| error.to_string())?;
        let index = batch
            .schema()
            .index_of(column_name)
            .map_err(|error| error.to_string())?;
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| format!("column {column_name} is not Date32"))?;
        values.extend(
            (0..array.len()).map(|index| (!array.is_null(index)).then(|| array.value(index))),
        );
    }
    Ok(values)
}

fn read_time64_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    let mut values = Vec::new();
    for batch in record_reader {
        let batch = batch.map_err(|error| error.to_string())?;
        let index = batch
            .schema()
            .index_of(column_name)
            .map_err(|error| error.to_string())?;
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<Time64MicrosecondArray>()
            .ok_or_else(|| format!("column {column_name} is not Time64(Microsecond)"))?;
        values.extend(
            (0..array.len()).map(|index| (!array.is_null(index)).then(|| array.value(index))),
        );
    }
    Ok(values)
}

fn read_float64_column(path: &Path, column_name: &str) -> Result<Vec<Option<f64>>, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    let mut values = Vec::new();
    for batch in record_reader {
        let batch = batch.map_err(|error| error.to_string())?;
        let index = batch
            .schema()
            .index_of(column_name)
            .map_err(|error| error.to_string())?;
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| format!("column {column_name} is not Float64"))?;
        values.extend(
            (0..array.len()).map(|index| (!array.is_null(index)).then(|| array.value(index))),
        );
    }
    Ok(values)
}

fn read_utf8_column(path: &Path, column_name: &str) -> Result<Vec<Option<String>>, String> {
    let reader = File::open(path).map_err(|error| error.to_string())?;
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .map_err(|error| error.to_string())?
        .build()
        .map_err(|error| error.to_string())?;
    let mut values = Vec::new();
    for batch in record_reader {
        let batch = batch.map_err(|error| error.to_string())?;
        let index = batch
            .schema()
            .index_of(column_name)
            .map_err(|error| error.to_string())?;
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| format!("column {column_name} is not Utf8"))?;
        values.extend(
            (0..array.len())
                .map(|index| (!array.is_null(index)).then(|| array.value(index).to_string())),
        );
    }
    Ok(values)
}

fn missing_tag_column_name(column_name: &str) -> String {
    format!("{column_name}__sas_missing_tag")
}

fn normalize_f64(value: f64) -> String {
    let mut token = format!("{value:.17}");
    while token.contains('.') && token.ends_with('0') {
        token.pop();
    }
    if token.ends_with('.') {
        token.pop();
    }
    if token == "-0" {
        token = "0".to_string();
    }
    token
}

fn tag_token(tag: Option<&str>) -> String {
    match tag {
        Some(tag) if !tag.is_empty() => tag.to_string(),
        _ => "<none>".to_string(),
    }
}

fn classify_parser_error(error: &ParserError) -> ProbeFailureKind {
    match error {
        ParserError::Unsupported(_) => ProbeFailureKind::Unsupported,
        ParserError::InvalidFormat(_) => ProbeFailureKind::InvalidFormat,
        ParserError::Io(_) => ProbeFailureKind::Io,
    }
}

fn panic_detail(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "panic payload was not a string".to_string()
    }
}
