---
name: systematic-debugging
description: Systematically diagnose bugs, failing tests, crashes, regressions, incorrect behavior, and unexpected runtime results. Use when behavior differs from expectations and the root cause is not already proven. Prioritize reproduction, evidence, hypothesis testing, root-cause identification, minimal fixes, regression tests, and verification over trial-and-error changes.
---

# Systematic Debugging

Debug by evidence.

Do not debug by repeatedly changing code until the symptom disappears.

The objective is:

reproduce
->
collect evidence
->
localize
->
form hypothesis
->
test hypothesis
->
identify root cause
->
implement minimal fix
->
add regression protection
->
verify

A plausible explanation is not a proven root cause.

A plausible fix is not a verified fix.

## 1. Core Rule

Never modify code merely because it looks suspicious.

Before implementing a non-trivial bug fix, establish:

1. What behavior is expected?
2. What behavior actually occurs?
3. Can the failure be reproduced?
4. Where does expected and actual behavior diverge?
5. What evidence identifies the root cause?
6. What is the smallest correct fix?
7. How will the fix be verified?

Skip unnecessary ceremony only when the defect and correction are genuinely trivial and directly verifiable.

## 2. Define the Failure Precisely

Convert vague bug reports into an observable failure.

Bad:

"Authentication is broken."

Better:

"POST /api/login returns HTTP 500 when a valid user logs in after their previous session has expired."

Record when relevant:

- input
- expected output
- actual output
- error message
- stack trace
- affected component
- environment
- frequency
- timing
- known trigger
- last known working state

Do not begin broad debugging until the failure is sufficiently defined.

## 3. Reproduce First

Whenever reasonably possible, reproduce the failure before modifying code.

Prefer the smallest reliable reproduction.

Possible methods:

- targeted test
- direct command
- minimal request
- isolated function invocation
- integration test
- controlled UI interaction
- minimal fixture
- benchmark
- deterministic script

A reproduction establishes a baseline against which the fix can be verified.

## 4. Record the Baseline

Before changing code, capture the relevant failing behavior.

Examples:

- failing test name
- exact exception
- HTTP status
- incorrect return value
- incorrect database state
- benchmark result
- relevant log sequence

The baseline should allow comparison after the fix.

## 5. If Reproduction Fails

Do not immediately start guessing.

Determine why reproduction differs from the reported environment.

Investigate:

- environment differences
- configuration
- dependency versions
- feature flags
- data state
- race conditions
- timing
- cache state
- operating system differences
- external service behavior
- deployment differences

If the issue cannot be reproduced, state that clearly.

Do not claim to have fixed an unverified failure merely because a suspicious code path was changed.

## 6. Start From the Failure Boundary

Begin investigation where incorrect behavior becomes observable.

Examples:

Exception:
-> stack trace

Incorrect API response:
-> endpoint/controller

Incorrect database value:
-> write path

UI defect:
-> component receiving incorrect state

Performance regression:
-> measured slow operation

Then trace backward toward the source.

Do not start from unrelated architecture merely because it is complex.

## 7. Use Existing Evidence

Prioritize concrete evidence:

1. reproducible behavior
2. failing tests
3. stack traces
4. logs
5. runtime values
6. code execution path
7. version control history
8. documentation
9. hypotheses

Do not invert this hierarchy by forming a theory first and searching only for evidence that confirms it.

## 8. Investigate the Relevant Code Path

Use `codebase-investigation` when the execution path is not already understood.

Trace only the relevant path.

Example:

request
->
router
->
middleware
->
controller
->
service
->
repository
->
database

Determine where expected and actual behavior first diverge.

That divergence is usually more valuable than the final location where the error becomes visible.

## 9. Trace Data Backward

When a bad value causes the failure, trace it toward its origin.

Example:

invalid rendered value
<-
API response
<-
serializer
<-
service
<-
database result
<-
incorrect write

At each stage determine:

- expected value
- actual value
- transformation
- validation
- mutation
- serialization
- default behavior

Do not fix the final consumer if the producer is the actual source of corruption.

## 10. Form Explicit Hypotheses

When the root cause is not obvious, create explicit hypotheses.

Example:

H1:
Expired sessions bypass validation during refresh.

H2:
Expiration timestamps are parsed in the wrong timezone.

H3:
The cache returns stale session state.

Each hypothesis must be testable.

Avoid vague hypotheses such as:

"Something is wrong with caching."

## 11. Rank Hypotheses

Prioritize hypotheses based on evidence and probability.

Prefer testing hypotheses that are:

- strongly supported by evidence
- easy to falsify
- close to the observed failure
- consistent with the execution path

Do not investigate exotic explanations before ordinary explanations when evidence does not justify it.

## 12. Test One Hypothesis at a Time

Use the smallest experiment that distinguishes between explanations.

Examples:

- inspect runtime value
- add temporary targeted logging
- run one test
- bypass one component
- compare cached and uncached behavior
- inspect database state
- invoke one function directly

