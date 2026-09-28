---
name: code-review
description: Review code changes for concrete correctness defects, regressions, edge cases, unsafe behavior, architectural violations, maintainability problems, and missing verification. Use for diffs, pull requests, commits, substantial changes, or before merging high-impact work. Prioritize actionable evidence over speculative findings and scale review effort according to change risk.
---

# Code Review

Review code for concrete problems.

The objective is not to maximize the number of findings.

The objective is to identify real issues that materially affect:

- correctness
- reliability
- security
- maintainability
- compatibility
- performance
- operability

A short review containing two verified defects is better than a long review containing twenty speculative concerns.

## 1. Core Rule

Every reported finding should answer:

1. Where is the problem?
2. What behavior is incorrect or risky?
3. Under what realistic conditions does it occur?
4. What evidence supports the finding?
5. What is the likely impact?

Do not report an issue merely because something could theoretically go wrong.

## 2. Review the Change, Not the Entire Repository

Default to a diff-first review.

Start with:

git status
->
git diff
->
changed files
->
affected behavior

Then inspect surrounding code only when necessary to understand the change.

Do not perform an exhaustive repository audit unless explicitly requested.

## 3. Establish Review Scope

Determine what is being reviewed.

Possible scopes:

- working tree changes
- staged changes
- commit
- commit range
- branch diff
- pull request
- specific files
- implementation associated with a task

Do not silently review an arbitrary scope.

If scope cannot be determined from context or repository state, establish it before making review conclusions.

## 4. Understand Intent

Before judging implementation, determine what the change is intended to accomplish.

Use available:

- user request
- task description
- specification
- issue
- tests
- commit message
- relevant documentation

A change cannot be meaningfully reviewed without understanding its intended behavior.

## 5. Inspect Repository State

When appropriate, inspect:

- git status
- relevant diff
- changed files
- new files
- deleted files
- dependency changes
- configuration changes
- tests changed with implementation

Distinguish task changes from unrelated pre-existing user changes.

Never revert unrelated user work during review.

## 6. Review in Layers

Review changes in this order:

### Layer 1 - Correctness

Does the implementation perform the intended behavior?

### Layer 2 - Regression Risk

Could existing behavior be broken?

### Layer 3 - Edge Cases

Are realistic boundary conditions handled?

### Layer 4 - Failure Behavior

Are errors handled correctly?

### Layer 5 - Security

Does the change introduce a meaningful security concern?

### Layer 6 - Architecture

Does the change violate important established boundaries?

### Layer 7 - Maintainability

Is there concrete unnecessary complexity or duplication?

### Layer 8 - Performance

Does the change introduce a plausible material performance regression?

### Layer 9 - Verification

Do tests and checks sufficiently cover the changed behavior?

Prioritize correctness over style.

## 7. Correctness Review

Check for concrete issues involving:

- incorrect conditions
- wrong return values
- missing branches
- state corruption
- incorrect transformations
- incorrect assumptions
- off-by-one errors
- null/undefined handling
- invalid lifecycle transitions
- resource leaks
- incorrect API usage
- incorrect persistence
- inconsistent state

Trace actual execution when necessary.

Do not infer behavior from isolated lines when surrounding control flow changes the conclusion.

## 8. Regression Review

Determine what existing behavior the change may affect.

Inspect:

- callers
- public interfaces
- shared utilities
- data models
- schemas
- configuration
- persistence
- serialization
- error semantics

A locally correct change can still create a regression elsewhere.

Use `codebase-investigation` when impact is not obvious.

## 9. Edge Cases

Look for realistic edge cases derived from actual inputs and system constraints.

Examples:

- empty collections
- missing optional values
- zero
- negative values
- maximum values
- duplicate events
- repeated requests
- expired state
- partial data
- malformed input
- concurrent operations
- retries

Do not invent absurd edge cases merely to produce findings.

## 10. Error Handling

Review whether failures are:

- detected
- propagated
- handled at the correct boundary
- logged appropriately
- converted into correct public behavior

Watch for:

- swallowed exceptions
- broad catch blocks
- silent fallback
- misleading success responses
- infinite retry
- missing cleanup
- lost error context

Do not demand error handling where propagation is already correct.

## 11. Security Review Boundary

During ordinary code review, identify obvious or directly relevant security problems.

Examples:

- missing authorization
- injection
- unsafe file access
- secret exposure
- unsafe deserialization
- insecure defaults

For substantial security analysis, use `security-review`.

Do not turn every ordinary review into a complete security audit.

## 12. Architecture Review

Check whether the change violates established architectural boundaries.

Examples:

