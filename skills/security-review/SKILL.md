---
name: security-review
description: Perform evidence-based security reviews of code, diffs, APIs, authentication, authorization, data flows, filesystem access, network boundaries, dependencies, configuration, and sensitive information handling. Use for security-sensitive changes, explicit security reviews, authentication or authorization changes, external input handling, secrets, uploads, database access, and high-impact code. Prioritize reachable and realistically exploitable findings over speculative vulnerability lists.
---

# Security Review

Perform security analysis based on concrete attack paths.

The objective is not to maximize the number of vulnerabilities reported.

The objective is to identify security weaknesses that are:

- reachable
- realistic
- evidence-backed
- materially impactful
- actionable

Prefer one demonstrated vulnerability over twenty theoretical possibilities.

## 1. Core Rule

A security finding should normally establish:

source
->
data/control flow
->
trust boundary
->
missing or insufficient control
->
sensitive sink or privileged operation
->
realistic impact

Do not report a vulnerability merely because a dangerous function or pattern exists.

Determine whether an attacker can actually reach and influence it.

## 2. Establish Security Scope

Determine what is being reviewed.

Possible scope:

- diff
- pull request
- authentication flow
- authorization logic
- API
- upload functionality
- database access
- filesystem operations
- external network access
- configuration
- secrets handling
- dependency change
- complete subsystem

Do not silently expand a targeted review into an entire repository audit.

## 3. Understand the System First

Before evaluating security, identify relevant:

- entry points
- trust boundaries
- privileged operations
- authentication mechanisms
- authorization mechanisms
- sensitive data
- external services
- persistence
- filesystem access
- network access

Use `codebase-investigation` when the relevant execution path is unclear.

## 4. Threat Model

For substantial reviews, establish a lightweight threat model.

Identify:

### Assets

What requires protection?

Examples:

- accounts
- credentials
- sessions
- personal data
- database records
- administrative operations
- filesystem
- infrastructure
- secrets

### Actors

Who may interact with the system?

Examples:

- unauthenticated user
- authenticated user
- privileged user
- administrator
- external service
- compromised dependency

### Trust Boundaries

Where does data cross between different levels of trust?

Examples:

browser
->
API

API
->
database

application
->
filesystem

application
->
external URL

user-controlled repository
->
build process

Do not build an elaborate threat model when the reviewed change is trivial.

## 5. Trace Sources to Sinks

For input-related vulnerabilities, trace the complete path.

Example:

HTTP parameter
->
request parser
->
validation
->
service
->
query construction
->
database

Determine whether attacker-controlled data reaches a sensitive operation.

Do not stop at:

"User input exists."

Do not stop at:

"SQL execution exists."

Establish whether the input can influence the SQL operation unsafely.

## 6. Reachability

Before reporting a vulnerability, determine whether the vulnerable path is reachable.

Check:

- route exposure
- authentication
- authorization
- feature flags
- environment conditions
- caller restrictions
- validation
- framework protections

Unreachable code is not equivalent to an exploitable production vulnerability.

It may still deserve mention if likely to become reachable, but classify the uncertainty accurately.

## 7. Authentication

Review authentication for:

- credential validation
- session generation
- session expiration
- token validation
- token rotation
- logout invalidation
- replay protection
- password reset
- account recovery
- multi-factor boundaries when applicable

Do not assume authentication exists merely because a user ID is present.

Trace how identity was established.

## 8. Authorization

Authentication and authorization are different.

For privileged operations determine:

1. Who is authenticated?
2. What resource is being accessed?
3. What permission is required?
4. Where is that permission checked?
5. Can the resource identifier be attacker-controlled?

Look for:

- IDOR
- horizontal privilege escalation
- vertical privilege escalation
- missing ownership checks
- role bypass
- authorization performed only in UI
- inconsistent authorization between endpoints

Authorization findings should identify the exact privileged operation that becomes accessible.

## 9. Input Validation

Inspect untrusted input from:

- HTTP requests
- forms
- headers
- cookies
- files
- URLs
- webhooks
- queues
- databases containing user-controlled data
- environment-controlled integrations
- external APIs

