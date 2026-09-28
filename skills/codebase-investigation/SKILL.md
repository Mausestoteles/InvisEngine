---
name: codebase-investigation
description: Efficiently investigate repositories, locate relevant code, trace execution paths, understand architecture, and collect evidence while minimizing context usage. Use before substantial modifications, debugging unfamiliar code, architecture analysis, or when the location or behavior of relevant code is not yet known.
---

# Codebase Investigation

Investigate codebases systematically and efficiently.

The objective is to understand the minimum necessary portion of the repository required to complete the current task correctly.

Do not attempt to understand the entire repository unless the task genuinely requires repository-wide analysis.

## 1. Core Principle

Use progressive discovery:

repository structure
->
targeted search
->
identify entry points
->
trace relevant references
->
read targeted code
->
form evidence-based understanding

Do NOT use:

read many files
->
load large amounts of context
->
attempt to understand everything

Context is a limited resource.

Every file and code section loaded should have a reason.

## 2. Start With the Question

Before searching the repository, define what must actually be discovered.

Examples:

- Where is authentication performed?
- Where is this API endpoint implemented?
- What code writes this database record?
- What calls this function?
- Where is this configuration consumed?
- Which component produces this error?
- What code path handles this request?
- Which tests define the expected behavior?

Do not begin with broad repository exploration when a narrower question is available.

## 3. Establish Repository Structure

When unfamiliar with the repository, first obtain a lightweight structural overview.

Identify relevant:

- top-level directories
- source directories
- test directories
- configuration
- package manifests
- build files
- entry points
- documentation
- service boundaries

Do not recursively read every directory or file.

The purpose is orientation, not exhaustive analysis.

## 4. Search Before Reading

Prefer targeted search before opening files.

Useful search targets include:

- function names
- class names
- API routes
- error messages
- configuration keys
- database table names
- environment variables
- component names
- event names
- command names
- imports
- exported symbols
- test descriptions

Preferred process:

search
->
identify candidate files
->
rank relevance
->
read relevant sections

Avoid opening files merely because their names appear potentially related.

## 5. Search Exact Evidence First

When investigating a known symptom, begin with the strongest available identifier.

Example error:

"Invalid session token"

Search:

"Invalid session token"

before searching generic terms such as:

session
authentication
token
user

Exact identifiers reduce unnecessary exploration.

## 6. Use Repository Tools Efficiently

Prefer deterministic repository tools before delegating exploration.

Examples:

File discovery:
-> glob / find

Text search:
-> grep / ripgrep / repository search

Symbol references:
-> language server / IDE references when available

History:
-> git log / git blame when relevant

Changes:
-> git diff

Dependencies:
-> package manifest / lockfile

Do not spawn a subagent merely to execute basic repository searches.

Use `agent-orchestration` when investigation becomes substantial enough to justify delegation.

## 7. Read Targeted Sections

Do not automatically read complete large files.

Prefer:

relevant function

relevant class

relevant configuration section

nearby surrounding context

imports when necessary

related callers/callees

Expand outward only when additional context is required.

Example:

Search identifies:

src/auth/session.ts:240

Prefer reading approximately the relevant function and surrounding code.

Do not automatically load the entire 2,000-line file.

## 8. Expand Context Progressively

Use progressive expansion.

Level 1:
Exact matching code.

Level 2:
Containing function/class/module.

Level 3:
Direct callers and dependencies.

Level 4:
Related subsystem.

Level 5:
Broader architecture.

Move to the next level only when the previous level is insufficient.

## 9. Trace Execution Paths

When behavior depends on control flow, trace the actual execution path.

Example:

HTTP request
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

Do not infer intermediate steps without evidence.

Identify concrete:

- functions
- modules
- interfaces
- calls
- data transformations
- side effects

Record important file and symbol locations.

## 10. Trace Data Flow

For bugs involving incorrect data, trace where the value originates and how it changes.

Use:

source
->
validation
->
transformation
->
storage
->
retrieval
->
consumer

At each stage determine:

- expected type
- actual type
- possible mutation
- validation
- serialization
- default values
- error handling

Do not focus only on the location where incorrect data becomes visible.

The root cause may occur earlier in the data flow.

## 11. Follow References Selectively

When a relevant function is discovered, investigate:

