//! Public validation contracts for corpus probes and differential checks.

/// Re-export of corpus fixture classification statuses.
pub mod corpus_fixture_status;
/// Re-export of sample-corpus sweep reports.
pub mod corpus_sweep_report;
/// Re-export of differential comparison column kinds.
pub mod differential_column_kind;
/// Re-export of differential comparison column specs.
pub mod differential_column_spec;
/// Re-export of differential fixture results.
pub mod differential_fixture_result;
/// Re-export of differential fixture specs.
pub mod differential_fixture_spec;
/// Re-export of differential validation reports.
pub mod differential_report;
/// Re-export of differential validation statuses.
pub mod differential_status;
/// Re-export of expected-invalid fixture descriptors.
pub mod invalid_fixture_case;
/// Re-export of probe failure categories.
pub mod probe_failure_kind;
/// Re-export of probe outcomes.
pub mod probe_outcome;
/// Re-export of per-file probe results.
pub mod probe_result;
/// Re-export of readable probe summaries.
pub mod readable_outcome;
/// Re-export of regression corpus cases.
pub mod regression_case;
/// Re-export of regression expectations.
pub mod regression_expectation;

use std::path::PathBuf;

use super::classify_sample_corpus_fixture;

/// High-level failure class observed while probing a fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeFailureKind {
    /// The fixture used an unsupported feature.
    Unsupported,
    /// The fixture violated the supported file format.
    InvalidFormat,
    /// The probe failed on filesystem I/O.
    Io,
    /// The probe panicked unexpectedly.
    Panic,
}

impl ProbeFailureKind {
    /// Return the stable machine-readable label for this failure kind.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::InvalidFormat => "invalid-format",
            Self::Io => "io",
            Self::Panic => "panic",
        }
    }
}

/// Readable probe summary for one fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadableOutcome {
    /// Row count declared in the parsed metadata.
    pub row_count: usize,
    /// Number of rows actually decoded during the probe.
    pub decoded_rows: usize,
}

/// Final probe outcome for one fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeOutcome {
    /// The fixture parsed and decoded successfully.
    Readable(ReadableOutcome),
    /// The fixture failed during probing.
    /// The fixture is expected to fail.
    /// The fixture is expected to fail.
    Failure {
        /// High-level failure kind.
        /// Expected high-level failure kind.
        /// Expected high-level failure kind.
        kind: ProbeFailureKind,
        /// Probe stage that failed.
        stage: &'static str,
        /// Human-readable failure detail.
        detail: String,
    },
}

/// Result of probing one concrete sample fixture path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeResult {
    /// Fixture file name used for reporting.
    pub file_name: String,
    /// Full path to the probed fixture.
    pub path: PathBuf,
    /// Readable or failure outcome for the fixture.
    pub outcome: ProbeOutcome,
}

impl ProbeResult {
    /// Return the readable outcome when probing succeeded.
    pub fn readable_outcome(&self) -> Option<&ReadableOutcome> {
        match &self.outcome {
            ProbeOutcome::Readable(outcome) => Some(outcome),
            ProbeOutcome::Failure { .. } => None,
        }
    }

    /// Return the failure kind when probing failed.
    pub fn failure_kind(&self) -> Option<ProbeFailureKind> {
        match self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { kind, .. } => Some(kind),
        }
    }

    /// Return the probe stage where a failure occurred.
    pub fn failure_stage(&self) -> Option<&'static str> {
        match self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { stage, .. } => Some(stage),
        }
    }

    /// Return the human-readable failure detail when probing failed.
    pub fn failure_detail(&self) -> Option<&str> {
        match &self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { detail, .. } => Some(detail),
        }
    }

    /// Report whether probing produced a readable outcome.
    pub fn is_readable(&self) -> bool {
        matches!(self.outcome, ProbeOutcome::Readable(_))
    }
}

/// Classification for a fixture after comparing probe output to expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusFixtureStatus {
    /// The fixture was readable and matched the happy-path expectation.
    /// The fixture is expected to parse and decode successfully.
    Readable,
    /// The fixture failed in a way that is intentionally tracked as invalid.
    ExpectedInvalid,
    /// The fixture failed unexpectedly and should be treated as a compatibility gap.
    CompatibilityFailure,
}

impl CorpusFixtureStatus {
    /// Return the stable machine-readable label for this corpus classification.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Readable => "readable",
            Self::ExpectedInvalid => "expected-invalid",
            Self::CompatibilityFailure => "compatibility-failure",
        }
    }
}

/// Expected-invalid fixture classification used by corpus sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidFixtureCase {
    /// Fixture file name expected to be invalid.
    pub file_name: &'static str,
    /// Failure kind expected for that fixture.
    /// Comparison kind used for the column.
    pub kind: ProbeFailureKind,
    /// Stable detail substring used to confirm the expected invalid failure.
    pub detail_contains: &'static str,
}

/// Aggregate report for probing a directory of sample fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusSweepReport {
    /// Root directory that was swept.
    pub root: PathBuf,
    /// Number of files considered during the sweep.
    pub total_files: usize,
    /// Number of files that produced readable probe results.
    pub readable_files: usize,
    /// Per-file probe results gathered during the sweep.
    /// Per-fixture differential validation results.
    pub results: Vec<ProbeResult>,
}

impl CorpusSweepReport {
    /// Return the number of swept files that were not readable.
    /// Return how many fixture results ended in mismatch or error.
    pub fn failure_count(&self) -> usize {
        self.total_files.saturating_sub(self.readable_files)
    }

