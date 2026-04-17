# Agent Rewrite Architecture

## Main Stages

### Phase 1: remove pure overhead
- retire Coding Supervisor
- merge Agilist + Minimal Work Finder
- stop using Git Committer as a separate role
- make Human Readability Refactorer optional only

### Phase 2: fix completion control
- add completion-ledger.md
- make supervisors report slice status only
- make PM and Project Reviewer read from ledger, not prose alone

### Phase 3: reduce rework in delivery
- merge TDD Test Writer + Minimal Developer
- merge Specification Planner + issue integration into Requirements Planner
- reduce automatic memory lookup hops

### Phase 4: harden boundaries
- every handoff includes:
- exact files
- exact criteria
- exact tests/commands
- explicit non-goals
- receiving role is told not to inspect beyond package unless blocked

## Agent Roles and Workflow

1. Project Manager
- user-facing
- owns completion ledger
- runs issue sync at cycle boundaries
- routes to lane supervisors
- decides whether project is done

2. Requirements Planner
- merges current user goal, active issues, spec updates, acceptance criteria
- proposes structured completion-ledger updates
- updates slice definition and spec
- consumes issue delta report
- updates spec and criteria
- **NOTE** this combines the current specification planner and issue plan integrator roles
- Requirements Planner owns spec.md + decision-log.md

3. Work Planner
- decomposes if needed
- selects next ready slice
- returns one bounded work package
- **NOTE** this combines the current agilist and minimal work finder roles, but with a sharper focus on slicing and package handoff instead of open-ended backlog management

4. Lane Supervisor
- frontend/backend/full-stack specialization
- may coordinate exactly one active slice at a time
- hands a bounded package to implementer
- returns slice result and uncovered criteria
- Reviewer is mandatory if any of the following apply:
    - >3 files modified
    - interface/schema changes
    - dependency injection / architecture changes
    - issue-linked regression risk
    - production-facing logic
    - deletion / refactor of existing working code
- Lane Supervisors are **prohibited** from: 
    - redefining project-level goals
    - editing the spec
    - re-slicing backlog items unless blocked
- If the slice appears incorrectly scoped, escalate to Work Planner
rather than redefining the slice locally.

5. Implementer
- red/green/refactor for one slice
- executes tests
- reports pass/fail/blocked against explicit criteria
- **NOTE** this combines the current test writer and minimal developer roles, but with a sharper focus on disciplined TDD execution instead of open-ended coding

6. Reviewer
- validates correctness, regression risk, complexity, and criteria coverage
- returns structured verdict with evidence link
- verdict options:
    - approved
    - revision_required
    - blocked

7. Issue Tracker
- detects external issue deltas only
- produces structured issue delta report

8. Project Reviewer
- final “done or not done” authority under PM

9. Memory Researcher
- only memory writer
- invoked only when durable knowledge needs recording or stale memory must be refreshed
- Memory writes are allowed only for:
    - cross-slice architectural decisions
    - durable interface contracts
    - resolved blockers likely to recur
    - environment / build constraints
    - user preferences that affect future slices

### Agents to Remove
- Agilist/Minimal Work Finder (merged into Work Planner)
- Specification Planner/Issue Plan Integrator (merged into Requirements Planner)
- TDD Test Writer/Minimal Developer (merged into Implementer)
- Git Committer
- Coding Supervisor (already deprecated in favor of lane supervisors)

### Work Package Schema

Each slice package must contain:

- objective
- exact files allowed to modify
- exact files allowed to read
- acceptance criteria for the slice
- required tests / commands
- known dependencies
- explicit non-goals
- rollback risk
- escalation conditions

### Escalation Rules

Implementer -> Lane Supervisor
Lane Supervisor -> Work Planner
Work Planner -> Requirements Planner
Requirements Planner -> Project Manager


## Artifact Ownership and Handoff

The critical design change: artifact ownership

