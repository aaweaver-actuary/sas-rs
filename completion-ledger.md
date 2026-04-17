# Completion Ledger

- pr_scope_id: capability_contracts_numeric_widths_and_honest_harness
- authoritative_spec: spec.md
- authoritative_request_plan: request-plan.md
- ledger_baseline_note: Fresh active ledger created on 2026-04-17 because the previous performance_push_and_completion ledger was complete for a materially different, subset-complete PR scope and is not authoritative after the full-spec rebaseline.
- pr_scope_state: complete
- outcome_status: complete

## PR Scope Summary

- objective: Replace subset-specific parser and transform contracts with matrix-capable physical and semantic schema interfaces, preserve the current supported subset through those new contracts, remove the hard 8-byte numeric assumption, make parser benchmarking drain streamed batches instead of timing parse setup only, and establish an honest first real-file probe and baseline on sample-sas-datasets/fts0003.sas7bdat.
- issue_membership: spec.md requirements 6, 7, and 19
- issue_delta_status: open_issue_tracked_non_blocking
- issue_delta_evidence: The last validated issue sync for this PR scope still showed only GitHub issue #1, SAS Sample Datasets, as request-relevant but non-blocking. A direct rerun from this environment during ledger closeout returned GitHub API 403, so this final ledger update carries forward that last validated non-blocking result instead of inventing a clean live sync. Because this step is ledger-only and introduces no new product changes, the active PR scope remains clear to close while the full request stays open for later corpus work.
- planning_status: The bounded `fts0003_real_file_probe_and_baseline` slice is now recorded with validation evidence, final PR review has cleared the active PR scope, and no next PR scope is started by this ledger-only closeout.

## State Transitions

1. 2026-04-17: intake -> pr_scope_defined
   - reason: request-plan.md rebaselined the request against the full authoritative spec and the existing completion-ledger.md described a different completed PR scope.
2. 2026-04-17: pr_scope_defined -> slice_ready
   - reason: Confirmed the active PR scope, bounded package, included requirements, and deferred non-goals.
3. 2026-04-17: slice_ready -> slice_in_progress
   - reason: Started the contract_gap_and_harness_assessment slice to evaluate the bounded package against the current repository state.
4. 2026-04-17: slice_in_progress -> slice_review
   - reason: Completed bounded-file review plus the required scoped cargo commands.
5. 2026-04-17: slice_review -> awaiting_issue_sync
   - reason: Assessment evidence was gathered and repo-qualified issue sync was re-run at the slice boundary.
6. 2026-04-17: awaiting_issue_sync -> slice_ready
   - reason: Open issue #1 is tracked for later request validation work but does not block the next bounded implementation slice for this PR scope.
7. 2026-04-17: slice_ready -> slice_in_progress
   - reason: Started the matrix_capable_contracts_numeric_widths_and_streamed_decode_benchmark implementation slice.
8. 2026-04-17: slice_in_progress -> slice_review
   - reason: Landed the bounded implementation changes, refreshed the scoped docs, and passed the required cargo test, cargo bench, and cargo fmt --check validations.
9. 2026-04-17: slice_review -> awaiting_issue_sync
   - reason: Re-ran issue sync at the slice boundary after the implementation evidence was gathered.
10. 2026-04-17: awaiting_issue_sync -> pr_review
    - reason: Acceptance gates 1 through 6 were satisfied for the active PR scope, pending the required real-file baseline closeout.
11. 2026-04-17: pr_review -> slice_ready
    - reason: PR review held the scope open until the bounded real-file probe and baseline on sample-sas-datasets/fts0003.sas7bdat was captured honestly.
12. 2026-04-17: slice_ready -> slice_in_progress
    - reason: Started the fts0003_real_file_probe_and_baseline slice.
13. 2026-04-17: slice_in_progress -> slice_review
    - reason: Completed the bounded real-file probe slice and gathered the required scoped test and benchmark evidence.
