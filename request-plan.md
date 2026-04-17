# Request Plan

- request_id: 2026-04-17-sas-rs-full-spec-rebaseline
- user_goal: Build SAS-rs from the attached spec into the fastest practical `.sas7bdat` reader possible, with a CLI that transforms to parquet, strong benchmarking, unit tests, idiomatic Rust, larger-than-memory performance, and the broader compatibility and correctness coverage explicitly required by spec.md.
- authoritative_spec: spec.md
- request_baseline_note: This is a materially reopened request against the full authoritative spec. The prior subset-complete request plan is no longer authoritative because it did not satisfy the full layout, endianness, encoding, compression, page and subheader, semantic typing, metadata preservation, fuzzing, regression, and proof obligations in spec.md.
- current_request_state: pr_scope_ready

## Ordered PR Scopes

1. capability_contracts_numeric_widths_and_honest_harness
   - status: complete
   - objective: Replace subset-specific parser and transform contracts with matrix-capable physical and semantic schema interfaces, preserve the current supported subset through them, remove the hard 8-byte numeric assumption, make parser benchmarking drain streamed batches instead of parse-entry timing, and use real sample datasets starting with sample-sas-datasets/fts0003.sas7bdat to test actual readability and establish an initial real-file baseline for future experiments.
   - included_requirements: 6, 7, 19
   - deferred_requirements: 1, 2, 3, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26
   - why_now: The current code hard-codes a narrow physical value model. Later layout, encoding, compression, and semantic work will be brittle unless the shared contracts and validation seams are generalized first, and the user explicitly wants the first real-dataset probe to happen now rather than after a synthetic-only review.

2. portable_core_decode_layouts_endianness_encodings_and_subheaders
   - status: ready
   - objective: Extend the core parser to handle 32-bit and 64-bit layouts, little-endian and big-endian variants, non-UTF-8 encodings with latin-1 support, and the broader subheader set required for those files.
   - included_requirements: 12, 13, 14, 18
   - deferred_requirements: 1, 2, 3, 4, 9, 10, 11, 15, 16, 17, 20, 21, 22, 23, 24, 25, 26
   - why_later: Depends on PR-01 providing matrix-capable contracts and keeping layout portability separate from compression and semantic policy.

3. compression_and_full_page_type_coverage
   - status: planned
   - objective: Add row compression, binary compression, and the broader page-type dispatcher coverage needed to process files outside the current META and DATA subset while preserving streaming behavior.
   - included_requirements: 15, 16, 17
   - deferred_requirements: 1, 2, 3, 4, 9, 10, 11, 20, 21, 22, 23, 24, 25, 26
   - why_later: Compression and page-dispatch expansion are tightly coupled, but they should build on the portable core decode from PR-02.

4. semantic_sas_typing_and_metadata_preservation
   - status: planned
   - objective: Map physical SAS values into honest Arrow and Parquet semantics, including dates, times, datetimes, duration-like values, labels, formats, informats, and SAS special missing values.
   - included_requirements: 11, 20, 21, 22
   - deferred_requirements: 1, 2, 3, 4, 9, 10, 23, 24, 25, 26
   - why_later: This scope needs the broader physical decode matrix in place first so semantic policy is applied once rather than duplicated across earlier parser changes.

5. robustness_corpus_fuzzing_and_differential_validation
   - status: planned
   - objective: Build the broadened safety net with malformed-input coverage, fuzzing, a real regression corpus, an automated sweep over sample-sas-datasets, and differential validation against a trusted reader.
   - included_requirements: 6, 23, 24
   - deferred_requirements: 1, 2, 3, 4, 9, 10, 25, 26
   - why_later: Validation hardening is most valuable after the compatibility and semantic matrix has expanded materially.

6. performance_memory_proof_and_request_closure
   - status: planned
   - objective: Prove that the broadened implementation still meets the request-level bar for streaming, bounded memory, CLI usability, parallel execution, and throughput on realistic workloads, then close the full request honestly.
   - included_requirements: 1, 2, 3, 4, 5, 8, 9, 10, 25, 26
   - deferred_requirements: none
   - why_later: This is the closure scope. It must run only after broad compatibility, semantic fidelity, and robustness work exist for the full spec.

## Active PR Scope

- active_pr_scope: portable_core_decode_layouts_endianness_encodings_and_subheaders

## Completed PR Scopes

- capability_contracts_numeric_widths_and_honest_harness

## Deferred PR Scopes

- none

## Blocked PR Scopes

- none

## Request Completion Gates

- Requirements 1 through 4 are satisfied with evidence on broadened realistic workloads, not only on the prior supported synthetic subset.
- Requirements 5 and 8 are satisfied across the delivered implementation and supporting documentation, not merely assumed as general quality bars.
- Requirements 6, 7, 23, and 24 are satisfied with automated tests, fuzzing, regression coverage, and benchmark harnesses that cover the broadened format matrix.
- Requirements 9, 10, 25, and 26 are proven on the broadened feature set, including layouts, encodings, compression, and richer value semantics.
- Requirements 11 through 22 are satisfied across the parser and transform pipeline with explicit Arrow and Parquet mapping behavior.
- Real-dataset evidence starts with sample-sas-datasets/fts0003.sas7bdat, expands honestly rather than inferred from synthetic fixtures alone, and includes an automated corpus sweep over sample-sas-datasets as compatibility support grows.
- Request completion requires an automated read-validation pass across the full sample-sas-datasets corpus, with fts0003.sas7bdat retained as the primary baseline-performance fixture for future experiments.
- No remaining uncovered spec items are implied complete; they remain tracked in unresolved PR scopes until evidence says otherwise.

## Final Response Readiness

- final_response_readiness: not ready
- reason: PR-01 is complete and PR-02 is the next ready scope, but the full request still has multiple unresolved planned scopes and no broadened compatibility proof yet.