- who calls it
- what it calls
- what data enters it
- what data leaves it
- what side effects it creates

Do not recursively follow every imported function.

Follow only references that can materially affect the current question.

## 12. Identify Existing Patterns

Before implementing a new behavior, search for similar existing behavior.

Examples:

Adding an API endpoint:
-> inspect comparable endpoints

Adding validation:
-> find existing validators

Adding database access:
-> inspect existing repository patterns

Adding UI:
-> inspect comparable components

Adding tests:
-> inspect nearby tests

Prefer established project conventions over inventing a new pattern.

## 13. Investigate Tests Early

Tests often provide concise evidence of intended behavior.

When relevant, inspect:

- tests for the affected component
- tests for similar behavior
- integration tests
- fixtures
- mocks
- snapshots

Use tests to understand:

- expected inputs
- expected outputs
- edge cases
- error behavior
- historical assumptions

Do not assume tests are always correct.

Treat them as evidence of intended behavior, not unquestionable authority.

## 14. Configuration Investigation

For configuration-related behavior, trace the configuration from definition to consumption.

Example:

environment variable
->
configuration loader
->
normalized configuration
->
service initialization
->
runtime usage

Do not assume a configuration value is used merely because it exists.

Determine where it is actually consumed.

## 15. Dependency Investigation

Before assuming external library behavior:

1. Identify the dependency.
2. Determine the installed version.
3. Inspect local types/source when useful.
4. Consult version-appropriate documentation if necessary.

Do not rely solely on remembered API behavior.

Use the `documentation-research` skill when external documentation becomes necessary.

## 16. Git History

Use Git history when it can answer a specific question.

Useful cases:

- Why was unusual code introduced?
- When did behavior change?
- Which change likely introduced a regression?
- Was a compatibility workaround intentional?

Useful tools:

git log
git blame
git show
git diff

Do not inspect large amounts of repository history without a concrete reason.

History is supporting evidence, not a default investigation step.

## 17. Generated and Vendor Files

Avoid spending context on generated or third-party code unless directly relevant.

Normally deprioritize:

- node_modules
- vendor
- build output
- dist
- generated clients
- generated schemas
- caches
- minified files
- lockfile internals

Inspect them only when the task specifically requires it.

## 18. Large Files

For large files:

1. Search within the file.
2. Identify relevant symbols or lines.
3. Read targeted sections.
4. Expand only when necessary.

Do not load a large file in full merely because one relevant function exists inside it.

## 19. Large Repositories

For monorepos or large repositories, determine the relevant subsystem first.

Example:

repository
|
+-- frontend
+-- backend
+-- worker
+-- shared
+-- infrastructure

If the task concerns backend authentication, begin with the backend and relevant shared authentication code.

Do not inspect frontend or infrastructure without evidence that they affect the issue.

## 20. Architecture Mapping

When architecture understanding is necessary, build the smallest useful map.

Example:

API
|
+-- Router
    |
    +-- Auth Middleware
    |
    +-- Controller
        |
        +-- Service
            |
            +-- Repository
                |
                +-- PostgreSQL

Do not produce an exhaustive architecture diagram unless requested.

Map only components relevant to the task.

## 21. Parallel Investigation

For substantial tasks, independent investigation may be parallelized.

Example:

Agent A:
Trace request path.

Agent B:
Investigate persistence layer.

Agent C:
Inspect relevant tests.

Each agent should have a non-overlapping objective.

Use the `agent-orchestration` skill for model selection and delegation decisions.

Do not create multiple agents to search the same repository area without a specific reason.

## 22. Investigation Agents

When delegating investigation, provide a precise question.

Bad:

"Explore the backend."

Preferred:

"Trace POST /api/session from the router to persistence. Identify every function that validates or modifies the session token. Do not modify files. Return file paths, symbols, and a concise execution path."

Investigation agents should normally be read-only.

Do not allow modification unless implementation is explicitly part of their assigned task.

## 23. Investigation Output

A completed investigation should return condensed evidence.

Recommended structure:

### Findings

What was discovered.

### Relevant Locations

File paths and important symbols.

### Execution or Data Flow

Only when relevant.

### Evidence

Important code behavior supporting the findings.

### Uncertainty

Anything that could not be verified.

### Recommended Next Step