- UI directly accessing persistence
- domain logic placed in transport layer
- internal implementation exposed publicly
- duplicated cross-cutting behavior
- bypassed service boundaries

Do not report architectural preferences merely because another design is possible.

A finding requires a concrete project convention, coupling problem, correctness concern, or meaningful maintenance cost.

## 13. Maintainability

Report maintainability problems only when they have a concrete cost.

Examples:

- duplicated complex logic likely to diverge
- unnecessary state
- deeply confusing control flow
- abstraction that obscures behavior
- hidden side effects
- misleading naming that can cause misuse

Do not report:

- personal style preferences
- harmless naming differences
- minor formatting
- subjective aesthetic concerns

when automated tooling or existing conventions already handle them.

## 14. Avoid Over-Engineering Recommendations

Do not recommend:

- new abstraction
- factory
- interface
- framework
- helper layer
- configuration system

unless the current change creates a concrete problem that the abstraction solves.

"Could be more extensible" is not sufficient.

Review current requirements, not hypothetical future systems.

## 15. Performance Findings

Performance findings require a plausible mechanism.

Good:

"This loop performs one database query per item. For N records this produces N additional round trips."

Weak:

"This might be slow."

When performance impact cannot be established from code alone, recommend measurement rather than asserting a defect.

Use `performance-analysis` for substantial performance work.

## 16. Concurrency

For concurrent or asynchronous code, inspect:

- shared state
- ordering
- synchronization
- cancellation
- retries
- idempotency
- duplicate execution
- stale state
- race conditions
- cleanup

Do not report a race merely because multiple asynchronous operations exist.

Identify the conflicting operations and state.

## 17. Database Changes

For database-related changes inspect:

- transaction boundaries
- constraints
- nullability
- indexes
- migration compatibility
- data preservation
- query count
- locking
- rollback behavior

Treat destructive schema changes as higher risk.

Use specialized migration review when available.

## 18. API Changes

Review:

- request validation
- response contract
- status codes
- authentication
- authorization
- backwards compatibility
- error responses
- idempotency when relevant

Determine whether existing consumers can continue functioning.

## 19. Dependency Changes

When dependencies change, determine:

- why dependency is needed
- whether project already provides equivalent functionality
- whether version constraints are appropriate
- whether API usage matches installed version
- whether dependency introduces material risk

Do not automatically reject new dependencies.

Do not automatically accept them either.

## 20. Test Review

Tests are part of the change.

Check whether tests:

- cover intended behavior
- cover confirmed regression
- contain meaningful assertions
- test failure paths where relevant
- avoid excessive mocking
- avoid implementation overfitting

Watch for implementation changes accompanied by weakened tests.

## 21. Detect Test Manipulation

High-priority warning signs:

- deleted failing test
- skipped test
- weakened assertion
- broad snapshot replacement
- changed expected value without behavioral justification
- excessive new mocking
- hardcoded fixture-specific behavior

Determine whether test changes reflect legitimate requirement changes before reporting them as defects.

## 22. Verify Findings

Before reporting a finding, verify it when practical.

Possible methods:

- trace execution
- inspect caller
- inspect type definition
- search usage
- run targeted test
- reproduce behavior
- consult version-specific documentation

Do not report a finding that is contradicted by surrounding code.

## 23. Evidence Standard

Classify potential findings internally.

### Verified

Directly demonstrated by code path, test, runtime behavior, or deterministic evidence.

### Strong

Code clearly produces incorrect behavior under realistic conditions.

### Speculative

Requires assumptions not established by available evidence.

Report Verified and Strong findings.

Normally omit Speculative findings.

If speculative risk is important enough to mention, explicitly state what remains unverified.

## 24. False Positive Suppression

Before reporting each finding, ask:

- Is the code actually reachable?
- Can the relevant input actually occur?
- Is the condition already validated upstream?
- Is cleanup performed elsewhere?
- Does the framework already guarantee this behavior?
- Is this intentionally handled by another layer?
- Does a test prove the behavior is intentional?
- Am I assuming behavior of an external dependency?

If uncertainty remains material, investigate before reporting.

## 25. Severity

Use severity based on impact and realistic likelihood.

### Critical

Likely severe production impact such as:

- major security compromise
- irreversible widespread data loss
- catastrophic integrity failure

Use rarely.

### High

Likely substantial functional or security impact:

- core functionality broken
- authorization bypass
- significant data corruption
- major production regression

### Medium

Real defect with limited or conditional impact:

- incorrect edge-case behavior
- localized regression
- meaningful reliability issue
- realistic failure under specific conditions

### Low

Concrete but limited problem:

- minor correctness issue
- small maintainability defect with identifiable cost
- limited operational problem

Do not inflate severity.