Determine:

- expected type
- allowed format
- size limits
- semantic constraints
- canonicalization
- validation location

Validation should occur at an appropriate trust boundary.

## 10. SQL Injection

Do not report SQL injection merely because SQL exists.

Trace:

attacker input
->
query construction
->
database execution

Determine whether:

- parameterized queries are used
- query identifiers are dynamically constructed
- raw SQL is involved
- ORM escape guarantees apply
- validation constrains dynamic values

Report only when attacker influence over executable query structure is realistically established.

## 11. Command Injection

Trace attacker-controlled data into:

- shell commands
- process execution
- scripts
- interpreters

Prefer APIs that pass arguments without shell interpretation.

Check whether:

- shell mode is enabled
- concatenation occurs
- escaping is incomplete
- executable path is attacker-controlled
- environment variables influence execution

Do not assume every subprocess call is command injection.

## 12. Cross-Site Scripting

For web applications trace:

untrusted content
->
rendering
->
HTML/DOM sink

Determine whether:

- framework escaping applies
- raw HTML APIs are used
- sanitization exists
- URL schemes are constrained
- stored content becomes executable

Differentiate:

- reflected XSS
- stored XSS
- DOM XSS

Do not report framework-default escaped interpolation as XSS without evidence of bypass.

## 13. SSRF

For server-side requests determine whether attacker-controlled input can influence:

- hostname
- protocol
- port
- path
- redirects

Check access to:

- localhost
- private networks
- metadata endpoints
- internal services
- alternate IP representations

Do not report SSRF merely because the application fetches external URLs.

Establish attacker control and reachable sensitive destinations.

## 14. Filesystem Security

Review attacker-controlled paths for:

- path traversal
- arbitrary file read
- arbitrary file write
- unsafe extraction
- symlink attacks
- overwrite behavior
- temporary file handling

Trace:

user-controlled path
->
normalization
->
base-directory enforcement
->
filesystem operation

String-prefix checks alone may be insufficient depending on canonicalization behavior.

## 15. File Uploads

For uploads inspect:

- size limits
- file type validation
- filename handling
- storage location
- execution possibility
- public accessibility
- overwrite behavior
- archive extraction
- metadata processing

Do not trust MIME type or filename extension alone when security depends on actual content.

## 16. Deserialization

Review deserialization of untrusted data.

Look for:

- object instantiation
- executable hooks
- unsafe binary formats
- polymorphic deserialization
- dynamic type resolution

Prefer data-only formats and constrained schemas.

Do not report ordinary JSON parsing as unsafe deserialization without a dangerous behavior.

## 17. Cryptography

For cryptographic code inspect:

- established algorithms
- key generation
- nonce/IV requirements
- randomness
- key storage
- password hashing
- comparison behavior
- transport assumptions

Do not recommend custom cryptography.

Do not claim cryptographic weakness without understanding the primitive and usage.

When uncertainty is material, consult authoritative documentation.

## 18. Password Handling

Check for:

- plaintext storage
- reversible encryption used as password storage
- weak password hashing
- missing salts where required
- inappropriate work factors
- password exposure in logs
- password inclusion in URLs

Prefer established password-hashing mechanisms.

## 19. Session Security

Inspect:

- entropy
- expiration
- invalidation
- rotation
- cookie attributes
- session fixation
- replay behavior
- server-side revocation

For cookies consider relevant:

- Secure
- HttpOnly
- SameSite

Apply according to application architecture rather than blindly requiring every attribute in every environment.

## 20. CSRF

For state-changing browser requests determine:

- authentication mechanism
- cookie behavior
- SameSite configuration
- CSRF token usage
- origin checks
- framework protections

Do not report CSRF for APIs that cannot be authenticated through ambient browser credentials without establishing an attack path.

## 21. CORS

Do not treat permissive CORS as automatically equivalent to authentication bypass.

Determine:

- allowed origins
- credentials mode
- exposed data
- authentication mechanism
- browser enforcement

Report concrete consequences.

## 22. Secrets

Search relevant changed code and configuration for:

- API keys
- passwords
- tokens
- private keys
- connection strings
- credentials

