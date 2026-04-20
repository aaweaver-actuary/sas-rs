# Journal

This file tracks the in-scope Rust surface and the evidence currently available for each area. Every section now uses checked-in measurements, validation-harness evidence, or an explicit note that no direct function-level performance measurement exists yet and points to the nearest verified measured path or validating harness.

## Sweep Summary

- In-scope Rust files: 28
- Function definitions inventoried: 392
- Latest evidence-backed rewrite scope: the PR-04 final `fts0003` closure kept the representative real-file Criterion surface under `.tmp/pr04-target` and accepted a tiered Parquet writer policy in `src/transform/sink.rs` after `transform_write_real_file/fts0003_wide_schema_serial` improved by roughly 11% to 13% at `p = 0.00 < 0.05`, direct release-CLI maximum resident set size on the same request shape fell from about `700 MB` to `706 MB` down to about `694 MB` to `699 MB`, `10rec` stayed neutral, `numeric_1000000_2` stayed neutral, and the tracked synthetic workloads showed no statistically significant regressions.
- Representative measurement bias: issue #1 favors checked-in sample-dataset results, and issue #7 still matters when interpreting parser-adjacent throughput paths.
- Correctness-history context: issues #6, #8, #9, and #10 remain relevant near parser truncation, encoding, and metadata edge handling.

## Inventory Baseline
- benches/assumption_probe.rs: 1 functions
- benches/parser_decode.rs: 4 functions
- benches/transform_write.rs: 4 functions
- fuzz/fuzz_targets/parser_entry.rs: 0 functions
- src/bin/differential_validate.rs: 2 functions
- src/bin/sample_corpus_sweep.rs: 2 functions
- src/cli.rs: 7 functions
- src/lib.rs: 0 functions
- src/main.rs: 1 functions
- src/parser/constants.rs: 0 functions
- src/parser/contracts.rs: 17 functions
- src/parser/mod.rs: 66 functions
- src/parser/offsets.rs: 8 functions
- src/transform/assumptions.rs: 3 functions
- src/transform/contracts.rs: 2 functions
- src/transform/mod.rs: 0 functions
- src/transform/pipeline.rs: 26 functions
- src/transform/sink.rs: 58 functions
- src/validation/contracts.rs: 16 functions
- src/validation/mod.rs: 27 functions
- tests/assumption_probe_contract.rs: 2 functions
- tests/cli_transform_contract.rs: 7 functions
- tests/parser_contract.rs: 6 functions
- tests/parser_decode_contract.rs: 23 functions
- tests/transform_contract.rs: 5 functions
- tests/transform_parser_integration.rs: 36 functions
- tests/validation_contract.rs: 6 functions
- tests/support/minimal_sas_fixture.rs: 63 functions

## File: benches/assumption_probe.rs

Coverage note: 1 function definitions were found in this file. This section is grounded in the checked-in assumption-probe memory entry plus a refreshed Criterion run on 2026-04-18.

### Function: projection_assumption_probe

- Location: benches/assumption_probe.rs:6
- Signature: `fn projection_assumption_probe(criterion: &mut Criterion) {`
- Role / observation: Criterion entrypoint for the synthetic projection probe over 8-column row-major batches while selecting columns 0, 3, and 5.
- Verified evidence: The documented quick run on 2026-04-18 measured `projection_assumption_probe/16384` at `[20.525 us 21.212 us 21.657 us]` and `projection_assumption_probe/131072` at `[165.90 us 167.41 us 168.65 us]`.
- Measurement scope: Those timings cover the full `projection_assumption_probe` harness calling `run_projection_probe` on synthetic batches built by `build_synthetic_row_batch`.
- Function-level status: This function is the measured entrypoint itself; no narrower timing inside the wrapper is separated in the checked-in evidence.

## File: benches/parser_decode.rs

Coverage note: 4 function definitions were found in this file. No checked-in artifact records isolated helper timings for this benchmark file, so each entry names the nearest verified measured path instead of inventing per-function numbers.

### Function: parser_decode_benchmark

- Location: benches/parser_decode.rs:23
- Signature: `fn parser_decode_benchmark(criterion: &mut Criterion) {`
- Role / observation: Criterion entrypoint for supported-subset streamed decode runs at 16,384, 131,072, and 262,144 rows plus a representative `fts0003` real-file baseline.
- Verified evidence: On 2026-04-18, `cargo bench --bench parser_decode -- --noplot --sample-size 25 --measurement-time 0.8 --warm-up-time 0.2 --save-baseline pr01_issue7_baseline` followed by the same command with `--baseline pr01_issue7_baseline` saved a direct Criterion comparison under `.tmp/pr01-target/criterion` plus `.tmp/pr01-baseline-console.txt` and `.tmp/pr01-candidate-console.txt`. The baseline medians for the tracked workloads were `16384` at `[908.98 us 912.28 us 916.74 us]`, `131072` at `[7.2534 ms 7.2741 ms 7.2944 ms]`, `262144` at `[14.454 ms 14.492 ms 14.540 ms]`, and `fts0003_probe` at `[332.58 ms 333.24 ms 334.15 ms]`.
- Measurement scope: Those are direct path-level Criterion measurements for the full parse-and-stream benchmark surface, including `parse()`, `next_batch()`, page reads, and row decoding. They are not isolated helper timings and they remain valid-file throughput evidence only.
- Function-level status: This wrapper now has direct benchmark evidence. Helper-level attribution still comes from the broader path comparison rather than isolated microbenchmarks.

### Function: build_benchmark_fixture

- Location: benches/parser_decode.rs:84
- Signature: `fn build_benchmark_fixture(row_count: usize) -> Vec<u8> {`
- Role / observation: Synthetic fixture builder for the supported-subset parser benchmark.
- Verified evidence: No direct timing is checked in for this helper. The covering measured paths are the `parser_decode_supported_subset` benchmark group described in code for 16,384, 131,072, and 262,144 rows, plus the broader PR-07 `fts0003` real-file outcome at `2.65 real`, `staged_rows=10275`, and `staged_batches=6`.
- Measurement scope: The recorded `fts0003` result does not isolate fixture construction; it names the nearest measured path that exercises the benchmark this helper feeds.
- Function-level status: No direct function-level measurement exists yet.

### Function: fts0003_path

- Location: benches/parser_decode.rs:97
- Signature: `fn fts0003_path() -> PathBuf {`
- Role / observation: Path helper for the representative wide real-file fixture used by the parser baseline.
- Verified evidence: No standalone timing is checked in for this helper. The covering measured path is the PR-07 `fts0003` real-file result: pass with `parser_subset=sas7bdat-32le-binary-compressed-v1`, `staged_rows=10275`, `staged_batches=6`, and `2.65 real`.
- Measurement scope: This helper only participates in the real-file baseline setup, so the recorded evidence is path-level rather than function-level.
- Function-level status: No direct function-level measurement exists yet.

### Function: probe_fts0003_via_parser_entrypoint

- Location: benches/parser_decode.rs:103
- Signature: `fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {`
- Role / observation: Real-file baseline probe that parses `fts0003.sas7bdat` and drains `next_batch()` until decode completion.
- Verified evidence: `benches/README.md` identifies this real-file probe as the honest streamed decode baseline, and the checked-in PR-07 notes record the corresponding representative `fts0003` path as pass with `parser_subset=sas7bdat-32le-binary-compressed-v1`, `staged_rows=10275`, `staged_batches=6`, and `2.65 real`.
- Measurement scope: The numeric result is for the broader release CLI transform path, not an isolated Criterion timing for just this helper, but it directly covers the same representative fixture and streamed-read surface.
- Function-level status: No narrower checked-in function-only timing exists yet.

## File: benches/transform_write.rs

Coverage note: 4 function definitions were found in this file. The repository now has direct Criterion timing for both the synthetic transform surface and representative real-file transform workloads saved under `.tmp/pr04-target`, plus repeatable direct release-CLI RSS evidence for `10rec` under `.tmp/pr03-10rec-rss-baseline.txt` and `.tmp/pr03-10rec-rss-candidate-final.txt` and for `fts0003` under `.tmp/pr04-fts0003-rss-baseline.txt` and `.tmp/pr04-fts0003-rss-candidate-hybrid.txt`, although individual helper timings are still not isolated.

### Function: open

- Location: benches/transform_write.rs:24
- Signature: `    fn open(`
- Role / observation: In-memory `SourceDataLoader` method used so the benchmark measures parser, transform, and sink work without filesystem source-open noise.
- Verified evidence: No standalone timing is checked in for this helper. The covering measured paths are the `transform_write` benchmark target described in `benches/README.md` and the PR-07 real-file transform results, especially `numeric_1000000_2.sas7bdat` serial at `0.14 real`, parallel at `0.09 real`, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those results cover broader end-to-end transform execution rather than isolating this loader method.
- Function-level status: No direct function-level measurement exists yet.

### Function: transform_write_benchmark

- Location: benches/transform_write.rs:32
- Signature: `fn transform_write_benchmark(criterion: &mut Criterion) {`
- Role / observation: Criterion entrypoint for the supported-subset end-to-end transform benchmark plus the representative real-file transform workloads `10rec.sas7bdat`, `fts0003.sas7bdat`, and `numeric_1000000_2.sas7bdat`.
- Verified evidence: On 2026-04-18, the saved representative baseline under `.tmp/pr04-target/criterion` was compared against the final hybrid candidate with `cargo bench --bench transform_write -- --noplot --sample-size 10 --measurement-time 0.3 --warm-up-time 0.1 --baseline pr04_fts0003_final_baseline`, producing `.tmp/pr04-candidate-console-hybrid.txt`. The accepted candidate preserved the PR-03 ultra-wide `10rec` safeguard while adding a statistics-only middle tier for `fts0003`-class schemas, improving `transform_write_real_file/fts0003_wide_schema_serial` by `[-13.047% -11.728% -10.938%]` with `p = 0.00 < 0.05`, while `10rec_wide_schema_serial` stayed neutral with `p = 0.75`, `numeric_1000000_2_serial` stayed neutral with `p = 0.38`, and the tracked synthetic workloads showed no statistically significant regressions.
- Measurement scope: Those timings are direct path-level Criterion measurements for the full parser-transform-sink path, including Parquet writing, on both synthetic fixtures and representative real files. Matching release-CLI runs under `.tmp/pr03-10rec-rss-baseline.txt`, `.tmp/pr03-10rec-rss-candidate-final.txt`, `.tmp/pr04-fts0003-rss-baseline.txt`, and `.tmp/pr04-fts0003-rss-candidate-hybrid.txt` also directly measure peak RSS on the representative `10rec` and `fts0003` request shapes. The evidence still does not isolate helper-level attribution or prove cold-cache behavior.
- Function-level status: This wrapper now has direct benchmark evidence on both synthetic and representative real-file paths; narrower helper attribution still comes from the broader path comparison rather than isolated microbenchmarks.

### Function: bench_request

- Location: benches/transform_write.rs:77
- Signature: `fn bench_request(output_path: PathBuf, worker_threads: usize) -> TransformRequest {`
- Role / observation: Helper that fixes the benchmark onto bounded-memory streaming with configurable worker-thread counts.
- Verified evidence: No standalone timing is checked in for this helper. The covering measured paths are the PR-07 bounded-memory real-file runs: `dates.sas7bdat` with `row_group_rows=2` and `0.00 real`, `10rec.sas7bdat` with `row_group_rows=4` and `0.92 real`, `fts0003.sas7bdat` with `row_group_rows=2048` and `2.65 real`, and `numeric_1000000_2.sas7bdat` with `row_group_rows=65536` at `0.14 real` serial and `0.09 real` with 4 worker threads.
- Measurement scope: Those results verify the request shape this helper constructs, but they do not isolate the helper's own cost.
- Function-level status: No direct function-level measurement exists yet.

### Function: build_benchmark_fixture

- Location: benches/transform_write.rs:104
- Signature: `fn build_benchmark_fixture(row_count: usize) -> Vec<u8> {`
- Role / observation: Synthetic fixture builder for the narrow supported-subset transform benchmark.
- Verified evidence: No direct timing is checked in for this helper. The covering measured paths are the `transform_write_supported_subset` benchmark target described in code and the broader PR-07 real-file outcomes for `numeric_1000000_2.sas7bdat`, `10rec.sas7bdat`, and `fts0003.sas7bdat`.
- Measurement scope: The cited real-file results measure end-to-end transform throughput and memory behavior, not fixture construction in isolation.
- Function-level status: No direct function-level measurement exists yet.
## File: fuzz/fuzz_targets/parser_entry.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass. README.md documents `cargo fuzz run parser_entry -- -max_total_time=30` as the nearest verified harness for this target; no checked-in timing is recorded, and this section does not claim malformed-input closure.

## File: src/bin/differential_validate.rs

Coverage note: 2 function definitions were found in this file. README.md documents this binary as a reviewable validation harness, and tests/validation_contract.rs plus src/validation/mod.rs define the nearest verified behavior; no checked-in timing isolates these wrapper functions.