    /// Return how many failures matched the expected-invalid catalog.
    pub fn expected_invalid_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                classify_sample_corpus_fixture(result) == CorpusFixtureStatus::ExpectedInvalid
            })
            .count()
    }

    /// Return how many failures remain unexpected compatibility gaps.
    pub fn compatibility_failure_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                classify_sample_corpus_fixture(result) == CorpusFixtureStatus::CompatibilityFailure
            })
            .count()
    }

    /// Render the sweep report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    pub fn render_text(&self) -> String {
        let mut lines = vec![format!(
            "SUMMARY\troot={}\ttotal={}\treadable={}\texpected_invalid={}\tcompatibility_failures={}\tfailures={}",
            self.root.display(),
            self.total_files,
            self.readable_files,
            self.expected_invalid_count(),
            self.compatibility_failure_count(),
            self.failure_count()
        )];

        for result in &self.results {
            match (&result.outcome, classify_sample_corpus_fixture(result)) {
                (ProbeOutcome::Readable(outcome), CorpusFixtureStatus::Readable) => {
                    lines.push(format!(
                        "RESULT\tstatus=readable\tfile={}\trows={}\tdecoded_rows={}",
                        result.file_name, outcome.row_count, outcome.decoded_rows
                    ));
                }
                (
                    ProbeOutcome::Failure {
                        kind,
                        stage,
                        detail,
                    },
                    status,
                ) => lines.push(format!(
                    "RESULT\tstatus={}\tfile={}\tkind={}\tstage={}\tdetail={}",
                    status.label(),
                    result.file_name,
                    kind.label(),
                    stage,
                    detail.replace('\n', " ")
                )),
                _ => unreachable!("readable results should not be reclassified as failures"),
            }
        }

        lines.join("\n")
    }
}

/// Expected probe outcome for a named regression fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionExpectation {
    /// The fixture is expected to parse and decode successfully.
    Readable,
    /// The fixture is expected to fail.
    Failure {
        /// Expected high-level failure kind.
        kind: ProbeFailureKind,
        /// Stable detail substring expected in the failure message.
        detail_contains: &'static str,
    },
}

/// Named regression fixture and its expected probe behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegressionCase {
    /// Fixture file name to probe.
    pub file_name: &'static str,
    /// Search tags describing why this fixture matters.
    pub tags: &'static [&'static str],
    /// Expected probe outcome for the fixture.
    pub expectation: RegressionExpectation,
}

/// Column-shape expectation used during differential parquet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferentialColumnKind {
    /// Compare the column as a microsecond timestamp.
    TimestampMicros,
    /// Compare the column as a date32 logical type.
    Date32,
    /// Compare the column as a microsecond time value.
    Time64Micros,
    /// Compare the numeric value plus its companion missing-tag column.
    NumericWithMissingTag,
}

impl DifferentialColumnKind {
    /// Return the stable machine-readable label for this comparison kind.
    pub fn label(&self) -> &'static str {
        match self {
            Self::TimestampMicros => "timestamp_micros",
            Self::Date32 => "date32",
            Self::Time64Micros => "time64_micros",
            Self::NumericWithMissingTag => "numeric_with_missing_tag",
        }
    }
}

/// Column-level comparison contract for a differential fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentialColumnSpec {
    /// Source column name to compare.
    pub source_name: &'static str,
    /// Comparison kind used for the column.
    pub kind: DifferentialColumnKind,
}

/// Differential validation contract for one fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentialFixtureSpec {
    /// Fixture file name to validate.
    pub file_name: &'static str,
    /// Columns selected when generating the parquet output.
    pub selected_columns: &'static [&'static str],
    /// Column-level expectations checked against the trusted reader.
    pub columns: &'static [DifferentialColumnSpec],
    /// Human-readable note describing why this fixture is included.
    pub note: &'static str,
}

/// Final status for one differential validation fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferentialStatus {
    /// The parquet output matched the trusted reader expectation.
    Matched,
    /// The parquet output disagreed with the trusted reader.
    Mismatched,
    /// Validation failed before a trustworthy comparison completed.
    Error,
    /// Validation was intentionally skipped because the trusted reader could not run.
    Skipped,
}

impl DifferentialStatus {
    /// Return the stable machine-readable label for this differential status.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Mismatched => "mismatched",
            Self::Error => "error",
            Self::Skipped => "skipped",
        }
    }
}

/// Result of running differential validation for one fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialFixtureResult {
    /// Fixture file name that was validated.
    pub file_name: String,
    /// Final comparison status for the fixture.
    pub status: DifferentialStatus,
    /// Human-readable detail about the comparison result.
    pub detail: String,
}

/// Aggregate report for a differential validation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialReport {
    /// Per-fixture differential validation results.
    pub results: Vec<DifferentialFixtureResult>,
}

impl DifferentialReport {
    /// Return how many fixture results ended in mismatch or error.
    pub fn failure_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                matches!(
                    result.status,
                    DifferentialStatus::Mismatched | DifferentialStatus::Error
                )
            })
            .count()
    }

    /// Return how many fixture results were skipped.
    pub fn skipped_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| result.status == DifferentialStatus::Skipped)
            .count()
    }

    /// Render the differential report as tab-separated summary and detail lines.
    /// Render the differential report as tab-separated summary and detail lines.
    pub fn render_text(&self) -> String {
        let matched = self
            .results
            .iter()
            .filter(|result| result.status == DifferentialStatus::Matched)
            .count();
        let mut lines = vec![format!(
            "SUMMARY\tfixtures={}\tmatched={}\tfailures={}\tskipped={}",
            self.results.len(),
            matched,
            self.failure_count(),
            self.skipped_count()
        )];

        for result in &self.results {
            lines.push(format!(
                "RESULT\tstatus={}\tfile={}\tdetail={}",
                result.status.label(),
                result.file_name,
                result.detail.replace('\n', " ")
            ));
        }

        lines.join("\n")
    }
}