A hypothesis test should answer a specific question.

Avoid making several production code changes simultaneously to see whether the bug disappears.

## 13. Falsification Over Confirmation

Attempt to disprove the current hypothesis.

Ask internally:

"What observation would show this explanation is wrong?"

Strong debugging tries to eliminate incorrect explanations.

Do not keep a favored theory alive by explaining away contradictory evidence.

## 14. Temporary Instrumentation

Temporary debugging instrumentation may be used when necessary.

Examples:

- targeted logs
- assertions
- counters
- timing measurements
- diagnostic output

Keep instrumentation:

- minimal
- targeted
- non-sensitive
- easy to remove

Remove temporary debugging code before completion unless it provides legitimate permanent operational value.

Never log secrets, credentials, tokens, or unnecessary identifying information.

## 15. Use Git History When Relevant

For regressions, history can substantially reduce search space.

Useful questions:

- When did the behavior change?
- Which commit modified the failing path?
- Was the suspicious behavior intentional?
- Did a dependency update coincide with the regression?

Use:

git log
git diff
git show
git blame

when they answer a concrete debugging question.

Do not inspect repository history indiscriminately.

## 16. Bisect When Appropriate

For a reproducible regression with an unknown introduction point, consider binary search through history.

Use Git bisect or an equivalent process when:

- there is a known good revision
- there is a known bad revision
- reproduction is sufficiently deterministic
- the revision range is large enough to justify it

Do not manually inspect dozens of commits when a deterministic bisect can locate the introduction point efficiently.

## 17. External Dependencies

If evidence points toward external library behavior:

1. identify the installed version
2. inspect local types/source when useful
3. check version-specific documentation
4. inspect relevant release notes when regression-related
5. verify assumptions experimentally when possible

Use `documentation-research` when appropriate.

Do not blame a dependency merely because internal code looks correct.

## 18. Environment Problems

Distinguish code defects from environment defects.

Potential environment causes include:

- missing variables
- incorrect configuration
- stale build output
- incompatible dependency versions
- corrupted cache
- missing migration
- platform-specific behavior
- external service configuration

Do not modify application logic to compensate for a broken development environment unless that behavior should genuinely be handled by the application.

## 19. Concurrency and Timing Bugs

For intermittent failures, investigate:

- shared mutable state
- missing synchronization
- race conditions
- ordering assumptions
- retries
- timeouts
- asynchronous lifecycle
- eventual consistency
- duplicate events
- stale cache
- clock assumptions

Do not "fix" timing bugs by arbitrarily increasing sleeps or timeouts unless the timeout itself is proven to be incorrect.

Prefer synchronization based on actual state or events.

## 20. Performance Bugs

Do not diagnose performance by intuition alone.

Measure.

Establish:

baseline
->
profile
->
identify bottleneck
->
change
->
measure again

Use `performance-analysis` for substantial performance investigations.

Do not optimize unrelated code because it appears inefficient.

## 21. Root Cause Standard

A root cause should explain:

- the observed failure
- why it occurs
- why it occurs under the relevant conditions
- why the proposed fix addresses it

Prefer evidence connecting the root cause directly to reproduction.

Example:

Weak:

"The cache code looks suspicious."

Strong:

"The refresh path reads the cached session without executing validateExpiration(). The reproduction enters this path, and forcing expiration validation causes the failing test to pass."

## 22. Distinguish Root Cause From Trigger

The trigger is not necessarily the defect.

Example:

Trigger:
User refreshes the page during token expiration.

Root cause:
Refresh path bypasses expiration validation.

Do not remove valid triggers merely to avoid defective behavior.

## 23. Fix the Root Cause

Prefer fixing the earliest correct point in the failure chain.

Avoid symptom suppression.

Bad:

Catch exception and return empty result.

Better:

Correct the invalid state that produces the exception.

Bad:

Hide UI error.

Better:

Correct malformed API data.

Bad:

Retry indefinitely.

Better:

Fix the race condition or define bounded retry behavior.

## 24. Make the Smallest Correct Fix

Once root cause is established, modify only what is necessary.

Avoid combining the bug fix with:

- unrelated refactoring
- formatting changes
- dependency upgrades
- architecture redesign
- cleanup
- renaming
- speculative improvements

A narrow diff is easier to reason about and verify.

## 25. Preserve Existing Behavior

Before changing behavior, identify what must remain unchanged.

Consider:

- public interfaces
- existing callers
- compatibility
- error semantics
- data formats
- persisted data
- performance expectations

A bug fix should not silently create unrelated behavioral changes.

## 26. Regression Test

When practical, create or update a test that fails because of the original defect and passes after the fix.

Preferred sequence:

reproduce failure
->
encode regression test
->
confirm test fails
->
implement fix
->
confirm test passes

The regression test should target the defect rather than implementation details.

## 27. Do Not Manipulate Tests