Do not reproduce discovered secrets in review output.

Redact them.

Example:

Bad:
`API_KEY=sk-actual-secret`

Preferred:
`API_KEY=<redacted>`

If a secret appears committed, report its location and recommend rotation when exposure is plausible.

## 23. Logging

Check whether logs expose:

- passwords
- tokens
- session identifiers
- private keys
- authorization headers
- sensitive personal data
- internal security details

Logs may leave the immediate application boundary.

Treat logging as a potential disclosure path.

## 24. Error Messages

Determine whether externally visible errors expose:

- stack traces
- filesystem paths
- credentials
- database details
- internal service names
- sensitive configuration

Do not suppress useful internal diagnostics unnecessarily.

Separate internal logging from external responses.

## 25. OPSEC and Identity Protection

Never expose the user's real-world identity during security review.

Do not reproduce or publish:

- real name
- personal email
- operating-system username
- machine name
- personal filesystem paths
- account identifiers
- workstation metadata

Use the user's approved tag or handle when attribution is required.

If the approved public identifier is unknown and one is required, ask the user.

Do not infer a handle from private information.

## 26. Sensitive Repository Metadata

Inspect public-facing changes for accidental disclosure through:

- README
- package metadata
- author fields
- comments
- example paths
- screenshots
- generated files
- build artifacts
- configuration
- deployment metadata

Use neutral placeholders when personal data is unnecessary.

## 27. Environment Variables

Environment variables are not automatically secure merely because they are environment variables.

Check:

- whether secrets reach client bundles
- whether values are logged
- whether defaults contain credentials
- whether public/private prefixes are correct
- whether configuration is committed

Do not print secret environment values during review.

## 28. Network Boundaries

Review outbound and inbound communication for:

- TLS assumptions
- certificate validation
- untrusted destinations
- internal service exposure
- webhook validation
- replay
- request signing

Security depends on actual trust boundaries, not merely the presence of HTTP requests.

## 29. Webhooks

For webhook consumers consider:

- signature verification
- timestamp validation
- replay protection
- raw body requirements
- secret handling
- idempotency

Do not trust source IP alone unless architecture explicitly guarantees it.

## 30. Dependencies

Review dependency changes for:

- unnecessary additions
- abandoned packages
- suspicious package names
- typosquatting
- unexpected install scripts
- version changes with security implications

Use authoritative package and security sources when external research is necessary.

Do not claim a dependency vulnerability from memory.

Verify version applicability.

## 31. Supply Chain

For build and CI changes inspect:

- unpinned external actions
- downloaded executables
- remote scripts
- package install hooks
- artifact provenance
- credential exposure
- excessive CI permissions

Treat code executed during build as privileged.

## 32. CI/CD Security

Check for:

- secrets exposed to untrusted code
- excessive repository permissions
- unsafe pull-request execution
- shell injection from branch names or inputs
- untrusted artifacts
- publishing credentials
- dangerous deployment triggers

CI often operates with more privileges than application code.

## 33. Database Security

Inspect:

- query parameterization
- authorization before data access
- tenant isolation
- destructive operations
- excessive privileges
- sensitive field exposure
- transaction boundaries

Do not confuse database-level access with application-level authorization.

## 34. Multi-Tenant Systems

For multi-tenant applications verify tenant boundaries.

Trace:

authenticated identity
->
tenant identity
->
resource query
->
authorization constraint

Look for queries that accept a resource ID without enforcing tenant ownership.

Treat cross-tenant access as high-impact.

## 35. Race Conditions and TOCTOU

Security checks followed by privileged operations may be vulnerable if relevant state can change between them.

Inspect:

check
->
time gap
->
use

particularly for:

- filesystem operations
- permissions
- resource ownership
- balances
- quotas
- one-time tokens

Do not report TOCTOU without a plausible concurrent state change.

## 36. Resource Exhaustion

Consider realistic denial-of-service paths:

- unbounded uploads
- unbounded loops
- recursive parsing
- uncontrolled concurrency
- expensive queries
- attacker-controlled memory allocation

