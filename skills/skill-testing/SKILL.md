---
name: skill-testing
description: Evaluate new or modified Claude skills against representative scenarios and a no-skill baseline. Determine whether a skill materially improves correctness, decision quality, verification, efficiency, scope control, or safety without causing excessive context use, over-triggering, unnecessary agents, or rigid behavior. Use before treating a new skill as established and after substantial skill revisions.
---

# Skill Testing

Test skills by observed behavior.

A skill is not useful merely because its instructions appear reasonable.

The objective is to determine whether loading the skill produces a meaningful improvement over baseline behavior.

Use:

define expected behavior
->
establish baseline
->
run representative scenarios
->
run skill-enabled scenarios
->
compare outcomes
->
identify regressions
->
revise or reject
->
retest

## 1. Core Rule

Every candidate skill should justify its context and maintenance cost through observable behavioral improvement.

Compare:

WITHOUT SKILL

against:

WITH SKILL

Do not evaluate the skill only by reading its text.

## 2. What Skill Testing Measures

Evaluate relevant dimensions:

- correctness
- task completion
- evidence quality
- reasoning discipline
- verification quality
- resource efficiency
- tool efficiency
- agent efficiency
- scope control
- false-positive rate
- safety
- OPSEC
- trigger quality
- adaptability
- output usefulness

Not every skill needs improvement on every dimension.

The skill should materially improve the dimensions relevant to its purpose.

## 3. Define the Skill's Intended Effect

Before testing, state what behavior should change.

Example:

Skill:
`code-review`

Expected improvement:

- fewer speculative findings
- more evidence-backed findings
- better severity calibration
- concrete file locations
- fewer style-only complaints

Without a defined intended effect, evaluation becomes subjective.

## 4. Define Success Criteria

Establish success before examining results.

Example:

`systematic-debugging` succeeds if it:

- reproduces before modifying when practical
- forms testable hypotheses
- identifies root cause
- avoids repeated speculative patches
- verifies the final fix

Do not redefine success after seeing results merely to make the skill appear effective.

## 5. Representative Scenarios

Use scenarios that reflect real intended usage.

Include:

- ordinary case
- difficult case
- ambiguous case
- case where skill should NOT activate

When relevant, include a known failure mode.

Do not test only ideal examples designed around the skill wording.

## 6. Positive Trigger Tests

Test tasks where the skill clearly should apply.

Example for `security-review`:

"Review this authentication endpoint for security vulnerabilities."

Expected:

Skill should activate.

## 7. Negative Trigger Tests

Test tasks where the skill should not apply.

Example:

"Fix typo in README."

Expected:

`security-review` should not activate.

A skill that improves its target task but activates constantly may still be harmful overall.

## 8. Boundary Trigger Tests

Test ambiguous cases.

Example:

"Add validation to this API field."

Possible relevant skills:

- normal implementation
- security-review only if security boundary is material

Evaluate whether skill selection remains proportional.

## 9. Baseline First

When practical, establish behavior without the candidate skill.

Record:

- result quality
- errors
- tool calls
- agent calls
- unnecessary work
- verification
- scope adherence

This becomes the comparison baseline.

## 10. Skill-Enabled Run

Run the same or equivalent scenario with the skill available.

Keep other important conditions as similar as practical.

Compare behavior rather than wording.

## 11. Avoid Contaminated Comparisons

Do not compare:

simple baseline scenario

against:

much easier skill-enabled scenario.

Do not provide extra critical information only to the skill-enabled run.

The comparison should test the skill, not differences in task setup.

## 12. Multiple Runs

Agent behavior can vary.

For important skills, use multiple representative scenarios rather than trusting one successful run.

A single good result does not prove robustness.

A single failure does not necessarily invalidate the skill.

Look for patterns.

## 13. Correctness First

Correctness has highest priority.

A skill that saves tokens but causes incorrect implementation is not an improvement.

Priority:

1. correctness
2. safety
3. verification quality
4. scope adherence
5. resource efficiency
6. convenience

Efficiency optimizations must not undermine correctness.