14. 2026-04-17: slice_review -> awaiting_issue_sync
    - reason: Re-checked PR-scope issue status at the slice boundary and carried forward the last validated non-blocking issue result after a direct GitHub API rerun returned 403 from this environment.
15. 2026-04-17: awaiting_issue_sync -> pr_review
    - reason: All active PR-scope completion gates, including the honest fts0003 baseline gate, are now satisfied.
16. 2026-04-17: pr_review -> complete
    - reason: Prior Project Reviewer clearance plus the final non-blocking issue-sync result clear the active PR scope for completion.

## Slice Execution Evidence

1. scope_rebaseline_and_assessment
   - status: complete
   - lane: backend
   - evidence: Replaced the stale active ledger, verified the active PR scope from request-plan.md, reviewed only the bounded package files, and gathered required command evidence.
2. contract_gap_and_harness_assessment
   - status: complete
   - lane: backend
   - evidence: The parser and transform surfaces were still subset-shaped around Numeric64/String, the parser rejected numeric widths other than 8 bytes, and benches/parser_decode.rs benchmarked parse() without draining streamed batches.
3. matrix_capable_contracts_numeric_widths_and_streamed_decode_benchmark
   - status: complete
   - lane: backend
   - evidence: Generalized parser columns to a physical numeric/string contract with deferred semantic hints and richer column metadata, removed the parser-core width==8 rejection in favor of preserving non-8-byte numeric cells as deferred raw bytes, threaded the new contract through transform planning and explicit deferred-materialization errors, extended the synthetic fixture/test surface to exercise a 4-byte numeric path honestly, and updated benches/parser_decode.rs to drain next_batch() fully before reporting throughput.
4. fts0003_real_file_probe_and_baseline
   - status: complete
   - lane: backend
   - evidence: Added a real-file parser probe and benchmark baseline on sample-sas-datasets/fts0003.sas7bdat through the existing SupportedSas7bdatParser entrypoint. The probe is honest about the current unsupported boundary: the file still stops at parse time with `Unsupported(WordSize(Bit32))`, so this slice records a real-file baseline without falsely claiming current 32-bit readability. Both the test and the benchmark are written to drain streamed decode fully if future 32-bit support lands.

## Next Slice

- selected_slice: none
- lane: none
- objective: None. This PR scope is complete, and this ledger-only update does not start the next PR scope.
- escalation_watch: Future 32-bit layout support belongs to the planned `portable_core_decode_layouts_endianness_encodings_and_subheaders` PR scope rather than this completed PR scope.

## Completion Gate Status

1. Parser contracts can represent physical numeric width and richer metadata without assuming only the current narrow value kinds.
   - status: pass
   - evidence: src/parser/contracts.rs now exposes physical `ColumnKind::Numeric` and `ColumnKind::String`, deferred semantic hints, optional format/informat/label metadata slots, and parsed numeric cells can be either materialized `Float64` or deferred raw bytes.
2. Transform contracts and sink planning compile against the new schema surface without hard-coding future SAS semantic policy prematurely.
   - status: pass
   - evidence: src/transform/sink.rs plans projections from the generalized parser schema, keeps the current Float64 fallback only for materialized numeric cells, and fails explicitly when a deferred-width numeric cell would otherwise force premature semantic policy.
3. The current supported subset still works end-to-end through the new contract surfaces.
   - status: pass
   - evidence: The scoped parser and transform integration tests still pass on the existing supported subset, including selection, filtering, batching, bounded-memory writes, and parallel batch execution.
4. Non-8-byte numeric widths are decoded or at least explicitly exercised through the supported uncompressed path with tests, removing the hard assumption from the parser core.
   - status: pass
   - evidence: The parser no longer rejects widths below 8 bytes during schema parsing, tests exercise a 4-byte numeric column through the uncompressed path, and the transform path reports deferred materialization explicitly instead of hiding the boundary.
