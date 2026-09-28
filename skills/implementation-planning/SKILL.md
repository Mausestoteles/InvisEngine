---
name: implementation-planning
description: Create implementation plans for non-trivial development tasks after sufficient investigation. Scale planning depth to task complexity, identify affected components, dependencies, risks, verification criteria, and parallelizable work without creating unnecessary process overhead. Use before substantial features, refactors, migrations, cross-component changes, or ambiguous implementations. Skip formal planning for trivial changes.
---

# Implementation Planning

Create plans that reduce implementation uncertainty.

Planning is a tool, not an objective.

The purpose of a plan is to make implementation:

- clearer
- safer
- more efficient
- easier to verify
- easier to delegate

Do not create planning bureaucracy for simple work.

## 1. Core Rule

Planning effort must be proportional to task complexity.

Use:

trivial task
-> no formal plan

small task
-> short internal or explicit plan

medium task
-> structured implementation plan

large task
-> investigation + architecture understanding + phased plan

critical task
-> phased plan + risks + verification gates + rollback considerations

Do not spend 30 minutes planning a 5-minute change.

Do not start a multi-system implementation with no plan.

## 2. Process Budget

Treat process overhead as a resource cost.

Planning consumes:

- tokens
- context
- execution time
- user attention
- agent calls

The expected benefit of planning should exceed its cost.

Use the minimum planning depth sufficient to execute safely.

## 3. Classify the Task

Before planning, classify the task.

### Trivial

Examples:

- typo
- simple constant change
- obvious one-line fix
- straightforward formatting
- known configuration value

Action:

Implement directly.

No formal plan.

### Small

Examples:

- isolated function modification
- simple component change
- straightforward test addition
- localized bug with known cause

Action:

Use a short plan.

Usually 2-5 implementation steps.

### Medium

Examples:

- new API endpoint
- moderate feature
- multi-file bug fix
- component refactor
- new integration

Action:

Create a structured plan.

### Large

Examples:

- subsystem feature
- cross-layer implementation
- significant refactor
- database migration
- new service integration
- architecture change

Action:

Investigate first.

Create phased implementation plan.

### Critical

Examples:

- authentication redesign
- authorization architecture
- destructive migration
- payment logic
- high-impact production infrastructure
- security boundary redesign

Action:

Detailed phased plan with explicit risk and verification gates.

## 4. Investigate Before Planning

Do not create detailed implementation plans based on assumptions about an unfamiliar codebase.

When necessary, use `codebase-investigation` first.

Planning should be based on:

- actual repository structure
- existing patterns
- relevant interfaces
- existing tests
- installed dependencies
- real constraints

Bad:

"Create AuthService."

when the repository already has an authentication service with another name and architecture.

Preferred:

"Extend `src/auth/sessionService.ts` because existing login and refresh flows already use this service."

## 5. Resolve Important Unknowns

Before finalizing a plan, identify unknowns that could materially change implementation.

Examples:

- which component owns the behavior
- whether an existing abstraction already exists
- database schema constraints
- external API version
- authentication model
- deployment restrictions

Resolve important unknowns before planning around them.

Do not block planning on irrelevant details.

## 6. Define the Objective

Start with the concrete desired outcome.

Example:

Bad:

"Improve authentication."

Better:

"Add server-side session revocation so logged-out sessions cannot authenticate subsequent API requests."

The objective should describe observable behavior.

## 7. Define Scope

Identify what is included.

Example:

In scope:

- session revocation storage
- logout behavior
- authentication middleware check
- tests

Out of scope:

- redesigning login UI
- changing password hashing
- replacing authentication provider

Explicit scope prevents accidental task expansion.

## 8. Identify Relevant Components

List only components expected to participate in implementation.

Example:

- authentication middleware
- session service
- session repository
- logout endpoint
- authentication tests

Do not list every nearby subsystem.

## 9. Identify Existing Patterns

Before proposing new structures, search for established repository patterns.

Examples:

- similar endpoint
- existing validator
- repository abstraction
- error-handling pattern
- migration convention
- test fixture

Prefer extending established patterns.

Do not invent architecture unnecessarily.

## 10. Plan Concrete Changes

Each plan step should describe a concrete implementation action.

Bad:

"Implement backend."

"Add security."

"Update frontend."

Preferred:

"Extend `SessionRepository` with revocation lookup using the existing repository query pattern."

"Update authentication middleware to reject sessions marked revoked before attaching the user context."

Steps should be executable.

## 11. Include Locations When Known

When repository investigation has identified relevant locations, include them.

Example:

`src/auth/sessionService.ts`
- add revocation check

`src/api/logout.ts`
- persist revocation during logout

`tests/auth/session.test.ts`
- add revoked-session regression test

Do not invent file paths.

If location remains unknown, state the component rather than fabricating a path.

## 12. Order by Dependency

Plan steps according to actual dependencies.

Example:

schema
->
repository
->
service
->
API
->
tests

Do not order steps arbitrarily.

A good plan makes execution dependencies obvious.

## 13. Identify Parallelizable Work

Mark independent workstreams when useful.

Example:

After interface is defined:

Agent A:
backend implementation

Agent B:
frontend integration

Agent C:
test fixtures

Use `agent-orchestration` to decide whether actual parallel execution is worthwhile.

Do not create artificial parallelism merely because tasks can theoretically be separated.

## 14. Separate Sequential Dependencies

If Task B requires Task A's output, make the dependency explicit.

Example:

1. Define database migration.
2. Apply repository model changes based on resulting schema.
3. Update service logic.
4. Add API behavior.
5. Verify integration.

Do not schedule dependent implementation concurrently.

## 15. Verification Is Part of the Plan

Every meaningful implementation plan should include verification.

Do not create:

implementation
->
done

Create:

implementation
->
targeted verification
->
broader verification when justified
->
completion

Use `testing-verification`.

## 16. Define Verification Criteria

Specify what demonstrates success.

Example:

Feature:

Revoked sessions cannot authenticate.

Verification:

- active session still authenticates
- revoked session returns unauthorized
- logout revokes current session
- unrelated authentication tests remain passing
- typecheck passes

Verification criteria should map to requirements.

## 17. Plan Regression Protection

For bug fixes, include regression protection.

Preferred:

1. reproduce defect
2. add failing regression test
3. implement fix
4. verify test passes
5. run related tests

Use `systematic-debugging` when root cause is not established.

## 18. Risk Identification

For medium or larger changes, identify meaningful risks.

Examples:

- backwards compatibility
- data migration
- authorization
- race conditions
- external API dependency
- deployment ordering
- performance
- cache invalidation

Do not produce generic risk lists.

Only include risks connected to the actual change.

## 19. Security Planning

When the change affects a security boundary, explicitly identify it.

Examples:

- authentication
- authorization
- tenant isolation
- secret handling
- external input
- filesystem access

Include appropriate security verification.

Use `security-review` for substantial security-sensitive work.

## 20. Data Migration Planning

For migrations determine:

- schema change
- existing data
- migration order
- compatibility during deployment
- rollback feasibility
- destructive behavior
- verification

Do not treat schema modification as merely editing a model definition.

## 21. Deployment Ordering

For changes spanning multiple deployable components, determine compatibility during rollout.

Example:

Bad sequence:

new frontend
->
expects new API
->
old backend still deployed

Safer sequence may be:

backwards-compatible backend
->
deploy backend
->
deploy frontend
->
remove compatibility later

Plan according to actual deployment architecture.

## 22. Rollback

For high-risk changes consider:

- can code be rolled back?
- can schema be rolled back?
- is data transformation reversible?
- will old code understand new data?
- are feature flags appropriate?

Do not add rollback complexity to trivial changes.

## 23. Feature Flags

Use feature flags when they provide concrete rollout value.

Examples:

- gradual release
- risky behavior
- compatibility transition
- operational kill switch

Do not add feature flags automatically.

Flags create maintenance cost and state complexity.

## 24. Avoid Speculative Architecture

Plan for known requirements.

Do not design elaborate infrastructure for hypothetical future needs.

Avoid:

- unnecessary generic frameworks
- premature plugin systems
- unnecessary interfaces
- speculative extensibility
- abstract factories without current need

Prefer the simplest architecture consistent with known requirements.

## 25. Avoid Refactor Creep

Do not turn feature planning into unrelated cleanup.

If unrelated technical debt is discovered:

- note it if materially relevant
- do not automatically include it

Include refactoring only when required for safe implementation or when explicitly requested.

## 26. Existing Dependencies First

Before planning a new dependency, determine whether:

- existing project dependency already solves it
- standard library solves it
- direct implementation is simpler

New dependencies should have a concrete justification.

## 27. Documentation Research

If implementation depends on external API or library behavior that is not verified:

- identify installed version
- consult appropriate documentation
- resolve important API assumptions

Use `documentation-research`.

Do not build plans around remembered APIs when verification is practical.

## 28. Testability

Prefer implementation approaches that can be verified.

If two approaches satisfy the requirement similarly, prefer the one with:

- observable behavior
- clear interfaces
- deterministic testing
- limited hidden state

Do not distort architecture solely for tests, but treat testability as a design property.

## 29. Define Completion

A plan should establish when implementation stops.

Example:

Complete when:

- requested endpoint exists
- authorization enforced
- persistence works
- regression tests pass
- typecheck passes
- API contract verified

Without completion criteria, implementation can expand indefinitely.

## 30. Plan Granularity

Plan steps should be large enough to represent meaningful work and small enough to execute independently.

Bad:

1. Write line 20.
2. Add variable.
3. Add if statement.

Also bad:

1. Implement entire system.

Preferred:

1. Add persistence support for revoked sessions.
2. Enforce revocation in authentication middleware.
3. Update logout flow.
4. Add regression tests.
5. Run verification.

## 31. Do Not Duplicate the Code

A plan is not a full implementation written in prose.

Do not specify every line of code unless exact algorithmic detail is necessary.

Preserve implementation flexibility.

## 32. Plans Are Not Immutable

If implementation reveals that an assumption is wrong:

stop
->
inspect evidence
->
update plan
->
continue

Do not blindly follow a plan contradicted by the repository.

## 33. Plan Validation

Before execution, check:

- objective matches user request
- scope is correct
- assumptions are supported
- relevant components identified
- dependencies ordered
- verification included
- unnecessary work excluded

For substantial plans, this validation can prevent expensive implementation mistakes.

## 34. Planning With Agents

Use subagents for planning only when independent investigation provides meaningful value.

Example:

Large feature:

Agent A:
investigate persistence

Agent B:
investigate API architecture

Agent C:
investigate frontend integration

Primary agent:
synthesize plan

Use `agent-orchestration`.

Do not spawn planning agents for simple changes.

## 35. Model Selection

Small plan:
-> primary agent

Normal cross-file plan:
-> capable mid-tier model

Complex architecture plan:
-> high-tier model when reasoning complexity justifies it

Do not automatically use the strongest model for every plan.

## 36. Plan Handoff

Plans intended for another agent or future session must contain enough context to execute without reconstructing the entire investigation.

Include:

- objective
- relevant locations
- important constraints
- ordered steps
- verification criteria
- unresolved assumptions

Do not include unnecessary exploration history.

## 37. OPSEC

Plans must respect global OPSEC requirements.

Do not include unnecessary:

- real names
- personal email addresses
- local usernames
- absolute personal paths
- secrets
- credentials
- internal sensitive infrastructure

Use neutral placeholders or the user's approved handle where attribution is required.

## 38. Planning Output Format

For medium and larger tasks prefer:

# Objective

Observable desired outcome.

# Scope

Included and excluded work.