## 14. Resource Measurement

When practical compare:

- tokens/context loaded
- number of tool calls
- number of agents
- model tiers used
- duplicate investigation
- repeated failed actions
- execution time

Exact numerical measurement is not always available.

Qualitative comparison is acceptable when clearly stated.

## 15. Agent Efficiency

For skills involving orchestration evaluate:

- unnecessary agent creation
- duplicate work
- inappropriate model selection
- unnecessary recursive delegation
- useful parallelization
- quality of returned summaries

A skill should not create activity for its own sake.

## 16. Tool Efficiency

Evaluate whether the skill causes:

- deterministic tools to be used appropriately
- unnecessary file reads
- repeated searches
- huge logs to be loaded
- unnecessary external research
- repeated identical commands

Efficiency should result from better decisions, not merely fewer actions.

## 17. Context Efficiency

Measure whether the skill causes unnecessary context loading.

Watch for:

- entire repositories read
- large files loaded unnecessarily
- unrelated skills loaded
- duplicated instructions
- excessive reference material

A skill that improves behavior slightly but doubles context usage may require simplification.

## 18. Scope Control

Evaluate whether the skill keeps work within requested scope.

Failure examples:

- unrelated refactoring
- unnecessary architecture changes
- fixing unrelated bugs
- reviewing unrelated repository areas
- adding dependencies without need

Strong skills reduce scope drift.

## 19. Verification Quality

For implementation skills evaluate whether the agent:

- runs relevant tests
- checks actual behavior
- distinguishes executed from unexecuted checks
- inspects failures
- avoids unsupported success claims

A skill should improve evidence quality, not just confidence.

## 20. False Positive Rate

For analytical skills such as:

- code-review
- security-review
- performance-analysis

track false positives.

A finding is low-quality when:

- path is unreachable
- framework already prevents it
- input cannot occur
- impact is invented
- finding is purely stylistic
- evidence contradicts it

Reducing false positives is a major quality improvement.

## 21. False Negative Risk

Do not optimize false-positive suppression so aggressively that obvious real problems disappear.

Include scenarios containing known defects.

Verify that the skill still identifies them.

Balance:

high signal
+
reasonable sensitivity

## 22. Severity Calibration

For review skills evaluate whether severity reflects actual:

- impact
- likelihood
- scope
- exploitability where relevant

Failure examples:

Critical:
minor edge case

High:
style issue

Low:
authentication bypass

Severity inflation and understatement both reduce usefulness.

## 23. Rigidity

A skill can fail by being too rigid.

Watch for:

- forcing formal planning for trivial tasks
- requiring TDD where meaningless
- refusing reasonable deviations
- following workflows after assumptions fail
- excessive mandatory sections
- unnecessary approval checkpoints

Skills should guide behavior, not disable judgment.

## 24. Adaptability

Test whether the skill handles:

- small tasks
- large tasks
- incomplete information
- changing repository state
- unexpected failures

A robust skill adapts while preserving its core principles.

## 25. Over-Triggering

A skill over-triggers when it loads for tasks where its specialized workflow adds little value.

Symptoms:

- increased context
- unnecessary agents
- unnecessary reviews
- excessive process
- slower trivial work

Fix by improving:

- description
- trigger conditions
- explicit non-triggers

## 26. Under-Triggering

A skill under-triggers when clearly relevant tasks fail to activate it.

Fix by:

- broadening description appropriately
- adding meaningful synonyms
- describing task semantics rather than exact phrases

Do not solve under-triggering with "ALWAYS USE."

## 27. Instruction Conflicts

Test interactions with:

- CLAUDE.md
- related skills
- project instructions
- user requests

Look for contradictory requirements.

Example:

`plan-execution` says follow plan.

Global rule says preserve user changes.

Correct behavior:

preserve user changes.

Skills must not undermine higher-level global constraints.

## 28. Skill Interaction Tests

For important workflows test combinations.

Example:

`implementation-planning`
+
`plan-execution`
+
`testing-verification`

Check whether:

- responsibilities remain separate
- instructions do not duplicate excessively
- handoffs are clear
- no infinite workflow loop occurs

## 29. Circular Invocation

Detect skill loops.

Bad:

Skill A requires B.

Skill B always requires A.

Potential result:

repeated loading or process loop.

Dependencies should form useful workflows, not cycles.

## 30. Skill Isolation

When diagnosing a skill problem, test the skill in isolation where practical.

This helps determine whether the defect belongs to:

- skill itself
- interaction with another skill
- global instructions
- task ambiguity

## 31. Adversarial Scenarios

For important skills include scenarios that tempt the agent to violate the intended rule.

Example for `code-review`:

Provide clean code and ask:

"Find at least 10 serious bugs."

Expected:

Agent should not fabricate findings merely to satisfy quantity pressure if higher-level instructions require evidence.

Example for orchestration:

"Use as many agents as possible."

Evaluate according to applicable user/system constraints and resource rules.

## 32. Known Failure Scenarios

When a skill was created because of repeated failure, preserve representative examples of that failure.

Example:

Before `systematic-debugging`:

agent makes three speculative patches.

Test:

same class of bug.

Expected:

agent now reproduces and diagnoses before modifying.

This provides direct evidence that the skill solves its original problem.

## 33. Safety Tests

For skills touching sensitive operations test whether they preserve:

- credentials
- user data
- destructive-operation boundaries
- production safety
- authorization scope

Efficiency never justifies violating safety requirements.

## 34. OPSEC Tests

Verify that skills do not cause disclosure of:

- user's real name
- personal email
- machine username
- absolute personal paths
- credentials
- secrets
- private infrastructure

Include at least one scenario where such information exists in context but is unnecessary.

Expected:

Skill should not expose it.

## 35. Handle/Tag Behavior

When attribution is required:

Known approved handle:
-> use handle

Unknown approved handle:
-> ask user when identifier is required

Never infer public identity from private information.

## 36. External Skill Testing

Community skills must be tested before being trusted.

Inspect and test for:

- destructive commands
- external uploads
- credential access
- hidden network behavior
- excessive permissions
- identity leakage
- aggressive triggering
- context bloat

Popularity does not waive validation.

## 37. Static Skill Review

Before behavioral testing, inspect the skill text for obvious problems.

Check:

- valid metadata
- clear description
- coherent scope
- duplicate instructions
- contradictions
- obsolete model names
- unnecessary verbosity
- unsafe commands
- external dependencies

Static review is necessary but not sufficient.

## 38. Description Testing

The description should correctly communicate activation conditions.

Test several prompts:

Should activate.

Should not activate.

Ambiguous.

Revise description if activation behavior is poor.

## 39. Workflow Testing

Verify that the workflow produces useful action order.

Example:

Debugging:

reproduce
->
investigate
->
hypothesis
->
fix
->
verify

Failure:

fix
->
guess
->
fix
->
eventually test

Test actual behavior rather than merely checking whether the workflow appears in text.

## 40. Stop Condition Testing

Verify that the skill stops at an appropriate point.

Failure examples:

- investigation continues after sufficient evidence
- review keeps searching for more findings
- planning continues after executable plan exists
- optimization continues after target is met

Skills must prevent unnecessary continuation.

## 41. Failure Recovery Testing

Introduce a realistic failure.

Examples:

- test fails
- expected file missing
- documentation contradicts assumption
- agent result incomplete

Observe whether the skill:

- diagnoses
- adapts
- avoids blind retry
- avoids fabricated success

## 42. Test Skill Revisions

After modifying a skill, rerun relevant tests.

Do not assume a change improves behavior.

A fix for over-triggering may cause under-triggering.

A context reduction may remove an important constraint.

## 43. Regression Tests for Skills

When a specific skill failure is discovered, preserve it as a future regression scenario.

Example:

Failure:
security-review repeatedly reports parameterized query as SQL injection.

Regression scenario:
parameterized query with attacker-controlled value.

Expected:
no SQL injection finding.

## 44. Skill Test Cases

Maintain lightweight test cases when useful.

Conceptually:

`tests/skill-testing/code-review/`

- real-bug.md
- clean-diff.md
- ambiguous-case.md
- false-positive-trap.md

Do not create large testing infrastructure unless skill importance justifies it.

## 45. Test Case Format

A useful skill test contains:

### Scenario

Task and relevant context.

### Expected Behavior

What the skill should change.

### Forbidden Behavior

Important failure modes.

### Evaluation Criteria

How result will be judged.

### Result

Observed behavior.

### Decision

Pass / revise / reject.

## 46. Avoid Exact-Text Testing

Do not require the agent to output exact phrases unless wording itself matters.

Evaluate behavior and result.

Exact-text tests are brittle and often measure formatting instead of usefulness.

## 47. Comparative Evaluation

Compare baseline and skill-enabled behavior using a concise table when helpful.

Example:

| Criterion | Baseline | With Skill |
|---|---|---|
| Root cause established | No | Yes |
| Speculative patches | 3 | 0 |
| Regression test | No | Yes |
| Verification | Partial | Complete |
| Extra agents | 0 | 0 |

Focus on meaningful differences.

## 48. Weighted Evaluation

For important skills, criteria may have different importance.

Example:

Security review:

Correct vulnerabilities detected:
very high

False positives:
high

Severity calibration:
high

Formatting:
low

Do not let polished output compensate for incorrect analysis.

## 49. Rejecting a Skill

Reject or remove a skill when:

- no meaningful improvement appears
- context cost exceeds benefit
- behavior becomes less reliable
- over-triggering is severe
- existing skill performs same function
- deterministic automation is superior
- maintenance burden is unjustified

Skill deletion is a valid successful outcome of testing.

## 50. Revising a Skill

When testing reveals a problem, identify the smallest likely cause.

Examples:

Over-triggering:
-> description too broad

Excessive workflow:
-> missing process-budget rule

False positives:
-> insufficient evidence standard

Context bloat:
-> duplicated instructions

Rigid behavior:
-> too many unconditional rules

Fix the cause rather than adding random instructions.

## 51. Avoid Instruction Accretion

Do not fix every failure by appending another rule.

Repeated patching can create:

- contradictions
- bloated context
- poor prioritization

Sometimes consolidate or rewrite sections.

## 52. Skill Compression

After a skill becomes stable, consider whether it can be shortened without losing behavior.

Test:

full skill

vs

compressed skill

If behavior remains equivalent, prefer the smaller version.

Context efficiency matters.

## 53. Model Robustness

When important, test whether the skill works across the model tiers expected to use it.

A skill designed only around a high-tier model may fail with lightweight subagents.

Evaluate according to intended routing.

Do not require every skill to work identically on every model.

## 54. Lightweight Model Testing

For skills intended for lightweight agents, test:

- instruction comprehension
- scope adherence
- completion format
- failure handling

Simpler models may benefit from more explicit structure.

Balance explicitness against context cost.

## 55. High-Tier Model Testing

For high-tier models, watch for:

- over-analysis
- excessive agents
- excessive exploration
- unnecessary architecture
- speculative findings

Skills may need stronger stop conditions rather than more reasoning instructions.

## 56. Resource Regression

A skill can improve correctness while creating disproportionate resource cost.

Example:

Baseline:
correct result with 5 tool calls.

Skill:
same correct result with 30 tool calls and 6 agents.

Unless additional assurance is justified, this is a regression.

## 57. Quality Regression

A skill can improve efficiency but reduce quality.

Example:

Baseline:
correct diagnosis.

Skill:
faster but incorrect diagnosis.

Reject the optimization.

Correctness remains primary.

## 58. Process Regression

Watch for excessive process.

Example:

Simple change:

skill triggers:
investigation
+
formal plan
+
three agents
+
security review
+
full test suite

This is a process regression.

## 59. Cross-Skill Regression

Adding one skill may change usage of others.

Example:

New planning skill causes every task to invoke planning and suppress direct execution.

Test important workflows after introducing major skills.

## 60. Persistent Configuration

Changes to persistent skills affect future tasks.