Do not report every potentially expensive operation as DoS.

Establish attacker control and meaningful resource amplification.

## 37. Unsafe Defaults

Check whether security-sensitive functionality defaults to:

- open access
- disabled verification
- broad permissions
- public exposure
- insecure transport

A secure default is preferable when practical.

Distinguish development-only defaults from production behavior.

## 38. Fail Open vs Fail Closed

For security controls determine behavior when:

- dependency fails
- configuration is missing
- verification throws
- identity service is unavailable
- policy cannot be loaded

Security-critical checks should generally not silently convert failures into authorization.

## 39. Security Tests

Look for relevant tests covering:

- unauthorized access
- privilege boundaries
- invalid credentials
- expired sessions
- malformed input
- tenant isolation
- dangerous paths

Missing tests are not automatically vulnerabilities.

Report missing tests as verification gaps when they materially affect confidence.

## 40. Validate Framework Assumptions

Modern frameworks often provide security controls automatically.

Before reporting a vulnerability, verify whether the framework already provides:

- escaping
- CSRF protection
- query parameterization
- cookie handling
- path normalization
- request validation

Do not assume protection exists.

Do not assume it does not exist.

Verify.

## 41. External Documentation

When security depends on library/framework behavior:

1. determine installed version
2. consult authoritative version-appropriate documentation
3. verify actual project configuration
4. apply conclusions to the concrete code path

Use `documentation-research` where appropriate.

Do not base important findings solely on remembered framework behavior.

## 42. Finding Evidence Standard

A strong finding contains:

### Source

Where attacker influence originates.

### Path

How attacker influence reaches the vulnerable operation.

### Missing Control

Which validation, authorization, isolation, or security boundary fails.

### Sink

Which sensitive operation is reached.

### Trigger

What the attacker must do.

### Impact

What capability or data is compromised.

If these cannot be established, investigate further before reporting.

## 43. Exploitability

Distinguish:

### Demonstrated

The path has been reproduced or directly established.

### Strongly Established

Code flow clearly supports exploitation under realistic conditions.

### Conditional

Requires a configuration or environmental assumption.

### Theoretical

No realistic attack path has been established.

Prioritize Demonstrated and Strongly Established findings.

Report Conditional findings when the condition is realistic and important.

Normally omit purely Theoretical findings.

## 44. Severity

Assign severity based on:

impact
x
realistic exploitability
x
affected scope

### Critical

Examples may include:

- unauthenticated remote code execution
- widespread irreversible sensitive data compromise
- direct compromise of critical infrastructure credentials

Use rarely.

### High

Examples may include:

- authentication bypass
- meaningful authorization bypass
- cross-tenant data access
- exploitable SQL injection
- arbitrary server-side file write
- major secret exposure

### Medium

Examples may include:

- constrained privilege escalation
- meaningful information disclosure
- realistic stored XSS
- limited SSRF
- exploitable security misconfiguration

### Low

Examples may include:

- limited information leakage
- defense-in-depth weakness
- low-impact security misconfiguration

Do not inflate severity.

## 45. False Positive Suppression

Before reporting, ask:

- Is the input attacker-controlled?
- Is the path reachable?
- Is authentication required?
- Is authorization already enforced elsewhere?
- Does validation constrain the value?
- Does the framework escape or parameterize it?
- Can the sensitive sink actually be influenced?
- Does the attacker gain a meaningful capability?
- Is the configuration required for exploitation actually used?

If these questions invalidate the attack path, discard the finding.

## 46. Do Not Produce Checklist Findings

Security checklists guide investigation.

They are not findings.

Bad:

"No rate limiting visible."

"Uses cookies."

"Accepts file uploads."

"Calls an external URL."

These facts are not vulnerabilities by themselves.

Establish security impact first.

## 47. Safe Validation

When validating a suspected vulnerability, prefer non-destructive methods.

Do not:

- damage data
- access unrelated user information
- expose secrets
- disrupt production
- create persistence
- exceed authorized scope

Use the minimum validation necessary to establish the issue.

## 48. Production Systems

Exercise additional caution when reviewing or validating against production systems.

