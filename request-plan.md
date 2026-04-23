# Request Plan

- request_id: 2026-04-22-performance-campaign-rebaseline
- user_goal: Run a discovery-first performance campaign that identifies the highest-value sas7bdat parser and transform hotspots, experiments with bounded competing implementations, statistically compares them, and lands only validated wins.
- authoritative_spec: user request on 2026-04-22 plus spec.md and the active performance sweep prompt
- request_baseline_note: This is a materially new request. The prior 2026-04-20 request plan for quality, parser-domain refactor, and documentation is no longer authoritative.
- current_request_state: blocked

## Request-Level Issue Status

- issue_sync_timestamp: 2026-04-22
- issue_status_source: repository-local issue evidence only; live GitHub issue querying was not available in this environment
- repository_local_open_issues: #1, #6, #7, #8, #9, #10
- issue_status_note: Issue #7 is the primary performance follow-up; Issue #1 requires representative benchmark coverage; #6/#8/#9/#10 remain active regression guardrails.

## Ordered PR Scopes

1. pr01_perf_discovery_and_measurement_rebaseline
   - status: complete
   - objective: Rebaseline the active request around a discovery-first performance campaign and produce the first ranked hotspot list before any behavior-changing optimization lands.
   - included_scope: fresh request and PR-scope artifacts; representative measurement contract including fts0003 and current wide or parallel-sensitive fixtures; profiler-backed evidence where available; a first-pass ranked hotspot list across parsing, lazy materialization, transform, parquet write, chunking, and parallelism; selection of the first hotspot target for candidate experimentation.
   - deferred_scope: winning code changes, broad parser rewrites, sink or parallelism rewrites, unrelated compatibility expansion, and any speculative merge that is not measurement-backed.
   - why_now: The user explicitly requires discovery and measurement before implementation, and the old completed request artifacts cannot remain authoritative.
2. pr02_parser_io_and_decode_hotspot_candidates
   - status: blocked
   - objective: Take the top parser-stage hotspot or small hotspot set from pr01 and run bounded competing implementations against the parser path.
   - included_scope: parser-stage I/O and decode candidates, repeated statistical comparison on parser benches plus representative real-file coverage including fts0003, correctness and regression validation, and immediate commit/push of a validated winner if one exists.
   - deferred_scope: lazy materialization, row-batch chunking, sink-write, and worker-scheduling changes unless strictly required for parser-only measurement.
   - why_now: Parser decode should be isolated from downstream costs before broader runtime changes land.
3. pr03_lazy_materialization_and_chunking_hotspot_candidates
   - status: planned
   - objective: Optimize the next-ranked hotspot set in row materialization, staging buffers, and chunking behavior after parser-stage wins are settled.
   - included_scope: bounded candidates for materialization and chunking surfaces, transform_write and representative real-file workloads, RSS plus throughput measurements, and immediate commit/push of a validated winner if one exists.
   - deferred_scope: sink-write internals, worker-threshold tuning, and broad interface redesign unless strictly needed for a zero-cost boundary.
   - why_now: Allocation churn and batching costs should be measured separately from parser-page access and sink-write costs.
4. pr04_transform_sink_and_parallelism_hotspot_candidates
   - status: planned
   - objective: Optimize transform execution, parquet write cost, and worker-thread or threshold behavior once parser and staging hotspots have been addressed.
   - included_scope: bounded competing candidates for transform, parquet sink, and parallelism settings; repeated statistical comparison in serial and parallel modes; regression validation; and immediate commit/push of a validated winner if one exists.
   - deferred_scope: reopening parser contracts, broad CLI redesign, and unrelated compatibility work.
   - why_now: Sink and scheduling changes are easier to review once upstream parser and materialization costs are better understood.
5. pr05_campaign_rerun_and_leaderboard_refresh
   - status: planned
   - objective: Re-run the accepted-wins stack across the representative suite, refresh the campaign evidence, and leave a clear next experiment backlog.
   - included_scope: repeated measurements across the representative suite, layered summary of accepted gains, refreshed journal and benchmark notes, and a ranked follow-up hotspot backlog.
   - deferred_scope: new speculative candidates that were not part of the accepted-wins stack.
   - why_now: The campaign needs an evidence-consolidation pass to prove the accepted wins compose cleanly and do not overfit to one benchmark.

## Active PR Scope

- active_pr_scope: pr02_parser_io_and_decode_hotspot_candidates

## Completed PR Scopes

- pr01_perf_discovery_and_measurement_rebaseline

## Deferred PR Scopes

- none

## Blocked PR Scopes

- pr02_parser_io_and_decode_hotspot_candidates

## Request Completion Gates

- The active request plan is rebaselined away from the completed parser-domain refactor and documentation request.
- A fresh PR ledger is created for each active PR scope in this performance campaign.
- pr01 produces a repository-specific hotspot ranking and representative measurement contract before any optimization candidate is merged.
- pr02, pr03, and pr04 each compare baseline versus bounded candidates with repeated measurements and land only validated winners.
- Representative benchmark coverage includes fts0003 and other checked-in datasets that exercise wide, narrow, compressed, and parallel-sensitive paths.
- Campaign evidence records whether the biggest wins are in parsing, lazy materialization, transform, parquet writing, chunking, or parallelism.
- Any winning optimization is committed and pushed promptly once validated for its scope.
- Final request completion is not declared while unresolved in-scope PR scopes remain.

## Final Response Readiness

- final_response_readiness: not_ready
- reason: pr02 parser-stage candidate work is blocked because the current checkout exposes only one recoverable bounded parser candidate and this mode cannot author the missing competing implementation.