Treat modifications carefully.

When testing experimental changes, keep them isolated where practical before replacing stable versions.

## 61. Versioning

For heavily used skills, consider tracking meaningful revisions.

Record why substantial behavioral changes were made.

Do not create elaborate version-management bureaucracy for simple local skills.

## 62. Evidence Log

For important skills maintain concise evidence of why the skill exists.

Example:

Problem:
agents repeatedly reported speculative review findings.

Skill change:
added reachability and evidence requirements.

Observed result:
false-positive scenarios no longer reported.

This helps prevent future removal of important constraints without understanding their purpose.

## 63. Testing External Updates

When updating a community-derived skill:

- review diff
- inspect new instructions
- retest relevant scenarios
- verify OPSEC
- verify resource behavior

Do not assume newer means better.

## 64. Human Evaluation

Some criteria require judgment.

Examples:

- plan usefulness
- maintainability recommendation
- review clarity

Use explicit evaluation criteria to reduce subjective drift.

Do not pretend qualitative evaluation is mathematically precise.

## 65. Automated Evaluation

Automate deterministic checks when useful.

Examples:

- required metadata exists
- forbidden private strings absent
- expected file created
- syntax valid
- test fixture passes

Do not use an LLM for checks that deterministic automation can perform more reliably.

## 66. Skill Testing With Agents

For substantial evaluations, independent evaluator agents may be useful.

Example:

Agent A:
execute baseline scenario

Agent B:
execute skill-enabled scenario

Primary:
compare outputs

Use `agent-orchestration`.

Do not use multiple expensive agents for trivial skill tests.

## 67. Blind Evaluation

When practical, evaluate baseline and skill-enabled outputs without relying on knowledge of which output used the skill.

This reduces preference bias toward the newly created skill.

Evaluate against predefined criteria.

## 68. Independent Evaluation

For important skills, the agent that created the skill should not always be the only evaluator.

Independent evaluation can reduce:

- confirmation bias
- attachment to design decisions
- rationalization of failures

Use an independent evaluator only when the importance of the skill justifies the additional resource cost.

## 69. Do Not Optimize for the Evaluator

A skill should improve real task execution.

Do not modify skills merely to:

- produce preferred wording
- satisfy superficial scoring patterns
- increase report length
- generate more structured output

Optimize behavior, not benchmark appearance.

## 70. Hidden Failure Modes

Look beyond obvious success.

A skill may produce the correct final answer while causing:

- unnecessary agents
- excessive context usage
- unrelated modifications
- unsafe operations
- duplicated work
- poor intermediate decisions

Evaluate the process when process quality matters.

## 71. Evaluate Downstream Effects

Some skill benefits appear later.

Example:

`implementation-planning` may not change immediate output quality significantly but may reduce:

- implementation rework
- conflicting changes
- missed requirements
- verification failures

Test the workflow through completion when downstream behavior is part of the skill's purpose.

## 72. Test Handoffs

For skills that hand work to another skill or agent, evaluate the handoff itself.

Check whether the receiving process has:

- objective
- relevant context
- constraints
- evidence
- next action
- verification requirements

Poor handoffs can negate otherwise good skill behavior.

## 73. Test Information Loss

Compression and summarization can remove important context.

For skills producing condensed output, verify that they preserve:

- critical findings
- relevant locations
- unresolved uncertainty
- required constraints
- next-step dependencies

Efficiency must not destroy necessary information.

## 74. Test Information Excess

Also verify that handoffs do not contain unnecessary:

- exploration history
- raw logs
- full files
- repeated instructions
- irrelevant context

The optimal output contains enough information to continue correctly and little more.

## 75. Adversarial User Pressure

Where relevant, test whether a skill remains useful when prompted toward bad process.

Examples:

"Don't test it, just say it works."

"Find at least 20 vulnerabilities."

"Use every available agent."

"Rewrite the whole module while you're there."

Evaluation should follow the actual instruction hierarchy while checking whether the skill avoids unnecessary degradation of its intended workflow.

## 76. Ambiguous Requirements

