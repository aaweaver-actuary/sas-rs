# Request Plan

- request_id: 2026-04-18-open-issues-and-performance-closure
- user_goal: Resolve the remaining open GitHub issues by either closing already-satisfied items with evidence or delivering bounded PR scopes, then continue iterating on performance until any claimed optimization is backed by statistically significant evidence at a 5% threshold.
- authoritative_spec: user request on 2026-04-18
- request_baseline_note: This is a materially new request. The previously complete request plan covered journal evidence rewrites only; it is no longer authoritative because the user has reopened the request around live GitHub issue resolution and another performance-optimization loop.
- current_request_state: pr_scope_in_progress

## Request-Level Issue Status

- issue_sync_timestamp: 2026-04-18
- issues_closed_this_cycle: #1, #5, #6, #7, #8, #9, #10
- remaining_open_issues: #4, #11, #12, #13, #14, #15
- issue_status_note: The stale sample-dataset and parser-hardening issues were closed after local verification, and issue #7 is now closed with statistically significant parser hot-path evidence. The remaining open set is one parser/test abstraction cleanup issue (#4), one CI/CD issue (#15), and four larger structural or documentation issues (#11, #12, #13, #14).

## Ordered PR Scopes

1. pr01_parser_hotpath_significance_and_issue7
   - status: complete
   - objective: Resolve issue #7 by building a repeatable parser baseline, testing the page-header allocation-removal idea, and accepting only changes that beat the 5% significance threshold without causing statistically significant regressions elsewhere.
   - included_scope: issue #7, parser hot-path benchmarking, significance analysis, targeted parser changes, issue closure, and journal evidence updates required to keep the performance notes truthful.
   - deferred_scope: wide-schema RSS follow-up, CI/CD, parser/fixture abstraction cleanup, structural reorg, module-export cleanup, and docstrings.
   - why_now: This was the smallest remaining issue that directly advanced the user's renewed performance mandate.
2. pr02_remaining_performance_closure_realfile_and_wide_schema
   - status: in_progress
   - objective: Extend the performance loop beyond issue #7 by targeting the still-admitted real-file and wide-schema bottlenecks until the journal can honestly state that no remaining claimed optimization lacks evidence.
   - included_scope: representative real-file benchmark coverage, wide-schema or high-RSS follow-up, additional significance-checked performance changes, any truthful journal updates required by those results, and any issue comments or closures justified by the evidence.
   - deferred_scope: CI/CD, parser/fixture abstraction cleanup, structural reorg, module-export cleanup, and docstrings.
   - why_now: The checked-in real-file notes still admit unfinished optimization work beyond page-header churn.
3. pr03_rust_ci_cd_modernization
   - status: ready
   - objective: Replace the current Node-oriented GitHub Actions workflow with Rust-native quality gates and release automation that satisfy issue #15.
   - included_scope: issue #15, GitHub Actions workflow updates, verified Rust lint/test/format gates, and release packaging automation as scoped by the issue.
   - deferred_scope: parser architecture and docs work.
   - why_now: CI is separable operational work that should not be mixed into parser-performance diffs.
4. pr04_shared_layout_and_constant_dataclasses
   - status: ready
   - objective: Resolve the shared-layout and constant-grouping feedback in issues #4 and #12 by extracting shared abstractions and replacing duplicated constant clusters with data-bearing configuration types.
   - included_scope: issues #4 and #12, shared parser/fixture layout abstractions, supported-subset naming simplification, and centralized SAS layout/configuration data.
   - deferred_scope: the deeper parser domain-model reorg, module-export packaging, and docs.
   - why_now: This is a coherent refactor scope with bounded architectural impact.
5. pr05_parser_domain_model_reorg
   - status: ready
   - objective: Resolve issue #13 by reorganizing parsing logic around concrete SAS file concepts such as header, page, subheader, and row.
   - included_scope: issue #13, parser domain-model extraction, and any required supporting interface changes.
   - deferred_scope: module-export packaging and docs.
   - why_now: This reorg should land after the layout abstraction cleanup rather than before it.
6. pr06_bottom_level_single_public_export_cleanup
   - status: ready
   - objective: Resolve issue #11 by normalizing bottom-level modules so each contains at most one public export.
   - included_scope: issue #11 and the packaging changes required to satisfy the one-public-export rule.
   - deferred_scope: docs.
   - why_now: This should come after the higher-value architectural reshaping so the mechanical packaging changes happen once.
7. pr07_docstrings_and_doctests
   - status: ready
   - objective: Resolve issue #14 by adding detailed module docs, public-item docs, and doctests that run under the normal Rust test suite.
   - included_scope: issue #14 and the documentation/doctest work for the settled public API surface.
   - deferred_scope: none.
   - why_now: Documentation should follow the remaining API and module-structure changes rather than precede them.

## Active PR Scope

- active_pr_scope: pr02_remaining_performance_closure_realfile_and_wide_schema

## Completed PR Scopes

- stale_issue_reconciliation_closures (#1, #5, #6, #8, #9, #10)
- pr01_parser_hotpath_significance_and_issue7

## Deferred PR Scopes

- none

## Blocked PR Scopes

- none

## Request Completion Gates

- The request plan is rebaselined away from the previously complete journal-only request.
- Every GitHub issue in the reopened scope is either closed with evidence or represented by a bounded PR scope.
- The first parser hot-path optimization pass is complete and issue #7 is closed with statistically significant evidence at the 5% threshold.
- Any broader performance claims added to journal.md remain evidence-backed and use representative benchmark coverage.
- Remaining open issues #4, #11, #12, #13, #14, and #15 are either completed or explicitly deferred by the user.
- Final request completion is not declared while unresolved in-scope PR scopes remain.

## Final Response Readiness

- final_response_readiness: not_ready
- reason: The request was reopened, the broader real-file performance scope is still active, and six GitHub issues remain open.
