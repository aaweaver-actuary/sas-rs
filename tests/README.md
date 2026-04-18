# `tests`

This directory contains the Rust test suites for the active SAS-rs crate.

## Test Layout

- [parser_contract.rs](parser_contract.rs): parser subset identity and explicit unsupported-feature rejection
- [parser_decode_contract.rs](parser_decode_contract.rs): metadata decode, batch decode, multi-page decode, and lazy-read honesty checks
- [transform_contract.rs](transform_contract.rs): transform request and sink-plan contract tests
- [transform_parser_integration.rs](transform_parser_integration.rs): parser-to-transform-to-Parquet integration, bounded-memory behavior, parallel batch reporting, and streaming honesty checks
- [cli_transform_contract.rs](cli_transform_contract.rs): CLI request shaping and end-to-end default-service behavior
- [assumption_probe_contract.rs](assumption_probe_contract.rs): deterministic checks for the small synthetic assumption probe
- [validation_contract.rs](validation_contract.rs): curated real-fixture regression corpus, sample-corpus sweep contract coverage, and differential-fixture selection
- [support/README.md](support/README.md): synthetic fixtures and tracked readers shared by the suites

## What These Tests Prove

The current suite is designed to prove three things:

1. the supported subset works end-to-end
2. unsupported cases fail explicitly instead of silently misparsing
3. the project’s lazy-read and bounded-memory claims are backed by tests that would fail if the implementation regressed to eager whole-file or whole-dataset behavior

## What These Tests Do Not Yet Prove

The suite is no longer purely synthetic, but it still does not yet prove:

- universal compatibility across the full real-world `.sas7bdat` corpus
- that every sample-sas-datasets file is currently readable end to end
- full SAS semantic fidelity beyond the explicitly differential-checked date/time and missing-value surface

Those are future-coverage concerns, not regressions in the current supported subset.