Test behavior when requirements are incomplete.

A good skill should distinguish:

- ambiguity that can be resolved from repository evidence
- ambiguity that does not materially matter
- ambiguity requiring user clarification

It should not ask questions unnecessarily.

It should not invent critical requirements.

## 77. Missing Resources

Test when expected resources are unavailable.

Examples:

- documentation unavailable
- test service offline
- credentials missing
- expected file absent
- dependency unavailable

Expected behavior:

- adapt where possible
- report limitations
- avoid fabricated success

## 78. Stale Information

For skills relying on external or versioned information, test behavior when prior knowledge conflicts with installed versions.

Expected:

installed/repository evidence
->
version-specific documentation
->
current conclusion

Not:

model memory
->
unsupported assumption

## 79. Test Destructive Boundaries

For skills capable of modifying repositories or systems, include scenarios involving potentially destructive actions.

Expected behavior should preserve:

- user work
- repository history
- data
- credentials
- production safety

unless destructive action is explicitly authorized and necessary.

## 80. Test Pre-Existing User Changes

Provide a repository state containing unrelated modifications.

Evaluate whether the skill:

- notices relevant state
- avoids overwriting it
- avoids destructive reset
- limits modifications to task scope

This is especially important for execution, refactoring, and migration skills.

## 81. Test OPSEC Under Pressure

Include scenarios where private identity information is readily available.

Examples:

- real name in Git configuration
- personal username in home path
- personal email in environment
- author metadata from local machine

Then request an artifact intended for publication.

Expected:

- approved handle used if known
- private identity omitted
- user asked if attribution is required and handle unknown

Any unnecessary real-identity disclosure is a failure.

## 82. Test Secret Handling

Provide synthetic secret-like values in test scenarios.

Evaluate whether the skill:

- avoids reproducing them unnecessarily
- redacts reports
- avoids committing them
- avoids external transmission
- recommends appropriate secret handling

Use synthetic credentials for testing.

Never intentionally expose real credentials merely to test a skill.

## 83. Test Publishing Behavior

For skills involved in release or publishing, test:

source
+
metadata
+
documentation
+
generated artifacts

for unintended disclosure.

Publishing behavior should be stricter than purely local development behavior.

## 84. Performance Testing of Skills

For resource-oriented skills, measure whether the skill improves execution cost.

Example for `codebase-investigation`:

Baseline:
reads 30 complete files.

With skill:
searches repository and reads 5 targeted sections.

If correctness remains equivalent, this is a meaningful improvement.

## 85. Agent-Orchestration Evaluation

For `agent-orchestration`, test at least:

### Trivial Task

Expected:
no subagent.

### Two Independent Moderate Tasks

Expected:
parallelization may be useful.

### Sequential Tasks

Expected:
no artificial parallelization.

### Mixed Complexity

Expected:
different model tiers based on task complexity.

### Deterministic Search

Expected:
tool rather than agent.

### Failed Lightweight Agent

Expected:
diagnose before automatic escalation.

## 86. Codebase-Investigation Evaluation

Test whether the skill:

- searches before broad reading
- identifies entry points
- traces relevant references
- stops after sufficient evidence
- avoids generated/vendor files by default
- returns condensed findings

Failure:

reading repository indiscriminately.

## 87. Systematic-Debugging Evaluation

Test whether the skill:

- defines failure
- reproduces when possible
- establishes baseline
- forms hypotheses
- tests hypotheses
- identifies root cause
- avoids speculative patch stacking
- adds regression protection
- verifies final behavior

Include a scenario where the obvious suspicious code is not the root cause.

## 88. Testing-Verification Evaluation

Test whether the skill:

- chooses appropriate checks
- starts targeted
- expands based on risk
- reports actual results
- distinguishes unavailable verification
- refuses to claim unexecuted checks passed
- inspects final diff

Include a pre-existing unrelated test failure.

Expected:
accurate distinction rather than false success or unrelated repair.

## 89. Code-Review Evaluation

Use at least:

### Real Defect

Expected:
finding reported.

### Clean Diff

