---
name: plan-execution
description: Execute existing implementation plans efficiently and safely. Validate the plan against current repository state, identify dependencies and parallelizable work, execute coherent batches, verify each meaningful stage, adapt when assumptions prove false, and stop rather than blindly following an invalid plan. Use when a concrete implementation plan already exists.
---

# Plan Execution

Execute implementation plans as controlled, evidence-driven work.

A plan is guidance based on previous knowledge.

It is not authority over current repository reality.

The objective is:

validate plan
->
establish current state
->
resolve dependencies
->
execute coherent work
->
verify
->
adapt when necessary
->
complete

Do not blindly execute stale or incorrect plans.

## 1. Core Rule

Before executing a plan, verify that its important assumptions still match reality.

Check:

- repository state
- relevant files
- architecture
- dependencies
- interfaces
- existing implementation
- user changes
- plan assumptions

If reality contradicts the plan, reality wins.

## 2. Do Not Re-Plan Without Need

Do not recreate an already sufficient plan.

Review it for correctness and begin execution.

Only return to substantial planning when:

- assumptions are invalid
- architecture changed
- requirements changed
- important dependencies were omitted
- execution reveals a major blocker

Avoid planning loops.

## 3. Establish the Objective

Identify the final observable outcome.

Do not reduce execution to checking boxes.

Every plan step must contribute to the requested result.

If the plan's steps no longer align with the objective, stop and reassess.

## 4. Inspect Current Repository State

Before substantial execution, inspect relevant state.

When appropriate:

- git status
- current branch
- changed files
- relevant existing implementation
- required configuration

Identify pre-existing user modifications.

Do not overwrite unrelated user work.

## 5. Validate Plan Assumptions

Check important assumptions before relying on them.

Examples:

- file exists
- interface still has expected shape
- dependency version matches plan
- schema has expected structure
- endpoint still exists
- previous phase was actually completed

Do not validate every trivial detail.

Focus on assumptions that could materially change execution.

## 6. Classify Plan Steps

Classify work as:

### Independent

Can execute without another unfinished step.

### Dependent

Requires output from another step.

### Verification

Proves completed work.

### Gate

Must succeed before later work proceeds.

### Optional

Useful but not required for objective.

This classification helps determine execution order.

## 7. Build the Dependency Order

Execute dependencies before consumers.

Example:

schema
->
model
->
repository
->
service
->
API
->
client

Do not parallelize work whose interfaces are not yet established.

## 8. Identify Parallel Work

Parallelize only genuinely independent substantial tasks.

Example:

After API contract is established:

Agent A:
backend implementation

Agent B:
frontend client implementation

Agent C:
independent test fixture work

Use `agent-orchestration`.

Do not parallelize merely to maximize agent activity.

## 9. Batch Coherent Work

Execute related work in coherent batches.

Example:

Batch 1:
data model + repository

Gate:
targeted repository tests

Batch 2:
service + API

Gate:
API tests

Batch 3:
client integration

Gate:
integration verification

Avoid:

modify 20 unrelated files
->
test everything at end

Early verification reduces debugging scope.

## 10. Keep Batches Small Enough to Diagnose

A batch should be large enough to accomplish meaningful work but small enough that failures can be localized.

If verification fails after a massive batch, diagnosis becomes expensive.

Prefer incremental evidence.

## 11. Execute Minimal Changes

Within each plan step, make the smallest correct changes necessary.

Do not add:

- unrelated refactors
- speculative abstractions
- dependency upgrades
- formatting churn
- unrelated cleanup

unless the plan or implementation genuinely requires them.

## 12. Follow Existing Patterns

Use repository conventions identified during planning.

Prefer existing:

- architecture
- naming
- error handling
- validation
- persistence
- testing
- configuration

Do not redesign the system while executing unless the plan proves structurally impossible.

## 13. Use Skills by Task

Load specialized skills only when relevant.

Examples:

Bug discovered:
-> `systematic-debugging`

Unknown code path:
-> `codebase-investigation`

Security-sensitive work:
-> `security-review`

Verification:
-> `testing-verification`

Do not load all skills for every plan.

## 14. Delegate Deliberately

Before delegating a plan step determine:

- whether delegation provides value
- required context
- minimum capable model
- allowed modification scope
- expected result

Use `agent-orchestration`.

Do not delegate trivial plan steps.

## 15. Give Agents Bounded Ownership

When agents modify code concurrently, avoid overlapping ownership.

Preferred:

Agent A:
`src/api/`

Agent B:
`src/frontend/`

Avoid:

Agent A and Agent B both modifying the same core files simultaneously.

When overlap is unavoidable, execute sequentially or coordinate explicitly.

## 16. Validate Agent Output

Subagent completion is not proof.

After delegated work:

- inspect changed files
- inspect diff
- verify requirements
- run appropriate checks

Do not continue dependent work based solely on "done."

## 17. Verification Gates

Run verification at meaningful boundaries.

Examples:

migration
->
schema verification

library implementation
->
unit tests

API implementation
->
API tests

integration
->
end-to-end verification

Use `testing-verification`.

## 18. Do Not Cross Failed Gates

If a required verification gate fails:

stop progression
->
diagnose
->
fix
->
rerun gate

Do not build additional dependent work on an unverified foundation.

## 19. Diagnose Failures

When execution fails, do not immediately rewrite the plan.

Determine whether the failure comes from:

- implementation defect
- incorrect plan assumption
- environment
- dependency
- pre-existing failure
- incorrect interface
- missing requirement

Use `systematic-debugging` when necessary.

## 20. Adapt the Plan

Plans may be updated when evidence requires it.

Minor deviation:

adjust current step and continue.

Major deviation:

stop
->
update relevant plan portion
->
validate dependencies
->
continue

Do not preserve a plan merely for consistency with previous text.

## 21. Scope Changes

If execution reveals additional work, determine whether it is:

### Required

Necessary for requested objective.

Include it.

### Blocking

Not originally planned but prevents completion.

Address minimally.

### Unrelated

Not required.

Record if significant and continue without fixing.

Do not allow execution to expand indefinitely.

## 22. New Requirements

If new user requirements arrive during execution:

- incorporate them
- determine affected completed work
- update remaining plan
- rerun affected verification

Do not restart unaffected work.

## 23. Maintain Progress State

For substantial plans track:

- completed
- in progress
- blocked
- remaining
- verification status

Keep progress concise.

Do not create excessive project-management overhead.

## 24. Completion Is Not Step Count

A plan is not complete merely because every listed step was executed.

Final outcome must match the objective.

Check the actual requested behavior.

## 25. Verify Final Integration

After all batches, verify integration across boundaries when relevant.

Individual components passing independently does not prove combined behavior.

Examples:

frontend + API

service + database

producer + consumer

migration + application

## 26. Inspect Final Diff

Before completion inspect the full resulting diff.

Look for:

- unrelated changes
- temporary debugging code
- duplicated implementation
- accidental generated files
- secrets
- personal information
- unintended dependency changes
- missed plan requirements

## 27. OPSEC

Preserve global OPSEC throughout execution.

Never unnecessarily introduce or expose:

- real names
- personal email addresses
- machine usernames
- personal paths
- credentials
- secrets
- tokens
- private infrastructure

Use the approved handle/tag where attribution is required.

If required and unknown, ask the user.

## 28. Publishing Gate

If execution ends in publishing or release, perform an OPSEC and security check before external exposure.

Inspect relevant:

- source
- metadata
- documentation
- manifests
- generated artifacts
- logs
- author fields
- examples

Do not publish sensitive identity or credential information.

## 29. Destructive Operations

Plans do not automatically authorize destructive actions.

Exercise additional caution with:

- file deletion
- database destruction
- history rewriting
- force push
- credential rotation
- production changes
- irreversible migrations

If destructive action was not clearly authorized and is not safely implied by the task, obtain appropriate confirmation.

## 30. Preserve User Work

Never use destructive cleanup to simplify execution.

Do not automatically:

- reset repository
- discard unrelated modifications
- overwrite user files
- clean untracked files
- revert unrelated commits

Work around existing state.

## 31. Resource Efficiency

Do not over-process execution.

Avoid:

- repeated full repository scans
- repeated full test suites after tiny steps
- excessive agents
- repeated plan rewriting
- duplicate investigations

Reuse verified knowledge while it remains valid.

## 32. Context Management

Keep active execution context focused.

For long plans maintain concise state:

- objective
- completed phases
- current phase
- key decisions
- relevant files
- unresolved blockers
- verification state

Do not repeatedly reload completed investigation.

## 33. Long Execution Sessions

When context becomes inefficient or execution must continue later, create a structured handoff.

Use `session-handoff`.

Do not rely on raw conversation history as the only execution state.

## 34. Execution Checkpoint

After each substantial phase record:

Completed:
what changed

Verified:
what checks passed

Remaining:
next work

Blocked:
only when applicable

Keep checkpoints concise.

## 35. Plan Drift

Periodically compare execution against:

- original objective
- current plan
- actual diff

Detect when implementation has drifted into unrelated work.

If drift occurs, stop unnecessary work and return to scope.

## 36. Completion Gate

Before declaring plan completion verify:

1. Objective achieved.
2. Required steps completed.
3. Required gates passed.
4. Integration verified where relevant.
5. Final diff inspected.
6. Temporary artifacts removed.
7. User changes preserved.
8. OPSEC preserved.
9. Known limitations documented.
10. No blocking required work remains.

## 37. Final Report

Report concisely:

### Implemented

What changed.

### Verified

What actually passed.

### Deviations

Material deviations from plan, if any.

### Remaining

Only unresolved required or relevant limitations.

Do not narrate every command.

## 38. Anti-Patterns

Avoid:

Blindly following stale plans.

Replanning after every minor discovery.

Executing dependent tasks in parallel.

Multiple agents editing the same files unnecessarily.

Testing only at the end.

Continuing after failed gates.

Trusting subagent completion without verification.

Expanding scope through unrelated cleanup.

Overwriting user changes.

Performing destructive actions because they appear in a plan.

Declaring completion because all checkboxes were marked.

## 39. Execution Decision Process

Use:

load plan
->
identify objective
->
inspect current state
->
validate important assumptions
->
classify dependencies
->
identify useful parallel work
->
execute smallest coherent batch
->
verify
    |
    failure -> diagnose -> fix/update plan -> verify
    |
    success
    v
next batch
->
final integration verification
->
inspect diff
->
compare against objective
->
complete

## 40. Default Principle

Reality overrides the plan.

Execute in dependency order.

Parallelize only independent work.

Use minimum necessary agents and model capability.

Verify meaningful stages early.

Do not cross failed gates.

Adapt when evidence changes.

Protect user work.

Complete the objective, not merely the checklist.