5. The parser benchmark drains streamed batches with next_batch instead of only timing parse setup.
   - status: pass
   - evidence: benches/parser_decode.rs now parses the fixture, drains next_batch() to exhaustion, asserts the decoded row count, and the required benchmark command completed successfully on that streamed decode path.
6. This PR does not claim new 32-bit, big-endian, non-UTF-8, compression, page-type, or SAS semantic typing support.
   - status: pass
   - evidence: The parser and transform READMEs now state that non-8-byte numeric cells are preserved with deferred semantics rather than claimed as fully typed support, and the unsupported layout, encoding, compression, page-type, and semantic boundaries remain explicit.
7. sample-sas-datasets/fts0003.sas7bdat is probed and benchmarked through the current parser entrypoint, and the unsupported 32-bit boundary is recorded honestly.
   - status: pass
   - evidence: tests/parser_decode_contract.rs and benches/parser_decode.rs both exercise `fts0003.sas7bdat` through the current parser entrypoint. The present outcome is parse-time `Unsupported(WordSize(Bit32))`, and the harnesses are structured to drain streamed decode only if future 32-bit support lands.

## Command Evidence

1. cargo test --test parser_contract --test transform_contract --test transform_parser_integration --test parser_decode_contract
   - result: pass
   - details: 17 scoped tests passed after the bounded implementation changes.
2. cargo bench --bench parser_decode -- --noplot --sample-size 10 --measurement-time 0.2 --warm-up-time 0.1
   - result: pass
   - details: The benchmark now drains streamed batches fully. Observed timings were about 882 to 892 microseconds at 16,384 rows, 7.09 to 7.27 milliseconds at 131,072 rows, and 14.12 to 14.30 milliseconds at 262,144 rows, with throughput around 18.0 to 18.6 million rows per second on the synthetic supported-subset fixture. Criterion reported a large regression versus the prior baseline because the harness now measures real streamed decode rather than parse-entry timing.
3. cargo fmt --check
   - result: pass
   - details: rustfmt produced no output after formatting the bounded changes.
4. github issue sync
   - result: open_issue_non_blocking
   - details: The last validated PR-scope issue sync still showed only issue #1, SAS Sample Datasets, as relevant to later corpus-backed validation work and non-blocking for this bounded scope.
5. cargo test --test parser_decode_contract parser_reads_the_real_fts0003_file_through_the_existing_entrypoint -- --exact
   - result: pass
   - details: The real-file probe test passed and confirmed the current honest boundary on sample-sas-datasets/fts0003.sas7bdat: parse stops with `Unsupported(WordSize(Bit32))`, not a streamed-decode success claim.
6. cargo bench --bench parser_decode -- --noplot --sample-size 10 --measurement-time 0.2 --warm-up-time 0.1
   - result: pass
   - details: The current benchmark run reported supported-subset throughput around 9.32 to 9.57 million rows per second at 16,384 rows, about 8.70 to 8.73 million rows per second at 131,072 rows, and about 8.69 to 8.71 million rows per second at 262,144 rows. The distinct `parser_decode_real_file_baseline/fts0003_probe` group completed in about 5.387 to 5.400 microseconds per probe while asserting the current parse-time `Unsupported(WordSize(Bit32))` boundary.
7. final issue sync closeout
   - result: open_issue_non_blocking
   - details: Direct `gh issue list --repo aaweaver-actuary/sas-rs --state open` and equivalent REST calls returned GitHub API 403 from this environment during ledger closeout, so the ledger carries forward the last validated non-blocking issue result rather than inventing a new live sync state.

## Blockers And Waivers

- blockers: none
- waivers: none

## Upward Report

- active_pr_scope_status: complete
- request_completion_signal: not ready
- residual_note: The active PR scope `capability_contracts_numeric_widths_and_honest_harness` is complete, including the honest fts0003 real-file probe and baseline that still records the current unsupported 32-bit boundary. The full request is not complete, and this ledger-only closeout does not start the next PR scope.
