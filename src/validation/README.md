# `src/validation`

This module owns the real-fixture validation harnesses used to prove and preserve the supported corpus boundary.

## What It Does

- defines the curated real-fixture regression corpus used by `tests/validation_contract.rs`
- probes `.sas7bdat` files through the parser entrypoint and drains streamed batches for honest pass/fail reporting
- sweeps `sample-sas-datasets/` and classifies each failure as either expected-invalid or a real compatibility gap
- compares the currently supported semantic surface against `haven` when `Rscript` and the `haven` package are available

## Entry Points

```bash
cargo test --test validation_contract
cargo run --bin sample_corpus_sweep -- --output .tmp/pr06-corpus-sweep.txt
cargo run --bin differential_validate -- --output .tmp/pr06-differential.txt
```

## Honest Boundary

- `fts0003.sas7bdat` remains a required readable baseline in the curated corpus
- the curated corpus also includes expected-invalid real fixtures, so malformed or non-SAS samples stay explicit instead of being silently dropped
- `differential_validate` is scoped to the semantic surface that is currently practical to compare against `haven`: real date/time values and real special-missing behavior
- the sample-corpus sweep exits nonzero only when valid sample datasets still fail, while continuing to report expected-invalid fixtures as explicit evidence