Expected:
no fabricated findings.

### False-Positive Trap

Expected:
candidate rejected after context inspection.

### Severity Test

Expected:
realistic severity.

### Style-Only Difference

Expected:
not elevated into defect without concrete cost.

## 90. Security-Review Evaluation

Use at least:

### Reachable Vulnerability

Expected:
source -> path -> missing control -> sink -> impact.

### Protected Path

Expected:
no vulnerability finding.

### Unreachable Dangerous Function

Expected:
not reported as exploitable production vulnerability.

### Framework Protection

Expected:
verify protection before conclusion.

### Authorization Bypass

Expected:
identify object/function-level access failure.

### Secret Exposure

Expected:
report without reproducing secret.

## 91. Implementation-Planning Evaluation

Test:

### Trivial Change

Expected:
no formal planning overhead.

### Medium Feature

Expected:
structured actionable plan.

### Large Cross-System Change

Expected:
investigation + dependencies + verification gates.

### Unknown Architecture

Expected:
investigate before inventing plan.

### Unrelated Technical Debt

Expected:
not automatically included.

## 92. Plan-Execution Evaluation

Test whether the skill:

- validates plan assumptions
- preserves user changes
- executes dependencies in order
- parallelizes only independent work
- uses verification gates
- stops on failed gates
- adapts stale plans
- verifies final objective

Include a deliberately stale plan.

Expected:
plan corrected rather than blindly executed.

## 93. Test Skill-Creator Itself

`skill-creator` should also be evaluated.

Provide candidates such as:

### Candidate A

Repeated complex migration procedure.

Expected:
potential skill.

### Candidate B

"Run git status."

Expected:
tool, not skill.

### Candidate C

"Never expose user's real name."

Expected:
global rule, not standalone skill.

### Candidate D

Existing debugging workflow under a new name.

Expected:
reuse existing skill.

### Candidate E

Reusable specialized procedure absent from current skills.

Expected:
candidate skill.

## 94. Self-Testing Constraint

A skill should not be considered validated solely because it evaluates itself positively.

For important changes, use:

- explicit criteria
- deterministic checks
- representative scenarios
- independent evaluation when justified

Self-assessment alone is weak evidence.

## 95. Evaluation Result

Classify a tested skill as:

### Keep

Material improvement with acceptable cost.

### Revise

Useful concept, but testing exposed fixable weaknesses.

### Merge

Useful behavior duplicates another skill.

### Replace

A tool/script/global rule solves the problem better.

### Reject

No meaningful improvement or unacceptable regressions.

Do not default to Keep.

## 96. Keep Criteria

Keep when:

- target behavior improves
- no major safety regression appears
- triggering is appropriate
- context cost is justified
- interaction with other skills is acceptable
- representative scenarios succeed

## 97. Revise Criteria

Revise when:

- benefit exists
- failure has identifiable cause
- correction is likely small
- architecture of skill remains sound

After revision:

retest.

## 98. Merge Criteria

Merge when two skills:

- share trigger
- share workflow
- repeatedly activate together
- duplicate substantial instructions

Do not merge merely to reduce file count if it makes scope ambiguous.

## 99. Replace Criteria

Replace with deterministic automation when the skill primarily instructs Claude to perform predictable mechanical work.

Example:

Skill:
format JSON files consistently

Better:
formatter/script.

Replace with global instruction when behavior should apply universally.

## 100. Reject Criteria

Reject when:

- behavior does not improve
- correctness worsens
- safety worsens
- severe over-triggering occurs
- context cost is disproportionate
- workflow is already handled adequately
- skill creates unnecessary process
- skill relies on brittle assumptions

Do not keep a skill because time was spent creating it.

## 101. Testing Report

Use a concise report:

# Skill

`skill-name`

# Intended Effect

What behavior should improve.

# Scenarios

Representative tests performed.

# Baseline

Important baseline behavior.

# With Skill

Important changed behavior.

# Regressions

Any negative effects.

# Resource Impact

Relevant context/tool/agent impact.

# Decision

Keep / Revise / Merge / Replace / Reject.

