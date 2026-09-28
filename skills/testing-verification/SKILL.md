---
name: testing-verification
description: Verify code changes, bug fixes, refactors, configuration changes, and implementations before completion is claimed. Use after meaningful changes and before reporting work as complete, fixed, passing, or ready. Select the smallest sufficient verification scope, run deterministic checks, inspect failures, prevent test manipulation, and report only evidence-backed results.
---

# Testing and Verification

Verification is part of implementation.

Code that has been written but not appropriately verified is not proven complete.

The objective is:

identify required evidence
->
run the smallest sufficient checks
->
inspect actual results
->
fix failures when appropriate
->
repeat verification
->
report evidence

Never substitute confidence for verification.

## 1. Core Rule

Do not claim that work is:

- complete
- fixed
- working
- passing
- correct
- production-ready
- regression-free

unless appropriate verification has actually been performed.

Statements about verification must correspond to checks that actually ran.

Bad:

"Everything should work."

"Tests should pass."

"The fix is complete."

Preferred:

"Targeted tests passed: 14/14."

"Type checking passed."

"The implementation was changed, but integration tests could not be run because the required service is unavailable."

## 2. Determine Verification Before Completion

Before declaring completion, determine what evidence would demonstrate correctness.

Examples:

Code logic:
-> targeted tests

Type changes:
-> type checker

Build configuration:
-> build

Formatting/style:
-> formatter/linter

API behavior:
-> integration test or direct request

Database migration:
-> migration test + schema verification

CLI:
-> execute representative commands

UI:
-> build + runtime/visual verification

Performance:
-> benchmark comparison

Do not use one verification mechanism to prove something it does not test.

Example:

Lint passing does not prove runtime correctness.

Compilation passing does not prove business logic correctness.

Unit tests passing do not automatically prove integration behavior.

## 3. Discover Project Verification Commands

Use project-defined commands whenever available.

Inspect relevant:

- CLAUDE.md
- package manifests
- Makefiles
- task runners
- CI configuration
- test configuration
- repository documentation

Prefer established project commands over inventing new verification procedures.

Examples:

npm test

npm run typecheck

npm run lint

npm run build

pytest

cargo test

go test

make test

Use the project's actual commands.

Do not assume commands based only on language or framework.

## 4. Verification Scope

Use the smallest verification scope that provides sufficient evidence.

Preferred progression:

changed behavior
->
targeted test
->
related component tests
->
integration tests
->
broader suite

Do not automatically run the entire repository test suite after every small modification.

Do not avoid broader verification when the change has broad impact.

## 5. Match Verification to Risk

### Low Risk

Examples:

- isolated utility change
- typo
- small deterministic transformation
- documentation-only modification

Possible verification:

- targeted test
- syntax/static check
- direct inspection

### Medium Risk

Examples:

- normal feature
- API modification
- component refactor
- validation change

Possible verification:

- targeted tests
- related test suite
- typecheck
- lint
- build where relevant

### High Risk

Examples:

- authentication
- authorization
- database migrations
- security-sensitive code
- concurrency
- payment logic
- shared infrastructure
- major architecture changes

Possible verification:

- targeted tests
- regression tests
- integration tests
- broader suite
- static analysis
- build
- independent review when justified

Verification effort should be proportional to potential impact.

## 6. Run Targeted Tests First

Prefer fast feedback.

Example:

Changed:

src/auth/session.ts

First run:

tests/auth/session.test.ts

rather than immediately running thousands of unrelated tests.

If targeted verification succeeds and the change has broader effects, expand verification appropriately.

This reduces iteration time and resource usage.

## 7. Verify the Original Requirement

Tests are not the only source of truth.

Before completion, compare the resulting behavior against the actual requested behavior.

Determine:

- Was every requested requirement implemented?
- Were edge conditions addressed?
- Did implementation drift from the original task?
- Did tests cover only part of the requirement?

A green test suite does not prove that an omitted requirement was implemented.

## 8. Bug Fix Verification

For bug fixes, use the original failure whenever possible.

Preferred sequence:

reproduce bug
->
create or identify failing regression test
->
confirm failure
->
implement fix
->
confirm regression test passes
->
run related verification

A bug fix is strongest when the exact previous failure is demonstrably prevented.

Use `systematic-debugging` when root cause has not yet been established.

## 9. Regression Tests

When practical, add a regression test for a confirmed defect.

The regression test should:

- fail because of the original defect
- pass because of the actual fix
- test observable behavior
- avoid unnecessary implementation coupling

Do not create meaningless tests merely to increase coverage.

## 10. Protect Tests From the Implementation

Tests should verify the implementation.

The implementation should not manipulate tests into passing.

Do not:

- delete failing tests
- skip failing tests
- weaken assertions
- change expected values without justification
- introduce broad mocks to avoid real behavior
- update snapshots blindly
- hardcode implementation specifically for test inputs

If an existing test is incorrect, establish why before changing it.

## 11. Avoid Test Overfitting

Passing existing tests is necessary but may not be sufficient.

Do not implement:

- hardcoded values matching fixtures
- special cases for known tests
- fake implementations used only to satisfy assertions
- behavior that fails for equivalent valid inputs

