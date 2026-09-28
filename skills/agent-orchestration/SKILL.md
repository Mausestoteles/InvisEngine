---
name: agent-orchestration
description: Efficiently orchestrate subagents, model selection, parallel work, context usage, and escalation. Use for substantial tasks where delegation or multiple independent workstreams may provide a concrete benefit.
---

# Agent Orchestration

Coordinate agents and model resources efficiently.

The objective is not to maximize agent usage.

The objective is to complete the task correctly using the minimum necessary combination of:

- model capability
- agents
- context
- tokens
- tool calls
- execution time

## 1. Primary Principle

The model running the primary agent does NOT determine which model subagents should use.

Never automatically assign the primary agent's model to every subagent.

Select the lowest-cost and lowest-capability model that can reliably complete each delegated task.

Example:

Primary agent:
Fable 5.1

Possible allocation:

- architecture analysis -> Fable 5.1
- normal implementation -> Opus 5.5
- code investigation -> Opus 5.5
- file discovery -> Opus 5
- simple validation -> Opus 5

Do NOT automatically allocate:

- Primary -> Fable 5.1
- Agent A -> Fable 5.1
- Agent B -> Fable 5.1
- Agent C -> Fable 5.1
- Agent D -> Fable 5.1

unless each task independently requires Fable 5.1-level capability.

Use exactly these models for each tier:

trivial -> Opus 5

normal -> Opus 5.5

complex -> Fable 5.1

If Opus 5 cannot be selected explicitly (for example, when a subagent tool only accepts the alias `opus`), use Opus 5.5 instead.

## 2. Decide Whether Delegation Is Necessary

Before spawning any subagent, evaluate:

1. Can a deterministic tool solve the task?
2. Can the primary agent complete it efficiently?
3. Is the task substantial enough to justify agent overhead?
4. Can the delegated work proceed independently?
5. Does isolated context provide a concrete benefit?
6. Does parallel execution materially reduce execution time?

If delegation provides no meaningful benefit, work directly.

## 3. Deterministic Tools Before Agents

Prefer deterministic tools for deterministic questions.

Examples:

File discovery:
-> find / glob / repository search

Text occurrence:
-> grep / search

Compilation:
-> compiler

Type correctness:
-> type checker

Formatting:
-> formatter

Known test behavior:
-> test runner

Repository state:
-> git

Structured data:
-> parser / query

Do not spawn an LLM agent merely to perform work that a deterministic tool can complete reliably and cheaply.

Bad:

Spawn Opus 5.5 agent
-> ask it to locate occurrences of a function

Preferred:

grep/search
-> obtain exact locations directly

## 4. Delegation Decision

Use subagents when one or more of the following applies:

- multiple independent workstreams exist
- work can run meaningfully in parallel
- isolated context prevents pollution of the primary context
- separate components require substantial investigation
- specialized reasoning is required
- a substantial bounded implementation can be delegated independently
- independent verification provides meaningful value

Do not use subagents for:

- trivial tasks
- simple searches
- basic file discovery
- one or two straightforward tool calls
- small single-file changes
- tightly sequential operations
- tasks requiring continuous shared context
- work where agent startup/context cost exceeds the task itself

## 5. Model Selection

For every delegated task, classify complexity before selecting a model.

### Trivial

Examples:

- locate files
- inspect simple configuration
- extract information
- formatting
- straightforward validation
- repetitive mechanical modifications
- simple documentation lookup
- basic repository searches

Default:

Opus 5.

### Normal

Examples:

- normal code implementation
- repository investigation
- debugging
- writing tests
- API integration
- moderate refactoring
- code analysis
- documentation research

Default:

Opus 5.5.

### Complex

Examples:

- architecture decisions
- difficult cross-system debugging
- ambiguous system behavior
- security-critical reasoning
- complex concurrency problems
- major migrations
- difficult multi-component design
- synthesis across large independent systems

Default:

Fable 5.1.

## 6. Escalation

Start with the lowest model tier reasonably capable of completing the task.

Preferred escalation:

Opus 5
->
Opus 5.5
->
Fable 5.1

Escalate only when justified by evidence.

Valid escalation reasons include:

- insufficient reasoning capability
- repeated inability to resolve the problem
- unexpected architectural complexity
- unresolved ambiguity requiring deeper reasoning
- security-critical analysis
- substantial cross-system dependencies

Do not escalate merely because a stronger model is available.

Do not escalate merely because the primary agent uses a stronger model.

## 7. Agent Budget

Use the minimum number of agents necessary.

Before spawning another agent, determine:

- what unique work it will perform
- whether another agent already covers that work
- whether it can operate independently
- whether the expected benefit exceeds the additional resource cost

Prefer:

2-3 well-scoped agents

over:

8-10 fragmented agents with overlapping responsibilities.

Do not create artificial parallelism.

## 8. Parallelization

Parallelize only independent work.

Good:

Agent A -> investigate database layer

Agent B -> investigate API layer

Agent C -> investigate frontend integration

Bad:

Agent A -> determine root cause

Agent B -> implement fix that depends on Agent A

Agent C -> test fix that depends on Agent B