Prefer static evidence, existing tests, staging, or safe reproduction.

Do not perform intrusive security testing against external systems without explicit authorization.

## 49. Review-Only Mode

If asked only to review:

- do not modify source
- do not rotate credentials
- do not change infrastructure
- do not exploit beyond safe validation

Return findings.

If asked to review and fix:

review
->
verify finding
->
implement minimal remediation
->
run security-relevant tests
->
run normal verification

Use `testing-verification`.

## 50. Parallel Security Review

For large systems, review may be divided by independent attack surface.

Example:

Agent A:
authentication and authorization

Agent B:
input and injection paths

Agent C:
filesystem/network boundaries

Agent D:
secrets and deployment

Use `agent-orchestration`.

Do not create multiple high-tier security agents for a small change.

## 51. Model Selection

Small security-sensitive diff:
-> mid-tier model may be sufficient

Normal security review:
-> capable mid-tier model

Complex cross-system security architecture:
-> high-tier model when justified

Use stronger models based on reasoning complexity, not merely because the task is labeled "security."

Follow `agent-orchestration`.

## 52. Independent Security Review

For high-impact changes, an independent security reviewer may be useful.

Examples:

- authentication redesign
- authorization changes
- cryptographic changes
- payment systems
- multi-tenant isolation
- secret-management changes
- CI/CD privilege changes
- externally exposed parsers

The independent reviewer should receive:

- intended security requirements
- relevant architecture
- resulting diff
- necessary repository context

Avoid anchoring the reviewer on the implementer's conclusions.

## 53. Verify Other Agents' Findings

Security findings produced by subagents are candidate findings, not established facts.

The primary agent must:

1. deduplicate findings
2. inspect supporting code
3. verify reachability
4. verify attacker control
5. verify missing security control
6. verify impact
7. reject unsupported findings

Do not aggregate vulnerability reports blindly.

## 54. Security Fixes

When fixing a confirmed vulnerability, prefer correcting the security boundary itself.

Examples:

Missing authorization:
-> enforce authorization before privileged operation

SQL injection:
-> parameterize query

Path traversal:
-> constrain and canonicalize allowed path

Secret exposure:
-> remove exposure and rotate compromised credential when appropriate

Do not merely hide the visible symptom.

## 55. Preserve Functionality

Security fixes should preserve legitimate behavior whenever possible.

Before remediation determine:

- valid users
- valid operations
- required compatibility
- existing clients
- intended data access

Do not solve a security problem by unnecessarily disabling legitimate functionality.

## 56. Defense in Depth

After fixing the root security defect, consider whether inexpensive defense-in-depth measures are justified.

Examples:

authorization
+
database tenant constraint

validation
+
safe API

secret redaction
+
restricted logging

Do not add layers merely to create the appearance of additional security.

Every layer should have a concrete purpose.

## 57. Security Regression Tests

Confirmed vulnerabilities should normally receive regression protection when practical.

Examples:

Authorization bypass:

unauthorized user
->
attempt privileged operation
->
access denied

Path traversal:

malicious path
->
request
->
operation rejected

Injection:

malicious input
->
parameterized execution
->
input treated as data

The regression test should prove the security property that previously failed.

## 58. Negative Security Tests

Security verification often requires testing what must NOT happen.

Examples:

- unauthorized user cannot access resource
- expired token cannot authenticate
- tenant A cannot access tenant B
- invalid webhook signature cannot trigger action
- untrusted path cannot escape allowed directory
- client cannot receive server secret

Negative tests are often more valuable than success-path tests for security boundaries.

## 59. Verify the Fix

After remediation:

1. rerun the original security reproduction when safe
2. run security regression tests
3. run relevant functional tests
4. verify legitimate behavior still works
5. inspect final diff
6. check for alternate vulnerable paths

Use `testing-verification`.

A security fix is not complete merely because the originally identified line changed.

## 60. Search for Equivalent Paths

After confirming a vulnerability, search for equivalent patterns elsewhere when justified.

Example:

Confirmed missing authorization in:

`updateProject()`

Search related:

`deleteProject()`
`getProject()`
`archiveProject()`

