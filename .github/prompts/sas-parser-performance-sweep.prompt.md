---
description: "Run a profiler- and benchmark-driven search for SAS parser performance opportunities and produce a ranked optimization backlog"
name: "Performance Sweep"
argument-hint: "Optional focus area, file, dataset, or performance hypothesis"
agent: "Request Manager"
model: "GPT-5.4"
---

Start a performance discovery cycle for this repository.

Objective:
- Move the project toward the fastest available `sas7bdat` parser.
- Prioritize improvements that matter for multi-GB files and efficient recode-to-parquet pipelines.
- Treat the prompt argument as the initial focus area. If no argument is supplied, inspect the full parse, decode, materialization, transform, and write path.
- Use a staged workflow: discovery first, candidate implementations second, measurement and selection third.

Project context:
- Review [spec](../../spec.md), [request-plan](../../request-plan.md), [completion-ledger](../../completion-ledger.md), and relevant benches under [benches](../../benches).
- Start from the most concrete hotspot available: a benchmark, profile result, failing throughput target, or a specific parser/write path.
- Use the repository's orchestration model. Begin with Issue Tracker, then route through planning before implementation.

Required workflow:
1. Run Issue Tracker and reconcile any requirement deltas before starting optimization work.
2. Define a bounded PR scope for performance discovery and experimentation.
3. First pass only: locate and list the most plausible optimization targets before changing behavior. Produce a concrete hotspot list first, especially:
   - avoidable `.clone()`, `.copy()`, `to_vec`, temporary buffers, or redundant allocations
   - tiny helpers or abstraction layers that should be inlined or fused
   - extra memory reads or writes along hot paths
   - non-zero-cost abstractions that can be refactored into clearer zero-cost boundaries
   - lazy materialization overhead, buffering strategy, or cache-unfriendly access patterns
   - chunking, batching, SIMD-friendly loops, or parallelization opportunities
4. Use standard measurement tools instead of intuition alone. Prefer existing benches first, then add targeted microbenchmarks or probes as needed.
5. Use profiler output to separate CPU-bound decode work, allocation churn, I/O waits, and parquet write costs.
6. For the top hotspot or small hotspot set, create multiple bounded implementation candidates designed to fit the optimization target. Favor small competing approaches over one speculative rewrite.
7. Benchmark the baseline and each candidate with repeated runs and statistical comparison. Use tooling that can show whether any observed difference is likely real rather than noise.
8. Keep interface boundaries explicit while refactoring toward zero-cost abstractions.
9. If one candidate shows a statistically significant and practically meaningful improvement with acceptable complexity and regression risk, select it, validate it, and immediately commit and push that focused enhancement before moving on to the next optimization slice.
10. If no candidate wins convincingly, record the result, keep the baseline, and move the work to the experiment backlog rather than merging speculative changes.

Preferred evidence sources:
- existing `cargo bench` targets
- targeted Criterion benchmarks or equivalent narrow benches
- Linux profiling tools such as `perf`
- Rust-oriented profilers such as `cargo flamegraph` or `samply` when available
- allocation-oriented evidence when allocator churn appears material
- repeated measurements with variance-aware output or confidence reporting

Required deliverables:
- A first-pass ranked list of optimization opportunities with expected impact, confidence, and risk.
- Evidence for each priority item: benchmark numbers, profile hotspots, allocation signals, or concrete code-path analysis.
- Candidate implementation summaries for the hotspot(s) taken to experiment, including why each candidate might win.
- A statistical comparison of baseline versus candidates, including which differences appear significant and which do not.
- A short experiment backlog with the next slices to run.
- Clear notes on whether the biggest wins are in parsing, lazy materialization, transformation, parquet writing, chunking, or parallel processing.
- If code changes are made, include the validation commands, before/after measurements, and whether the winning candidate was committed and pushed.

Execution constraints:
- Keep scope tight and evidence-backed.
- Prefer minimal, testable slices over broad speculative rewrites.
- Follow TDD when adding benchmarks, tests, or code changes.
- Do not stop at generic advice; produce repository-specific findings and next actions.
- Do not merge or push a candidate unless the measurements support it.
- Once a candidate is selected and validated, push that enhancement promptly as a focused change before starting the next one.

Response format:
1. `First Pass Hotspots`: concrete hotspots and why they matter.
2. `Measurements`: benchmark and profiler evidence.
3. `Candidates`: bounded implementation options for the top hotspot(s).
4. `Results`: statistical comparison, chosen winner if any, and why.
5. `Follow-Through`: commit/push status, next experiments, files touched, commands run, and remaining risks.