Dependent operations should execute sequentially.

## 9. Avoid Duplicate Investigation

Do not assign substantially identical investigations to multiple agents unless independent verification is specifically valuable.

Before delegating, check whether:

- the primary agent already obtained the information
- another subagent is already investigating it
- existing tool output answers the question
- repository search can answer it directly

Avoid:

Agent A -> investigate authentication bug

Agent B -> investigate authentication bug

Agent C -> investigate authentication bug

Prefer:

Agent A -> authentication middleware

Agent B -> token persistence

Agent C -> frontend authentication flow

## 10. Define Agent Scope

Every subagent must receive a bounded task.

Specify:

- objective
- relevant scope
- files/components when known
- whether modification is allowed
- expected output
- completion criteria
- relevant constraints

Bad:

"Investigate the repository."

Preferred:

"Inspect the authentication middleware and determine where expired JWT tokens are rejected. Do not modify files. Return relevant files, functions, execution path, and supporting evidence."

## 11. Context Efficiency

Context is a limited resource.

Do not provide every subagent with the entire primary context.

Provide only information relevant to its assigned task.

Likewise, subagents should not return their complete exploration history.

Return condensed results containing:

- findings
- evidence
- relevant file locations
- changes made
- verification performed
- unresolved issues
- information required by the primary agent

Avoid dumping large files, logs, or command output into the primary context when a concise summary is sufficient.

## 12. Recursive Delegation

Subagents must follow the same resource rules.

A subagent must not automatically spawn additional agents.

Before recursively delegating, it must determine:

- why another agent is necessary
- what unique work it will perform
- why direct execution is insufficient
- minimum required model capability

Avoid deep agent trees.

Prefer shallow orchestration unless complexity genuinely requires recursive delegation.

## 13. Verification Agents

Do not automatically create a separate verification agent for every task.

For ordinary changes, the implementing agent or primary agent should normally run appropriate deterministic verification:

- tests
- build
- typecheck
- lint
- static analysis

Use an independent verification/review agent when the risk or complexity justifies the additional cost.

Examples:

- security-sensitive changes
- major architecture changes
- large migrations
- complex concurrency changes
- high-impact production code
- substantial cross-component modifications

## 14. Failure Handling

Do not respond to agent failure by immediately retrying with a stronger model.

First determine why the task failed.

Possible causes:

- unclear instructions
- insufficient context
- incorrect scope
- missing files
- unavailable tools
- incorrect assumptions
- task dependency not completed
- insufficient model capability

Fix the actual cause.

Escalate model capability only when capability is the limiting factor.

## 15. Agent Result Validation

Subagent output is not automatically correct.

The primary agent remains responsible for integrating and validating delegated work.

Verify important claims against:

- repository contents
- tool output
- tests
- documentation
- other direct evidence

Do not treat agent confidence as evidence.

## 16. Skills and Agents

Provide subagents only with skills relevant to their assigned task.

Examples:

Bug investigation:
-> systematic-debugging

Repository exploration:
-> codebase-investigation

Security analysis:
-> security-review

Verification:
-> testing-verification

Performance investigation:
-> performance-analysis

Do not load every available skill into every subagent.

## 17. OPSEC

All agents inherit global OPSEC and identity-protection rules.

Never expose the user's real identity through:

- prompts
- agent context
- generated code
- comments
- paths
- documentation
- logs
- metadata
- commits
- publishing

Use the user's approved tag or handle when an identifier is required.

If the approved public identifier is unknown and required, ask the user.

Do not infer it from local system information.

## 18. Example Orchestration

Task:

"Investigate and fix a performance regression affecting API requests."

Possible execution:

Primary Fable 5.1
|
+-- direct repository search
|
+-- Agent A - Opus 5.5
|   Investigate API request path
|
+-- Agent B - Opus 5.5
|   Investigate database queries
|
+-- Agent C - Opus 5
    Collect existing benchmark/test locations

Primary receives condensed findings.

If database behavior is identified as the likely root cause:

Primary
|
+-- Opus 5.5
    Implement targeted fix

Then deterministic verification:

tests
->
benchmark
->
typecheck
->
diff inspection

Do not create additional agents unless new independent work appears.

## 19. Anti-Patterns

Avoid:

High-tier model everywhere.

Agent for every tool call.

Agent for grep.

Agent for reading one file.

Multiple agents solving the same problem.

Large recursive agent trees.

Passing entire context to every agent.

Returning entire exploration histories.

Escalating after one minor failure.

Parallelizing dependent tasks.

Using agents because they are available rather than because they are useful.

## 20. Final Decision Rule

For every unit of work, prefer this decision sequence:

Can a deterministic tool solve it?
-> use the tool.

Otherwise, can the primary agent efficiently solve it?
-> primary agent handles it.

Otherwise, would delegation provide concrete value?
-> delegate.

If delegating:

What is the minimum capable model?
-> use that model.

Can independent work run in parallel?
-> parallelize only those parts.

When results return:
-> validate and integrate.

The strongest model is an escalation resource, not the default worker.

Agents are an optimization mechanism, not a default execution strategy.