Do not automatically perform a repository-wide audit.

Search closely related patterns where the same root cause may reasonably exist.

## 61. Variant Analysis

For confirmed vulnerabilities, determine whether the defect represents a broader pattern.

Examples:

Root cause:
authorization performed only in frontend

Possible variants:
other privileged endpoints may rely on frontend checks

Root cause:
unsafe path concatenation helper

Possible variants:
all callers of that helper

Root cause:
unsafe raw SQL utility

Possible variants:
other user-controlled callers

Variant analysis should follow evidence, not speculation.

## 62. Avoid Security Theater

Do not recommend controls that do not materially reduce the identified risk.

Examples of potentially meaningless recommendations:

- encryption where authorization is the actual problem
- hashing non-secret identifiers
- adding random validation unrelated to attack path
- hiding endpoint names
- adding comments instead of controls
- adding generic security headers to fix server-side authorization

Security measures must address a concrete threat.

## 63. Do Not Confuse Obscurity With Security

Do not treat:

- hidden URLs
- undocumented endpoints
- unpredictable filenames
- client-side checks
- obscure parameter names

as sufficient authorization or access control.

Security boundaries must remain valid when implementation details are known to an attacker.

## 64. Client-Side Trust

Treat client-controlled state as untrusted unless cryptographically protected and correctly verified.

Do not rely on the client to enforce:

- permissions
- prices
- account ownership
- roles
- quotas
- privileged flags
- resource identifiers

Server-side privileged operations must enforce their own security requirements.

## 65. Server-Side Trust

Server-side data is not automatically trustworthy.

Data may originate from:

- previous user input
- compromised integration
- imported files
- database records
- queues
- webhooks

Consider provenance rather than location alone.

## 66. Canonicalization

Security checks involving identifiers, URLs, or paths may require canonicalization.

Potential issues include:

- encoded traversal
- Unicode normalization
- alternate IP representations
- case differences
- symbolic links
- URL redirects
- hostname normalization

Do not add canonicalization complexity unless relevant to the concrete boundary.

## 67. Redirects

For security-sensitive outbound requests, consider redirect behavior.

A validated public URL may redirect to:

- localhost
- internal network
- metadata service
- alternate protocol

When SSRF protection depends on destination validation, determine whether redirects preserve that protection.

## 68. Parser Differentials

Security problems can occur when multiple components interpret the same data differently.

Examples:

proxy vs application

validator vs parser

frontend vs backend

URL parser vs HTTP client

Look for differential interpretation when evidence suggests it.

Do not assume parser differential attacks without a relevant multi-parser boundary.

## 69. Integer and Size Boundaries

For security-sensitive sizes, quotas, balances, and counters consider:

- integer overflow
- underflow
- negative values
- unit conversion
- truncation
- maximum allocation
- multiplication overflow

Report only when language/runtime behavior and reachable input make the issue realistic.

## 70. Business Logic Security

Not all security defects are technical injection vulnerabilities.

Review important business invariants.

Examples:

- user cannot approve their own restricted operation
- coupon cannot be reused
- balance cannot become invalid
- order cannot be paid twice
- invitation cannot be accepted by unrelated account
- privilege cannot be granted through ordinary profile update

Trace actual business rules.

## 71. State Transitions

For security-sensitive workflows, model allowed state transitions.

Example:

pending
->
approved
->
executed

Check whether attackers can:

- skip required states
- repeat transitions
- reverse protected transitions
- execute transition without permission

This is particularly relevant for:

- payments
- approvals
- account recovery
- invitations
- administrative workflows

## 72. Replay and Idempotency

For operations that should occur once, consider replay.

Examples:

- payment callbacks
- webhook events
- password-reset tokens
- email verification
- invitation tokens
- signed actions

Determine whether repeated valid requests create unintended effects.

## 73. Rate Limiting

Do not report missing rate limiting automatically.

Consider it when an operation is realistically abuse-sensitive.

Examples:

- login attempts
- password reset
- verification codes
- expensive computation
- account enumeration
- message sending
- resource creation

State the concrete abuse scenario.