- Project Manager owns authoritative completion-ledger.md state
    - other roles may propose updates, but only PM may integrate them into the ledger
- Requirements Planner owns spec.md
- Work Planner owns backlog / slice package
- Implementer owns code + tests for one slice
- Reviewer owns review verdict against explicit criteria
- Issue Tracker owns issue-to-plan coverage report
- Memory Researcher owns .memories/*

Once each role owns one artifact, rework drops because the next role consumes a bounded package instead of regenerating one.

### restriction on creating duplicate artifacts:
	•	agents may not create replacement artifacts for another role’s owned artifact
	•	required changes must be proposed through the owning role

### `completion-ledger.md`

The completion ledger is the single source of truth for what’s done, what’s not done, and what’s blocked. It is the only artifact that tracks progress against user goals and acceptance criteria. 

The Project Manager owns the authoritative structure and final state
of completion-ledger.md.

Other agents may propose updates in structured form, but only the PM
may finalize status transitions.

For each request:
- user goal
- required criteria
- implied criteria from issues/spec
- pass/fail/unknown for each criterion
- evidence link
    - Allowed evidence types:
        - passing test output
        - file diff / commit diff reference
        - issue closure reference
        - reviewer approval note
        - user confirmation
        - decision-log reference
- remaining slices
- blockers/waivers

### Project Manager and Project Reviewer rules:
- no final response unless all criteria are pass or explicitly waived
- no supervisor may report “done”; they may only report “slice complete”
- only PM may upgrade project state from in progress to done, after Project Reviewer and Issue Tracker both clear

### `decision-log.md`

Architecture decisions and reversals need their own durable log.

Purpose:
- record major implementation decisions
- alternatives considered
- rationale
- risks accepted
- reversal conditions

This prevents repeated rediscovery of prior reasoning.

## Request Lifecycle State Machine

Each request must exist in exactly one state.

Allowed states:
- intake
- requirements_defined
- slice_ready
- slice_in_progress
- slice_review
- blocked
- awaiting_issue_sync
- project_review
- complete

Allowed transitions:
- intake -> requirements_defined
- requirements_defined -> slice_ready
- slice_ready -> slice_in_progress
- slice_in_progress -> slice_review
- slice_review -> slice_ready
- slice_review -> project_review
- blocked -> slice_ready
- blocked -> requirements_defined
- blocked -> intake
- project_review -> complete
- project_review -> slice_ready
- slice_review -> awaiting_issue_sync
- awaiting_issue_sync -> project_review
- awaiting_issue_sync -> slice_ready

Only the Project Manager may transition a request to complete.
Lane Supervisors may only transition slices, never the project.

## Retry Budget Rules

Each slice may be retried at most 2 times without upstream escalation.

### After:

- 1 failed retry -> Lane Supervisor review required
- 2 failed retries -> mandatory escalation to Work Planner
- No third local retry allowed.

## Scope Delta Rule

If newly discovered work materially expands the slice beyond
the defined work package, the slice must be escalated as
scope_delta rather than continued locally.

Escalate as scope_delta if any of the following become true:
	•	files outside the allowed modify/read set are required
	•	acceptance criteria must change
	•	a new interface/schema is needed but not specified
	•	a second subsystem becomes necessary
	•	estimated slice size doubles relative to original package


## FAQ

### Who may mark "blocked"?

- Implementer may report blocked
- Reviewer may report blocked
- Lane Supervisor may escalate blocked
- only Project Manager may update the canonical request state in the ledger

### What is the simplest happy path workflow?

1.	PM defines or refreshes request
2.	Issue Tracker checks deltas
3.	Requirements Planner updates spec/criteria
4.	Work Planner emits one slice package
5.	Lane Supervisor coordinates one slice
6.	Implementer executes
7.	Reviewer validates when required
8.	PM updates ledger
9.	PM routes next slice or advances to project review
10.	Project Reviewer checks completion
11.	PM marks complete if all criteria pass or are waived