### Function: main

- Location: src/bin/differential_validate.rs:7
- Signature: `fn main() {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: README.md documents `cargo run --bin differential_validate -- --output .tmp/pr06-differential.txt` as the differential validation harness, and tests/validation_contract.rs verifies that the supported semantic surface still includes the `dates.sas7bdat` and `missing_test.sas7bdat` fixture set it depends on.
- Measurement scope: No checked-in runtime measurement isolates this wrapper. The nearest verified evidence is harness-level rather than timing-level, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed fixtures.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: usage

- Location: src/bin/differential_validate.rs:58
- Signature: `fn usage(message: &str) -> ! {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: README.md documents `cargo run --bin differential_validate -- --output .tmp/pr06-differential.txt` as the differential validation harness, and tests/validation_contract.rs verifies that the supported semantic surface still includes the `dates.sas7bdat` and `missing_test.sas7bdat` fixture set it depends on.
- Measurement scope: No checked-in runtime measurement isolates this wrapper. The nearest verified evidence is harness-level rather than timing-level, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed fixtures.
- Function-level status: No direct function-level performance measurement exists yet.

## File: src/bin/sample_corpus_sweep.rs

Coverage note: 2 function definitions were found in this file. README.md documents this binary as the sample-corpus validation harness, and tests/validation_contract.rs plus src/validation/mod.rs define the nearest verified behavior; no checked-in timing isolates these wrapper functions.

### Function: main

- Location: src/bin/sample_corpus_sweep.rs:7
- Signature: `fn main() {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: README.md documents `cargo run --bin sample_corpus_sweep -- --output .tmp/pr06-corpus-sweep.txt` as the corpus-sweep harness, and tests/validation_contract.rs verifies mixed-result reporting, explicit expected-invalid handling, and the curated readable-fixture baseline that this binary reports.
- Measurement scope: No checked-in runtime measurement isolates this wrapper. The nearest verified evidence is harness-level rather than timing-level, and it does not imply malformed-input closure or full compatibility beyond the fixtures explicitly classified today.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: usage

- Location: src/bin/sample_corpus_sweep.rs:78
- Signature: `fn usage(message: &str) -> ! {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: README.md documents `cargo run --bin sample_corpus_sweep -- --output .tmp/pr06-corpus-sweep.txt` as the corpus-sweep harness, and tests/validation_contract.rs verifies mixed-result reporting, explicit expected-invalid handling, and the curated readable-fixture baseline that this binary reports.
- Measurement scope: No checked-in runtime measurement isolates this wrapper. The nearest verified evidence is harness-level rather than timing-level, and it does not imply malformed-input closure or full compatibility beyond the fixtures explicitly classified today.
- Function-level status: No direct function-level performance measurement exists yet.

## File: src/cli.rs

Coverage note: 7 function definitions were found in this file. No checked-in artifact isolates CLI parsing, error formatting, or request construction by helper. The nearest verified measured path is the PR-07 release CLI transform set in benches/pr07_real_file_notes.md: dates.sas7bdat at 0.00 real, issue_pandas.sas7bdat at 0.00 real, sample_bincompressed.sas7bdat at 0.00 real, 10rec.sas7bdat at 0.92 real, fts0003.sas7bdat at 2.65 real, and numeric_1000000_2.sas7bdat at 0.14 real serial versus 0.09 real with four workers.

### Function: into_request

- Location: src/cli.rs:50
- Signature: `    fn into_request(self) -> TransformRequest {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/cli.rs:87
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: exit_code

- Location: src/cli.rs:101
- Signature: `    pub fn exit_code(&self) -> ExitCode {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/cli.rs:110
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: source

- Location: src/cli.rs:119
- Signature: `    fn source(&self) -> Option<&(dyn Error + 'static)> {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: run<I, T>

- Location: src/cli.rs:127
- Signature: `pub fn run<I, T>(args: I) -> Result<CommandOutcome, CliError>`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

### Function: run_with_service<I, T, S>

- Location: src/cli.rs:140
- Signature: `pub fn run_with_service<I, T, S>(args: I, service: &S) -> Result<CommandOutcome, CliError>`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

## File: src/lib.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass. This module-export surface is covered indirectly by the verified parser, transform, CLI, and validation harnesses cited elsewhere; no direct function-level performance measurement applies here.

## File: src/main.rs

Coverage note: 1 function definitions were found in this file. No checked-in artifact isolates the process entrypoint itself. The nearest verified measured path is the same PR-07 release CLI transform set named in the src/cli.rs section above.

### Function: main

- Location: src/main.rs:3
- Signature: `fn main() -> ExitCode {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-07 release `sasrs transform` workload set named in the section note above.
- Measurement scope: Those results cover full CLI parse, request construction, parser, transform, and sink execution on valid files only; they do not isolate this helper, and they are not malformed-input evidence.
- Function-level status: No direct function-level measurement exists yet.

## File: src/parser/constants.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass. The constant table is exercised indirectly by the parser entrypoints, parser contract suites, and the representative real-file paths cited elsewhere; no standalone timing is checked in for this surface.

## File: src/parser/contracts.rs

Coverage note: 17 function definitions were found in this file. These parser contracts and value wrappers are exercised indirectly by the refreshed `parser_decode` benchmark and the parser contract suites; no checked-in timing isolates individual helper methods in this file.

### Function: supported_subset_name

- Location: src/parser/contracts.rs:57
- Signature: `fn supported_subset_name(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: supported_subset

- Location: src/parser/contracts.rs:102
- Signature: `pub fn supported_subset(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: new

- Location: src/parser/contracts.rs:128
- Signature: `    pub fn new(source_name: &'a str, reader: BoxedParserDataSource) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: from_bytes

- Location: src/parser/contracts.rs:135
- Signature: `    pub fn from_bytes(source_name: &'a str, bytes: Vec<u8>) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: from_reader<R>

- Location: src/parser/contracts.rs:139
- Signature: `    pub fn from_reader<R>(source_name: &'a str, reader: R) -> Self`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: label

- Location: src/parser/contracts.rs:163
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: code

- Location: src/parser/contracts.rs:189
- Signature: `    pub fn code(&self) -> char {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: from_code

- Location: src/parser/contracts.rs:197
- Signature: `    pub fn from_code(tag: char) -> Option<Self> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: deferred_bytes

- Location: src/parser/contracts.rs:231
- Signature: `    pub fn deferred_bytes(raw_bytes: Vec<u8>) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: as_f64

- Location: src/parser/contracts.rs:238
- Signature: `    pub fn as_f64(&self) -> Option<f64> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: raw_bits

- Location: src/parser/contracts.rs:245
- Signature: `    pub fn raw_bits(&self) -> Option<u64> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: width_bytes

- Location: src/parser/contracts.rs:252
- Signature: `    pub fn width_bytes(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: raw_bytes

- Location: src/parser/contracts.rs:259
- Signature: `    pub fn raw_bytes(&self) -> Option<&[u8]> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: missing_tag

- Location: src/parser/contracts.rs:266
- Signature: `    pub fn missing_tag(&self) -> Option<SasMissingTag> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: from

- Location: src/parser/contracts.rs:275
- Signature: `    fn from(value: f64) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: fmt

- Location: src/parser/contracts.rs:340
- Signature: `    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: new_streaming

- Location: src/parser/contracts.rs:354
- Signature: `    pub(crate) fn new_streaming(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks supported-subset naming, layout metadata, and explicit rejection behavior for this contract surface.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these contract helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: src/parser/mod.rs

Coverage note: 66 function definitions were found in this file. The page, pointer, batch-loading, value-decoding, metadata, decompression, and remaining parser entries are now grounded in the refreshed parser benchmark evidence, the checked-in real-file notes, and explicit no-direct-measurement caveats where only broader path coverage exists.

### Function: fmt

- Location: src/parser/mod.rs:57
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: fmt

- Location: src/parser/mod.rs:104
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse

- Location: src/parser/mod.rs:116
- Signature: `    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError>;`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse

- Location: src/parser/mod.rs:123
- Signature: `    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: from_code

- Location: src/parser/mod.rs:203
- Signature: `    fn from_code(code: u8) -> Option<Self> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: decode

- Location: src/parser/mod.rs:217
- Signature: `    fn decode(self, bytes: &[u8]) -> Result<String, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: from_header_prefix

- Location: src/parser/mod.rs:242
- Signature: `    fn from_header_prefix(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: word_size_bytes

- Location: src/parser/mod.rs:271
- Signature: `    fn word_size_bytes(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: page_header_size

- Location: src/parser/mod.rs:278
- Signature: `    fn page_header_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: subheader_pointer_size

- Location: src/parser/mod.rs:285
- Signature: `    fn subheader_pointer_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: subheader_data_offset

- Location: src/parser/mod.rs:292
- Signature: `    fn subheader_data_offset(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: column_attrs_entry_size

- Location: src/parser/mod.rs:299
- Signature: `    fn column_attrs_entry_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: subheader_signature_size

- Location: src/parser/mod.rs:303
- Signature: `    fn subheader_signature_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: row_size_min_len

- Location: src/parser/mod.rs:310
- Signature: `    fn row_size_min_len(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: column_format_min_len

- Location: src/parser/mod.rs:317
- Signature: `    fn column_format_min_len(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: row_size_offsets

- Location: src/parser/mod.rs:324
- Signature: `    fn row_size_offsets(self) -> RowSizeOffsets {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_word

- Location: src/parser/mod.rs:339
- Signature: `    fn read_word(self, bytes: &[u8], offset: usize) -> Result<u64, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_supported_subset

- Location: src/parser/mod.rs:347
- Signature: `fn parse_supported_subset(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: next_batch

- Location: src/parser/mod.rs:527
- Signature: `    pub fn next_batch(&mut self, batch_size_rows: usize) -> Result<Option<RowBatch>, ParserError> {`
- Role / observation: Streaming parser entrypoint that fills pending rows as needed, drains up to `batch_size_rows`, and returns ordered `RowBatch` windows.
- Verified evidence: The refreshed 2026-04-18 `parser_decode` Criterion run measures the path that repeatedly calls `next_batch(8192)` until exhaustion: `parser_decode_supported_subset/16384` at `[851.18 us 882.37 us 914.94 us]`, `parser_decode_supported_subset/131072` at `[6.9686 ms 7.0261 ms 7.0639 ms]`, `parser_decode_supported_subset/262144` at `[14.000 ms 14.189 ms 14.478 ms]`, and `parser_decode_real_file_baseline/fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: Those timings cover the full parse-and-stream loop in `benches/parser_decode.rs`, including `parse()`, `fill_pending_rows`, `load_next_row_source`, page reads, and row decoding. They are valid-file throughput measurements only, not malformed-input robustness evidence.
- Function-level status: No isolated timing for `next_batch` alone is checked in; the benchmark above is the nearest direct measured path.

### Function: fill_pending_rows

- Location: src/parser/mod.rs:550
- Signature: `    fn fill_pending_rows(&mut self, min_rows: usize) -> Result<(), ParserError> {`
- Role / observation: Internal batch-filling loop that keeps loading row sources until enough decoded rows are buffered, sources are exhausted, or the declared row count is reached.
- Verified evidence: The same refreshed 2026-04-18 `parser_decode` run covers this loop while draining supported-subset fixtures at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, and `[14.000 ms 14.189 ms 14.478 ms]`, plus the representative valid-file `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: The benchmark measures broader streamed decode throughput rather than this loop in isolation, so it does not separate buffer-filling cost from page I/O or row parsing cost.
- Function-level status: No direct function-level measurement exists yet; the refreshed streamed-decode benchmark is the nearest verified measured path.

### Function: load_next_row_source

- Location: src/parser/mod.rs:573
- Signature: `    fn load_next_row_source(&mut self) -> Result<(), ParserError> {`
- Role / observation: Batch-loading helper that reads the next referenced page, materializes subheader-backed rows, and appends raw-row slices into `pending_rows`.
- Verified evidence: The saved 2026-04-18 issue-7 baseline/candidate comparison under `.tmp/pr01-target/criterion` measures the parse-and-stream path containing this helper. The final candidate improved three tracked synthetic workloads with no statistically significant real-file regression: `16384` mean time change `[-2.0708% -1.1598% -0.3293%]` with `p = 0.01 < 0.05`, `131072` `[-2.7074% -2.2883% -1.8668%]` with `p = 0.00 < 0.05`, `262144` `[-2.0690% -1.6579% -1.2329%]` with `p = 0.00 < 0.05`, and `fts0003_probe` `p = 0.59`.
- Measurement scope: That comparison is still path-level rather than helper-isolated. It shows that this batch-loading region participated in the measured candidate win, but it does not uniquely attribute the sub-1% improvement to row loading versus the narrower `read_page_header` allocation removal.
- Function-level status: No isolated timing for `load_next_row_source` is checked in yet.

### Function: decoded_row_count

- Location: src/parser/mod.rs:623
- Signature: `    fn decoded_row_count(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_meta_page

- Location: src/parser/mod.rs:628
- Signature: `fn parse_meta_page(`
- Role / observation: Metadata-page walker that parses subheader pointers, accumulates column metadata, and records any row sources exposed by META, MIX, or AMD pages.
- Verified evidence: Every iteration of the refreshed 2026-04-18 `parser_decode` benchmark calls `parser.parse(...)` before batch draining, so the measured streamed-decode path includes metadata-page parsing on the supported synthetic fixtures and the representative valid-file `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]` alongside the synthetic groups at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, and `[14.000 ms 14.189 ms 14.478 ms]`.
- Measurement scope: The timings are for full parse-and-stream runs, not `parse_meta_page` in isolation. They show that metadata-page parsing is on the measured valid-file path, but they do not imply malformed-metadata robustness or attribute a specific share of runtime to this helper.
- Function-level status: No direct function-level timing is checked in yet.

### Function: parse_row_size_subheader

- Location: src/parser/mod.rs:726
- Signature: `fn parse_row_size_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_column_size_subheader

- Location: src/parser/mod.rs:780
- Signature: `fn parse_column_size_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_column_text_subheader

- Location: src/parser/mod.rs:796
- Signature: `fn parse_column_text_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_column_name_subheader

- Location: src/parser/mod.rs:808
- Signature: `fn parse_column_name_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_column_attrs_subheader

- Location: src/parser/mod.rs:848
- Signature: `fn parse_column_attrs_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: parse_column_format_subheader

- Location: src/parser/mod.rs:907
- Signature: `fn parse_column_format_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: signature_is_recognized

- Location: src/parser/mod.rs:948
- Signature: `fn signature_is_recognized(signature: u32) -> bool {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: effective_subheader_compression

- Location: src/parser/mod.rs:962
- Signature: `fn effective_subheader_compression(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: mix_raw_data_offset

- Location: src/parser/mod.rs:973
- Signature: `fn mix_raw_data_offset(`
- Role / observation: MIX-page helper that computes where raw row bytes begin after the page header, subheader pointer table, and optional alignment padding.
- Verified evidence: No direct timing is checked in for MIX-page offset calculation itself. The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` run, which times complete parse-and-stream executions at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, `[14.000 ms 14.189 ms 14.478 ms]`, and `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: Those measurements prove the parser completes representative valid-file parse and batch paths, but the checked-in artifacts do not isolate MIX-page offset arithmetic or prove that every measured fixture exercised this exact helper.
- Function-level status: No direct function-level measurement exists yet; the full streamed parser benchmark is the nearest verified measured path.

### Function: parse_subheader_row

- Location: src/parser/mod.rs:998
- Signature: `fn parse_subheader_row(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: decompress_row_rle

- Location: src/parser/mod.rs:1029
- Signature: `fn decompress_row_rle(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {`
- Role / observation: Row-compression decoder that expands subheader-backed payloads into full row bytes before column slicing begins.
- Verified evidence: No isolated timing is checked in for this helper. The nearest verified measured path is the PR-07 release CLI run on `issue_pandas.sas7bdat`, which passed with `parser_subset=sas7bdat-32le-row-compressed-v1`, `staged_rows=7`, `staged_batches=4`, and `0.00 real`; parser contract coverage also drains a row-compressed mixed-page fixture end to end.
- Measurement scope: That result proves the current end-to-end path can decode representative row-compressed rows, but it is broader than `decompress_row_rle` alone and does not attribute a specific share of runtime to the decompressor. It also does not imply malformed-row robustness from valid-file throughput evidence.
- Function-level status: No direct function-level measurement exists yet; the `issue_pandas.sas7bdat` real-file transform path is the nearest verified measured path.

### Function: decompress_row_binary

- Location: src/parser/mod.rs:1138
- Signature: `fn decompress_row_binary(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {`
- Role / observation: Binary-compression decoder that reconstructs full row bytes from control-word and back-reference payloads before value parsing.
- Verified evidence: No isolated timing is checked in for this helper. The refreshed 2026-04-18 `parser_decode_real_file_baseline/fts0003_probe` benchmark measured the representative binary-compressed parser path at `[320.81 ms 321.89 ms 322.82 ms]`, and the PR-07 real-file notes also record `sample_bincompressed.sas7bdat` as pass with `parser_subset=sas7bdat-64le-binary-compressed-v1`, `staged_rows=5`, `staged_batches=3`, and `0.00 real`, plus `fts0003.sas7bdat` as pass with `parser_subset=sas7bdat-32le-binary-compressed-v1`, `staged_rows=10275`, `staged_batches=6`, and `2.65 real`.
- Measurement scope: Those measurements cover full parse-and-stream or end-to-end transform execution on valid binary-compressed files. They do not isolate `decompress_row_binary`, and they should not be read as proof about malformed compressed payload handling.
- Function-level status: No direct function-level measurement exists yet; the refreshed `fts0003_probe` parser benchmark is the nearest verified measured path.

### Function: finalize_columns

- Location: src/parser/mod.rs:1242
- Signature: `fn finalize_columns(metadata: &PartialMetadata) -> Result<Vec<SasColumn>, ParserError> {`
- Role / observation: Final metadata assembler that validates column definitions, resolves names, labels, and formats from text blobs, and attaches semantic hints before streaming starts.
- Verified evidence: No isolated timing is checked in for this helper. The refreshed 2026-04-18 `parser_decode` benchmark times complete parse-and-stream runs, so `parse()` necessarily includes `finalize_columns` before the measured batch drains at `[842.51 us 855.29 us 879.01 us]`, `[6.7352 ms 6.7765 ms 6.8178 ms]`, `[13.585 ms 13.702 ms 13.833 ms]`, and `fts0003_probe` at `[320.81 ms 321.89 ms 322.82 ms]`. Parser contract coverage also verifies that semantic metadata survives on synthetic fixtures and on the real `dates.sas7bdat` metadata path.
- Measurement scope: The benchmark evidence is for the broader valid-file parse path, not for `finalize_columns` in isolation, and the checked-in tests establish correctness rather than speed. The valid-file timings do not imply malformed-metadata robustness.
- Function-level status: No direct function-level measurement exists yet; the refreshed `parser_decode` parse-and-stream benchmark is the nearest verified measured path.

### Function: parse_row

- Location: src/parser/mod.rs:1325
- Signature: `fn parse_row(`
- Role / observation: Per-row decoder that slices each column from the raw row buffer and delegates to numeric or text decoding before batching.
- Verified evidence: No isolated timing is checked in for this helper. The refreshed 2026-04-18 `parser_decode` benchmark drains the full streamed decode path on supported fixtures at `[842.51 us 855.29 us 879.01 us]`, `[6.7352 ms 6.7765 ms 6.8178 ms]`, and `[13.585 ms 13.702 ms 13.833 ms]`, plus the representative real-file `fts0003_probe` at `[320.81 ms 321.89 ms 322.82 ms]`. The PR-07 real-file notes also show broader end-to-end decode-heavy paths completing on `10rec.sas7bdat`, `sample_bincompressed.sas7bdat`, and `fts0003.sas7bdat`.
- Measurement scope: Those measurements prove `parse_row` sits on the measured valid-file decode path, but they include surrounding page reads, decompression, and downstream batch assembly. They do not isolate row parsing alone or prove truncated-row behavior from throughput evidence.
- Function-level status: No direct function-level measurement exists yet; the refreshed `parser_decode` streamed benchmark is the nearest verified measured path.

### Function: parse_numeric_value

- Location: src/parser/mod.rs:1353
- Signature: `fn parse_numeric_value(`
- Role / observation: Numeric-cell decoder that returns deferred bytes for 1-byte through 7-byte values and full `f64` plus SAS missing-tag metadata for 8-byte cells.
- Verified evidence: No isolated timing is checked in for this helper. The refreshed 2026-04-18 `parser_decode` benchmark repeatedly decodes the synthetic fixture's numeric column at `[842.51 us 855.29 us 879.01 us]`, `[6.7352 ms 6.7765 ms 6.8178 ms]`, and `[13.585 ms 13.702 ms 13.833 ms]`, and the PR-07 real-file notes record the numeric-heavy `numeric_1000000_2.sas7bdat` transform path at `0.14 real` serial and `0.09 real` with four workers. Contract coverage proves that narrow numerics remain deferred instead of being rejected and that real SAS special missing tags remain distinguishable.
- Measurement scope: The measured paths are broader than `parse_numeric_value` alone and do not isolate the relative cost of 1-byte through 7-byte handling, 8-byte `f64` decoding, or missing-tag interpretation. The checked-in results also do not imply malformed-input resilience beyond the exercised valid files.
- Function-level status: No direct function-level measurement exists yet; the refreshed `parser_decode` benchmark and the million-row transform result are the nearest verified measured paths.

### Function: decorate_format_name

- Location: src/parser/mod.rs:1386
- Signature: `fn decorate_format_name(`
- Role / observation: Metadata formatter that appends width and precision suffixes to resolved SAS format names before column metadata is finalized.
- Verified evidence: No isolated timing is checked in for this helper. The nearest verified measured path is the PR-07 release CLI run on `dates.sas7bdat`, which passed with `parsed_rows=19`, `staged_rows=15`, `selection_applied=applied`, `filter_applied=applied`, and `0.00 real`; parser contract coverage also verifies synthetic and real metadata paths that preserve format names and semantic hints for `DATETIME`, `DATE`, and `TIME` columns.
- Measurement scope: The `dates.sas7bdat` result is an end-to-end transform measurement, not a timing for format-name decoration by itself. It shows this metadata path participates in a verified valid-file run, but it does not quantify the helper's cost.
- Function-level status: No direct function-level measurement exists yet; the `dates.sas7bdat` real-file transform path is the nearest verified measured path.

### Function: semantic_type_from_metadata

- Location: src/parser/mod.rs:1401
- Signature: `fn semantic_type_from_metadata(metadata: &ColumnMetadata) -> SemanticTypeHint {`
- Role / observation: Metadata classifier that chooses the column's semantic hint from resolved format and informat names.
- Verified evidence: No isolated timing is checked in for this helper. The nearest verified measured path is again the PR-07 `dates.sas7bdat` release CLI transform at `0.00 real` with selection and filtering enabled, while parser contract coverage confirms that synthetic fixture formats and the real `dates.sas7bdat` metadata map to `DateTime`, `Date`, `Time`, and `Duration` as expected.
- Measurement scope: The measured CLI run is broader than semantic-hint inference alone, and the tests establish correctness rather than speed. The evidence therefore supports only path participation, not a standalone cost claim.
- Function-level status: No direct function-level measurement exists yet; the `dates.sas7bdat` real-file transform path is the nearest verified measured path.

### Function: semantic_type_from_format_name

- Location: src/parser/mod.rs:1415
- Signature: `fn semantic_type_from_format_name(format_name: &str) -> Option<SemanticTypeHint> {`
- Role / observation: Prefix-based classifier that maps decorated SAS format names onto deferred date, time, datetime, or duration hints.
- Verified evidence: No isolated timing is checked in for this helper. The nearest verified measured path is the PR-07 `dates.sas7bdat` release CLI transform at `0.00 real`, and parser contract coverage verifies both synthetic metadata fixtures and the real `dates.sas7bdat` column formats.
- Measurement scope: That measured path covers the surrounding parser and transform flow, not this string-prefix matcher in isolation. It should not be read as proof of every possible format-name edge case beyond the exercised fixtures.
- Function-level status: No direct function-level measurement exists yet; the `dates.sas7bdat` real-file transform path is the nearest verified measured path.

### Function: decode_sas_missing_tag

- Location: src/parser/mod.rs:1451
- Signature: `fn decode_sas_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {`
- Role / observation: Special-missing decoder that inspects the raw `f64` payload and preserves SAS dot, underscore, and letter-tag variants instead of flattening them.
- Verified evidence: No direct measured missing-tag benchmark is checked in for this helper. The nearest verified measured path is the PR-07 `numeric_1000000_2.sas7bdat` transform workload at `0.14 real` serial and `0.09 real` with four workers, while parser and transform contract coverage verifies real special-missing tags from `missing_test.sas7bdat` and big-endian narrow-numeric missing-tag materialization.
- Measurement scope: The measured numeric throughput paths are broader than missing-tag decoding itself and do not isolate how often this branch executes. They therefore support only the claim that numeric decoding is exercised on representative workloads, not a direct timing for `decode_sas_missing_tag`.
- Function-level status: No direct function-level measurement exists yet; the million-row numeric transform path is the nearest verified measured path.

### Function: parse_subheader_pointers

- Location: src/parser/mod.rs:1466
- Signature: `fn parse_subheader_pointers(`
- Role / observation: Pointer-table parser that validates subheader pointer bounds and records offset, length, and compression metadata for later page handling.
- Verified evidence: The refreshed 2026-04-18 `parser_decode` benchmark times parse-and-stream runs whose `parse()` phase must build row sources from page metadata, with synthetic groups at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, `[14.000 ms 14.189 ms 14.478 ms]`, plus the representative valid-file `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: The benchmark does not break out pointer-table parsing separately, so it is evidence for the broader valid-file parser path rather than for `parse_subheader_pointers` alone.
- Function-level status: No isolated timing is checked in yet.

### Function: subheader_slice

- Location: src/parser/mod.rs:1521
- Signature: `fn subheader_slice(page: &[u8], pointer: SubheaderPointer) -> Result<&[u8], ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: ensure_column_capacity

- Location: src/parser/mod.rs:1528
- Signature: `fn ensure_column_capacity(metadata: &mut PartialMetadata, len: usize) {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: ensure_remainder

- Location: src/parser/mod.rs:1534
- Signature: `fn ensure_remainder(subheader: &[u8], layout: DecodeLayout) -> Result<(), ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: resolve_text

- Location: src/parser/mod.rs:1551
- Signature: `fn resolve_text(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_page_header

- Location: src/parser/mod.rs:1582
- Signature: `fn read_page_header(`
- Role / observation: Per-page header reader used during `parse_supported_subset` to classify each page before deciding whether the parser needs the full page body.
- Verified evidence: Issue `#7` was tested directly by replacing the per-page `Vec<u8>` allocation in this helper with a reusable fixed 40-byte scratch buffer and comparing the existing `parser_decode` benchmark surface against a saved Criterion baseline. The final candidate comparison shows statistically significant improvements on all three tracked synthetic workloads: `16384` mean time change `[-2.0708% -1.1598% -0.3293%]` with `p = 0.01 < 0.05`, `131072` `[-2.7074% -2.2883% -1.8668%]` with `p = 0.00 < 0.05`, and `262144` `[-2.0690% -1.6579% -1.2329%]` with `p = 0.00 < 0.05`. The representative real-file `fts0003_probe` showed no statistically significant regression with `p = 0.59`.
- Measurement scope: This is direct evidence for the exact candidate change, but it is still a path-level parser benchmark rather than an isolated header-read microbenchmark. The saved result supports keeping the change while remaining honest that the observed gain is small and not uniquely attributable beyond this one-code-change comparison.
- Function-level status: No isolated timing for `read_page_header` alone is checked in; the saved Criterion comparison is the nearest direct evidence.

### Function: read_page

- Location: src/parser/mod.rs:1592
- Signature: `fn read_page(`
- Role / observation: Full-page reader used both during metadata discovery and when `load_next_row_source` materializes row data from a referenced page.
- Verified evidence: The refreshed 2026-04-18 `parser_decode` benchmark measures parse-and-stream runs that call `read_page` on the synthetic supported-subset fixtures at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, and `[14.000 ms 14.189 ms 14.478 ms]`, plus the representative valid-file `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: The benchmark proves full-page reads are on the measured valid-file decode path, but it does not isolate page-read cost from pointer parsing, row decoding, or allocation overhead.
- Function-level status: No isolated timing is checked in yet.

### Function: page_offset

- Location: src/parser/mod.rs:1604
- Signature: `fn page_offset(`
- Role / observation: Offset helper that converts header size, page size, and page index into the absolute read location used by parser page I/O.
- Verified evidence: No direct timing is checked in for this arithmetic helper. The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` run, which exercises page reads at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, `[14.000 ms 14.189 ms 14.478 ms]`, and `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: Those timings cover the broader valid-file page-I/O path that depends on `page_offset`; they do not isolate the helper's own cost.
- Function-level status: No direct function-level measurement exists yet.

### Function: read_exact_at

- Location: src/parser/mod.rs:1614
- Signature: `fn read_exact_at(`
- Role / observation: Seek-and-read primitive underneath both `read_page_header` and `read_page` for parser page access.
- Verified evidence: The refreshed 2026-04-18 `parser_decode` benchmark measures valid-file parse-and-stream runs whose page access ultimately flows through `read_exact_at`, with synthetic groups at `[851.18 us 882.37 us 914.94 us]`, `[6.9686 ms 7.0261 ms 7.0639 ms]`, `[14.000 ms 14.189 ms 14.478 ms]`, plus the representative valid-file `fts0003_probe` at `[328.83 ms 329.18 ms 329.51 ms]`.
- Measurement scope: The recorded timings are for end-to-end parser throughput on valid files, not a syscall-level microbenchmark for this helper. They therefore do not attribute specific cost to `seek` versus `read_exact` or imply malformed-input resilience.
- Function-level status: No direct function-level measurement exists yet.

### Function: read_text_ref

- Location: src/parser/mod.rs:1625
- Signature: `fn read_text_ref(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_u16

- Location: src/parser/mod.rs:1637
- Signature: `fn read_u16(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u16, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_u32

- Location: src/parser/mod.rs:1647
- Signature: `fn read_u32(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u32, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_u64

- Location: src/parser/mod.rs:1657
- Signature: `fn read_u64(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u64, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: byte_at

- Location: src/parser/mod.rs:1671
- Signature: `fn byte_at(bytes: &[u8], offset: usize, message: &'static str) -> Result<u8, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: ensure_len

- Location: src/parser/mod.rs:1678
- Signature: `fn ensure_len(bytes: &[u8], min_len: usize, message: &'static str) -> Result<(), ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: pointer_compression_mode

- Location: src/parser/mod.rs:1685
- Signature: `fn pointer_compression_mode(compression: u8) -> CompressionMode {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: read_subheader_signature

- Location: src/parser/mod.rs:1693
- Signature: `fn read_subheader_signature(subheader: &[u8], layout: DecodeLayout) -> Result<u32, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: decode_text_bytes

- Location: src/parser/mod.rs:1707
- Signature: `fn decode_text_bytes(bytes: &[u8], text_encoding_code: u8) -> Result<String, ParserError> {`
- Role / observation: Text decoder that maps the SAS encoding code to a supported codec and converts column names, labels, formats, and string cell bytes into Rust `String` values.
- Verified evidence: No isolated timing is checked in for this helper. The nearest verified measured path is the PR-07 release CLI run on `issue_pandas.sas7bdat`, which passed with `parser_subset=sas7bdat-32le-row-compressed-v1`, `staged_rows=7`, `staged_batches=4`, and `0.00 real`; parser contract coverage also verifies a latin-1 fixture, the real GB18030 first row from `issue_pandas.sas7bdat`, and multiple real non-UTF-8 sample datasets.
- Measurement scope: The measured `issue_pandas.sas7bdat` result is broader than `decode_text_bytes` alone and only proves that the current valid-file path completes with representative encoded text. It does not turn `fts0003` into direct encoding evidence, and it does not imply malformed-text robustness from successful throughput runs.
- Function-level status: No direct function-level measurement exists yet; the `issue_pandas.sas7bdat` real-file transform path is the nearest verified measured path.

### Function: trim_padded_bytes

- Location: src/parser/mod.rs:1714
- Signature: `fn trim_padded_bytes(bytes: &[u8]) -> &[u8] {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: io_error

- Location: src/parser/mod.rs:1722
- Signature: `fn io_error(error: std::io::Error) -> ParserError {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: empty_metadata

- Location: src/parser/mod.rs:1730
- Signature: `    fn empty_metadata() -> PartialMetadata {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: column_name_subheader_underflow_returns_an_error

- Location: src/parser/mod.rs:1751
- Signature: `    fn column_name_subheader_underflow_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: column_attrs_subheader_underflow_returns_an_error

- Location: src/parser/mod.rs:1769
- Signature: `    fn column_attrs_subheader_underflow_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

### Function: row_size_subheader_short_tail_returns_an_error

- Location: src/parser/mod.rs:1787
- Signature: `    fn row_size_subheader_short_tail_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative real-file `fts0003.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, and `dates.sas7bdat` results recorded in benches/pr07_real_file_notes.md where relevant to the helper surface. Existing parser and transform contract tests provide the corresponding correctness harnesses.
- Measurement scope: Those results cover broader valid-file parse-and-stream or end-to-end transform paths rather than this helper in isolation. They therefore support path participation, not a standalone cost claim, and they do not imply malformed-input robustness beyond the explicitly exercised fixtures and rejection cases.
- Function-level status: No direct function-level performance measurement exists yet unless the entry already cites a narrower measured path above.

## File: src/parser/offsets.rs

Coverage note: 8 function definitions were found in this file. These header-offset helpers are exercised indirectly by the refreshed `parser_decode` benchmark and the parser contract suites; no checked-in timing isolates the offset calculations themselves.

### Function: default

- Location: src/parser/offsets.rs:29
- Signature: `    fn default() -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: new

- Location: src/parser/offsets.rs:35
- Signature: `    pub fn new() -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: header_prefix_len

- Location: src/parser/offsets.rs:49
- Signature: `    pub fn header_prefix_len(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: alignment_padding_len

- Location: src/parser/offsets.rs:53
- Signature: `    pub fn alignment_padding_len(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: header_size_offset

- Location: src/parser/offsets.rs:61
- Signature: `    pub fn header_size_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: page_size_offset

- Location: src/parser/offsets.rs:65
- Signature: `    pub fn page_size_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: page_count_offset

- Location: src/parser/offsets.rs:69
- Signature: `    pub fn page_count_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: test_parser_offsets

- Location: src/parser/offsets.rs:79
- Signature: `    fn test_parser_offsets() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Verified evidence: The nearest verified measured path is the refreshed 2026-04-18 `parser_decode` benchmark together with the representative `fts0003.sas7bdat` real-file baseline described in benches/README.md and benches/pr07_real_file_notes.md. Parser contract coverage also locks the expected header-offset behavior and malformed-header rejection paths that depend on this file.
- Measurement scope: Those measurements cover the broader valid-file parse-and-stream path rather than these offset helpers in isolation. They therefore support participation in the measured parser path, not a standalone cost claim, and they do not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: src/transform/assumptions.rs

Coverage note: 3 function definitions were found in this file. This section is grounded in the checked-in `projection_assumption_probe` Criterion results from 2026-04-18: `projection_assumption_probe/16384` at `[20.525 us 21.212 us 21.657 us]` and `projection_assumption_probe/131072` at `[165.90 us 167.41 us 168.65 us]`.

### Function: selected_cell_count

- Location: src/transform/assumptions.rs:9
- Signature: `    pub fn selected_cell_count(&self) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the checked-in `projection_assumption_probe` benchmark named in the section note above.
- Measurement scope: The benchmark covers the full synthetic projection probe over 8-column row-major batches selecting columns 0, 3, and 5; it does not isolate this helper from the surrounding checksum loop or fixture setup.
- Function-level status: No direct function-level measurement exists yet.

### Function: build_synthetic_row_batch

- Location: src/transform/assumptions.rs:20
- Signature: `pub fn build_synthetic_row_batch(row_count: usize, column_count: usize) -> Vec<u64> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the checked-in `projection_assumption_probe` benchmark named in the section note above.
- Measurement scope: The benchmark covers the full synthetic projection probe over 8-column row-major batches selecting columns 0, 3, and 5; it does not isolate this helper from the surrounding checksum loop or fixture setup.
- Function-level status: No direct function-level measurement exists yet.

### Function: run_projection_probe

- Location: src/transform/assumptions.rs:26
- Signature: `pub fn run_projection_probe(batch: &[u64], plan: &ProjectionProbePlan) -> ProjectionProbeResult {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the checked-in `projection_assumption_probe` benchmark named in the section note above.
- Measurement scope: The benchmark covers the full synthetic projection probe over 8-column row-major batches selecting columns 0, 3, and 5; it does not isolate this helper from the surrounding checksum loop or fixture setup.
- Function-level status: No direct function-level measurement exists yet.

## File: src/transform/contracts.rs

Coverage note: 2 function definitions were found in this file. No checked-in artifact isolates these tiny contract helpers. The nearest verified measured path is the bounded-memory real-file CLI set in benches/pr07_real_file_notes.md: dates.sas7bdat with row_group_rows=2 and 0.00 real, 10rec.sas7bdat with row_group_rows=4 and 0.92 real, fts0003.sas7bdat with row_group_rows=2048 and 2.65 real, and numeric_1000000_2.sas7bdat with row_group_rows=65536 at 0.14 real serial and 0.09 real with four workers. Those same notes keep the wide-schema RSS caveat visible for 10rec and fts0003.

### Function: label

- Location: src/transform/contracts.rs:53
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the bounded-memory real-file CLI set named in the section note above.
- Measurement scope: Every PR-07 run used `ExecutionModel::BoundedMemory`, and the recorded row-group caps stayed bounded at 2, 2, 2, 4, 2048, and 65536 rows. That is path-level evidence for the contract claim, not an isolated timing for this helper.
- Function-level status: No direct function-level measurement exists yet.

### Function: supports_larger_than_memory_inputs

- Location: src/transform/contracts.rs:60
- Signature: `    pub fn supports_larger_than_memory_inputs(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the bounded-memory real-file CLI set named in the section note above.
- Measurement scope: Every PR-07 run used `ExecutionModel::BoundedMemory`, and the recorded row-group caps stayed bounded at 2, 2, 2, 4, 2048, and 65536 rows. That is path-level evidence for the contract claim, not an isolated timing for this helper.
- Function-level status: No direct function-level measurement exists yet.

## File: src/transform/mod.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass. This re-export surface is exercised indirectly by the checked-in `transform_write` benchmark and the PR-07 release CLI transform runs; no direct function-level performance measurement applies here.

## File: src/transform/pipeline.rs

Coverage note: 26 function definitions were found in this file. This section is grounded in the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 release CLI results for `dates.sas7bdat`, `issue_pandas.sas7bdat`, `sample_bincompressed.sas7bdat`, `10rec.sas7bdat`, `fts0003.sas7bdat`, and `numeric_1000000_2.sas7bdat`. These are valid-file path measurements only.

### Function: run

- Location: src/transform/pipeline.rs:16
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError>;`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: open

- Location: src/transform/pipeline.rs:20
- Signature: `    fn open(&self, source: &SourceContract)`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: open

- Location: src/transform/pipeline.rs:28
- Signature: `    fn open(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/pipeline.rs:44
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/transform/pipeline.rs:52
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from

- Location: src/transform/pipeline.rs:60
- Signature: `    fn from(error: std::io::Error) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: deferred

- Location: src/transform/pipeline.rs:75
- Signature: `    pub fn deferred() -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_execution

- Location: src/transform/pipeline.rs:85
- Signature: `    pub fn from_execution(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: not_yet_implemented

- Location: src/transform/pipeline.rs:108
- Signature: `    pub fn not_yet_implemented(request: TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: with_sink

- Location: src/transform/pipeline.rs:113
- Signature: `    pub fn with_sink(request: TransformRequest, sink: ParquetSinkReport) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: decoded_rows_staged

- Location: src/transform/pipeline.rs:122
- Signature: `    pub fn decoded_rows_staged(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: parquet_written

- Location: src/transform/pipeline.rs:135
- Signature: `    pub fn parquet_written(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: summary

- Location: src/transform/pipeline.rs:148
- Signature: `    pub fn summary(&self) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/transform/pipeline.rs:179
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: label

- Location: src/transform/pipeline.rs:192
- Signature: `    pub fn label(&self) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/pipeline.rs:207
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/transform/pipeline.rs:215
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from

- Location: src/transform/pipeline.rs:223
- Signature: `    fn from(error: ParquetSinkError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from

- Location: src/transform/pipeline.rs:229
- Signature: `    fn from(error: SourceDataLoaderError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from

- Location: src/transform/pipeline.rs:235
- Signature: `    fn from(error: ParserError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: from

- Location: src/transform/pipeline.rs:241
- Signature: `    fn from(error: TransformExecutionError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/pipeline.rs:252
- Signature: `    pub fn new(sink: S) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: run

- Location: src/transform/pipeline.rs:261
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/pipeline.rs:279
- Signature: `    pub fn new(loader: L, parser: P, sink: S) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: run

- Location: src/transform/pipeline.rs:294
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

### Function: bool_label

- Location: src/transform/pipeline.rs:318
- Signature: `fn bool_label(value: bool) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover end-to-end valid-file parser-to-transform-to-sink execution rather than isolating this helper. Parallel improvement claims should stay bounded to the measured synthetic 262,144-row workload and the million-row numeric real-file run.
- Function-level status: No direct function-level measurement exists yet.

## File: src/transform/sink.rs

Coverage note: 58 function definitions were found in this file. This section is grounded in the refreshed 2026-04-18 `transform_write` comparison under `.tmp/pr04-target` plus the PR-03 `10rec` and PR-04 `fts0003` release-CLI RSS runs. The sink surface now has direct representative evidence that the accepted ultra-wide writer safeguard still materially improves `10rec`, while the added wide-schema statistics-only tier materially improves the `fts0003` path without the output-size blowup seen in the earlier dictionary-disabled probe.

### Function: from_request

- Location: src/transform/sink.rs:33
- Signature: `    pub fn from_request(request: &TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the PR-04 Criterion comparison under `.tmp/pr04-target`, where `transform_write_real_file/fts0003_wide_schema_serial` improved by `[-13.047% -11.728% -10.938%]` with `p = 0.00 < 0.05`, `transform_write_real_file/10rec_wide_schema_serial` stayed neutral with `p = 0.75`, `transform_write_real_file/numeric_1000000_2_serial` stayed neutral with `p = 0.38`, and the tracked synthetic transform workloads showed no statistically significant regressions.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. The comparison validates warmed local-file parser-transform-sink throughput with Parquet writing, and the matching release-CLI runs in `.tmp/pr03-10rec-rss-baseline.txt` and `.tmp/pr03-10rec-rss-candidate-final.txt` directly validate peak-RSS reduction on the representative `10rec` surface. The evidence still does not isolate helper-level attribution or prove cold-cache behavior.
- Function-level status: No direct function-level measurement exists yet.

### Function: prepare

- Location: src/transform/sink.rs:52
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError>;`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the PR-04 Criterion comparison under `.tmp/pr04-target`, where `transform_write_real_file/fts0003_wide_schema_serial` improved by `[-13.047% -11.728% -10.938%]` with `p = 0.00 < 0.05`, `transform_write_real_file/10rec_wide_schema_serial` stayed neutral with `p = 0.75`, `transform_write_real_file/numeric_1000000_2_serial` stayed neutral with `p = 0.38`, and the tracked synthetic transform workloads showed no statistically significant regressions.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. The comparison validates warmed local-file parser-transform-sink throughput with Parquet writing, and the matching release-CLI runs in `.tmp/pr03-10rec-rss-baseline.txt` and `.tmp/pr03-10rec-rss-candidate-final.txt` directly validate peak-RSS reduction on the representative `10rec` surface. The evidence still does not isolate helper-level attribution or prove cold-cache behavior.
- Function-level status: No direct function-level measurement exists yet.

### Function: stage_batches

- Location: src/transform/sink.rs:56
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the PR-04 Criterion comparison under `.tmp/pr04-target`, where `transform_write_real_file/fts0003_wide_schema_serial` improved by `[-13.047% -11.728% -10.938%]` with `p = 0.00 < 0.05`, `transform_write_real_file/10rec_wide_schema_serial` stayed neutral with `p = 0.75`, `transform_write_real_file/numeric_1000000_2_serial` stayed neutral with `p = 0.38`, and the tracked synthetic transform workloads showed no statistically significant regressions.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. The comparison validates warmed local-file parser-transform-sink throughput with Parquet writing, and the matching release-CLI runs in `.tmp/pr03-10rec-rss-baseline.txt` and `.tmp/pr03-10rec-rss-candidate-final.txt` directly validate peak-RSS reduction on the representative `10rec` surface. The evidence still does not isolate helper-level attribution or prove cold-cache behavior.
- Function-level status: No direct function-level measurement exists yet.

### Function: skeleton

- Location: src/transform/sink.rs:76
- Signature: `    pub fn skeleton(plan: ParquetSinkPlan) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the PR-04 Criterion comparison under `.tmp/pr04-target`, where `transform_write_real_file/fts0003_wide_schema_serial` improved by `[-13.047% -11.728% -10.938%]` with `p = 0.00 < 0.05`, `transform_write_real_file/10rec_wide_schema_serial` stayed neutral with `p = 0.75`, `transform_write_real_file/numeric_1000000_2_serial` stayed neutral with `p = 0.38`, and the tracked synthetic transform workloads showed no statistically significant regressions.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. The comparison validates warmed local-file parser-transform-sink throughput with Parquet writing, and the matching release-CLI runs in `.tmp/pr03-10rec-rss-baseline.txt` and `.tmp/pr03-10rec-rss-candidate-final.txt` directly validate peak-RSS reduction on the representative `10rec` surface. The evidence still does not isolate helper-level attribution or prove cold-cache behavior.
- Function-level status: No direct function-level measurement exists yet.

### Function: decoded_rows_staged

- Location: src/transform/sink.rs:88
- Signature: `    pub fn decoded_rows_staged(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: parquet_written

- Location: src/transform/sink.rs:106
- Signature: `    pub fn parquet_written(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: label

- Location: src/transform/sink.rs:134
- Signature: `    pub fn label(&self) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/sink.rs:149
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/transform/sink.rs:157
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_request

- Location: src/transform/sink.rs:174
- Signature: `    pub fn from_request(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: output_column_count

- Location: src/transform/sink.rs:228
- Signature: `    pub fn output_column_count(&self) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: selection_applied

- Location: src/transform/sink.rs:232
- Signature: `    pub fn selection_applied(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: filter_applied

- Location: src/transform/sink.rs:236
- Signature: `    pub fn filter_applied(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply

- Location: src/transform/sink.rs:240
- Signature: `    fn apply(&self, batch: RowBatch) -> Result<ExecutedBatch, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply_serial

- Location: src/transform/sink.rs:254
- Signature: `    fn apply_serial(&self, batch: RowBatch) -> Result<TypedBatch, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply_parallel

- Location: src/transform/sink.rs:263
- Signature: `    fn apply_parallel(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply_rows

- Location: src/transform/sink.rs:286
- Signature: `    fn apply_rows(&self, rows: &[ParsedRow]) -> Result<TypedBatchChunk, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: row_matches

- Location: src/transform/sink.rs:314
- Signature: `    fn row_matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_request

- Location: src/transform/sink.rs:329
- Signature: `    fn from_request(request: &TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: threads_for

- Location: src/transform/sink.rs:342
- Signature: `    fn threads_for(&self, row_count: usize) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_source

- Location: src/transform/sink.rs:366
- Signature: `    fn from_source(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_source_column

- Location: src/transform/sink.rs:405
- Signature: `    fn from_source_column(column: &SasColumn) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: data_type

- Location: src/transform/sink.rs:418
- Signature: `    fn data_type(&self) -> DataType {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: is_nullable

- Location: src/transform/sink.rs:429
- Signature: `    fn is_nullable(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: missing_tag_column_name

- Location: src/transform/sink.rs:434
- Signature: `fn missing_tag_column_name(column_name: &str) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: primary_field_metadata

- Location: src/transform/sink.rs:438
- Signature: `fn primary_field_metadata(column: &SasColumn) -> HashMap<String, String> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: missing_tag_field_metadata

- Location: src/transform/sink.rs:471
- Signature: `fn missing_tag_field_metadata(column: &SasColumn) -> HashMap<String, String> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: from_chunks

- Location: src/transform/sink.rs:487
- Signature: `    fn from_chunks(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: with_capacity

- Location: src/transform/sink.rs:529
- Signature: `    fn with_capacity(kind: ProjectionKind, capacity: usize) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: push

- Location: src/transform/sink.rs:548
- Signature: `    fn push(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: extend

- Location: src/transform/sink.rs:590
- Signature: `    fn extend(&mut self, other: Self) {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_float64

- Location: src/transform/sink.rs:617
- Signature: `fn materialized_float64(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_date32

- Location: src/transform/sink.rs:630
- Signature: `fn materialized_date32(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_time64_micros

- Location: src/transform/sink.rs:645
- Signature: `fn materialized_time64_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_timestamp_micros

- Location: src/transform/sink.rs:657
- Signature: `fn materialized_timestamp_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_duration_micros

- Location: src/transform/sink.rs:672
- Signature: `fn materialized_duration_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: materialized_numeric_parts

- Location: src/transform/sink.rs:684
- Signature: `fn materialized_numeric_parts(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: decode_deferred_numeric

- Location: src/transform/sink.rs:700
- Signature: `fn decode_deferred_numeric(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: decode_materialized_missing_tag

- Location: src/transform/sink.rs:731
- Signature: `fn decode_materialized_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: expect_whole_number

- Location: src/transform/sink.rs:746
- Signature: `fn expect_whole_number(value: f64, column_name: &str) -> Result<i32, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: parse

- Location: src/transform/sink.rs:766
- Signature: `    fn parse(expression: &str, metadata: &SasMetadata) -> Result<Self, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: matches

- Location: src/transform/sink.rs:795
- Signature: `    fn matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: parse

- Location: src/transform/sink.rs:841
- Signature: `    fn parse(token: &str, expression: &str) -> Result<Self, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply_numeric

- Location: src/transform/sink.rs:855
- Signature: `    fn apply_numeric(&self, actual: f64, expected: f64) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: apply_string

- Location: src/transform/sink.rs:866
- Signature: `    fn apply_string(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: parse

- Location: src/transform/sink.rs:892
- Signature: `    fn parse(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: new

- Location: src/transform/sink.rs:928
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: fmt

- Location: src/transform/sink.rs:936
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: default_worker_threads

- Location: src/transform/sink.rs:949
- Signature: `fn default_worker_threads() -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: transform_thread_pool

- Location: src/transform/sink.rs:955
- Signature: `fn transform_thread_pool(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: prepare

- Location: src/transform/sink.rs:982
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: stage_batches

- Location: src/transform/sink.rs:988
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: prepare

- Location: src/transform/sink.rs:1032
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: stage_batches

- Location: src/transform/sink.rs:1039
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: build_arrow_schema

- Location: src/transform/sink.rs:1107
- Signature: `fn build_arrow_schema(execution: &TransformExecution, metadata: &SasMetadata) -> Schema {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: typed_batch_to_record_batch

- Location: src/transform/sink.rs:1133
- Signature: `fn typed_batch_to_record_batch(batch: TypedBatch, schema: Arc<Schema>) -> RecordBatch {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured path is the PR-02 Criterion comparison under `.tmp/pr02-target`, where the accepted compact missing-tag buffering candidate materially improved `transform_write_real_file/fts0003_wide_schema_serial` at `p = 0.00 < 0.05` while the tracked synthetic and other real-file workloads remained neutral.
- Measurement scope: Those results cover the broader valid-file parser-transform-sink path rather than `typed_batch_to_record_batch` in isolation, but this helper is directly involved because the accepted candidate changed how missing-tag sidecar columns are converted into Arrow UTF-8 arrays. The comparison still does not isolate peak-RSS effects or prove cold-cache behavior.
- Function-level status: No direct function-level measurement exists yet.

### Function: strip_quotes

- Location: src/transform/sink.rs:1156
- Signature: `fn strip_quotes(token: &str) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

### Function: arrow_schema_preserves_parser_metadata_including_informats

- Location: src/transform/sink.rs:1178
- Signature: `    fn arrow_schema_preserves_parser_metadata_including_informats() {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Verified evidence: No direct function-level measurement exists yet. The nearest verified measured paths are the refreshed 2026-04-18 `transform_write_supported_subset/262144` benchmark at 1 thread `[38.728 ms 39.313 ms 40.082 ms]` / `[6.5401 Melem/s 6.6681 Melem/s 6.7688 Melem/s]` and 4 threads `[36.063 ms 36.435 ms 36.992 ms]` / `[7.0865 Melem/s 7.1948 Melem/s 7.2692 Melem/s]`, plus the PR-07 real-file CLI runs: `dates.sas7bdat` `0.00 real`, `issue_pandas.sas7bdat` `0.00 real`, `sample_bincompressed.sas7bdat` `0.00 real`, `10rec.sas7bdat` `0.92 real`, `fts0003.sas7bdat` `2.65 real`, and `numeric_1000000_2.sas7bdat` `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover valid-file transform and sink execution rather than isolating this helper. PR-03 directly measures the representative `10rec.sas7bdat` wide-schema path at about `238 MB` to `240 MB` maximum resident set size on the accepted candidate versus about `1.12 GB` to `1.13 GB` on the saved baseline, and PR-04 now directly measures the representative `fts0003.sas7bdat` path at about `694 MB` to `699 MB` maximum resident set size on the accepted hybrid candidate versus about `700 MB` to `706 MB` on the saved baseline while throughput improved significantly on the same request shape.
- Function-level status: No direct function-level measurement exists yet.

## File: src/validation/contracts.rs

Coverage note: 16 function definitions were found in this file. The nearest verified evidence is the validation harness set documented in README.md and exercised by tests/validation_contract.rs; no checked-in timing isolates these report and classification helpers.

### Function: label

- Location: src/validation/contracts.rs:14
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: readable_outcome

- Location: src/validation/contracts.rs:48
- Signature: `    pub fn readable_outcome(&self) -> Option<&ReadableOutcome> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: failure_kind

- Location: src/validation/contracts.rs:55
- Signature: `    pub fn failure_kind(&self) -> Option<ProbeFailureKind> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: failure_stage

- Location: src/validation/contracts.rs:62
- Signature: `    pub fn failure_stage(&self) -> Option<&'static str> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: failure_detail

- Location: src/validation/contracts.rs:69
- Signature: `    pub fn failure_detail(&self) -> Option<&str> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: is_readable

- Location: src/validation/contracts.rs:76
- Signature: `    pub fn is_readable(&self) -> bool {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: label

- Location: src/validation/contracts.rs:89
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: failure_count

- Location: src/validation/contracts.rs:114
- Signature: `    pub fn failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: expected_invalid_count

- Location: src/validation/contracts.rs:118
- Signature: `    pub fn expected_invalid_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: compatibility_failure_count

- Location: src/validation/contracts.rs:127
- Signature: `    pub fn compatibility_failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: render_text

- Location: src/validation/contracts.rs:137
- Signature: `    pub fn render_text(&self) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: label

- Location: src/validation/contracts.rs:204
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: label

- Location: src/validation/contracts.rs:237
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: failure_count

- Location: src/validation/contracts.rs:260
- Signature: `    pub fn failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: skipped_count

- Location: src/validation/contracts.rs:272
- Signature: `    pub fn skipped_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: render_text

- Location: src/validation/contracts.rs:279
- Signature: `    pub fn render_text(&self) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, and `differential_validate` harnesses, and tests/validation_contract.rs verifies regression-corpus tags, expected-invalid policy, mixed-result accounting, and supported differential fixtures for this contract surface.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

## File: src/validation/mod.rs

Coverage note: 27 function definitions were found in this file. The nearest verified evidence is the validation harness set documented in README.md and exercised by tests/validation_contract.rs; no checked-in timing isolates the orchestration, canonicalization, or trusted-reader interop helpers in this module.

### Function: new

- Location: src/validation/mod.rs:283
- Signature: `    fn new(kind: ProbeFailureKind, stage: &'static str, detail: impl Into<String>) -> Self {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: from_parser

- Location: src/validation/mod.rs:291
- Signature: `    fn from_parser(stage: &'static str, error: ParserError) -> Self {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: sample_corpus_root

- Location: src/validation/mod.rs:302
- Signature: `pub fn sample_corpus_root() -> PathBuf {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: real_regression_cases

- Location: src/validation/mod.rs:306
- Signature: `pub fn real_regression_cases() -> &'static [RegressionCase] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: differential_fixture_specs

- Location: src/validation/mod.rs:310
- Signature: `pub fn differential_fixture_specs() -> &'static [DifferentialFixtureSpec] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: expected_invalid_sample_fixtures

- Location: src/validation/mod.rs:314
- Signature: `pub fn expected_invalid_sample_fixtures() -> &'static [InvalidFixtureCase] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: classify_sample_corpus_fixture

- Location: src/validation/mod.rs:318
- Signature: `pub fn classify_sample_corpus_fixture(result: &ProbeResult) -> CorpusFixtureStatus {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: probe_file

- Location: src/validation/mod.rs:336
- Signature: `pub fn probe_file(path: &Path, batch_size_rows: usize) -> ProbeResult {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: sweep_sample_corpus

- Location: src/validation/mod.rs:403
- Signature: `pub fn sweep_sample_corpus(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: run_differential_validation

- Location: src/validation/mod.rs:435
- Signature: `pub fn run_differential_validation(output_root: &Path) -> DifferentialReport {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: run_differential_fixture

- Location: src/validation/mod.rs:443
- Signature: `fn run_differential_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: run_transform_fixture

- Location: src/validation/mod.rs:511
- Signature: `fn run_transform_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: canonicalize_parquet_fixture

- Location: src/validation/mod.rs:554
- Signature: `fn canonicalize_parquet_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: extend_value_lines<T, F>

- Location: src/validation/mod.rs:598
- Signature: `fn extend_value_lines<T, F>(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: canonicalize_haven_fixture

- Location: src/validation/mod.rs:631
- Signature: `fn canonicalize_haven_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: first_mismatch_detail

- Location: src/validation/mod.rs:682
- Signature: `fn first_mismatch_detail(local_lines: &[String], trusted_lines: &[String]) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_total_rows

- Location: src/validation/mod.rs:696
- Signature: `fn read_total_rows(path: &Path) -> Result<usize, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_timestamp_column

- Location: src/validation/mod.rs:713
- Signature: `fn read_timestamp_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_date32_column

- Location: src/validation/mod.rs:738
- Signature: `fn read_date32_column(path: &Path, column_name: &str) -> Result<Vec<Option<i32>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_time64_column

- Location: src/validation/mod.rs:763
- Signature: `fn read_time64_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_float64_column

- Location: src/validation/mod.rs:788
- Signature: `fn read_float64_column(path: &Path, column_name: &str) -> Result<Vec<Option<f64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_utf8_column

- Location: src/validation/mod.rs:813
- Signature: `fn read_utf8_column(path: &Path, column_name: &str) -> Result<Vec<Option<String>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: missing_tag_column_name

- Location: src/validation/mod.rs:839
- Signature: `fn missing_tag_column_name(column_name: &str) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: normalize_f64

- Location: src/validation/mod.rs:843
- Signature: `fn normalize_f64(value: f64) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: tag_token

- Location: src/validation/mod.rs:857
- Signature: `fn tag_token(tag: Option<&str>) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: classify_parser_error

- Location: src/validation/mod.rs:864
- Signature: `fn classify_parser_error(error: &ParserError) -> ProbeFailureKind {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: panic_detail

- Location: src/validation/mod.rs:872
- Signature: `fn panic_detail(payload: &Box<dyn std::any::Any + Send>) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Verified evidence: README.md documents the `validation_contract`, `sample_corpus_sweep`, `differential_validate`, and `cargo fuzz run parser_entry -- -max_total_time=30` harnesses, while tests/validation_contract.rs verifies the curated regression cases, expected-invalid fixture policy, and supported differential fixture list that this module drives.
- Measurement scope: No checked-in runtime measurement isolates this helper. The nearest verified evidence is harness-level validation behavior, not function-level timing, and it does not imply malformed-input closure or compatibility beyond the explicitly listed readable and expected-invalid cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/assumption_probe_contract.rs

Coverage note: 2 function definitions were found in this file. These tests are executable behavioral checks for the projection-assumption path; the nearest checked-in timing remains the refreshed `projection_assumption_probe` Criterion run from 2026-04-18.

### Function: synthetic_row_batches_are_row_major

- Location: tests/assumption_probe_contract.rs:6
- Signature: `fn synthetic_row_batches_are_row_major() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks the row-major fixture shape and checksum behavior around `run_projection_probe`, and the nearest checked-in timing is the refreshed `projection_assumption_probe` run at `16384` rows `[20.525 us 21.212 us 21.657 us]` and `131072` rows `[165.90 us 167.41 us 168.65 us]`.
- Measurement scope: Those Criterion results measure the benchmark entrypoint rather than this test wrapper. This section is behavioral coverage only.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: projection_assumption_probe_reports_a_deterministic_checksum

- Location: tests/assumption_probe_contract.rs:13
- Signature: `fn projection_assumption_probe_reports_a_deterministic_checksum() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks the row-major fixture shape and checksum behavior around `run_projection_probe`, and the nearest checked-in timing is the refreshed `projection_assumption_probe` run at `16384` rows `[20.525 us 21.212 us 21.657 us]` and `131072` rows `[165.90 us 167.41 us 168.65 us]`.
- Measurement scope: Those Criterion results measure the benchmark entrypoint rather than this test wrapper. This section is behavioral coverage only.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/cli_transform_contract.rs

Coverage note: 7 function definitions were found in this file. These tests verify CLI request construction, default execution choices, and real Parquet output; the nearest checked-in measured path is the PR-07 release `sasrs transform` workload set.

### Function: new

- Location: tests/cli_transform_contract.rs:24
- Signature: `    fn new() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: single_request

- Location: tests/cli_transform_contract.rs:30
- Signature: `    fn single_request(&self) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: run

- Location: tests/cli_transform_contract.rs:36
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: transform_command_builds_the_reviewable_stub_request

- Location: tests/cli_transform_contract.rs:43
- Signature: `fn transform_command_builds_the_reviewable_stub_request() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: transform_command_writes_a_real_parquet_file_through_the_default_service

- Location: tests/cli_transform_contract.rs:96
- Signature: `fn transform_command_writes_a_real_parquet_file_through_the_default_service() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: transform_command_defaults_to_the_streaming_execution_path

- Location: tests/cli_transform_contract.rs:125
- Signature: `fn transform_command_defaults_to_the_streaming_execution_path() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_float64_column

- Location: tests/cli_transform_contract.rs:143
- Signature: `fn read_float64_column(path: &std::path::Path, column_index: usize) -> Vec<f64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies CLI request construction, default streaming behavior, and real Parquet output through the default service. The nearest checked-in measured path is the PR-07 release `sasrs transform` workload set: `dates.sas7bdat` at `0.00 real`, `10rec.sas7bdat` at `0.92 real`, `fts0003.sas7bdat` at `2.65 real`, and `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers.
- Measurement scope: Those results cover full CLI parse plus parser, transform, and sink execution on valid files only. They do not isolate these test helpers, and they are not malformed-input evidence.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/parser_contract.rs

Coverage note: 6 function definitions were found in this file. These tests pin supported-subset metadata and explicit rejection behavior; the nearest checked-in measured parser path remains the streamed `fts0003` baseline described in benches/README.md and benches/pr07_real_file_notes.md.

### Function: supported_subset_is_named_and_exposed_in_the_metadata

- Location: tests/parser_contract.rs:10
- Signature: `fn supported_subset_is_named_and_exposed_in_the_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_exposes_32_bit_little_endian_layout_metadata

- Location: tests/parser_contract.rs:27
- Signature: `fn parser_exposes_32_bit_little_endian_layout_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_exposes_32_bit_layout_metadata_when_header_offsets_are_padded

- Location: tests/parser_contract.rs:44
- Signature: `fn parser_exposes_32_bit_layout_metadata_when_header_offsets_are_padded() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_exposes_big_endian_layout_metadata

- Location: tests/parser_contract.rs:61
- Signature: `fn parser_exposes_big_endian_layout_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_rejects_malformed_word_size_headers

- Location: tests/parser_contract.rs:78
- Signature: `fn parser_rejects_malformed_word_size_headers() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: unsupported_page_types_return_a_structured_error

- Location: tests/parser_contract.rs:95
- Signature: `fn unsupported_page_types_return_a_structured_error() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file locks supported-subset metadata and explicit malformed-header rejection. The nearest checked-in measured parser path is the streamed `fts0003.sas7bdat` baseline described in benches/README.md and the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper, and it does not imply malformed-input closure beyond the explicitly asserted rejection cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/parser_decode_contract.rs

Coverage note: 23 function definitions were found in this file. These tests verify streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling; the nearest checked-in measured parser path remains the streamed `fts0003` baseline described in benches/README.md and benches/pr07_real_file_notes.md.

### Function: sample_dataset_path

- Location: tests/parser_decode_contract.rs:24
- Signature: `fn sample_dataset_path(file_name: &str) -> PathBuf {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: probe_sample_via_parser_entrypoint

- Location: tests/parser_decode_contract.rs:30
- Signature: `fn probe_sample_via_parser_entrypoint(file_name: &str) -> RealFileProbeOutcome {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: probe_fts0003_via_parser_entrypoint

- Location: tests/parser_decode_contract.rs:68
- Signature: `fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: assert_real_file_is_readable

- Location: tests/parser_decode_contract.rs:72
- Signature: `fn assert_real_file_is_readable(file_name: &str) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: drain_all_rows

- Location: tests/parser_decode_contract.rs:87
- Signature: `fn drain_all_rows(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: numeric_missing_tag

- Location: tests/parser_decode_contract.rs:101
- Signature: `fn numeric_missing_tag(value: &ParsedValue) -> Option<SasMissingTag> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_metadata_and_batches_from_the_supported_subset_fixture

- Location: tests/parser_decode_contract.rs:109
- Signature: `fn parser_decodes_metadata_and_batches_from_the_supported_subset_fixture() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_uncompressed_32_bit_little_endian_fixture_end_to_end

- Location: tests/parser_decode_contract.rs:195
- Signature: `fn parser_decodes_uncompressed_32_bit_little_endian_fixture_end_to_end() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_latin1_strings_without_claiming_utf8_only_support

- Location: tests/parser_decode_contract.rs:222
- Signature: `fn parser_decodes_latin1_strings_without_claiming_utf8_only_support() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_uncompressed_big_endian_fixture_end_to_end

- Location: tests/parser_decode_contract.rs:247
- Signature: `fn parser_decodes_uncompressed_big_endian_fixture_end_to_end() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_supported_subset_across_multiple_data_pages

- Location: tests/parser_decode_contract.rs:288
- Signature: `fn parser_decodes_supported_subset_across_multiple_data_pages() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_defers_multi_page_row_reads_until_batches_are_requested

- Location: tests/parser_decode_contract.rs:321
- Signature: `fn parser_defers_multi_page_row_reads_until_batches_are_requested() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_row_compressed_rows_stored_across_meta_and_mix_pages

- Location: tests/parser_decode_contract.rs:363
- Signature: `fn parser_decodes_row_compressed_rows_stored_across_meta_and_mix_pages() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_binary_compressed_rows_from_meta_subheaders

- Location: tests/parser_decode_contract.rs:408
- Signature: `fn parser_decodes_binary_compressed_rows_from_meta_subheaders() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_preserves_non_8_byte_numeric_cells_without_parser_core_rejection

- Location: tests/parser_decode_contract.rs:447
- Signature: `fn parser_preserves_non_8_byte_numeric_cells_without_parser_core_rejection() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_reads_the_real_10rec_file_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:493
- Signature: `fn parser_reads_the_real_10rec_file_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_reads_the_real_fts0003_file_through_the_compressed_entrypoint

- Location: tests/parser_decode_contract.rs:509
- Signature: `fn parser_reads_the_real_fts0003_file_through_the_compressed_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_reads_real_binary_compressed_samples_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:525
- Signature: `fn parser_reads_real_binary_compressed_samples_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_reads_real_non_utf8_samples_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:548
- Signature: `fn parser_reads_real_non_utf8_samples_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_decodes_real_gb18030_text_values_honestly

- Location: tests/parser_decode_contract.rs:566
- Signature: `fn parser_decodes_real_gb18030_text_values_honestly() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_infers_semantic_types_and_column_metadata_from_fixture_formats

- Location: tests/parser_decode_contract.rs:588
- Signature: `fn parser_infers_semantic_types_and_column_metadata_from_fixture_formats() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_preserves_real_dates_fixture_semantic_metadata

- Location: tests/parser_decode_contract.rs:680
- Signature: `fn parser_preserves_real_dates_fixture_semantic_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_exposes_real_special_missing_tags_without_flattening_them

- Location: tests/parser_decode_contract.rs:710
- Signature: `fn parser_exposes_real_special_missing_tags_without_flattening_them() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies streamed decode behavior, real-file readability, encoding coverage, semantic metadata, and special missing-value handling. The nearest checked-in measured parser path is the `parser_decode` real-file `fts0003` baseline described in benches/README.md together with the PR-07 release note for `fts0003.sas7bdat` at `2.65 real` with `staged_rows=10275` and `staged_batches=6`.
- Measurement scope: The cited result is a broader parser-plus-transform release path rather than a timing for this test wrapper. It is useful as the nearest verified measured path, but it does not imply unsupported or malformed files are handled beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/transform_contract.rs

Coverage note: 5 function definitions were found in this file. These tests pin the transform contract and bounded-memory sink planning; the nearest checked-in measured path is the `transform_write` benchmark plus the PR-07 release CLI transform set.

### Function: stub_transform_service_returns_a_structured_not_yet_implemented_report

- Location: tests/transform_contract.rs:17
- Signature: `fn stub_transform_service_returns_a_structured_not_yet_implemented_report() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file pins the transform contract, bounded-memory row-group planning, and physical-numeric acceptance. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not claim refactor cleanliness or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bounded_memory_execution_path_is_explicit_in_the_contract

- Location: tests/transform_contract.rs:33
- Signature: `fn bounded_memory_execution_path_is_explicit_in_the_contract() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file pins the transform contract, bounded-memory row-group planning, and physical-numeric acceptance. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not claim refactor cleanliness or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parquet_sink_plan_caps_row_groups_to_the_bounded_memory_budget

- Location: tests/transform_contract.rs:42
- Signature: `fn parquet_sink_plan_caps_row_groups_to_the_bounded_memory_budget() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file pins the transform contract, bounded-memory row-group planning, and physical-numeric acceptance. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not claim refactor cleanliness or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: transform_execution_accepts_physical_numeric_columns_without_forcing_semantics

- Location: tests/transform_contract.rs:61
- Signature: `fn transform_execution_accepts_physical_numeric_columns_without_forcing_semantics() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file pins the transform contract, bounded-memory row-group planning, and physical-numeric acceptance. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not claim refactor cleanliness or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: example_request

- Location: tests/transform_contract.rs:110
- Signature: `fn example_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file pins the transform contract, bounded-memory row-group planning, and physical-numeric acceptance. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not claim refactor cleanliness or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/transform_parser_integration.rs

Coverage note: 36 function definitions were found in this file. These integration tests pin selection, filtering, bounded-memory staging, parallel batches, semantic typing, and real-fixture transform behavior; the nearest checked-in measured path is the `transform_write` benchmark plus the PR-07 release CLI transform set.

### Function: open

- Location: tests/transform_parser_integration.rs:37
- Signature: `    fn open(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: open

- Location: tests/transform_parser_integration.rs:52
- Signature: `    fn open(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: prepare

- Location: tests/transform_parser_integration.rs:70
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: stage_batches

- Location: tests/transform_parser_integration.rs:76
- Signature: `    fn stage_batches(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_writes_selected_and_filtered_parquet_output

- Location: tests/transform_parser_integration.rs:105
- Signature: `fn parser_transform_service_writes_selected_and_filtered_parquet_output() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_rejects_unsupported_filter_expressions

- Location: tests/transform_parser_integration.rs:140
- Signature: `fn parser_transform_service_rejects_unsupported_filter_expressions() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_materializes_non_8_byte_numeric_columns_with_filtering

- Location: tests/transform_parser_integration.rs:155
- Signature: `fn parser_transform_service_materializes_non_8_byte_numeric_columns_with_filtering() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_materializes_big_endian_non_8_byte_missing_tags

- Location: tests/transform_parser_integration.rs:202
- Signature: `fn parser_transform_service_materializes_big_endian_non_8_byte_missing_tags() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_uses_bounded_memory_batches_for_multi_page_output

- Location: tests/transform_parser_integration.rs:251
- Signature: `fn parser_transform_service_uses_bounded_memory_batches_for_multi_page_output() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_reports_parallel_batch_execution_when_worker_threads_are_used

- Location: tests/transform_parser_integration.rs:286
- Signature: `fn parser_transform_service_reports_parallel_batch_execution_when_worker_threads_are_used() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_starts_batching_before_the_full_dataset_is_read

- Location: tests/transform_parser_integration.rs:314
- Signature: `fn parser_transform_service_starts_batching_before_the_full_dataset_is_read() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_projects_semantic_numeric_columns_into_arrow_types

- Location: tests/transform_parser_integration.rs:347
- Signature: `fn parser_transform_service_projects_semantic_numeric_columns_into_arrow_types() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_preserves_real_date_metadata_in_parquet_schema

- Location: tests/transform_parser_integration.rs:454
- Signature: `fn parser_transform_service_preserves_real_date_metadata_in_parquet_schema() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parser_transform_service_preserves_real_special_missing_values_with_sidecar_tags

- Location: tests/transform_parser_integration.rs:494
- Signature: `fn parser_transform_service_preserves_real_special_missing_values_with_sidecar_tags() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: example_request

- Location: tests/transform_parser_integration.rs:524
- Signature: `fn example_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: unsupported_filter_request

- Location: tests/transform_parser_integration.rs:549
- Signature: `fn unsupported_filter_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: narrow_numeric_request

- Location: tests/transform_parser_integration.rs:574
- Signature: `fn narrow_numeric_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: narrow_numeric_filtered_request

- Location: tests/transform_parser_integration.rs:599
- Signature: `fn narrow_numeric_filtered_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: truncated_numeric_bytes

- Location: tests/transform_parser_integration.rs:605
- Signature: `fn truncated_numeric_bytes(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: truncated_missing_numeric_bytes

- Location: tests/transform_parser_integration.rs:618
- Signature: `fn truncated_missing_numeric_bytes(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: truncate_numeric_storage_bytes

- Location: tests/transform_parser_integration.rs:628
- Signature: `fn truncate_numeric_storage_bytes(raw: Vec<u8>, endianness: Endianness, width: usize) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bounded_memory_request

- Location: tests/transform_parser_integration.rs:635
- Signature: `fn bounded_memory_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: parallel_batch_request

- Location: tests/transform_parser_integration.rs:662
- Signature: `fn parallel_batch_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: semantic_fixture_request

- Location: tests/transform_parser_integration.rs:689
- Signature: `fn semantic_fixture_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: real_dates_request

- Location: tests/transform_parser_integration.rs:714
- Signature: `fn real_dates_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: real_missing_request

- Location: tests/transform_parser_integration.rs:741
- Signature: `fn real_missing_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: first_batch_only_request

- Location: tests/transform_parser_integration.rs:768
- Signature: `fn first_batch_only_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_parquet_schema

- Location: tests/transform_parser_integration.rs:795
- Signature: `fn read_parquet_schema(path: &Path) -> Vec<(String, String)> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_total_rows

- Location: tests/transform_parser_integration.rs:807
- Signature: `fn read_total_rows(path: &Path) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_optional_float64_column

- Location: tests/transform_parser_integration.rs:818
- Signature: `fn read_optional_float64_column(path: &Path, column_index: usize) -> Vec<Option<f64>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_float64_column

- Location: tests/transform_parser_integration.rs:839
- Signature: `fn read_float64_column(path: &Path, column_index: usize) -> Vec<f64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_optional_utf8_column

- Location: tests/transform_parser_integration.rs:846
- Signature: `fn read_optional_utf8_column(path: &Path, column_index: usize) -> Vec<Option<String>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_utf8_column

- Location: tests/transform_parser_integration.rs:867
- Signature: `fn read_utf8_column(path: &Path, column_index: usize) -> Vec<String> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_optional_i32_column

- Location: tests/transform_parser_integration.rs:874
- Signature: `fn read_optional_i32_column(path: &Path, column_index: usize) -> Vec<Option<i32>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_optional_i64_column

- Location: tests/transform_parser_integration.rs:895
- Signature: `fn read_optional_i64_column(path: &Path, column_index: usize) -> Vec<Option<i64>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read_field_metadata

- Location: tests/transform_parser_integration.rs:934
- Signature: `fn read_field_metadata(path: &Path, field_name: &str) -> HashMap<String, String> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies end-to-end selection, filtering, bounded-memory staging, parallel batch reporting, semantic Arrow mapping, and real date and missing-value preservation. The nearest checked-in measured path is the `transform_write` benchmark together with the PR-07 release CLI transform results, including `numeric_1000000_2.sas7bdat` at `0.14 real` serial versus `0.09 real` with four workers, `10rec.sas7bdat` at `0.92 real`, and `fts0003.sas7bdat` at `2.65 real`.
- Measurement scope: Those numbers cover end-to-end parser, transform, and sink work on supported fixtures or representative valid files. They do not isolate these test helpers, and they do not imply malformed-input closure, refactor cleanliness, or documentation completeness.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/validation_contract.rs

Coverage note: 6 function definitions were found in this file. These tests are the reviewable executable contract for the validation harnesses documented in README.md; no checked-in timing isolates the test wrappers themselves.

### Function: real_regression_corpus_includes_required_categories_and_baseline_fixture

- Location: tests/validation_contract.rs:12
- Signature: `fn real_regression_corpus_includes_required_categories_and_baseline_fixture() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: differential_fixtures_cover_the_supported_semantic_surface

- Location: tests/validation_contract.rs:44
- Signature: `fn differential_fixtures_cover_the_supported_semantic_surface() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: sample_corpus_sweep_reports_mixed_results_honestly

- Location: tests/validation_contract.rs:69
- Signature: `fn sample_corpus_sweep_reports_mixed_results_honestly() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: invalid_sample_fixture_policy_is_explicit_and_reviewable

- Location: tests/validation_contract.rs:102
- Signature: `fn invalid_sample_fixture_policy_is_explicit_and_reviewable() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: expected_invalid_sample_fixtures_match_their_current_probe_results

- Location: tests/validation_contract.rs:121
- Signature: `fn expected_invalid_sample_fixtures_match_their_current_probe_results() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: curated_real_regression_cases_match_their_current_expectations

- Location: tests/validation_contract.rs:135
- Signature: `fn curated_real_regression_cases_match_their_current_expectations() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This file verifies the curated regression-corpus tags, supported differential fixtures, mixed-result sweep accounting, explicit expected-invalid policy, and current readable-fixture expectations that the README-documented validation harnesses rely on.
- Measurement scope: No checked-in runtime measurement isolates this test wrapper. The nearest verified evidence is harness-level validation behavior, and it does not imply malformed-input closure or universal compatibility beyond the explicitly listed cases.
- Function-level status: No direct function-level performance measurement exists yet.

## File: tests/support/minimal_sas_fixture.rs

Coverage note: 63 function definitions were found in this file. These helpers underpin the parser, CLI, transform, and validation contract suites; no standalone timing is checked in for fixture construction, read tracking, or temporary-path helpers.

### Function: bit64_little

- Location: tests/support/minimal_sas_fixture.rs:51
- Signature: `    pub const fn bit64_little() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bit64_big

- Location: tests/support/minimal_sas_fixture.rs:58
- Signature: `    pub const fn bit64_big() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bit32_little

- Location: tests/support/minimal_sas_fixture.rs:65
- Signature: `    pub const fn bit32_little() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: word_size_bytes

- Location: tests/support/minimal_sas_fixture.rs:72
- Signature: `    fn word_size_bytes(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: page_header_size

- Location: tests/support/minimal_sas_fixture.rs:79
- Signature: `    fn page_header_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: subheader_pointer_size

- Location: tests/support/minimal_sas_fixture.rs:86
- Signature: `    fn subheader_pointer_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: subheader_data_offset

- Location: tests/support/minimal_sas_fixture.rs:93
- Signature: `    fn subheader_data_offset(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_attrs_entry_size

- Location: tests/support/minimal_sas_fixture.rs:100
- Signature: `    fn column_attrs_entry_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: row_size_offsets

- Location: tests/support/minimal_sas_fixture.rs:104
- Signature: `    fn row_size_offsets(self) -> (usize, usize, usize, usize, usize) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: numeric_bytes

- Location: tests/support/minimal_sas_fixture.rs:111
- Signature: `    fn numeric_bytes(self, value: f64) -> [u8; 8] {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: default

- Location: tests/support/minimal_sas_fixture.rs:120
- Signature: `    fn default() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bytes_read

- Location: tests/support/minimal_sas_fixture.rs:179
- Signature: `    pub fn bytes_read(&self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: record_read

- Location: tests/support/minimal_sas_fixture.rs:183
- Signature: `    fn record_read(&self, count: usize) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: new

- Location: tests/support/minimal_sas_fixture.rs:195
- Signature: `    pub fn new(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: read

- Location: tests/support/minimal_sas_fixture.rs:204
- Signature: `    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: seek

- Location: tests/support/minimal_sas_fixture.rs:212
- Signature: `    fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: supported_fixture_definition

- Location: tests/support/minimal_sas_fixture.rs:217
- Signature: `pub fn supported_fixture_definition() -> FixtureDefinition {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: supported_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:255
- Signature: `pub fn supported_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bit32_little_endian_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:259
- Signature: `pub fn bit32_little_endian_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: bit32_little_endian_padded_header_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:265
- Signature: `pub fn bit32_little_endian_padded_header_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: big_endian_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:272
- Signature: `pub fn big_endian_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: latin1_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:278
- Signature: `pub fn latin1_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: compressed_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:288
- Signature: `pub fn compressed_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: row_compressed_mixed_page_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:292
- Signature: `pub fn row_compressed_mixed_page_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: binary_compressed_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:315
- Signature: `pub fn binary_compressed_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: unsupported_page_type_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:320
- Signature: `pub fn unsupported_page_type_fixture_bytes(page_type: u16) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: malformed_word_size_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:332
- Signature: `pub fn malformed_word_size_fixture_bytes(word_size_marker: u8) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_fixture_file

- Location: tests/support/minimal_sas_fixture.rs:338
- Signature: `pub fn write_fixture_file(definition: &FixtureDefinition, path: &Path) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: unique_tmp_path

- Location: tests/support/minimal_sas_fixture.rs:342
- Signature: `pub fn unique_tmp_path(prefix: &str, extension: &str) -> PathBuf {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: page_count_for

- Location: tests/support/minimal_sas_fixture.rs:355
- Signature: `pub fn page_count_for(definition: &FixtureDefinition) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: lazy_parse_read_budget

- Location: tests/support/minimal_sas_fixture.rs:362
- Signature: `pub fn lazy_parse_read_budget(page_count: usize) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: first_batch_read_budget

- Location: tests/support/minimal_sas_fixture.rs:366
- Signature: `pub fn first_batch_read_budget(page_count: usize) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: tracked_reader

- Location: tests/support/minimal_sas_fixture.rs:370
- Signature: `pub fn tracked_reader(bytes: Vec<u8>) -> (TrackingCursor, Arc<ReadMonitor>) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: tracked_reader_with_monitor

- Location: tests/support/minimal_sas_fixture.rs:376
- Signature: `pub fn tracked_reader_with_monitor(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> TrackingCursor {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: build_fixture

- Location: tests/support/minimal_sas_fixture.rs:380
- Signature: `pub fn build_fixture(definition: &FixtureDefinition) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: name

- Location: tests/support/minimal_sas_fixture.rs:470
- Signature: `    pub fn name(&self) -> &str {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: build_subheaders

- Location: tests/support/minimal_sas_fixture.rs:477
- Signature: `fn build_subheaders(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: row_size_subheader

- Location: tests/support/minimal_sas_fixture.rs:510
- Signature: `fn row_size_subheader(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_size_subheader

- Location: tests/support/minimal_sas_fixture.rs:556
- Signature: `fn column_size_subheader(column_count: usize, layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_text_subheader

- Location: tests/support/minimal_sas_fixture.rs:573
- Signature: `fn column_text_subheader(text_blob: &[u8], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_name_subheader

- Location: tests/support/minimal_sas_fixture.rs:591
- Signature: `fn column_name_subheader(column_name_refs: &[TextRef], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_attrs_subheader

- Location: tests/support/minimal_sas_fixture.rs:612
- Signature: `fn column_attrs_subheader(columns: &[FixtureColumn], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_format_subheader

- Location: tests/support/minimal_sas_fixture.rs:653
- Signature: `fn column_format_subheader(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_header

- Location: tests/support/minimal_sas_fixture.rs:704
- Signature: `fn write_header(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_meta_page

- Location: tests/support/minimal_sas_fixture.rs:751
- Signature: `fn write_meta_page(bytes: &mut [u8], subheaders: &[Vec<u8>], layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: build_compressed_fixture

- Location: tests/support/minimal_sas_fixture.rs:790
- Signature: `fn build_compressed_fixture(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: encode_rle_copy_row

- Location: tests/support/minimal_sas_fixture.rs:895
- Signature: `fn encode_rle_copy_row(bytes: &[u8]) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: encode_binary_literal_row

- Location: tests/support/minimal_sas_fixture.rs:906
- Signature: `fn encode_binary_literal_row(bytes: &[u8]) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_subheader_row_page

- Location: tests/support/minimal_sas_fixture.rs:915
- Signature: `fn write_subheader_row_page(bytes: &mut [u8], payloads: &[Vec<u8>], layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_mix_page

- Location: tests/support/minimal_sas_fixture.rs:961
- Signature: `fn write_mix_page(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_data_page

- Location: tests/support/minimal_sas_fixture.rs:993
- Signature: `fn write_data_page(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_row

- Location: tests/support/minimal_sas_fixture.rs:1023
- Signature: `fn write_row(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: subheader_remainder

- Location: tests/support/minimal_sas_fixture.rs:1077
- Signature: `fn subheader_remainder(len: usize, layout: FixtureLayout) -> u16 {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: normalized_column_metadata

- Location: tests/support/minimal_sas_fixture.rs:1081
- Signature: `fn normalized_column_metadata(definition: &FixtureDefinition) -> Vec<FixtureColumnMetadata> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: tagged_missing_numeric_bytes

- Location: tests/support/minimal_sas_fixture.rs:1088
- Signature: `pub fn tagged_missing_numeric_bytes(layout: FixtureLayout, tag: char) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: append_text

- Location: tests/support/minimal_sas_fixture.rs:1098
- Signature: `fn append_text(blob: &mut Vec<u8>, value: &str) -> TextRef {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: column_width

- Location: tests/support/minimal_sas_fixture.rs:1113
- Signature: `fn column_width(column: &FixtureColumn) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_padded_ascii

- Location: tests/support/minimal_sas_fixture.rs:1120
- Signature: `fn write_padded_ascii(bytes: &mut [u8], offset: usize, len: usize, value: &str) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_text_ref

- Location: tests/support/minimal_sas_fixture.rs:1129
- Signature: `fn write_text_ref(bytes: &mut [u8], offset: usize, text_ref: TextRef, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_word

- Location: tests/support/minimal_sas_fixture.rs:1135
- Signature: `fn write_word(bytes: &mut [u8], offset: usize, value: u64, layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_u16

- Location: tests/support/minimal_sas_fixture.rs:1142
- Signature: `fn write_u16(bytes: &mut [u8], offset: usize, value: u16, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_u32

- Location: tests/support/minimal_sas_fixture.rs:1150
- Signature: `fn write_u32(bytes: &mut [u8], offset: usize, value: u32, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

### Function: write_u64

- Location: tests/support/minimal_sas_fixture.rs:1158
- Signature: `fn write_u64(bytes: &mut [u8], offset: usize, value: u64, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Verified evidence: This helper library underpins tests/parser_contract.rs, tests/parser_decode_contract.rs, tests/cli_transform_contract.rs, tests/transform_parser_integration.rs, and tests/validation_contract.rs, which use it to build synthetic fixtures, temporary outputs, and read-budget probes.
- Measurement scope: No checked-in timing isolates fixture construction, cursor tracking, or temporary-path helpers. The nearest verified evidence is harness-level coverage from the dependent test suites rather than standalone performance data.
- Function-level status: No direct function-level performance measurement exists yet.

