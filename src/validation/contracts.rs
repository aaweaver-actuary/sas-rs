pub mod corpus_fixture_status;
pub mod corpus_sweep_report;
pub mod differential_column_kind;
pub mod differential_column_spec;
pub mod differential_fixture_result;
pub mod differential_fixture_spec;
pub mod differential_report;
pub mod differential_status;
pub mod invalid_fixture_case;
pub mod probe_failure_kind;
pub mod probe_outcome;
pub mod probe_result;
pub mod readable_outcome;
pub mod regression_case;
pub mod regression_expectation;

use std::path::PathBuf;

use super::classify_sample_corpus_fixture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeFailureKind {
    Unsupported,
    InvalidFormat,
    Io,
    Panic,
}

impl ProbeFailureKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::InvalidFormat => "invalid-format",
            Self::Io => "io",
            Self::Panic => "panic",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadableOutcome {
    pub row_count: usize,
    pub decoded_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeOutcome {
    Readable(ReadableOutcome),
    Failure {
        kind: ProbeFailureKind,
        stage: &'static str,
        detail: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeResult {
    pub file_name: String,
    pub path: PathBuf,
    pub outcome: ProbeOutcome,
}

impl ProbeResult {
    pub fn readable_outcome(&self) -> Option<&ReadableOutcome> {
        match &self.outcome {
            ProbeOutcome::Readable(outcome) => Some(outcome),
            ProbeOutcome::Failure { .. } => None,
        }
    }

    pub fn failure_kind(&self) -> Option<ProbeFailureKind> {
        match self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { kind, .. } => Some(kind),
        }
    }

    pub fn failure_stage(&self) -> Option<&'static str> {
        match self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { stage, .. } => Some(stage),
        }
    }

    pub fn failure_detail(&self) -> Option<&str> {
        match &self.outcome {
            ProbeOutcome::Readable(_) => None,
            ProbeOutcome::Failure { detail, .. } => Some(detail),
        }
    }

    pub fn is_readable(&self) -> bool {
        matches!(self.outcome, ProbeOutcome::Readable(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusFixtureStatus {
    Readable,
    ExpectedInvalid,
    CompatibilityFailure,
}

impl CorpusFixtureStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Readable => "readable",
            Self::ExpectedInvalid => "expected-invalid",
            Self::CompatibilityFailure => "compatibility-failure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidFixtureCase {
    pub file_name: &'static str,
    pub kind: ProbeFailureKind,
    pub detail_contains: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusSweepReport {
    pub root: PathBuf,
    pub total_files: usize,
    pub readable_files: usize,
    pub results: Vec<ProbeResult>,
}

impl CorpusSweepReport {
    pub fn failure_count(&self) -> usize {
        self.total_files.saturating_sub(self.readable_files)
    }

    pub fn expected_invalid_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                classify_sample_corpus_fixture(result) == CorpusFixtureStatus::ExpectedInvalid
            })
            .count()
    }

    pub fn compatibility_failure_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                classify_sample_corpus_fixture(result) == CorpusFixtureStatus::CompatibilityFailure
            })
            .count()
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionExpectation {
    Readable,
    Failure {
        kind: ProbeFailureKind,
        detail_contains: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegressionCase {
    pub file_name: &'static str,
    pub tags: &'static [&'static str],
    pub expectation: RegressionExpectation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferentialColumnKind {
    TimestampMicros,
    Date32,
    Time64Micros,
    NumericWithMissingTag,
}

impl DifferentialColumnKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::TimestampMicros => "timestamp_micros",
            Self::Date32 => "date32",
            Self::Time64Micros => "time64_micros",
            Self::NumericWithMissingTag => "numeric_with_missing_tag",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentialColumnSpec {
    pub source_name: &'static str,
    pub kind: DifferentialColumnKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentialFixtureSpec {
    pub file_name: &'static str,
    pub selected_columns: &'static [&'static str],
    pub columns: &'static [DifferentialColumnSpec],
    pub note: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferentialStatus {
    Matched,
    Mismatched,
    Error,
    Skipped,
}

impl DifferentialStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Mismatched => "mismatched",
            Self::Error => "error",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialFixtureResult {
    pub file_name: String,
    pub status: DifferentialStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialReport {
    pub results: Vec<DifferentialFixtureResult>,
}

impl DifferentialReport {
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

    pub fn skipped_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| result.status == DifferentialStatus::Skipped)
            .count()
    }

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