# Current Architecture

Only relevant existing structure.

# Implementation Plan

1. Concrete step.
2. Concrete step.
3. Concrete step.

# Dependencies

Important ordering or external dependencies.

# Risks

Only meaningful task-specific risks.

# Verification

Concrete success checks.

# Open Questions

Only unresolved questions that materially affect implementation.

Do not create empty sections merely to follow the template.

## 39. Small Task Output

For small tasks, avoid the full template.

Example:

Plan:

1. Update session expiration check in `sessionService`.
2. Add regression test for exact expiration boundary.
3. Run targeted auth tests and typecheck.

That is sufficient.

## 40. Critical Task Output

For critical work, add:

- security boundaries
- migration strategy
- deployment sequence
- rollback considerations
- compatibility requirements
- verification gates

Use additional detail only because risk justifies it.

## 41. Implementation Phases

Large plans may be divided into phases.

Example:

Phase 1:
Introduce backwards-compatible data model.

Gate:
Migration and existing tests pass.

Phase 2:
Implement new behavior.

Gate:
Targeted integration tests pass.

Phase 3:
Enable new behavior.

Gate:
Operational verification.

Each phase should leave the system in a coherent state where practical.

## 42. Verification Gates

For high-risk plans, define gates that must pass before continuing.

Example:

Migration
->
schema verification

then:

backend
->
integration tests

then:

frontend
->
end-to-end verification

Do not continue through failed critical gates.

## 43. Minimize Work in Progress

Prefer completing coherent slices over starting many partially dependent modifications simultaneously.

A smaller amount of completed, verified work is easier to reason about than many unfinished changes.

## 44. Vertical Slices

When appropriate, plan vertical slices that produce testable behavior.

Example:

Instead of:

all database work
->
all backend work
->
all frontend work

consider:

minimal end-to-end behavior
->
verify
->
expand behavior
->
verify

Use whichever structure best matches system dependencies.

## 45. Stop Conditions

Stop planning when:

- objective is clear
- relevant architecture is understood
- important unknowns are resolved
- implementation steps are actionable
- dependencies are identified
- verification is defined

Do not continue refining the plan merely to make it longer.

## 46. Planning Failure Signals

Planning should pause and return to investigation when:

- relevant component cannot be identified
- major architecture assumptions remain unsupported
- external API behavior is unknown and critical
- requirements conflict
- implementation depends on unknown data constraints

Do not compensate for missing information with invented details.

## 47. Anti-Patterns

Avoid:

Formal plans for trivial edits.

Planning before repository investigation.

Inventing file paths.

Inventing APIs.

Huge speculative architecture.

Unrelated refactoring.

Generic risk lists.

Generic security checklists.

Plans without verification.

Plans without completion criteria.

Parallelizing dependent steps.

Adding dependencies without justification.

Writing implementation code disguised as a plan.

Following outdated plans despite contradictory evidence.

Using high-tier models for trivial planning.

## 48. Decision Process

Use:

What is the requested outcome?
->
How complex/risky is the task?
->
Is formal planning justified?
    |
    no -> implement directly
    |
    yes
    v
Is repository understanding sufficient?
    |
    no -> investigate
    |
    yes
    v
Resolve material unknowns
->
identify affected components
->
identify dependencies
->
define concrete implementation steps
->
identify meaningful risks
->
define verification criteria
->
remove unnecessary work
->
execute

## 49. Completion Standard

A plan is complete when another capable agent could execute it without guessing about major implementation decisions while still retaining reasonable implementation flexibility.

The plan should contain enough information to prevent avoidable mistakes.

It should not contain enough unnecessary detail to become implementation itself.

## 50. Default Principle

Investigate before planning.

Scale process to complexity.

Resolve important uncertainty before implementation.

Plan concrete changes, not vague intentions.

Order work by dependency.

Parallelize only independent work.

Include verification from the beginning.

Avoid speculative architecture.

Stop planning when the plan is sufficient.
