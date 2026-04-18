# Journal

This file is a comprehensive, intentionally speculative journal over the in-scope Rust surface. Coverage is prioritized over certainty; every function entry includes a role note, an idea, a hypothesis, an experiment idea, and a current evidence line.

## Sweep Summary

- In-scope Rust files: 28
- Function definitions inventoried: 392
- Performance-note bias: issue #7 page-header allocation / heap churn on parser hot paths.
- Experiment-note bias: issue #1 encourages real sample datasets for follow-up measurements.
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

Coverage note: 1 function definitions were found in this file.

### Function: projection_assumption_probe

- Location: benches/assumption_probe.rs:6
- Signature: `fn projection_assumption_probe(criterion: &mut Criterion) {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

## File: benches/parser_decode.rs

Coverage note: 4 function definitions were found in this file.

### Function: parser_decode_benchmark

- Location: benches/parser_decode.rs:23
- Signature: `fn parser_decode_benchmark(criterion: &mut Criterion) {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: build_benchmark_fixture

- Location: benches/parser_decode.rs:84
- Signature: `fn build_benchmark_fixture(row_count: usize) -> Vec<u8> {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: fts0003_path

- Location: benches/parser_decode.rs:97
- Signature: `fn fts0003_path() -> PathBuf {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: probe_fts0003_via_parser_entrypoint

- Location: benches/parser_decode.rs:103
- Signature: `fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

## File: benches/transform_write.rs

Coverage note: 4 function definitions were found in this file.

### Function: open

- Location: benches/transform_write.rs:24
- Signature: `    fn open(`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: transform_write_benchmark

- Location: benches/transform_write.rs:32
- Signature: `fn transform_write_benchmark(criterion: &mut Criterion) {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: bench_request

- Location: benches/transform_write.rs:77
- Signature: `fn bench_request(output_path: PathBuf, worker_threads: usize) -> TransformRequest {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

### Function: build_benchmark_fixture

- Location: benches/transform_write.rs:104
- Signature: `fn build_benchmark_fixture(row_count: usize) -> Vec<u8> {`
- Role / observation: Benchmark helper or entrypoint used to exercise a focused performance scenario.
- Speculative idea: keep the harness honest by comparing synthetic fixtures with at least one real sample dataset.
- Hypothesis: representativeness matters more than micro-optimizing the benchmark wrapper.
- Experiment idea: rerun the benchmark with a real sample fixture and compare variance plus throughput.
- Result/evidence: benchmark intent is explicit in code; no new measurements were taken for this journal entry.

## File: fuzz/fuzz_targets/parser_entry.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass.

## File: src/bin/differential_validate.rs

Coverage note: 2 function definitions were found in this file.

### Function: main

- Location: src/bin/differential_validate.rs:7
- Signature: `fn main() {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: usage

- Location: src/bin/differential_validate.rs:58
- Signature: `fn usage(message: &str) -> ! {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

## File: src/bin/sample_corpus_sweep.rs

Coverage note: 2 function definitions were found in this file.

### Function: main

- Location: src/bin/sample_corpus_sweep.rs:7
- Signature: `fn main() {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: usage

- Location: src/bin/sample_corpus_sweep.rs:78
- Signature: `fn usage(message: &str) -> ! {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

## File: src/cli.rs

Coverage note: 7 function definitions were found in this file.

### Function: into_request

- Location: src/cli.rs:50
- Signature: `    fn into_request(self) -> TransformRequest {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: fmt

- Location: src/cli.rs:87
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: exit_code

- Location: src/cli.rs:101
- Signature: `    pub fn exit_code(&self) -> ExitCode {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: fmt

- Location: src/cli.rs:110
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: source

- Location: src/cli.rs:119
- Signature: `    fn source(&self) -> Option<&(dyn Error + 'static)> {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: run<I, T>

- Location: src/cli.rs:127
- Signature: `pub fn run<I, T>(args: I) -> Result<CommandOutcome, CliError>`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

### Function: run_with_service<I, T, S>

- Location: src/cli.rs:140
- Signature: `pub fn run_with_service<I, T, S>(args: I, service: &S) -> Result<CommandOutcome, CliError>`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

## File: src/lib.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass.

## File: src/main.rs

Coverage note: 1 function definitions were found in this file.

### Function: main

- Location: src/main.rs:3
- Signature: `fn main() -> ExitCode {`
- Role / observation: CLI-facing orchestration, request-construction, or reporting helper.
- Speculative idea: reduce avoidable string and path churn only if repeated CLI runs make it measurable.
- Hypothesis: maintainability dominates speed for this surface unless command dispatch becomes hot in automation.
- Experiment idea: benchmark repeated command parsing and request construction in a tight loop.
- Result/evidence: source review only; no standalone measurement was taken for this entry.

## File: src/parser/constants.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass.

## File: src/parser/contracts.rs

Coverage note: 17 function definitions were found in this file.

### Function: supported_subset_name

- Location: src/parser/contracts.rs:57
- Signature: `fn supported_subset_name(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: supported_subset

- Location: src/parser/contracts.rs:102
- Signature: `pub fn supported_subset(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: new

- Location: src/parser/contracts.rs:128
- Signature: `    pub fn new(source_name: &'a str, reader: BoxedParserDataSource) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from_bytes

- Location: src/parser/contracts.rs:135
- Signature: `    pub fn from_bytes(source_name: &'a str, bytes: Vec<u8>) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from_reader<R>

- Location: src/parser/contracts.rs:139
- Signature: `    pub fn from_reader<R>(source_name: &'a str, reader: R) -> Self`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: label

- Location: src/parser/contracts.rs:163
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: code

- Location: src/parser/contracts.rs:189
- Signature: `    pub fn code(&self) -> char {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from_code

- Location: src/parser/contracts.rs:197
- Signature: `    pub fn from_code(tag: char) -> Option<Self> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: deferred_bytes

- Location: src/parser/contracts.rs:231
- Signature: `    pub fn deferred_bytes(raw_bytes: Vec<u8>) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: as_f64

- Location: src/parser/contracts.rs:238
- Signature: `    pub fn as_f64(&self) -> Option<f64> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: raw_bits

- Location: src/parser/contracts.rs:245
- Signature: `    pub fn raw_bits(&self) -> Option<u64> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: width_bytes

- Location: src/parser/contracts.rs:252
- Signature: `    pub fn width_bytes(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: raw_bytes

- Location: src/parser/contracts.rs:259
- Signature: `    pub fn raw_bytes(&self) -> Option<&[u8]> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: missing_tag

- Location: src/parser/contracts.rs:266
- Signature: `    pub fn missing_tag(&self) -> Option<SasMissingTag> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from

- Location: src/parser/contracts.rs:275
- Signature: `    fn from(value: f64) -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: fmt

- Location: src/parser/contracts.rs:340
- Signature: `    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: new_streaming

- Location: src/parser/contracts.rs:354
- Signature: `    pub(crate) fn new_streaming(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

## File: src/parser/mod.rs

Coverage note: 66 function definitions were found in this file.

### Function: fmt

- Location: src/parser/mod.rs:57
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: fmt

- Location: src/parser/mod.rs:104
- Signature: `    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse

- Location: src/parser/mod.rs:116
- Signature: `    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError>;`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse

- Location: src/parser/mod.rs:123
- Signature: `    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from_code

- Location: src/parser/mod.rs:203
- Signature: `    fn from_code(code: u8) -> Option<Self> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decode

- Location: src/parser/mod.rs:217
- Signature: `    fn decode(self, bytes: &[u8]) -> Result<String, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: from_header_prefix

- Location: src/parser/mod.rs:242
- Signature: `    fn from_header_prefix(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: word_size_bytes

- Location: src/parser/mod.rs:271
- Signature: `    fn word_size_bytes(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: page_header_size

- Location: src/parser/mod.rs:278
- Signature: `    fn page_header_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: subheader_pointer_size

- Location: src/parser/mod.rs:285
- Signature: `    fn subheader_pointer_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: subheader_data_offset

- Location: src/parser/mod.rs:292
- Signature: `    fn subheader_data_offset(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: column_attrs_entry_size

- Location: src/parser/mod.rs:299
- Signature: `    fn column_attrs_entry_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: subheader_signature_size

- Location: src/parser/mod.rs:303
- Signature: `    fn subheader_signature_size(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: row_size_min_len

- Location: src/parser/mod.rs:310
- Signature: `    fn row_size_min_len(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: column_format_min_len

- Location: src/parser/mod.rs:317
- Signature: `    fn column_format_min_len(self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: row_size_offsets

- Location: src/parser/mod.rs:324
- Signature: `    fn row_size_offsets(self) -> RowSizeOffsets {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_word

- Location: src/parser/mod.rs:339
- Signature: `    fn read_word(self, bytes: &[u8], offset: usize) -> Result<u64, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_supported_subset

- Location: src/parser/mod.rs:347
- Signature: `fn parse_supported_subset(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: next_batch

- Location: src/parser/mod.rs:527
- Signature: `    pub fn next_batch(&mut self, batch_size_rows: usize) -> Result<Option<RowBatch>, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: fill_pending_rows

- Location: src/parser/mod.rs:550
- Signature: `    fn fill_pending_rows(&mut self, min_rows: usize) -> Result<(), ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: load_next_row_source

- Location: src/parser/mod.rs:570
- Signature: `    fn load_next_row_source(&mut self) -> Result<(), ParserError> {`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: decoded_row_count

- Location: src/parser/mod.rs:623
- Signature: `    fn decoded_row_count(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_meta_page

- Location: src/parser/mod.rs:628
- Signature: `fn parse_meta_page(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: parse_row_size_subheader

- Location: src/parser/mod.rs:726
- Signature: `fn parse_row_size_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_column_size_subheader

- Location: src/parser/mod.rs:780
- Signature: `fn parse_column_size_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_column_text_subheader

- Location: src/parser/mod.rs:796
- Signature: `fn parse_column_text_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_column_name_subheader

- Location: src/parser/mod.rs:808
- Signature: `fn parse_column_name_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_column_attrs_subheader

- Location: src/parser/mod.rs:848
- Signature: `fn parse_column_attrs_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_column_format_subheader

- Location: src/parser/mod.rs:907
- Signature: `fn parse_column_format_subheader(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: signature_is_recognized

- Location: src/parser/mod.rs:948
- Signature: `fn signature_is_recognized(signature: u32) -> bool {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: effective_subheader_compression

- Location: src/parser/mod.rs:962
- Signature: `fn effective_subheader_compression(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: mix_raw_data_offset

- Location: src/parser/mod.rs:973
- Signature: `fn mix_raw_data_offset(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: parse_subheader_row

- Location: src/parser/mod.rs:998
- Signature: `fn parse_subheader_row(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decompress_row_rle

- Location: src/parser/mod.rs:1029
- Signature: `fn decompress_row_rle(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decompress_row_binary

- Location: src/parser/mod.rs:1138
- Signature: `fn decompress_row_binary(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: finalize_columns

- Location: src/parser/mod.rs:1242
- Signature: `fn finalize_columns(metadata: &PartialMetadata) -> Result<Vec<SasColumn>, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_row

- Location: src/parser/mod.rs:1325
- Signature: `fn parse_row(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_numeric_value

- Location: src/parser/mod.rs:1353
- Signature: `fn parse_numeric_value(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decorate_format_name

- Location: src/parser/mod.rs:1386
- Signature: `fn decorate_format_name(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: semantic_type_from_metadata

- Location: src/parser/mod.rs:1401
- Signature: `fn semantic_type_from_metadata(metadata: &ColumnMetadata) -> SemanticTypeHint {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: semantic_type_from_format_name

- Location: src/parser/mod.rs:1415
- Signature: `fn semantic_type_from_format_name(format_name: &str) -> Option<SemanticTypeHint> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decode_sas_missing_tag

- Location: src/parser/mod.rs:1451
- Signature: `fn decode_sas_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: parse_subheader_pointers

- Location: src/parser/mod.rs:1466
- Signature: `fn parse_subheader_pointers(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: subheader_slice

- Location: src/parser/mod.rs:1521
- Signature: `fn subheader_slice(page: &[u8], pointer: SubheaderPointer) -> Result<&[u8], ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: ensure_column_capacity

- Location: src/parser/mod.rs:1528
- Signature: `fn ensure_column_capacity(metadata: &mut PartialMetadata, len: usize) {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: ensure_remainder

- Location: src/parser/mod.rs:1534
- Signature: `fn ensure_remainder(subheader: &[u8], layout: DecodeLayout) -> Result<(), ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: resolve_text

- Location: src/parser/mod.rs:1551
- Signature: `fn resolve_text(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_page_header

- Location: src/parser/mod.rs:1579
- Signature: `fn read_page_header(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: read_page

- Location: src/parser/mod.rs:1592
- Signature: `fn read_page(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: page_offset

- Location: src/parser/mod.rs:1604
- Signature: `fn page_offset(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: read_exact_at

- Location: src/parser/mod.rs:1614
- Signature: `fn read_exact_at(`
- Role / observation: Parser hot-path helper around page or pointer handling; issue #7 makes allocation churn the main speculative concern.
- Speculative idea: reuse page and pointer scratch buffers here to cut heap churn on wide files.
- Hypothesis: repeated temporary allocation matters more than arithmetic in this region.
- Experiment idea: profile fts0003.sas7bdat and 10rec.sas7bdat with allocation sampling before and after buffer reuse.
- Result/evidence: issue #7 already points at this region, but this entry is still speculative and unprofiled in the current sweep.

### Function: read_text_ref

- Location: src/parser/mod.rs:1625
- Signature: `fn read_text_ref(`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_u16

- Location: src/parser/mod.rs:1637
- Signature: `fn read_u16(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u16, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_u32

- Location: src/parser/mod.rs:1647
- Signature: `fn read_u32(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u32, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_u64

- Location: src/parser/mod.rs:1657
- Signature: `fn read_u64(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u64, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: byte_at

- Location: src/parser/mod.rs:1671
- Signature: `fn byte_at(bytes: &[u8], offset: usize, message: &'static str) -> Result<u8, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: ensure_len

- Location: src/parser/mod.rs:1678
- Signature: `fn ensure_len(bytes: &[u8], min_len: usize, message: &'static str) -> Result<(), ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: pointer_compression_mode

- Location: src/parser/mod.rs:1685
- Signature: `fn pointer_compression_mode(compression: u8) -> CompressionMode {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: read_subheader_signature

- Location: src/parser/mod.rs:1693
- Signature: `fn read_subheader_signature(subheader: &[u8], layout: DecodeLayout) -> Result<u32, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: decode_text_bytes

- Location: src/parser/mod.rs:1707
- Signature: `fn decode_text_bytes(bytes: &[u8], text_encoding_code: u8) -> Result<String, ParserError> {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: trim_padded_bytes

- Location: src/parser/mod.rs:1714
- Signature: `fn trim_padded_bytes(bytes: &[u8]) -> &[u8] {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: io_error

- Location: src/parser/mod.rs:1722
- Signature: `fn io_error(error: std::io::Error) -> ParserError {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: empty_metadata

- Location: src/parser/mod.rs:1730
- Signature: `    fn empty_metadata() -> PartialMetadata {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: column_name_subheader_underflow_returns_an_error

- Location: src/parser/mod.rs:1751
- Signature: `    fn column_name_subheader_underflow_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: column_attrs_subheader_underflow_returns_an_error

- Location: src/parser/mod.rs:1769
- Signature: `    fn column_attrs_subheader_underflow_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: row_size_subheader_short_tail_returns_an_error

- Location: src/parser/mod.rs:1787
- Signature: `    fn row_size_subheader_short_tail_returns_an_error() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

## File: src/parser/offsets.rs

Coverage note: 8 function definitions were found in this file.

### Function: default

- Location: src/parser/offsets.rs:29
- Signature: `    fn default() -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: new

- Location: src/parser/offsets.rs:35
- Signature: `    pub fn new() -> Self {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: header_prefix_len

- Location: src/parser/offsets.rs:49
- Signature: `    pub fn header_prefix_len(&self) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: alignment_padding_len

- Location: src/parser/offsets.rs:53
- Signature: `    pub fn alignment_padding_len(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: header_size_offset

- Location: src/parser/offsets.rs:61
- Signature: `    pub fn header_size_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: page_size_offset

- Location: src/parser/offsets.rs:65
- Signature: `    pub fn page_size_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: page_count_offset

- Location: src/parser/offsets.rs:69
- Signature: `    pub fn page_count_offset(&self, header_prefix: &[u8]) -> usize {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

### Function: test_parser_offsets

- Location: src/parser/offsets.rs:79
- Signature: `    fn test_parser_offsets() {`
- Role / observation: Parser helper for layout, subheader, row, or streaming page handling.
- Speculative idea: use sample datasets from issue #1 to test whether branch reduction or allocation reuse matters more.
- Hypothesis: locality and temporary allocation patterns compound across many page/subheader calls.
- Experiment idea: instrument this function on one narrow fixture and one representative real sample dataset.
- Result/evidence: based on source reading and existing tests only; no direct microbenchmark was run for this function.

## File: src/transform/assumptions.rs

Coverage note: 3 function definitions were found in this file.

### Function: selected_cell_count

- Location: src/transform/assumptions.rs:9
- Signature: `    pub fn selected_cell_count(&self) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: build_synthetic_row_batch

- Location: src/transform/assumptions.rs:20
- Signature: `pub fn build_synthetic_row_batch(row_count: usize, column_count: usize) -> Vec<u64> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: run_projection_probe

- Location: src/transform/assumptions.rs:26
- Signature: `pub fn run_projection_probe(batch: &[u64], plan: &ProjectionProbePlan) -> ProjectionProbeResult {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

## File: src/transform/contracts.rs

Coverage note: 2 function definitions were found in this file.

### Function: label

- Location: src/transform/contracts.rs:53
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: supports_larger_than_memory_inputs

- Location: src/transform/contracts.rs:60
- Signature: `    pub fn supports_larger_than_memory_inputs(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

## File: src/transform/mod.rs

Coverage note: no `fn` definitions were found in this file during the inventory pass.

## File: src/transform/pipeline.rs

Coverage note: 26 function definitions were found in this file.

### Function: run

- Location: src/transform/pipeline.rs:16
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError>;`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: open

- Location: src/transform/pipeline.rs:20
- Signature: `    fn open(&self, source: &SourceContract)`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: open

- Location: src/transform/pipeline.rs:28
- Signature: `    fn open(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/pipeline.rs:44
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: fmt

- Location: src/transform/pipeline.rs:52
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from

- Location: src/transform/pipeline.rs:60
- Signature: `    fn from(error: std::io::Error) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: deferred

- Location: src/transform/pipeline.rs:75
- Signature: `    pub fn deferred() -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_execution

- Location: src/transform/pipeline.rs:85
- Signature: `    pub fn from_execution(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: not_yet_implemented

- Location: src/transform/pipeline.rs:108
- Signature: `    pub fn not_yet_implemented(request: TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: with_sink

- Location: src/transform/pipeline.rs:113
- Signature: `    pub fn with_sink(request: TransformRequest, sink: ParquetSinkReport) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: decoded_rows_staged

- Location: src/transform/pipeline.rs:122
- Signature: `    pub fn decoded_rows_staged(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: parquet_written

- Location: src/transform/pipeline.rs:135
- Signature: `    pub fn parquet_written(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: summary

- Location: src/transform/pipeline.rs:148
- Signature: `    pub fn summary(&self) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: fmt

- Location: src/transform/pipeline.rs:179
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: label

- Location: src/transform/pipeline.rs:192
- Signature: `    pub fn label(&self) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/pipeline.rs:207
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: fmt

- Location: src/transform/pipeline.rs:215
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from

- Location: src/transform/pipeline.rs:223
- Signature: `    fn from(error: ParquetSinkError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from

- Location: src/transform/pipeline.rs:229
- Signature: `    fn from(error: SourceDataLoaderError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from

- Location: src/transform/pipeline.rs:235
- Signature: `    fn from(error: ParserError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from

- Location: src/transform/pipeline.rs:241
- Signature: `    fn from(error: TransformExecutionError) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/pipeline.rs:252
- Signature: `    pub fn new(sink: S) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: run

- Location: src/transform/pipeline.rs:261
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/pipeline.rs:279
- Signature: `    pub fn new(loader: L, parser: P, sink: S) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: run

- Location: src/transform/pipeline.rs:294
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: bool_label

- Location: src/transform/pipeline.rs:318
- Signature: `fn bool_label(value: bool) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

## File: src/transform/sink.rs

Coverage note: 58 function definitions were found in this file.

### Function: from_request

- Location: src/transform/sink.rs:33
- Signature: `    pub fn from_request(request: &TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: prepare

- Location: src/transform/sink.rs:52
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError>;`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: stage_batches

- Location: src/transform/sink.rs:56
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: skeleton

- Location: src/transform/sink.rs:76
- Signature: `    pub fn skeleton(plan: ParquetSinkPlan) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: decoded_rows_staged

- Location: src/transform/sink.rs:88
- Signature: `    pub fn decoded_rows_staged(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: parquet_written

- Location: src/transform/sink.rs:106
- Signature: `    pub fn parquet_written(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: label

- Location: src/transform/sink.rs:134
- Signature: `    pub fn label(&self) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/sink.rs:149
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: fmt

- Location: src/transform/sink.rs:157
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_request

- Location: src/transform/sink.rs:174
- Signature: `    pub fn from_request(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: output_column_count

- Location: src/transform/sink.rs:228
- Signature: `    pub fn output_column_count(&self) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: selection_applied

- Location: src/transform/sink.rs:232
- Signature: `    pub fn selection_applied(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: filter_applied

- Location: src/transform/sink.rs:236
- Signature: `    pub fn filter_applied(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply

- Location: src/transform/sink.rs:240
- Signature: `    fn apply(&self, batch: RowBatch) -> Result<ExecutedBatch, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply_serial

- Location: src/transform/sink.rs:254
- Signature: `    fn apply_serial(&self, batch: RowBatch) -> Result<TypedBatch, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply_parallel

- Location: src/transform/sink.rs:263
- Signature: `    fn apply_parallel(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply_rows

- Location: src/transform/sink.rs:286
- Signature: `    fn apply_rows(&self, rows: &[ParsedRow]) -> Result<TypedBatchChunk, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: row_matches

- Location: src/transform/sink.rs:314
- Signature: `    fn row_matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_request

- Location: src/transform/sink.rs:329
- Signature: `    fn from_request(request: &TransformRequest) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: threads_for

- Location: src/transform/sink.rs:342
- Signature: `    fn threads_for(&self, row_count: usize) -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_source

- Location: src/transform/sink.rs:366
- Signature: `    fn from_source(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_source_column

- Location: src/transform/sink.rs:405
- Signature: `    fn from_source_column(column: &SasColumn) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: data_type

- Location: src/transform/sink.rs:418
- Signature: `    fn data_type(&self) -> DataType {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: is_nullable

- Location: src/transform/sink.rs:429
- Signature: `    fn is_nullable(&self) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: missing_tag_column_name

- Location: src/transform/sink.rs:434
- Signature: `fn missing_tag_column_name(column_name: &str) -> String {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: primary_field_metadata

- Location: src/transform/sink.rs:438
- Signature: `fn primary_field_metadata(column: &SasColumn) -> HashMap<String, String> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: missing_tag_field_metadata

- Location: src/transform/sink.rs:471
- Signature: `fn missing_tag_field_metadata(column: &SasColumn) -> HashMap<String, String> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: from_chunks

- Location: src/transform/sink.rs:487
- Signature: `    fn from_chunks(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: with_capacity

- Location: src/transform/sink.rs:529
- Signature: `    fn with_capacity(kind: ProjectionKind, capacity: usize) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: push

- Location: src/transform/sink.rs:548
- Signature: `    fn push(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: extend

- Location: src/transform/sink.rs:590
- Signature: `    fn extend(&mut self, other: Self) {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_float64

- Location: src/transform/sink.rs:617
- Signature: `fn materialized_float64(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_date32

- Location: src/transform/sink.rs:630
- Signature: `fn materialized_date32(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_time64_micros

- Location: src/transform/sink.rs:645
- Signature: `fn materialized_time64_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_timestamp_micros

- Location: src/transform/sink.rs:657
- Signature: `fn materialized_timestamp_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_duration_micros

- Location: src/transform/sink.rs:672
- Signature: `fn materialized_duration_micros(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: materialized_numeric_parts

- Location: src/transform/sink.rs:684
- Signature: `fn materialized_numeric_parts(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: decode_deferred_numeric

- Location: src/transform/sink.rs:700
- Signature: `fn decode_deferred_numeric(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: decode_materialized_missing_tag

- Location: src/transform/sink.rs:731
- Signature: `fn decode_materialized_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: expect_whole_number

- Location: src/transform/sink.rs:746
- Signature: `fn expect_whole_number(value: f64, column_name: &str) -> Result<i32, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: parse

- Location: src/transform/sink.rs:766
- Signature: `    fn parse(expression: &str, metadata: &SasMetadata) -> Result<Self, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: matches

- Location: src/transform/sink.rs:795
- Signature: `    fn matches(&self, row: &ParsedRow) -> Result<bool, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: parse

- Location: src/transform/sink.rs:841
- Signature: `    fn parse(token: &str, expression: &str) -> Result<Self, TransformExecutionError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply_numeric

- Location: src/transform/sink.rs:855
- Signature: `    fn apply_numeric(&self, actual: f64, expected: f64) -> bool {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: apply_string

- Location: src/transform/sink.rs:866
- Signature: `    fn apply_string(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: parse

- Location: src/transform/sink.rs:892
- Signature: `    fn parse(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: new

- Location: src/transform/sink.rs:928
- Signature: `    pub fn new(message: impl Into<String>) -> Self {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: fmt

- Location: src/transform/sink.rs:936
- Signature: `    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: default_worker_threads

- Location: src/transform/sink.rs:949
- Signature: `fn default_worker_threads() -> usize {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: transform_thread_pool

- Location: src/transform/sink.rs:955
- Signature: `fn transform_thread_pool(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: prepare

- Location: src/transform/sink.rs:982
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: stage_batches

- Location: src/transform/sink.rs:988
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: prepare

- Location: src/transform/sink.rs:1032
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: stage_batches

- Location: src/transform/sink.rs:1039
- Signature: `    fn stage_batches(`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: build_arrow_schema

- Location: src/transform/sink.rs:1107
- Signature: `fn build_arrow_schema(execution: &TransformExecution, metadata: &SasMetadata) -> Schema {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: typed_batch_to_record_batch

- Location: src/transform/sink.rs:1133
- Signature: `fn typed_batch_to_record_batch(batch: TypedBatch, schema: Arc<Schema>) -> RecordBatch {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: strip_quotes

- Location: src/transform/sink.rs:1156
- Signature: `fn strip_quotes(token: &str) -> &str {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

### Function: arrow_schema_preserves_parser_metadata_including_informats

- Location: src/transform/sink.rs:1178
- Signature: `    fn arrow_schema_preserves_parser_metadata_including_informats() {`
- Role / observation: Transform helper for planning, projection, typed staging, or sink orchestration.
- Speculative idea: look for fewer transient allocations, better chunk sizing, and stricter serial-vs-parallel thresholds.
- Hypothesis: call frequency and allocation shape matter more here than individual branch cost.
- Experiment idea: compare serial and parallel runs on numeric_1000000_2.sas7bdat and one wide-schema sample.
- Result/evidence: based on code structure and existing repository history; not a fresh optimization claim.

## File: src/validation/contracts.rs

Coverage note: 16 function definitions were found in this file.

### Function: label

- Location: src/validation/contracts.rs:14
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: readable_outcome

- Location: src/validation/contracts.rs:48
- Signature: `    pub fn readable_outcome(&self) -> Option<&ReadableOutcome> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: failure_kind

- Location: src/validation/contracts.rs:55
- Signature: `    pub fn failure_kind(&self) -> Option<ProbeFailureKind> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: failure_stage

- Location: src/validation/contracts.rs:62
- Signature: `    pub fn failure_stage(&self) -> Option<&'static str> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: failure_detail

- Location: src/validation/contracts.rs:69
- Signature: `    pub fn failure_detail(&self) -> Option<&str> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: is_readable

- Location: src/validation/contracts.rs:76
- Signature: `    pub fn is_readable(&self) -> bool {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: label

- Location: src/validation/contracts.rs:89
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: failure_count

- Location: src/validation/contracts.rs:114
- Signature: `    pub fn failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: expected_invalid_count

- Location: src/validation/contracts.rs:118
- Signature: `    pub fn expected_invalid_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: compatibility_failure_count

- Location: src/validation/contracts.rs:127
- Signature: `    pub fn compatibility_failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: render_text

- Location: src/validation/contracts.rs:137
- Signature: `    pub fn render_text(&self) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: label

- Location: src/validation/contracts.rs:204
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: label

- Location: src/validation/contracts.rs:237
- Signature: `    pub fn label(&self) -> &'static str {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: failure_count

- Location: src/validation/contracts.rs:260
- Signature: `    pub fn failure_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: skipped_count

- Location: src/validation/contracts.rs:272
- Signature: `    pub fn skipped_count(&self) -> usize {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: render_text

- Location: src/validation/contracts.rs:279
- Signature: `    pub fn render_text(&self) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

## File: src/validation/mod.rs

Coverage note: 27 function definitions were found in this file.

### Function: new

- Location: src/validation/mod.rs:283
- Signature: `    fn new(kind: ProbeFailureKind, stage: &'static str, detail: impl Into<String>) -> Self {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: from_parser

- Location: src/validation/mod.rs:291
- Signature: `    fn from_parser(stage: &'static str, error: ParserError) -> Self {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: sample_corpus_root

- Location: src/validation/mod.rs:302
- Signature: `pub fn sample_corpus_root() -> PathBuf {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: real_regression_cases

- Location: src/validation/mod.rs:306
- Signature: `pub fn real_regression_cases() -> &'static [RegressionCase] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: differential_fixture_specs

- Location: src/validation/mod.rs:310
- Signature: `pub fn differential_fixture_specs() -> &'static [DifferentialFixtureSpec] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: expected_invalid_sample_fixtures

- Location: src/validation/mod.rs:314
- Signature: `pub fn expected_invalid_sample_fixtures() -> &'static [InvalidFixtureCase] {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: classify_sample_corpus_fixture

- Location: src/validation/mod.rs:318
- Signature: `pub fn classify_sample_corpus_fixture(result: &ProbeResult) -> CorpusFixtureStatus {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: probe_file

- Location: src/validation/mod.rs:336
- Signature: `pub fn probe_file(path: &Path, batch_size_rows: usize) -> ProbeResult {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: sweep_sample_corpus

- Location: src/validation/mod.rs:403
- Signature: `pub fn sweep_sample_corpus(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: run_differential_validation

- Location: src/validation/mod.rs:435
- Signature: `pub fn run_differential_validation(output_root: &Path) -> DifferentialReport {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: run_differential_fixture

- Location: src/validation/mod.rs:443
- Signature: `fn run_differential_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: run_transform_fixture

- Location: src/validation/mod.rs:511
- Signature: `fn run_transform_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: canonicalize_parquet_fixture

- Location: src/validation/mod.rs:554
- Signature: `fn canonicalize_parquet_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: extend_value_lines<T, F>

- Location: src/validation/mod.rs:598
- Signature: `fn extend_value_lines<T, F>(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: canonicalize_haven_fixture

- Location: src/validation/mod.rs:631
- Signature: `fn canonicalize_haven_fixture(`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: first_mismatch_detail

- Location: src/validation/mod.rs:682
- Signature: `fn first_mismatch_detail(local_lines: &[String], trusted_lines: &[String]) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_total_rows

- Location: src/validation/mod.rs:696
- Signature: `fn read_total_rows(path: &Path) -> Result<usize, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_timestamp_column

- Location: src/validation/mod.rs:713
- Signature: `fn read_timestamp_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_date32_column

- Location: src/validation/mod.rs:738
- Signature: `fn read_date32_column(path: &Path, column_name: &str) -> Result<Vec<Option<i32>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_time64_column

- Location: src/validation/mod.rs:763
- Signature: `fn read_time64_column(path: &Path, column_name: &str) -> Result<Vec<Option<i64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_float64_column

- Location: src/validation/mod.rs:788
- Signature: `fn read_float64_column(path: &Path, column_name: &str) -> Result<Vec<Option<f64>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: read_utf8_column

- Location: src/validation/mod.rs:813
- Signature: `fn read_utf8_column(path: &Path, column_name: &str) -> Result<Vec<Option<String>>, String> {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: missing_tag_column_name

- Location: src/validation/mod.rs:839
- Signature: `fn missing_tag_column_name(column_name: &str) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: normalize_f64

- Location: src/validation/mod.rs:843
- Signature: `fn normalize_f64(value: f64) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: tag_token

- Location: src/validation/mod.rs:857
- Signature: `fn tag_token(tag: Option<&str>) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: classify_parser_error

- Location: src/validation/mod.rs:864
- Signature: `fn classify_parser_error(error: &ParserError) -> ProbeFailureKind {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

### Function: panic_detail

- Location: src/validation/mod.rs:872
- Signature: `fn panic_detail(payload: &Box<dyn std::any::Any + Send>) -> String {`
- Role / observation: Validation helper for corpus probing, canonicalization, or trusted-reader interop.
- Speculative idea: separate parser, filesystem, canonicalization, and subprocess costs before optimizing anything.
- Hypothesis: orchestration and I/O dominate more than local string handling in this layer.
- Experiment idea: time each stage independently on the current differential and corpus-sweep fixtures.
- Result/evidence: derived from code reading and existing validation structure, not from fresh profiling.

## File: tests/assumption_probe_contract.rs

Coverage note: 2 function definitions were found in this file.

### Function: synthetic_row_batches_are_row_major

- Location: tests/assumption_probe_contract.rs:6
- Signature: `fn synthetic_row_batches_are_row_major() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: projection_assumption_probe_reports_a_deterministic_checksum

- Location: tests/assumption_probe_contract.rs:13
- Signature: `fn projection_assumption_probe_reports_a_deterministic_checksum() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/cli_transform_contract.rs

Coverage note: 7 function definitions were found in this file.

### Function: new

- Location: tests/cli_transform_contract.rs:24
- Signature: `    fn new() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: single_request

- Location: tests/cli_transform_contract.rs:30
- Signature: `    fn single_request(&self) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: run

- Location: tests/cli_transform_contract.rs:36
- Signature: `    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: transform_command_builds_the_reviewable_stub_request

- Location: tests/cli_transform_contract.rs:43
- Signature: `fn transform_command_builds_the_reviewable_stub_request() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: transform_command_writes_a_real_parquet_file_through_the_default_service

- Location: tests/cli_transform_contract.rs:96
- Signature: `fn transform_command_writes_a_real_parquet_file_through_the_default_service() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: transform_command_defaults_to_the_streaming_execution_path

- Location: tests/cli_transform_contract.rs:125
- Signature: `fn transform_command_defaults_to_the_streaming_execution_path() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_float64_column

- Location: tests/cli_transform_contract.rs:143
- Signature: `fn read_float64_column(path: &std::path::Path, column_index: usize) -> Vec<f64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/parser_contract.rs

Coverage note: 6 function definitions were found in this file.

### Function: supported_subset_is_named_and_exposed_in_the_metadata

- Location: tests/parser_contract.rs:10
- Signature: `fn supported_subset_is_named_and_exposed_in_the_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_exposes_32_bit_little_endian_layout_metadata

- Location: tests/parser_contract.rs:27
- Signature: `fn parser_exposes_32_bit_little_endian_layout_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_exposes_32_bit_layout_metadata_when_header_offsets_are_padded

- Location: tests/parser_contract.rs:44
- Signature: `fn parser_exposes_32_bit_layout_metadata_when_header_offsets_are_padded() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_exposes_big_endian_layout_metadata

- Location: tests/parser_contract.rs:61
- Signature: `fn parser_exposes_big_endian_layout_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_rejects_malformed_word_size_headers

- Location: tests/parser_contract.rs:78
- Signature: `fn parser_rejects_malformed_word_size_headers() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: unsupported_page_types_return_a_structured_error

- Location: tests/parser_contract.rs:95
- Signature: `fn unsupported_page_types_return_a_structured_error() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/parser_decode_contract.rs

Coverage note: 23 function definitions were found in this file.

### Function: sample_dataset_path

- Location: tests/parser_decode_contract.rs:24
- Signature: `fn sample_dataset_path(file_name: &str) -> PathBuf {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: probe_sample_via_parser_entrypoint

- Location: tests/parser_decode_contract.rs:30
- Signature: `fn probe_sample_via_parser_entrypoint(file_name: &str) -> RealFileProbeOutcome {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: probe_fts0003_via_parser_entrypoint

- Location: tests/parser_decode_contract.rs:68
- Signature: `fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: assert_real_file_is_readable

- Location: tests/parser_decode_contract.rs:72
- Signature: `fn assert_real_file_is_readable(file_name: &str) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: drain_all_rows

- Location: tests/parser_decode_contract.rs:87
- Signature: `fn drain_all_rows(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: numeric_missing_tag

- Location: tests/parser_decode_contract.rs:101
- Signature: `fn numeric_missing_tag(value: &ParsedValue) -> Option<SasMissingTag> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_metadata_and_batches_from_the_supported_subset_fixture

- Location: tests/parser_decode_contract.rs:109
- Signature: `fn parser_decodes_metadata_and_batches_from_the_supported_subset_fixture() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_uncompressed_32_bit_little_endian_fixture_end_to_end

- Location: tests/parser_decode_contract.rs:195
- Signature: `fn parser_decodes_uncompressed_32_bit_little_endian_fixture_end_to_end() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_latin1_strings_without_claiming_utf8_only_support

- Location: tests/parser_decode_contract.rs:222
- Signature: `fn parser_decodes_latin1_strings_without_claiming_utf8_only_support() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_uncompressed_big_endian_fixture_end_to_end

- Location: tests/parser_decode_contract.rs:247
- Signature: `fn parser_decodes_uncompressed_big_endian_fixture_end_to_end() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_supported_subset_across_multiple_data_pages

- Location: tests/parser_decode_contract.rs:288
- Signature: `fn parser_decodes_supported_subset_across_multiple_data_pages() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_defers_multi_page_row_reads_until_batches_are_requested

- Location: tests/parser_decode_contract.rs:321
- Signature: `fn parser_defers_multi_page_row_reads_until_batches_are_requested() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_row_compressed_rows_stored_across_meta_and_mix_pages

- Location: tests/parser_decode_contract.rs:363
- Signature: `fn parser_decodes_row_compressed_rows_stored_across_meta_and_mix_pages() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_binary_compressed_rows_from_meta_subheaders

- Location: tests/parser_decode_contract.rs:408
- Signature: `fn parser_decodes_binary_compressed_rows_from_meta_subheaders() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_preserves_non_8_byte_numeric_cells_without_parser_core_rejection

- Location: tests/parser_decode_contract.rs:447
- Signature: `fn parser_preserves_non_8_byte_numeric_cells_without_parser_core_rejection() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_reads_the_real_10rec_file_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:493
- Signature: `fn parser_reads_the_real_10rec_file_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_reads_the_real_fts0003_file_through_the_compressed_entrypoint

- Location: tests/parser_decode_contract.rs:509
- Signature: `fn parser_reads_the_real_fts0003_file_through_the_compressed_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_reads_real_binary_compressed_samples_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:525
- Signature: `fn parser_reads_real_binary_compressed_samples_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_reads_real_non_utf8_samples_through_the_existing_entrypoint

- Location: tests/parser_decode_contract.rs:548
- Signature: `fn parser_reads_real_non_utf8_samples_through_the_existing_entrypoint() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_decodes_real_gb18030_text_values_honestly

- Location: tests/parser_decode_contract.rs:566
- Signature: `fn parser_decodes_real_gb18030_text_values_honestly() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_infers_semantic_types_and_column_metadata_from_fixture_formats

- Location: tests/parser_decode_contract.rs:588
- Signature: `fn parser_infers_semantic_types_and_column_metadata_from_fixture_formats() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_preserves_real_dates_fixture_semantic_metadata

- Location: tests/parser_decode_contract.rs:680
- Signature: `fn parser_preserves_real_dates_fixture_semantic_metadata() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_exposes_real_special_missing_tags_without_flattening_them

- Location: tests/parser_decode_contract.rs:710
- Signature: `fn parser_exposes_real_special_missing_tags_without_flattening_them() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/transform_contract.rs

Coverage note: 5 function definitions were found in this file.

### Function: stub_transform_service_returns_a_structured_not_yet_implemented_report

- Location: tests/transform_contract.rs:17
- Signature: `fn stub_transform_service_returns_a_structured_not_yet_implemented_report() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bounded_memory_execution_path_is_explicit_in_the_contract

- Location: tests/transform_contract.rs:33
- Signature: `fn bounded_memory_execution_path_is_explicit_in_the_contract() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parquet_sink_plan_caps_row_groups_to_the_bounded_memory_budget

- Location: tests/transform_contract.rs:42
- Signature: `fn parquet_sink_plan_caps_row_groups_to_the_bounded_memory_budget() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: transform_execution_accepts_physical_numeric_columns_without_forcing_semantics

- Location: tests/transform_contract.rs:61
- Signature: `fn transform_execution_accepts_physical_numeric_columns_without_forcing_semantics() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: example_request

- Location: tests/transform_contract.rs:110
- Signature: `fn example_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/transform_parser_integration.rs

Coverage note: 36 function definitions were found in this file.

### Function: open

- Location: tests/transform_parser_integration.rs:37
- Signature: `    fn open(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: open

- Location: tests/transform_parser_integration.rs:52
- Signature: `    fn open(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: prepare

- Location: tests/transform_parser_integration.rs:70
- Signature: `    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: stage_batches

- Location: tests/transform_parser_integration.rs:76
- Signature: `    fn stage_batches(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_writes_selected_and_filtered_parquet_output

- Location: tests/transform_parser_integration.rs:105
- Signature: `fn parser_transform_service_writes_selected_and_filtered_parquet_output() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_rejects_unsupported_filter_expressions

- Location: tests/transform_parser_integration.rs:140
- Signature: `fn parser_transform_service_rejects_unsupported_filter_expressions() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_materializes_non_8_byte_numeric_columns_with_filtering

- Location: tests/transform_parser_integration.rs:155
- Signature: `fn parser_transform_service_materializes_non_8_byte_numeric_columns_with_filtering() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_materializes_big_endian_non_8_byte_missing_tags

- Location: tests/transform_parser_integration.rs:202
- Signature: `fn parser_transform_service_materializes_big_endian_non_8_byte_missing_tags() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_uses_bounded_memory_batches_for_multi_page_output

- Location: tests/transform_parser_integration.rs:251
- Signature: `fn parser_transform_service_uses_bounded_memory_batches_for_multi_page_output() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_reports_parallel_batch_execution_when_worker_threads_are_used

- Location: tests/transform_parser_integration.rs:286
- Signature: `fn parser_transform_service_reports_parallel_batch_execution_when_worker_threads_are_used() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_starts_batching_before_the_full_dataset_is_read

- Location: tests/transform_parser_integration.rs:314
- Signature: `fn parser_transform_service_starts_batching_before_the_full_dataset_is_read() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_projects_semantic_numeric_columns_into_arrow_types

- Location: tests/transform_parser_integration.rs:347
- Signature: `fn parser_transform_service_projects_semantic_numeric_columns_into_arrow_types() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_preserves_real_date_metadata_in_parquet_schema

- Location: tests/transform_parser_integration.rs:454
- Signature: `fn parser_transform_service_preserves_real_date_metadata_in_parquet_schema() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parser_transform_service_preserves_real_special_missing_values_with_sidecar_tags

- Location: tests/transform_parser_integration.rs:494
- Signature: `fn parser_transform_service_preserves_real_special_missing_values_with_sidecar_tags() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: example_request

- Location: tests/transform_parser_integration.rs:524
- Signature: `fn example_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: unsupported_filter_request

- Location: tests/transform_parser_integration.rs:549
- Signature: `fn unsupported_filter_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: narrow_numeric_request

- Location: tests/transform_parser_integration.rs:574
- Signature: `fn narrow_numeric_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: narrow_numeric_filtered_request

- Location: tests/transform_parser_integration.rs:599
- Signature: `fn narrow_numeric_filtered_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: truncated_numeric_bytes

- Location: tests/transform_parser_integration.rs:605
- Signature: `fn truncated_numeric_bytes(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: truncated_missing_numeric_bytes

- Location: tests/transform_parser_integration.rs:618
- Signature: `fn truncated_missing_numeric_bytes(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: truncate_numeric_storage_bytes

- Location: tests/transform_parser_integration.rs:628
- Signature: `fn truncate_numeric_storage_bytes(raw: Vec<u8>, endianness: Endianness, width: usize) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bounded_memory_request

- Location: tests/transform_parser_integration.rs:635
- Signature: `fn bounded_memory_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: parallel_batch_request

- Location: tests/transform_parser_integration.rs:662
- Signature: `fn parallel_batch_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: semantic_fixture_request

- Location: tests/transform_parser_integration.rs:689
- Signature: `fn semantic_fixture_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: real_dates_request

- Location: tests/transform_parser_integration.rs:714
- Signature: `fn real_dates_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: real_missing_request

- Location: tests/transform_parser_integration.rs:741
- Signature: `fn real_missing_request(output_path: PathBuf) -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: first_batch_only_request

- Location: tests/transform_parser_integration.rs:768
- Signature: `fn first_batch_only_request() -> TransformRequest {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_parquet_schema

- Location: tests/transform_parser_integration.rs:795
- Signature: `fn read_parquet_schema(path: &Path) -> Vec<(String, String)> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_total_rows

- Location: tests/transform_parser_integration.rs:807
- Signature: `fn read_total_rows(path: &Path) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_optional_float64_column

- Location: tests/transform_parser_integration.rs:818
- Signature: `fn read_optional_float64_column(path: &Path, column_index: usize) -> Vec<Option<f64>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_float64_column

- Location: tests/transform_parser_integration.rs:839
- Signature: `fn read_float64_column(path: &Path, column_index: usize) -> Vec<f64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_optional_utf8_column

- Location: tests/transform_parser_integration.rs:846
- Signature: `fn read_optional_utf8_column(path: &Path, column_index: usize) -> Vec<Option<String>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_utf8_column

- Location: tests/transform_parser_integration.rs:867
- Signature: `fn read_utf8_column(path: &Path, column_index: usize) -> Vec<String> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_optional_i32_column

- Location: tests/transform_parser_integration.rs:874
- Signature: `fn read_optional_i32_column(path: &Path, column_index: usize) -> Vec<Option<i32>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_optional_i64_column

- Location: tests/transform_parser_integration.rs:895
- Signature: `fn read_optional_i64_column(path: &Path, column_index: usize) -> Vec<Option<i64>> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read_field_metadata

- Location: tests/transform_parser_integration.rs:934
- Signature: `fn read_field_metadata(path: &Path, field_name: &str) -> HashMap<String, String> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/validation_contract.rs

Coverage note: 6 function definitions were found in this file.

### Function: real_regression_corpus_includes_required_categories_and_baseline_fixture

- Location: tests/validation_contract.rs:12
- Signature: `fn real_regression_corpus_includes_required_categories_and_baseline_fixture() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: differential_fixtures_cover_the_supported_semantic_surface

- Location: tests/validation_contract.rs:44
- Signature: `fn differential_fixtures_cover_the_supported_semantic_surface() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: sample_corpus_sweep_reports_mixed_results_honestly

- Location: tests/validation_contract.rs:69
- Signature: `fn sample_corpus_sweep_reports_mixed_results_honestly() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: invalid_sample_fixture_policy_is_explicit_and_reviewable

- Location: tests/validation_contract.rs:102
- Signature: `fn invalid_sample_fixture_policy_is_explicit_and_reviewable() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: expected_invalid_sample_fixtures_match_their_current_probe_results

- Location: tests/validation_contract.rs:121
- Signature: `fn expected_invalid_sample_fixtures_match_their_current_probe_results() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: curated_real_regression_cases_match_their_current_expectations

- Location: tests/validation_contract.rs:135
- Signature: `fn curated_real_regression_cases_match_their_current_expectations() {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

## File: tests/support/minimal_sas_fixture.rs

Coverage note: 63 function definitions were found in this file.

### Function: bit64_little

- Location: tests/support/minimal_sas_fixture.rs:51
- Signature: `    pub const fn bit64_little() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bit64_big

- Location: tests/support/minimal_sas_fixture.rs:58
- Signature: `    pub const fn bit64_big() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bit32_little

- Location: tests/support/minimal_sas_fixture.rs:65
- Signature: `    pub const fn bit32_little() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: word_size_bytes

- Location: tests/support/minimal_sas_fixture.rs:72
- Signature: `    fn word_size_bytes(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: page_header_size

- Location: tests/support/minimal_sas_fixture.rs:79
- Signature: `    fn page_header_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: subheader_pointer_size

- Location: tests/support/minimal_sas_fixture.rs:86
- Signature: `    fn subheader_pointer_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: subheader_data_offset

- Location: tests/support/minimal_sas_fixture.rs:93
- Signature: `    fn subheader_data_offset(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_attrs_entry_size

- Location: tests/support/minimal_sas_fixture.rs:100
- Signature: `    fn column_attrs_entry_size(self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: row_size_offsets

- Location: tests/support/minimal_sas_fixture.rs:104
- Signature: `    fn row_size_offsets(self) -> (usize, usize, usize, usize, usize) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: numeric_bytes

- Location: tests/support/minimal_sas_fixture.rs:111
- Signature: `    fn numeric_bytes(self, value: f64) -> [u8; 8] {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: default

- Location: tests/support/minimal_sas_fixture.rs:120
- Signature: `    fn default() -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bytes_read

- Location: tests/support/minimal_sas_fixture.rs:179
- Signature: `    pub fn bytes_read(&self) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: record_read

- Location: tests/support/minimal_sas_fixture.rs:183
- Signature: `    fn record_read(&self, count: usize) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: new

- Location: tests/support/minimal_sas_fixture.rs:195
- Signature: `    pub fn new(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> Self {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: read

- Location: tests/support/minimal_sas_fixture.rs:204
- Signature: `    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: seek

- Location: tests/support/minimal_sas_fixture.rs:212
- Signature: `    fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: supported_fixture_definition

- Location: tests/support/minimal_sas_fixture.rs:217
- Signature: `pub fn supported_fixture_definition() -> FixtureDefinition {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: supported_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:255
- Signature: `pub fn supported_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bit32_little_endian_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:259
- Signature: `pub fn bit32_little_endian_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: bit32_little_endian_padded_header_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:265
- Signature: `pub fn bit32_little_endian_padded_header_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: big_endian_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:272
- Signature: `pub fn big_endian_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: latin1_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:278
- Signature: `pub fn latin1_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: compressed_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:288
- Signature: `pub fn compressed_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: row_compressed_mixed_page_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:292
- Signature: `pub fn row_compressed_mixed_page_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: binary_compressed_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:315
- Signature: `pub fn binary_compressed_fixture_bytes() -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: unsupported_page_type_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:320
- Signature: `pub fn unsupported_page_type_fixture_bytes(page_type: u16) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: malformed_word_size_fixture_bytes

- Location: tests/support/minimal_sas_fixture.rs:332
- Signature: `pub fn malformed_word_size_fixture_bytes(word_size_marker: u8) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_fixture_file

- Location: tests/support/minimal_sas_fixture.rs:338
- Signature: `pub fn write_fixture_file(definition: &FixtureDefinition, path: &Path) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: unique_tmp_path

- Location: tests/support/minimal_sas_fixture.rs:342
- Signature: `pub fn unique_tmp_path(prefix: &str, extension: &str) -> PathBuf {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: page_count_for

- Location: tests/support/minimal_sas_fixture.rs:355
- Signature: `pub fn page_count_for(definition: &FixtureDefinition) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: lazy_parse_read_budget

- Location: tests/support/minimal_sas_fixture.rs:362
- Signature: `pub fn lazy_parse_read_budget(page_count: usize) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: first_batch_read_budget

- Location: tests/support/minimal_sas_fixture.rs:366
- Signature: `pub fn first_batch_read_budget(page_count: usize) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: tracked_reader

- Location: tests/support/minimal_sas_fixture.rs:370
- Signature: `pub fn tracked_reader(bytes: Vec<u8>) -> (TrackingCursor, Arc<ReadMonitor>) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: tracked_reader_with_monitor

- Location: tests/support/minimal_sas_fixture.rs:376
- Signature: `pub fn tracked_reader_with_monitor(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> TrackingCursor {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: build_fixture

- Location: tests/support/minimal_sas_fixture.rs:380
- Signature: `pub fn build_fixture(definition: &FixtureDefinition) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: name

- Location: tests/support/minimal_sas_fixture.rs:470
- Signature: `    pub fn name(&self) -> &str {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: build_subheaders

- Location: tests/support/minimal_sas_fixture.rs:477
- Signature: `fn build_subheaders(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: row_size_subheader

- Location: tests/support/minimal_sas_fixture.rs:510
- Signature: `fn row_size_subheader(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_size_subheader

- Location: tests/support/minimal_sas_fixture.rs:556
- Signature: `fn column_size_subheader(column_count: usize, layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_text_subheader

- Location: tests/support/minimal_sas_fixture.rs:573
- Signature: `fn column_text_subheader(text_blob: &[u8], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_name_subheader

- Location: tests/support/minimal_sas_fixture.rs:591
- Signature: `fn column_name_subheader(column_name_refs: &[TextRef], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_attrs_subheader

- Location: tests/support/minimal_sas_fixture.rs:612
- Signature: `fn column_attrs_subheader(columns: &[FixtureColumn], layout: FixtureLayout) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_format_subheader

- Location: tests/support/minimal_sas_fixture.rs:653
- Signature: `fn column_format_subheader(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_header

- Location: tests/support/minimal_sas_fixture.rs:704
- Signature: `fn write_header(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_meta_page

- Location: tests/support/minimal_sas_fixture.rs:751
- Signature: `fn write_meta_page(bytes: &mut [u8], subheaders: &[Vec<u8>], layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: build_compressed_fixture

- Location: tests/support/minimal_sas_fixture.rs:790
- Signature: `fn build_compressed_fixture(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: encode_rle_copy_row

- Location: tests/support/minimal_sas_fixture.rs:895
- Signature: `fn encode_rle_copy_row(bytes: &[u8]) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: encode_binary_literal_row

- Location: tests/support/minimal_sas_fixture.rs:906
- Signature: `fn encode_binary_literal_row(bytes: &[u8]) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_subheader_row_page

- Location: tests/support/minimal_sas_fixture.rs:915
- Signature: `fn write_subheader_row_page(bytes: &mut [u8], payloads: &[Vec<u8>], layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_mix_page

- Location: tests/support/minimal_sas_fixture.rs:961
- Signature: `fn write_mix_page(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_data_page

- Location: tests/support/minimal_sas_fixture.rs:993
- Signature: `fn write_data_page(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_row

- Location: tests/support/minimal_sas_fixture.rs:1023
- Signature: `fn write_row(`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: subheader_remainder

- Location: tests/support/minimal_sas_fixture.rs:1077
- Signature: `fn subheader_remainder(len: usize, layout: FixtureLayout) -> u16 {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: normalized_column_metadata

- Location: tests/support/minimal_sas_fixture.rs:1081
- Signature: `fn normalized_column_metadata(definition: &FixtureDefinition) -> Vec<FixtureColumnMetadata> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: tagged_missing_numeric_bytes

- Location: tests/support/minimal_sas_fixture.rs:1088
- Signature: `pub fn tagged_missing_numeric_bytes(layout: FixtureLayout, tag: char) -> Vec<u8> {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: append_text

- Location: tests/support/minimal_sas_fixture.rs:1098
- Signature: `fn append_text(blob: &mut Vec<u8>, value: &str) -> TextRef {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: column_width

- Location: tests/support/minimal_sas_fixture.rs:1113
- Signature: `fn column_width(column: &FixtureColumn) -> usize {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_padded_ascii

- Location: tests/support/minimal_sas_fixture.rs:1120
- Signature: `fn write_padded_ascii(bytes: &mut [u8], offset: usize, len: usize, value: &str) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_text_ref

- Location: tests/support/minimal_sas_fixture.rs:1129
- Signature: `fn write_text_ref(bytes: &mut [u8], offset: usize, text_ref: TextRef, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_word

- Location: tests/support/minimal_sas_fixture.rs:1135
- Signature: `fn write_word(bytes: &mut [u8], offset: usize, value: u64, layout: FixtureLayout) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_u16

- Location: tests/support/minimal_sas_fixture.rs:1142
- Signature: `fn write_u16(bytes: &mut [u8], offset: usize, value: u16, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_u32

- Location: tests/support/minimal_sas_fixture.rs:1150
- Signature: `fn write_u32(bytes: &mut [u8], offset: usize, value: u32, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

### Function: write_u64

- Location: tests/support/minimal_sas_fixture.rs:1158
- Signature: `fn write_u64(bytes: &mut [u8], offset: usize, value: u64, endianness: Endianness) {`
- Role / observation: Contract test or test helper included as executable documentation for one scenario.
- Speculative idea: if suite time grows, fold repeated setup into parameterized helpers or cached fixture bytes.
- Hypothesis: the best improvement here is faster experimentation, not faster production runtime.
- Experiment idea: compare wall-clock test time before and after deduplicating setup.
- Result/evidence: evidence here is behavioral coverage only; no timing work was run during the sweep.