Only when useful.

Do not return a diary of every command executed.

## 24. Evidence Levels

Distinguish between:

### Verified

Directly confirmed from code, tests, configuration, command output, or documentation.

### Strongly Indicated

Multiple pieces of evidence support the conclusion, but direct runtime verification has not occurred.

### Hypothesis

Plausible explanation requiring verification.

Never present a hypothesis as a verified fact.

## 25. Avoid Premature Conclusions

Finding code that appears suspicious does not establish root cause.

Before concluding that code causes a bug, determine whether:

- it executes in the failing path
- relevant conditions are satisfied
- data reaches it as expected
- tests or runtime behavior support the conclusion

Use `systematic-debugging` for actual root-cause analysis.

## 26. Avoid Premature Modification

Investigation and implementation are separate activities.

When the task is explicitly investigation-only:

- do not modify code
- do not refactor
- do not fix unrelated issues
- do not update dependencies

Return findings.

When investigation precedes implementation, establish sufficient understanding before editing.

## 27. Avoid Scope Expansion

During investigation, unrelated issues may be discovered.

Do not automatically investigate them deeply.

Record significant unrelated findings when necessary, then remain focused on the requested task.

Example:

Requested:
Investigate login timeout.

Discovered:
Unrelated deprecated API call.

Action:
Note it if relevant.

Do not abandon the login investigation to refactor the deprecated API.

## 28. Context Budget

Continuously evaluate whether additional context is necessary.

Before loading another file, ask internally:

"What question will this file answer?"

If there is no concrete answer, do not load it yet.

Before following another reference:

"Can this reference affect the behavior under investigation?"

If not, stop following it.

## 29. Stop Conditions

Stop investigating when enough evidence exists to proceed safely.

Typical stop condition:

- relevant component identified
- execution path understood
- important dependencies identified
- expected behavior understood
- modification location identified
- major side effects identified
- remaining uncertainty is acceptable

Do not continue exploring merely because more repository content exists.

## 30. OPSEC During Investigation

Repository investigation must respect global OPSEC rules.

Do not unnecessarily reproduce:

- real names
- personal email addresses
- machine usernames
- absolute personal paths
- credentials
- tokens
- secrets
- private infrastructure information

Redact sensitive information from investigation summaries.

Use the user's approved handle when attribution is required.

If the correct public identifier is required but unknown, ask the user.

## 31. Example Investigation

Task:

"Find why expired sessions sometimes remain valid."

Step 1:
Search for session expiration terminology.

Step 2:
Locate session validation entry points.

Step 3:
Identify middleware responsible for session checks.

Step 4:
Trace expiration timestamp from persistence to validation.

Step 5:
Inspect relevant tests.

Step 6:
Trace callers only where necessary.

Step 7:
Compare intended behavior with actual code.

Possible result:

Relevant path:

request
->
auth middleware
->
validateSession()
->
sessionRepository.find()
->
expiration comparison

Relevant files:

src/auth/middleware.ts
src/auth/session.ts
src/db/sessionRepository.ts
tests/auth/session.test.ts

Finding:

Expiration is validated in validateSession(), but one refresh path bypasses validateSession() and reads the session directly from the repository.

At this point, investigation can stop.

Do not continue exploring unrelated authentication modules unless required.

## 32. Anti-Patterns

Avoid:

Reading the entire repository.

Opening files before searching.

Loading complete large files unnecessarily.

Following every import recursively.

Using agents for simple grep operations.

Reading generated files by default.

Investigating unrelated findings.

Assuming library behavior from memory.

Treating suspicious code as proven root cause.

Returning raw exploration history.

Continuing investigation after sufficient evidence exists.

## 33. Investigation Decision Process

Use this sequence:

What must be answered?
->
What is the strongest searchable identifier?
->
Search repository.
->
Which results are most relevant?
->
Read targeted sections.
->
Trace only necessary callers, dependencies, and data.
->
Inspect tests/configuration when relevant.
->
Determine whether evidence is sufficient.
->
If yes: stop and summarize.
->
If no: expand investigation one level.

## 34. Completion Standard

An investigation is complete when it provides enough verified information for the next action without unnecessary repository exploration.

The goal is not maximum repository knowledge.

The goal is minimum sufficient understanding with strong evidence.