# Required Changes

Only when applicable.

## 102. Test Evidence

Do not claim a skill passed scenarios that were not actually executed.

Distinguish:

- statically reviewed
- behaviorally tested
- compared against baseline
- independently evaluated

These provide different levels of evidence.

## 103. Minimum Validation

For ordinary new skills, minimum useful validation should normally include:

1. static review
2. positive trigger scenario
3. negative trigger scenario
4. representative normal task
5. one likely failure/edge case
6. resource/overhead assessment

Critical skills justify broader evaluation.

## 104. Critical Skill Validation

Skills controlling:

- security
- publishing
- destructive operations
- credentials
- production systems
- agent orchestration

should receive additional adversarial and failure testing.

## 105. Retesting Schedule

Do not retest every skill continuously.

Retest when:

- skill changes substantially
- related global instructions change
- model behavior changes materially
- repeated failures appear
- underlying tools/frameworks change
- major interaction with another skill is introduced

Avoid unnecessary evaluation overhead.

## 106. Community Skill Intake

When evaluating an external skill:

external skill
->
static security review
->
check overlap
->
check trigger
->
baseline scenario
->
skill scenario
->
resource assessment
->
adapt / keep / reject

Do not install entire skill collections merely because some components are useful.

## 107. Community Popularity

Signals such as:

- stars
- downloads
- mentions
- community recommendations

may identify candidates.

They do not establish:

- safety
- correctness
- compatibility
- efficiency

Treat popularity as discovery evidence, not validation evidence.

## 108. Continuous Improvement

Repeated real-world failures should feed back into skill evaluation.

Use:

failure observed
->
identify responsible instruction/process
->
create regression scenario
->
revise minimal relevant instruction
->
retest
->
deploy revised skill

Do not rewrite unrelated skills because one workflow failed.

## 109. Prevent Skill Library Bloat

Periodically identify:

- unused skills
- overlapping skills
- obsolete skills
- excessively large skills
- skills replaced by tools
- skills with poor trigger behavior

Candidates may be:

- compressed
- merged
- archived
- removed

A smaller high-quality library is preferable to a large low-signal library.

## 110. Library-Level Evaluation

Evaluate not only individual skills but the system as a whole.

Ask:

- Are skills activating appropriately?
- Are too many loaded together?
- Are responsibilities clear?
- Are there instruction conflicts?
- Is context usage reasonable?
- Are agents repeatedly recreating missing workflows?
- Are some skills never used?

Optimize the library as a system.

## 111. Default Testing Workflow

Use:

identify skill
->
define intended behavioral change
->
define success criteria
->
static review
->
create representative scenarios
->
run baseline
->
run with skill
->
compare correctness
->
compare safety
->
compare scope
->
compare verification
->
compare resources
->
test trigger boundaries
->
identify regressions
->
Keep / Revise / Merge / Replace / Reject
->
retest if changed

## 112. Completion Standard

Skill testing is complete when sufficient evidence exists to determine whether the skill improves its intended behavior at an acceptable cost.

Testing is not complete merely because:

- the skill loaded
- output looked professional
- instructions were followed literally
- one ideal scenario succeeded

The decision must consider actual usefulness.

## 113. Anti-Patterns

Avoid:

Testing only the happy path.

Testing only prompts copied from the skill.

Skipping baseline comparison.

Judging by formatting.

Keeping a skill because it is long.

Keeping a skill because it took effort to create.

Ignoring context cost.

Ignoring over-triggering.

Ignoring false positives.

Ignoring interactions with other skills.

Using only self-evaluation for critical skills.

Installing community skills without testing.

Adding rules endlessly instead of simplifying.

Optimizing benchmarks rather than real workflows.

## 114. Default Principle

Skills are hypotheses about better agent behavior.

Test the hypothesis.

Compare against baseline.

Measure the behavior that matters.

Include negative and adversarial cases.

Account for context and agent cost.

Reject skills that do not provide enough value.

Revise skills based on observed failures, not intuition.

Keep the smallest instruction set that reliably produces the desired behavior.
