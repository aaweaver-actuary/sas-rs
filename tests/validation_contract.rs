#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::validation::{
    CorpusFixtureStatus, ProbeFailureKind, RegressionExpectation,
    classify_sample_corpus_fixture, differential_fixture_specs,
    expected_invalid_sample_fixtures, probe_file, real_regression_cases, sample_corpus_root,
    sweep_sample_corpus,
};

#[test]
fn real_regression_corpus_includes_required_categories_and_baseline_fixture() {
    let cases = real_regression_cases();

    assert!(
        cases
            .iter()
            .any(|case| case.file_name == "fts0003.sas7bdat"),
        "the curated regression corpus should retain fts0003 as a required baseline fixture"
    );
    assert!(
        cases.iter().any(|case| case.tags.contains(&"wide_rows")),
        "the curated regression corpus should cover wide rows"
    );
    assert!(
        cases.iter().any(|case| case.tags.contains(&"many_pages")),
        "the curated regression corpus should cover many-page datasets"
    );
    assert!(
        cases
            .iter()
            .any(|case| case.tags.contains(&"unusual_strings")),
        "the curated regression corpus should cover unusual strings honestly"
    );
    assert!(
        cases
            .iter()
            .any(|case| case.tags.contains(&"semantic_dates_times")),
        "the curated regression corpus should retain semantic date/time fixtures"
    );
}

#[test]
fn differential_fixtures_cover_the_supported_semantic_surface() {
    let specs = differential_fixture_specs();

    assert_eq!(specs.len(), 2);
    assert!(specs.iter().any(|spec| spec.file_name == "dates.sas7bdat"));
    assert!(
        specs
            .iter()
            .any(|spec| spec.file_name == "missing_test.sas7bdat")
    );

    let dates = specs
        .iter()
        .find(|spec| spec.file_name == "dates.sas7bdat")
        .expect("dates differential spec should exist");
    assert_eq!(dates.selected_columns, ["dt", "dates", "times"]);

    let missings = specs
        .iter()
        .find(|spec| spec.file_name == "missing_test.sas7bdat")
        .expect("missing_test differential spec should exist");
    assert_eq!(missings.selected_columns, ["var1", "var7", "var9"]);
}

#[test]
fn sample_corpus_sweep_reports_mixed_results_honestly() {
    let root = minimal_sas_fixture::unique_tmp_path("validation-corpus-sweep", "dir");
    std::fs::create_dir_all(&root).expect("temporary corpus directory should be created");

    std::fs::write(
        root.join("supported.sas7bdat"),
        minimal_sas_fixture::supported_fixture_bytes(),
    )
    .expect("supported fixture should be written");
    std::fs::write(
        root.join("malformed-word-size.sas7bdat"),
        minimal_sas_fixture::malformed_word_size_fixture_bytes(0x11),
    )
    .expect("malformed fixture should be written");

    let report = sweep_sample_corpus(&root, 64, None)
        .expect("corpus sweep should complete even when files fail validation");

    assert_eq!(report.total_files, 2);
    assert_eq!(report.readable_files, 1);
    assert_eq!(report.failure_count(), 1);
    assert_eq!(report.expected_invalid_count(), 0);
    assert_eq!(report.compatibility_failure_count(), 1);
    assert!(
        report
            .results
            .iter()
            .any(|result| matches!(result.failure_kind(), Some(ProbeFailureKind::InvalidFormat))),
        "the malformed file should still be surfaced as an invalid-format failure"
    );
}

#[test]
fn invalid_sample_fixture_policy_is_explicit_and_reviewable() {
    let fixtures = expected_invalid_sample_fixtures();

    assert_eq!(fixtures.len(), 18);
    assert!(
        fixtures.iter().any(|fixture| fixture.file_name == "FileFromJMP.sas7bdat"),
        "missing-magic fixtures should stay explicitly listed"
    );
    assert!(
        fixtures.iter().any(|fixture| fixture.file_name == "corrupt.sas7bdat"),
        "the malformed header fixture should stay explicitly listed"
    );
    assert!(
        fixtures.iter().any(|fixture| fixture.file_name == "zero_variables.sas7bdat"),
        "the missing-row-size fixture should stay explicitly listed"
    );
}

#[test]
fn expected_invalid_sample_fixtures_match_their_current_probe_results() {
    for fixture in expected_invalid_sample_fixtures() {
        let result = probe_file(&sample_corpus_root().join(fixture.file_name), 1_024);

        assert_eq!(
            classify_sample_corpus_fixture(&result),
            CorpusFixtureStatus::ExpectedInvalid,
            "{} should stay classified as expected-invalid",
            fixture.file_name
        );
    }
}

#[test]
fn curated_real_regression_cases_match_their_current_expectations() {
    for case in real_regression_cases() {
        let path = sample_corpus_root().join(case.file_name);
        let result = probe_file(&path, 1_024);

        match case.expectation {
            RegressionExpectation::Readable => {
                let readable = result.readable_outcome().unwrap_or_else(|| {
                    panic!("{} should be readable: {:?}", case.file_name, result)
                });
                assert_eq!(
                    readable.decoded_rows, readable.row_count,
                    "{} should drain the full streamed decode path",
                    case.file_name
                );
            }
            RegressionExpectation::Failure {
                kind,
                detail_contains,
            } => {
                assert_eq!(
                    result.failure_kind(),
                    Some(kind),
                    "unexpected failure kind for {}",
                    case.file_name
                );
                let detail = result.failure_detail().unwrap_or_default();
                assert!(
                    detail.contains(detail_contains),
                    "failure detail for {} should mention {:?}, got {:?}",
                    case.file_name,
                    detail_contains,
                    detail
                );
            }
        }
    }
}