Never make a failing test pass by weakening legitimate expectations.

Do not:

- remove assertions
- skip the test
- broaden expected values unnecessarily
- mock away the failing behavior
- change expected behavior merely to match the bug

Change tests only when the expected behavior itself is proven incorrect or intentionally changed.

## 28. Verify the Fix

After implementation:

1. rerun the original reproduction
2. run the regression test
3. run relevant nearby tests
4. run appropriate type/lint/build checks
5. inspect the diff
6. check for unintended behavior

Use `testing-verification` for the broader verification workflow.

Do not claim success before verification.

## 29. Verification Scope

Match verification effort to risk.

Small isolated fix:

targeted test
+
relevant static checks

Moderate fix:

targeted tests
+
related test suite
+
type/lint/build as appropriate

Critical or cross-system fix:

targeted reproduction
+
regression test
+
related integration tests
+
broader verification
+
independent review when justified

Do not run an extremely expensive full suite when a targeted check provides sufficient evidence, unless project requirements require it.

## 30. Failed Fix

If the proposed fix does not resolve the original reproduction:

Do not stack another speculative fix on top.

Instead:

1. revert or reconsider the ineffective change
2. inspect new evidence
3. update hypotheses
4. repeat diagnosis

Avoid accumulating speculative modifications.

## 31. Multiple Causes

Some failures have multiple contributing causes.

Do not stop after fixing one cause if the original reproduction still fails.

Likewise, do not assume multiple causes without evidence.

Resolve causes until the defined failure no longer occurs and verification passes.

## 32. Parallel Debugging

Use parallel agents only when investigation can be divided into independent domains.

Example:

Agent A:
Trace authentication execution path.

Agent B:
Investigate session persistence.

Agent C:
Inspect regression history.

Avoid:

Agent A:
Find root cause.

Agent B:
Find root cause.

Agent C:
Find root cause.

unless independent diagnosis is intentionally required for a high-risk issue.

Use `agent-orchestration` for delegation and model selection.

## 33. Debugging Agent Scope

A debugging subagent should receive:

- precise symptom
- expected behavior
- actual behavior
- reproduction when available
- relevant scope
- whether modification is allowed
- expected output

Preferred investigation-only output:

- reproduction status
- root cause or hypotheses
- supporting evidence
- relevant files/symbols
- uncertainty
- recommended next step

Do not return a complete diary of exploration.

## 34. Model Efficiency

Do not use the strongest model automatically.

Examples:

Simple failing assertion:
-> lightweight or mid-tier model

Normal application bug:
-> mid-tier model

Complex distributed/concurrency/security defect:
-> high-tier model when justified

Follow `agent-orchestration`.

Model capability should match debugging complexity.

## 35. Stop Guessing Threshold

If two or more speculative attempts fail, stop modifying code.

Return to diagnosis.

Re-establish:

- reproduction
- execution path
- evidence
- hypotheses

Repeated failed patches indicate insufficient understanding.

Do not continue trial-and-error indefinitely.

## 36. OPSEC During Debugging

Debugging output can contain sensitive information.

Never unnecessarily expose:

- real names
- personal emails
- usernames
- credentials
- tokens
- cookies
- private keys
- internal URLs
- personal filesystem paths
- sensitive request payloads

Redact sensitive values in logs and reports.

Use the user's approved tag or handle when attribution is required.

Do not introduce sensitive debugging information into committed source code.

## 37. Completion Report

A completed debugging task should concisely state:

### Root Cause

What caused the defect.

### Fix

What changed.

### Evidence

What established the diagnosis.

### Verification

What tests/checks actually ran.

### Remaining Uncertainty

Only when applicable.

Do not claim checks were performed if they were not.

## 38. Debugging Decision Process

Use this sequence:

Define failure
->
Can it be reproduced?
->
Establish baseline
->
Locate failure boundary
->
Trace execution/data backward
->
Form testable hypotheses
->
Falsify hypotheses
->
Identify root cause
->
Create regression protection
->
Implement minimal fix
->
Run original reproduction
->
Run relevant verification
->
Inspect diff
->
Complete

If any stage reveals insufficient understanding, return to investigation rather than guessing.

## 39. Anti-Patterns

Avoid:

Changing code before reproducing.

Fixing suspicious-looking code without evidence.

Making multiple speculative changes simultaneously.

Retrying the same failed approach.

Increasing sleeps to hide races.

Catching exceptions to hide defects.

Weakening tests.

Blaming dependencies without verification.

Loading the entire repository.

Spawning agents for trivial debugging.

Using high-tier models for simple failures.

Declaring success because code looks correct.

Leaving temporary debugging code behind.

Confusing the trigger with the root cause.

Treating correlation as causation.

## 40. Default Principle

Reproduce before fixing.

Measure before concluding.

Trace before guessing.

Falsify before accepting a hypothesis.

Fix causes, not symptoms.

Change as little as necessary.

Add regression protection.

Verify before declaring success.
