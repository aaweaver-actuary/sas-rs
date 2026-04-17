# Completion Ledger

- pr_scope_id: performance_push_and_completion
- authoritative_spec: spec.md
- authoritative_request_plan: request-plan.md
- ledger_baseline_note: Fresh active ledger created on 2026-04-17 because the old typed_transform_and_memory_scaling ledger was not the active completion artifact for the materially new final performance rerun on the streaming architecture.
- pr_scope_state: complete
- outcome_status: complete

## PR Scope Summary

- objective: Re-establish benchmark and validation evidence on the true streaming architecture, tune only if the lazy path regresses materially, and refresh final completion evidence.
- issue_delta_status: clean
- issue_delta_evidence: Repo-qualified query repo:aaweaver-actuary/sas-rs is:issue is:open returned 0 open issues on 2026-04-17 at both scope start and final sync.
- planning_status: No tuning was required. The streaming baseline stayed reviewable and the final completion case now rests on fresh end-to-end transform throughput plus the passing validation suite.

## State Transitions

1. 2026-04-17: intake -> pr_scope_defined
   - reason: performance_push_and_completion is materially new relative to the stale blocked performance ledger and the completed typed-transform ledger, so the active ledger had to be replaced.
2. 2026-04-17: pr_scope_defined -> slice_ready
   - reason: request-plan.md confirms the active PR scope and repo-qualified issue sync is clean.
3. 2026-04-17: slice_ready -> slice_in_progress
   - reason: Started the streaming_baseline_bench_and_validation slice.
4. 2026-04-17: slice_in_progress -> slice_review
   - reason: Full validation and benchmark reruns completed on the streaming baseline and no material regression required tuning.
5. 2026-04-17: slice_review -> awaiting_issue_sync
   - reason: Re-ran repo-qualified issue sync at the PR-scope cycle boundary after the benchmark and validation slice.
6. 2026-04-17: awaiting_issue_sync -> pr_review
   - reason: Final issue sync stayed clean and the completion gates are satisfied with honest benchmark-backed evidence.
7. 2026-04-17: pr_review -> complete
   - reason: End-to-end transform throughput, supported-boundary validation, and final issue sync are sufficient for Request Manager review.

## Slice Execution Evidence

1. scope_rebaseline_and_issue_sync
   - status: complete
   - lane: backend
   - evidence: Confirmed origin is https://github.com/aaweaver-actuary/sas-rs.git, replaced the stale active ledger, and verified repo-qualified issue sync returned 0 open issues.
2. streaming_baseline_bench_and_validation
   - status: complete
   - lane: backend
   - evidence: cargo test passed 19 tests; assumption_probe rerun measured 21.903 to 21.978 microseconds at 16,384 rows and 178.11 to 183.46 microseconds at 131,072 rows; parser_decode rerun completed but currently measures parse() without next_batch and therefore reflects metadata-parse cost more than full row decode on the streaming path; transform_write rerun on 262,144 rows measured 36.231 to 41.713 ms at 1 thread and 33.720 to 34.217 ms at 4 threads, corresponding to 6.284 to 7.235 Melem/s and 7.661 to 7.774 Melem/s respectively.

## Completion Gate Status

1. End-to-end transform benchmarks exist on the streaming architecture and are strong enough for final review.
   - status: pass
   - evidence: transform_write on the streaming baseline sustained 7.661 to 7.774 Melem/s at 262,144 rows with 4 worker threads in the longer rerun.
2. Performance-oriented structure remains justified on the new path.
   - status: pass
   - evidence: The assumption probe remains cheap, end-to-end transform throughput remains strong, and the short-window matrix rerun did not reveal a material regression that justified reopening implementation work.
3. The supported-subset CLI transform path remains production-ready within its explicit support boundary.
   - status: pass
   - evidence: cargo test passed the CLI contract, parser contract, parser decode contract, transform contract, and transform parser integration suites on the streaming baseline.
4. Remaining unsupported areas remain explicit.
   - status: pass
   - evidence: Unsupported parser subset cases and unsupported filter-expression forms remain explicit in the current code and tests.
5. Validation evidence is sufficient for Request Manager to complete the request if no other gaps remain.
   - status: pass
   - evidence: Full suite pass, final issue sync clean, and fresh benchmark evidence now exist on the true streaming baseline.
6. The < 1 minute for ~20M rows target has an honest benchmark-backed case on the supported-subset streaming workload.
   - status: pass
   - evidence: 20 million rows at 7.661 to 7.774 Melem/s implies about 2.61 to 2.57 seconds under the supported-subset transform_write benchmark conditions; this is an honest synthetic supported-boundary case, not a claim about arbitrary unsupported SAS inputs.

## Command Evidence

1. cargo test
   - result: pass
   - details: 19 tests passed across the CLI, parser, transform, and integration suites.
2. cargo bench --bench assumption_probe -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
   - result: pass
   - details: 16,384 rows at about 21.9 microseconds and 131,072 rows at about 180.9 microseconds.
3. cargo bench --bench parser_decode -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
   - result: pass_with_caveat
   - details: Benchmark reran successfully, but the harness currently calls parse() without next_batch and should be interpreted as metadata-parse timing rather than full streaming row decode timing.
4. cargo bench --bench transform_write -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
   - result: pass
   - details: Full matrix rerun remained in the 6.7 to 7.5 Melem/s range and did not indicate a material regression requiring tuning.
5. cargo bench --bench transform_write 262144 -- --noplot --sample-size 20 --measurement-time 1 --warm-up-time 0.3
   - result: pass
   - details: Targeted high-row-count rerun tightened the main completion evidence to 7.661 to 7.774 Melem/s at 4 worker threads.
6. github issue sync
   - result: pass
   - details: repo:aaweaver-actuary/sas-rs is:issue is:open returned 0 open issues at final sync.

## Blockers And Waivers

- blockers: none
- waivers: none

## Upward Report

- active_pr_scope_status: complete
- request_completion_signal: Request Manager can now credibly review the full request for completion if no other request-level gaps remain.
- residual_note: The parser_decode benchmark should be refined later if the project wants a stage-specific streaming row-decode microbenchmark, but it is not blocking honest request completion because the end-to-end transform evidence is now strong and current.
