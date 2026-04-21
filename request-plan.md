# Request Plan

- request_id: 2026-04-20-quality-and-parser-domain-rebaseline
- user_goal: Rebaseline the crate around a green quality baseline, then reshape the parser into clearer SAS-shaped domain units with composed structs, expressive higher-level parsing outputs, and a stable path into Parquet conversion, and only then document the settled public surface.
- authoritative_spec: user request on 2026-04-20
- request_baseline_note: This is a materially new request. The previously active request plan centered on performance closure and open-issue reconciliation; it is no longer authoritative because the user has reframed the work around lint/test stabilization, parser-domain reorganization, struct composition, SAS-familiar naming, and public API documentation.
- current_request_state: ready_for_final_request_review

## Request-Level Issue Status

- issue_sync_timestamp: 2026-04-20
- issue_status_source: repository-local issue evidence only; live GitHub issue querying was not available in this environment
- repository_local_open_issues: #14, #15
- issue_status_note: Issue #14 is satisfied by the completed public-docstrings sweep. Issue #15 remains satisfied for the current code path by the restored and re-run quality gates.

## Ordered PR Scopes

1. pr01_quality_baseline_and_refactor_stabilization
   - status: complete
   - objective: Run the real formatting, lint, and test gates against the current in-flight refactor and fix any failures required to restore a trustworthy green baseline.
   - included_scope: cargo fmt/clippy/test validation, fixes required by those checks, limited refactor repairs needed to make the current parser changes coherent, and a concrete inventory of parser seams that still need architectural work.
   - deferred_scope: broad public naming changes, docstring sweep, CI/release automation redesign, and larger parser-domain rewrites that are not required to get the baseline green.
   - why_now: Deeper design work should not proceed while the active worktree has unverified parser refactor changes.
2. pr02_public_parser_contracts_and_sas_vocabulary
   - status: complete
   - objective: Define the stable parser-facing contracts and familiar SAS vocabulary for the public surface before larger internal rewrites continue.
   - included_scope: interface-first review of public exports, SAS-familiar names for header/page/subheader/row/column/value concepts, and the public/internal boundary decisions needed to support the remaining refactor scopes.
   - deferred_scope: deep parser algorithm rewrites and the final docstring sweep.
   - why_now: Interface boundaries should be settled before internal consolidation so later scopes do not churn public naming twice.
3. pr03_parser_unit_consolidation_and_composed_structs
   - status: complete
   - objective: Consolidate duplicated logical units and compose parser state out of smaller SAS-specific structs where that improves clarity and maintainability.
   - included_scope: layout/header/page/subheader/metadata accumulation structure cleanup, replacement of flat primitive-heavy state with composed structs where justified, and reductions in scattered parser constants and offset handling.
   - deferred_scope: public documentation sweep and unrelated feature expansion.
   - why_now: This is the main architectural cleanup pass once the interfaces are settled.
4. pr04_expressive_row_value_model_and_parquet_handoff
   - status: complete
   - objective: Align row and value parsing around expressive higher-level structs and enums that make the parser-to-transform-to-parquet path easier to follow.
   - included_scope: row/value/date-datetime/character-numeric modeling, parser outputs that expose richer intermediate representations, explicit row-batch and schema handoff contracts, and transform-seam adjustments needed for a cleaner parquet handoff.
   - deferred_scope: release automation and unrelated packaging churn.
   - why_now: This is where the human-readable intermediate representation goal becomes concrete.
5. pr05_public_docstrings_and_doctest_sweep
   - status: complete
   - objective: Add docstrings to all intentional public exports and add or repair doctests where they provide stable value.
   - included_scope: crate/module/public-item documentation across the settled public API surface, with only minimal code changes required to keep examples truthful.
   - deferred_scope: none.
   - why_now: Documentation should land against the stabilized API and module structure, not against a moving target.

## Active PR Scope

- active_pr_scope: none

## Completed PR Scopes

- pr01_quality_baseline_and_refactor_stabilization
- pr02_public_parser_contracts_and_sas_vocabulary
- pr03_parser_unit_consolidation_and_composed_structs
- pr04_expressive_row_value_model_and_parquet_handoff
- pr05_public_docstrings_and_doctest_sweep

## Deferred PR Scopes

- none

## Blocked PR Scopes

- none

## Request Completion Gates

- The request plan is rebaselined away from the old performance-and-issue-closure plan.
- A fresh PR ledger is created for each active PR scope under this new request.
- pr01 leaves the repository green on formatting, clippy, tests, and any parser-adjacent quality checks needed by the touched surface.
- pr02 establishes stable public parser/domain contracts and SAS-familiar naming before large internal rewrites proceed.
- pr03 and pr04 leave parsing centered on expressive higher-level structs instead of scattered primitive-heavy state.
- pr04 leaves a clear parser-to-transform handoff that supports parquet conversion without another major domain-model rewrite.
- pr05 leaves all intentional public exports documented.
- Final request completion is not declared while unresolved in-scope PR scopes remain.

## Final Response Readiness

- final_response_readiness: ready
- reason: All planned PR scopes are complete and the final documentation-quality gate bundle is green, so the request is ready for final request review.