Severity is not a measure of how interesting a finding is.

## 26. Confidence

For reported findings, maintain an internal confidence assessment.

Prefer reporting findings with high confidence.

For lower-confidence findings with potentially severe consequences, investigate further before deciding whether to report.

Do not fill the report with uncertain observations.

## 27. Prioritize Findings

Order findings by:

1. severity
2. confidence
3. likelihood
4. affected scope

Do not prioritize cosmetic concerns above correctness.

## 28. Parallel Review

For large or high-risk changes, review dimensions may be delegated independently.

Example:

Agent A:
correctness and regressions

Agent B:
security

Agent C:
tests and compatibility

Agent D:
performance, only when relevant

Use `agent-orchestration` to determine whether parallel review justifies the resource cost.

Do not automatically spawn multiple review agents.

## 29. Model Selection

Small diff:
-> primary agent or lightweight/mid-tier model

Normal review:
-> mid-tier model

Large/high-risk cross-system review:
-> high-tier model or specialized parallel review when justified

Do not use the strongest available model for every small diff.

Follow `agent-orchestration`.

## 30. Independent Review

For high-risk implementation, an independent reviewer can provide value.

The reviewer should receive:

- intended requirement
- resulting diff
- relevant repository context

Avoid providing the implementer's full reasoning history unless necessary.

This reduces anchoring on the implementation approach.

## 31. Review Existing Findings

When another reviewer or agent supplies findings, verify them before presenting them as established defects.

Do not aggregate multiple agent reports blindly.

Deduplicate overlapping findings.

Reject findings unsupported by code.

## 32. Do Not Modify During Review

When the task is review-only:

- do not modify source
- do not refactor
- do not update tests
- do not fix findings automatically

Return findings.

If the user requested review-and-fix, separate:

review
->
confirm findings
->
implement
->
verify

## 33. OPSEC

Review output must not unnecessarily expose:

- real names
- personal email addresses
- local usernames
- personal filesystem paths
- secrets
- credentials
- tokens
- private keys
- sensitive internal infrastructure

Use the user's approved tag or handle when attribution is required.

If an approved public identifier is required but unknown, ask the user.

## 34. Review Output

Lead with findings.

For each finding provide:

### Severity - Short Title

Location:
`path/to/file.ext:line`

Problem:
Concise description.

Trigger:
Realistic condition causing the problem.

Impact:
Concrete consequence.

Evidence:
Why the conclusion follows from the code.

Suggested direction:
Minimal correction direction when useful.

Avoid long essays.

## 35. No Findings

If no concrete findings remain after review, state that no verified review findings were identified.

Then state relevant verification limitations when applicable.

Do not invent minor findings merely to make the review appear useful.

"No findings" is a valid review result.

## 36. Review Summary

After findings, optionally summarize:

- scope reviewed
- verification performed
- important limitations

Do not repeat every finding in the summary.

## 37. Example Finding

### High - Authorization Check Bypassed During Direct Update

Location:
`src/api/users/update.ts:84`

Problem:
The direct update path writes the requested account before checking whether the authenticated user owns that account.

Trigger:
An authenticated user submits another account ID to the endpoint.

Impact:
The user can modify another user's account data.

Evidence:
The handler passes the request ID directly to `updateUser()` while the ownership check is executed only by the separate read path.

Suggested direction:
Perform authorization against the target account before the update operation.

This is actionable because it identifies:

- exact location
- reachable path
- realistic trigger
- concrete impact
- supporting execution behavior

## 38. Bad Findings

Avoid:

"Consider adding more comments."

"This function is long."

"This could possibly have a race condition."

"Maybe use a design pattern."

"Consider more error handling."

"This query might be slow."

"Could add more tests."

These statements are not findings without concrete evidence and impact.

## 39. Review Decision Process

Use:

determine scope
->
understand intent
->
inspect diff
->
identify affected behavior
->
inspect surrounding context
->
generate candidate findings
->
verify each candidate
->
discard speculative findings
->
assign realistic severity
->
deduplicate
->
report actionable findings

## 40. Completion Standard

A review is complete when:

- relevant diff was inspected
- intended behavior was understood
- affected execution paths were considered
- tests were reviewed when relevant
- candidate findings were verified
- speculative findings were removed
- severity reflects realistic impact
- OPSEC was preserved
- limitations are stated

The objective is not to prove that code is perfect.

The objective is to identify concrete problems with high signal and low noise.

## 41. Default Principle

Review evidence, not aesthetics.

Correctness before style.

Realistic failures before theoretical possibilities.

Verify before reporting.

Prefer one proven defect over ten speculative concerns.

Do not manufacture findings.