## 74. Account Enumeration

Review authentication and recovery flows for differences that reveal whether an account exists.

Potential signals:

- response message
- status code
- timing
- reset behavior

Assess actual sensitivity and threat model before assigning severity.

## 75. Sensitive Data Exposure

Determine whether responses expose more data than required.

Examples:

- password hashes
- internal IDs with security meaning
- private profile fields
- authentication metadata
- administrative fields
- secrets
- cross-tenant records

Trace serialization and response construction.

## 76. Mass Assignment

When request objects are mapped directly into models or persistence, determine whether attackers can set protected fields.

Examples:

- role
- admin
- ownerId
- tenantId
- balance
- status
- verified

Prefer explicit allowlists for security-sensitive updates.

## 77. Object-Level Authorization

For operations referencing an object ID, determine whether authorization applies to the specific object.

Example:

authenticated user
+
valid document ID

does not imply:

authorized access to that document.

Trace ownership or permission enforcement.

## 78. Function-Level Authorization

Privileged endpoints must enforce required roles or permissions independently.

Do not rely solely on:

- hidden navigation
- frontend route guards
- disabled buttons
- undocumented endpoints

Review server-side enforcement.

## 79. Cache Security

For caches inspect:

- user-specific cache keys
- tenant separation
- authorization-sensitive content
- invalidation
- stale privileged state
- shared response caching

A cache key missing user or tenant context can create cross-user disclosure.

Establish the actual cache behavior before reporting.

## 80. Temporary Files

For temporary files consider:

- predictable names
- permissions
- cleanup
- symlinks
- sensitive contents
- shared directories

Use safe platform primitives where available.

## 81. Archive Extraction

For ZIP/TAR extraction consider:

archive entry
->
normalized destination
->
allowed extraction root

Watch for:

- `../`
- absolute paths
- symlinks
- overwrite behavior

Do not trust archive filenames.

## 82. Regular Expressions

For attacker-controlled input processed by complex regexes, consider catastrophic backtracking only when the engine and pattern make it plausible.

Do not label every regex as ReDoS.

Establish:

- attacker-controlled input
- problematic pattern
- meaningful computational amplification

## 83. Template Injection

When untrusted content enters a template engine, determine whether it is treated as:

- data
or
- executable template syntax

Trace the actual rendering API and configuration.

## 84. Prototype and Object Pollution

For languages/frameworks where relevant, inspect unsafe merging of attacker-controlled object keys.

Pay particular attention to:

- recursive merges
- dynamic property assignment
- special prototype keys

Verify runtime/library behavior before reporting.

## 85. Header Security

Inspect attacker-controlled headers when they affect:

- redirects
- generated URLs
- authentication
- proxy behavior
- cache behavior
- security decisions

Do not produce generic header checklists unrelated to the application.

## 86. Proxy and Origin Assumptions

Applications behind proxies may rely on:

- forwarded IP
- host
- protocol
- client certificate headers

Determine whether these values are trusted only from known infrastructure.

Do not assume forwarded headers are trustworthy by default.

## 87. Authorization Consistency

When multiple routes perform the same privileged operation, compare authorization behavior.

Example:

REST endpoint -> checks permission

GraphQL mutation -> does not

Internal RPC -> does not

Security controls should cover every reachable privileged path.

## 88. Failures and Cleanup

Security-sensitive resources should be cleaned up correctly after failure.

Examples:

- temporary credentials
- partial uploads
- locks
- transactions
- temporary files
- pending authorization state

Partial failure should not leave unintended privileged state.

## 89. Secret Rotation

When a real credential has plausibly been exposed:

1. do not reproduce it
2. remove it from exposed locations
3. identify affected usage
4. recommend rotation/revocation
5. consider history/artifact exposure where relevant

Deleting the secret from the current file may not invalidate an already exposed credential.

## 90. Git History and Secrets

If a secret was committed, removing it in a later commit does not remove it from previous history.

Determine whether history cleanup is necessary based on exposure and repository context.

Credential rotation generally remains necessary when compromise is plausible.

Do not rewrite repository history without explicit authorization.

## 91. Publishing Gate

