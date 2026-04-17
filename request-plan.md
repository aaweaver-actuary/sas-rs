# Request Plan

- request_id: 2026-04-16-sas-rs-fastest-sas7bdat-reader
- user_goal: Build SAS-rs from the attached spec into the fastest practical `.sas7bdat` reader possible, with a CLI that transforms to parquet, strong benchmarking, unit tests, idiomatic Rust, and a credible path to larger-than-memory performance.
- authoritative_spec: spec.md
- request_baseline_note: This is a materially new request. Prior completion artifacts, if any, are not authoritative unless explicitly revalidated against this spec.
- current_request_state: request_scoped

## Ordered PR Scopes

1. foundation_transform_path
   - status: ready
   - objective: Establish the core crate architecture, CLI contract, transform pipeline interfaces, parquet sink path, benchmark harness, and first validation of performance assumptions.
   - why_now: This creates the end-to-end skeleton and measurement discipline needed before deeper parser optimization work.

2. parser_core_and_streaming_decode
   - status: planned
   - objective: Implement the first high-throughput `.sas7bdat` parsing and row/page decoding pipeline with streaming-oriented interfaces and correctness coverage.
   - why_later: Depends on the scope-1 contracts and benchmark harness.

3. typed_transform_and_memory_scaling
   - status: planned
   - objective: Add column selection/filtering, robust type mapping, larger-than-memory execution strategy, and transform correctness for wider datasets.
   - why_later: Depends on parser data contracts and sink integration.

4. performance_push_and_completion
   - status: planned
   - objective: Optimize hot paths, harden benchmark evidence, close remaining spec gaps, and prepare final production-ready validation.
   - why_later: Requires stable end-to-end behavior before aggressive tuning.

## Active PR Scope

- active_pr_scope: foundation_transform_path

## Completed PR Scopes

- none

## Deferred PR Scopes

- none

## Blocked PR Scopes

- none

## Request Completion Gates

- A CLI exists for transforming `.sas7bdat` input to parquet output.
- The implementation exposes a clear, testable, idiomatic Rust parsing and transform architecture.
- Benchmarks exist for useful pipeline stages and are used to validate performance assumptions.
- Unit tests exist for core behavior and interfaces.
- The system demonstrates a path toward larger-than-memory datasets.
- The repository contains evidence-backed progress toward the sub-minute transform target for approximately 20M rows.
- Remaining uncovered spec items, if any, are explicitly tracked in future PR scopes rather than implied complete.

## Final Response Readiness

- final_response_readiness: not ready
- reason: No PR scopes have been executed yet for this request.