Implement the general requirement.

Tests verify correctness.

They do not define loopholes to exploit.

## 12. Strong Assertions

Prefer tests that verify meaningful behavior.

Weak:

result is not null

result exists

array length > 0

function did not throw

Stronger:

result equals expected domain value

state changed correctly

specific error is returned

database contains expected record

unauthorized operation is rejected

Prefer assertions tied to the contract being tested.

## 13. Test Behavior, Not Internals

Prefer observable behavior over unnecessary implementation details.

Tests should generally survive legitimate internal refactoring.

Avoid excessive assertions about:

- private functions
- internal call ordering
- temporary intermediate state
- implementation-specific structure

unless those details are themselves part of the required contract.

## 14. Mock Conservatively

Do not mock everything by default.

Mocks are appropriate when isolating:

- external services
- expensive operations
- nondeterministic boundaries
- failure conditions difficult to reproduce otherwise

Prefer real project components where practical.

Excessive mocking can produce tests that verify the mock architecture rather than the actual system.

## 15. Test Failure Paths

Do not verify only successful behavior.

When relevant, test:

- invalid input
- missing resources
- permission failures
- dependency failures
- malformed data
- timeout behavior
- duplicate requests
- boundary values

Verification should reflect realistic failure modes.

## 16. Concurrency Verification

For concurrency-sensitive code, avoid tests based only on arbitrary sleep durations.

Prefer:

- synchronization primitives
- deterministic coordination
- race detectors
- repeated stress execution when appropriate
- explicit event/state observation

Do not interpret one successful timing-dependent execution as proof of correctness.

## 17. Type Checking

Run type checking when changes affect typed code.

Type checking is particularly relevant after:

- interface changes
- function signature changes
- model/schema changes
- dependency updates
- cross-module refactoring

Do not assume tests cover all type-level consumers.

## 18. Linting

Use linting to detect:

- invalid patterns
- style violations
- suspicious constructs
- project-specific rules

Do not confuse lint success with functional verification.

Treat lint as one verification layer.

## 19. Build Verification

Run the project build when changes may affect:

- compilation
- bundling
- generated artifacts
- imports
- dependency resolution
- production configuration

A development test environment may not exercise production build behavior.

## 20. Runtime Verification

When feasible and relevant, execute the actual changed behavior.

Examples:

API:
-> send representative request

CLI:
-> invoke command

Service:
-> start and exercise endpoint

Parser:
-> process representative input

Library:
-> call public interface

Direct runtime evidence can reveal integration failures that static checks miss.

## 21. UI Verification

UI correctness cannot always be proven by unit tests alone.

When appropriate:

implement
->
build
->
render
->
inspect
->
compare with expected result
->
adjust
->
repeat

Verify relevant:

- layout
- responsive behavior
- interaction
- loading states
- error states
- accessibility behavior

Use available browser or visual tools when they provide meaningful evidence.

## 22. Database Verification

For database changes, consider:

- migration applies successfully
- expected schema exists
- rollback behavior when required
- existing data remains valid
- constraints behave correctly
- queries still work
- data transformations preserve intended values

Treat destructive migrations as high-risk.

Do not assume migration syntax alone proves safety.

## 23. API Verification

For API changes, verify relevant:

- request validation
- response schema
- status codes
- authentication
- authorization
- error behavior
- backwards compatibility
- side effects

Prefer contract-level verification where available.

## 24. Configuration Verification

Configuration changes should be verified through actual consumption when practical.

Do not only inspect the configuration file.

Verify that:

configuration
->
loads
->
parses
->
reaches consumer
->
produces intended behavior

## 25. Security Verification

For security-sensitive changes, ordinary functional tests may be insufficient.

Consider:

- unauthorized access tests
- malformed input
- privilege boundaries
- secret exposure
- injection paths
- logging behavior
- secure defaults

Use `security-review` for substantial security-sensitive work.

## 26. Performance Verification

Performance claims require measurement.

Do not claim:

"faster"

"more efficient"

"lower memory"

based solely on code appearance.

Use:

baseline
->
change
->
same measurement
->
comparison

Use `performance-analysis` for substantial optimization work.

## 27. Inspect the Diff

Before completion, inspect the actual change set.

Check for:

- unrelated modifications
- accidental formatting
- debug code
- temporary logs
- commented-out code
- unintended test changes
- generated files
- secrets
- personal information
- unexpected dependency changes

Verification includes verifying what changed, not only executing tests.

## 28. Preserve User Changes

Do not revert unrelated existing modifications in order to obtain a clean test state.

Distinguish:

- changes produced by the current task
- pre-existing user changes
- unrelated repository state

If pre-existing changes interfere with verification, report the limitation instead of destroying them.

## 29. OPSEC Verification

Before externally sharing or publishing output, inspect for accidental disclosure.

Check relevant artifacts for:

- real names
- personal email addresses
- machine usernames
- absolute personal paths
- API keys
- tokens
- credentials
- private infrastructure
- sensitive environment values

Use the user's approved tag or handle when attribution is required.

If the correct public identifier is required but unknown, ask the user.

## 30. Verification Failures

When a verification step fails:

1. Read the actual failure.
2. Determine whether it relates to the current change.
3. Diagnose the cause.
4. Fix the implementation when appropriate.
5. Rerun the relevant verification.
6. Continue until sufficient evidence exists.

Do not blindly rerun the same failing command.

Do not hide the failure.

## 31. Pre-Existing Failures

If verification reveals a failure that existed before the current change:

- establish that it is pre-existing when possible
- do not automatically fix unrelated failures
- determine whether it prevents verification of the current task
- report it accurately

Do not claim the entire suite passes when it does not.

Example:

"Targeted tests for the changed component pass. The full suite still contains two pre-existing failures in X and Y."

## 32. Unavailable Verification

Sometimes a verification step cannot be executed.

Examples:

- external service unavailable
- missing credentials
- unsupported platform
- unavailable hardware
- required environment missing

Do not fabricate success.

State:

- what was verified
- what could not be verified
- why
- remaining risk

## 33. Fresh Evidence

Completion claims should rely on verification performed after the relevant changes.

Do not rely on:

- tests run before the final modification
- historical CI results
- another agent's unsupported claim
- expected behavior
- previous successful builds

After material changes, rerun the checks affected by those changes.

## 34. Subagent Results

Do not automatically trust:

"Agent reports tests pass."

When important, inspect:

- actual diff
- test output
- command result
- relevant artifacts

The primary agent remains responsible for final integration.

Use independent verification agents only when justified by risk or complexity.

Follow `agent-orchestration`.

## 35. Verification Agents

A separate verification agent may be useful for:

- security-sensitive changes
- large refactors
- architecture changes
- complex migrations
- high-impact production logic

The verifier should receive the requirement and resulting change, not the implementer's entire reasoning history.

This provides a more independent evaluation.

Do not create a verifier agent for every trivial modification.

## 36. Verification Loop

Verification is iterative.

Use:

change
->
verify
->
failure?
    |
    yes
    ->
    diagnose
    ->
    fix
    ->
    verify again
    |
    no
    ->
    continue

Do not stop after the first failed verification.

Do not stop after a fix without rerunning the failed check.

## 37. Resource Efficiency

Verification must also respect resource constraints.

Prefer:

targeted test
before
full suite

Prefer:

existing deterministic check
before
LLM reviewer

Prefer:

relevant component build
when available
before
expensive repository-wide operation

However, do not optimize verification cost to the point that evidence becomes insufficient.

Correctness takes priority over resource savings.

## 38. Project-Specific Verification

When the same manual verification steps repeatedly occur, encode them in project-specific verification instructions or skills.

Examples:

Frontend project:

build
->
browser render
->
responsive check
->
console errors
->
critical interaction

Database project:

migration
->
schema validation
->
integration tests
->
rollback check

API project:

contract tests
->
authentication
->
validation
->
error responses

Prefer explicit project-specific verification over repeatedly reconstructing the process.

## 39. Completion Gate

Before claiming completion, check:

1. Was the requested behavior implemented?
2. Was the original problem or requirement verified?
3. Did relevant targeted tests run?
4. Did required static checks run?
5. Did relevant integration/build checks run?
6. Were failures investigated?
7. Were tests preserved rather than weakened?
8. Was the final diff inspected?
9. Were temporary debugging artifacts removed?
10. Were unrelated user changes preserved?
11. Were OPSEC requirements maintained?
12. Are any verification limitations still unresolved?

Only then report completion.

## 40. Verification Report

Keep the final verification report concise.

Preferred format:

### Verification

- Targeted tests: passed - 14/14
- Typecheck: passed
- Build: passed
- Integration test: not run - required external service unavailable
- Diff inspection: completed
- Remaining limitation: integration behavior not directly verified

Report actual results.

Do not report checks that were not performed.

## 41. Anti-Patterns

Avoid:

"Should work."

"Looks correct."

Claiming tests passed without running them.

Running only lint and calling the feature verified.

Running only compilation and calling behavior verified.

Weakening tests.

Deleting failing tests.

Skipping failing tests.

Blindly updating snapshots.

Hardcoding test-specific behavior.

Mocking the system under test excessively.

Using arbitrary sleeps for concurrency verification.

Trusting subagent success claims without evidence.

Running enormous test suites unnecessarily.

Ignoring relevant integration checks to save tokens.

Claiming performance improvements without measurements.

Claiming completion after the final change without rerunning affected checks.

## 42. Default Verification Sequence

For normal code changes:

understand requirement
->
implement
->
run targeted tests
->
fix failures
->
run relevant static checks
->
run broader checks when justified
->
inspect diff
->
compare against original requirement
->
report actual evidence

For bug fixes:

reproduce
->
confirm failure
->
implement minimal fix
->
confirm original failure is resolved
->
run regression test
->
run related tests
->
inspect diff
->
report evidence

## 43. Default Principle

Evidence before claims.

Use deterministic verification wherever possible.

Verify the behavior that actually changed.

Start narrow and expand according to risk.

Never weaken legitimate checks merely to obtain a passing result.

Never claim a check passed unless it actually ran and passed.

A task is complete only when the available evidence is sufficient to support that conclusion.