Before public publishing or release, perform an OPSEC/security disclosure check.

Inspect relevant:

- source
- documentation
- examples
- metadata
- manifests
- generated artifacts
- logs
- screenshots when available
- author fields
- absolute paths
- environment examples

Look for:

- real identity
- credentials
- private URLs
- internal infrastructure
- personal paths
- unintended sensitive data

Use the approved handle/tag where attribution is required.

If the approved public identifier is unknown and attribution is necessary, ask the user.

## 92. Security Review Output

Lead with concrete findings.

For each finding use:

### Severity - Finding Title

Location:
`path/to/file.ext:line`

Source:
Where attacker influence originates.

Path:
How the data or control reaches the vulnerable operation.

Missing Control:
What security boundary is absent or insufficient.

Sink:
The sensitive operation.

Trigger:
What realistic attacker action causes exploitation.

Impact:
What the attacker gains or what security property fails.

Evidence:
Why the attack path follows from the code.

Remediation:
Minimal correction direction.

Confidence:
High / Medium when useful.

Do not include exploit payloads unless they are necessary for authorized validation.

## 93. Example Finding

### High - Cross-Tenant Project Update

Location:
`src/api/projects/update.ts:71`

Source:
Authenticated request parameter `projectId`.

Path:
`projectId`
->
`updateProject()`
->
database update

Missing Control:
No ownership or tenant authorization is performed for the selected project.

Sink:
Database update of the target project.

Trigger:
An authenticated tenant user submits a valid project ID belonging to another tenant.

Impact:
Cross-tenant modification of project data.

Evidence:
The query selects the project only by project ID and the handler performs no tenant comparison before writing.

Remediation:
Constrain the operation by both project ID and authorized tenant or perform an equivalent authorization check before mutation.

This is a security finding because the complete attack path is established.

## 94. Example Rejected Finding

Candidate:

"Potential SQL injection because this file performs a database query."

Investigation:

User input
->
ORM parameter
->
parameterized query

Result:

Reject finding.

The existence of a database query does not establish SQL injection.

## 95. No Findings

If no evidence-backed vulnerabilities are identified, state that no verified security findings were identified within the reviewed scope.

Then state important limitations.

Example:

"No verified security findings were identified in the reviewed authentication diff. Deployment configuration and external identity-provider settings were outside the review scope."

Do not invent vulnerabilities to produce a non-empty report.

## 96. Security Review Decision Process

Use:

determine scope
->
identify assets
->
identify entry points
->
identify trust boundaries
->
trace attacker-controlled sources
->
identify sensitive sinks
->
inspect controls between source and sink
->
generate candidate findings
->
verify reachability
->
verify exploitability
->
verify impact
->
discard speculative findings
->
assign realistic severity
->
report actionable findings

## 97. Completion Standard

A security review is complete when:

- scope is defined
- relevant trust boundaries are understood
- relevant attacker-controlled inputs were considered
- privileged operations were inspected
- authentication/authorization were checked where relevant
- candidate findings were validated
- false positives were removed
- severity reflects realistic impact
- secrets and identity information were protected
- limitations are stated
- no unsupported security claims remain

## 98. Anti-Patterns

Avoid:

Checklist dumping.

Reporting dangerous function names without tracing inputs.

Calling every external request SSRF.

Calling every SQL query SQL injection.

Calling every HTML value XSS.

Calling every cookie CSRF.

Calling every asynchronous operation a race.

Treating client-side checks as authorization.

Assuming framework protections without verification.

Ignoring framework protections without verification.

Reporting inaccessible code as remotely exploitable.

Inflating severity.

Exposing secrets inside the security report.

Using the user's real identity in examples or reports.

Performing destructive exploitation to prove a finding.

Blindly trusting security subagents.

Automatically using the strongest model.

## 99. Default Principle

Trace the attack path.

Verify attacker control.

Verify reachability.

Verify the missing security boundary.

Verify the sensitive sink.

Establish realistic impact.

Suppress unsupported findings.

Protect secrets and identity throughout the review.

Security review is evidence-based analysis, not vulnerability keyword matching.
