# Feature: Platform Core

## A. Feature Context

**Feature ID**: `fdd-fdd-example-feature-platform-core`
**Status**: IMPLEMENTED

### 1. Overview

This feature establishes the platform foundation for the Task Management API. It defines the system boundary (HTTP + OpenAPI), authentication and authorization entry points, the SQLite persistence adapter boundary, and operational runtime concerns required for an on-prem deployment.

### 2. Purpose

Provide a stable, reusable platform layer so that business-domain features can focus on domain rules and invariants. Platform concerns are centralized here to keep the domain/application core isolated from infrastructure details.

### 3. Actors

- `fdd-fdd-example-actor-external-client` - Calls the API using the OpenAPI-defined contract.
- `fdd-fdd-example-actor-individual-user` - Indirectly relies on platform behavior for authentication and access control.
- `fdd-fdd-example-actor-team-member` - Indirectly relies on platform behavior for authentication and access control.
- `fdd-fdd-example-actor-team-admin` - Indirectly relies on platform behavior for authentication and access control.

### 4. References

- Overall Design: [DESIGN.md](../../DESIGN.md)
- Features Manifest: [FEATURES.md](../FEATURES.md)
- Dependencies: None
- API Contract (authoritative): [openapi.yaml](../../../api/openapi.yaml)
- Domain Schemas (authoritative): [schemas/](../../../schemas/)

## B. Actor Flows (FDL)

### Token Issuance Flow

- [x] **ID**: `fdd-fdd-example-feature-platform-core-flow-token-issuance`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-external-client`

**Success Scenarios**:
- Client obtains a bearer token using valid credentials.

**Error Scenarios**:
- Invalid credentials result in a client-visible authentication failure.
- Rate limiting or temporary server unavailability prevents token issuance.

**Steps**:
1. [x] - `ph-1` - Client prepares authentication request for `POST /auth/tokens` - `inst-prepare-request`
2. [x] - `ph-1` - Client submits JSON body with credentials to the token endpoint - `inst-submit-request`
3. [x] - `ph-1` - System validates request shape against OpenAPI contract - `inst-validate-request-shape`
4. [x] - `ph-1` - **IF** credentials are invalid: - `inst-if-invalid-credentials`
   1. [x] - `ph-1` - **RETURN** 401 error response with standard error shape - `inst-return-401`
5. [x] - `ph-1` - Issue a new access token and persist its metadata - `inst-issue-token`
6. [x] - `ph-1` - **RETURN** 201 response containing `access_token` and `token_type` - `inst-return-201`
<!-- fdd-id-content -->

### Authenticated API Call Flow

- [x] **ID**: `fdd-fdd-example-feature-platform-core-flow-authenticated-call`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-external-client`

**Success Scenarios**:
- Client calls a protected endpoint with a valid bearer token.

**Error Scenarios**:
- Missing token or invalid token results in authentication failure.
- Authenticated user lacks permission for requested operation.

**Steps**:
1. [x] - `ph-1` - Client sends request to a protected endpoint with `Authorization: Bearer <token>` - `inst-send-protected-request`
2. [x] - `ph-1` - System authenticates request by verifying bearer token - `inst-authenticate-bearer`
3. [x] - `ph-1` - **IF** token is missing or invalid: - `inst-if-token-invalid`
   1. [x] - `ph-1` - **RETURN** 401 error response with standard error shape - `inst-return-401-invalid`
4. [x] - `ph-1` - Evaluate authorization policy for the requested operation - `inst-evaluate-policy`
5. [x] - `ph-1` - **IF** policy denies access: - `inst-if-policy-deny`
   1. [x] - `ph-1` - **RETURN** 403 error response with standard error shape - `inst-return-403`
6. [x] - `ph-1` - Route request to appropriate application handler - `inst-route-to-handler`
7. [x] - `ph-1` - **RETURN** handler response mapped to OpenAPI response schema - `inst-return-handler-response`
<!-- fdd-id-content -->

### Database Busy Handling Flow

- [x] **ID**: `fdd-fdd-example-feature-platform-core-flow-db-busy-handling`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-external-client`

**Success Scenarios**:
- Under brief SQLite lock contention, the system retries within a bounded window and completes the request.

**Error Scenarios**:
- Under sustained contention, the system returns a clear transient failure response.

**Steps**:
1. [x] - `ph-1` - System begins a request transaction with the SQLite adapter - `inst-begin-transaction`
2. [x] - `ph-1` - **TRY** - `inst-try-db-op`
   1. [x] - `ph-1` - Execute a write operation within the transaction - `inst-exec-write`
   2. [x] - `ph-1` - Commit transaction on success - `inst-commit`
3. [x] - `ph-1` - **CATCH** database busy error: - `inst-catch-busy`
   1. [x] - `ph-1` - Wait using bounded backoff strategy - `inst-backoff`
   2. [x] - `ph-1` - Retry the transaction up to the configured limit - `inst-retry-limit`
4. [x] - `ph-1` - **IF** retry budget exceeded: - `inst-if-budget-exceeded`
   1. [x] - `ph-1` - **RETURN** 503 error response indicating transient contention - `inst-return-503`
5. [x] - `ph-1` - **RETURN** successful response after completing transaction - `inst-return-success`
<!-- fdd-id-content -->

## C. Algorithms (FDL)

### Verify Bearer Token

- [x] **ID**: `fdd-fdd-example-feature-platform-core-algo-verify-bearer-token`

<!-- fdd-id-content -->
**Input**: HTTP request headers, expected token type, token store
**Output**: authenticated principal or authentication error

**Steps**:
1. [x] - `ph-1` - Extract Authorization header value - `inst-extract-auth-header`
2. [x] - `ph-1` - **IF** Authorization header is missing: - `inst-if-missing-header`
   1. [x] - `ph-1` - **RETURN** authentication error (missing token) - `inst-return-missing-token`
3. [x] - `ph-1` - Parse bearer token from header format - `inst-parse-bearer`
4. [x] - `ph-1` - **IF** header format is invalid: - `inst-if-invalid-format`
   1. [x] - `ph-1` - **RETURN** authentication error (invalid format) - `inst-return-invalid-format`
5. [x] - `ph-1` - Lookup token record in token store - `inst-lookup-token`
6. [x] - `ph-1` - **IF** token record does not exist: - `inst-if-token-not-found`
   1. [x] - `ph-1` - **RETURN** authentication error (unknown token) - `inst-return-unknown-token`
7. [x] - `ph-1` - **IF** token is expired or revoked: - `inst-if-token-inactive`
   1. [x] - `ph-1` - **RETURN** authentication error (inactive token) - `inst-return-inactive-token`
8. [x] - `ph-1` - Build principal from token record (user identity and context) - `inst-build-principal`
9. [x] - `ph-1` - **RETURN** principal - `inst-return-principal`
<!-- fdd-id-content -->

### Map Domain and Infrastructure Errors to API Errors

- [x] **ID**: `fdd-fdd-example-feature-platform-core-algo-map-errors`

<!-- fdd-id-content -->
**Input**: internal error classification, request context
**Output**: HTTP status code and API error payload

**Steps**:
1. [x] - `ph-1` - Identify the error category for this failure - `inst-identify-category`
2. [x] - `ph-1` - Choose a default response of 500 with safe `code` and generic message - `inst-default-500`
3. [x] - `ph-1` - **IF** error category indicates request validation failure: - `inst-if-validation`
   1. [x] - `ph-1` - Choose 400 response with `code` and `message` - `inst-choose-400`
4. [x] - `ph-1` - **IF** error category indicates authentication failure: - `inst-if-authentication`
   1. [x] - `ph-1` - Choose 401 response with `code` and `message` - `inst-choose-401`
5. [x] - `ph-1` - **IF** error category indicates authorization failure: - `inst-if-authorization`
   1. [x] - `ph-1` - Choose 403 response with `code` and `message` - `inst-choose-403`
6. [x] - `ph-1` - **IF** error category indicates missing resource: - `inst-if-not-found`
   1. [x] - `ph-1` - Choose 404 response with `code` and `message` - `inst-choose-404`
7. [x] - `ph-1` - **IF** error category indicates conflict: - `inst-if-conflict`
   1. [x] - `ph-1` - Choose 409 response with `code` and `message` - `inst-choose-409`
8. [x] - `ph-1` - **IF** error category indicates transient infrastructure failure: - `inst-if-transient`
   1. [x] - `ph-1` - Choose 503 response with `code` and `message` - `inst-choose-503`
9. [x] - `ph-1` - Ensure the selected payload matches OpenAPI error schema - `inst-ensure-schema`
10. [x] - `ph-1` - **RETURN** the selected error response - `inst-return-selected`
<!-- fdd-id-content -->

### Execute SQLite Transaction with Bounded Retry

- [x] **ID**: `fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry`

<!-- fdd-id-content -->
**Input**: operation intent, retry policy, sqlite connection pool
**Output**: operation result or transient failure

**Steps**:
1. [x] - `ph-1` - Initialize attempt counter - `inst-init-attempts`
2. [x] - `ph-1` - **WHILE** attempts within retry budget: - `inst-while-budget`
   1. [x] - `ph-1` - Acquire database connection from pool - `inst-acquire-conn`
   2. [x] - `ph-1` - **TRY** - `inst-try-tx`
      1. [x] - `ph-1` - Begin transaction - `inst-begin-tx`
      2. [x] - `ph-1` - Execute operation within transaction - `inst-execute-op`
      3. [x] - `ph-1` - Commit transaction - `inst-commit-tx`
      4. [x] - `ph-1` - **RETURN** operation result - `inst-return-result`
   3. [x] - `ph-1` - **CATCH** database busy error: - `inst-catch-busy`
      1. [x] - `ph-1` - Roll back transaction if needed - `inst-rollback`
      2. [x] - `ph-1` - Wait using backoff schedule - `inst-wait-backoff`
      3. [x] - `ph-1` - Increase attempt counter - `inst-inc-attempts`
   4. [x] - `ph-1` - **CATCH** non-busy database error: - `inst-catch-non-busy`
      1. [x] - `ph-1` - Roll back transaction if needed - `inst-rollback-non-busy`
      2. [x] - `ph-1` - **RETURN** failure mapped to non-transient category - `inst-return-non-transient`
3. [x] - `ph-1` - **RETURN** transient failure after retry budget exceeded - `inst-return-transient-failure`
<!-- fdd-id-content -->

### Record Request Audit Metadata

- [x] **ID**: `fdd-fdd-example-feature-platform-core-algo-request-audit`

<!-- fdd-id-content -->
**Input**: request context, principal, result status
**Output**: audit entry persisted or emitted

**Steps**:
1. [x] - `ph-1` - Build audit record (request id, actor id, operation, status, timestamp) - `inst-build-audit`
2. [x] - `ph-1` - **TRY** - `inst-try-persist-audit`
   1. [x] - `ph-1` - Persist audit record using SQLite adapter - `inst-persist-audit`
3. [x] - `ph-1` - **CATCH** any errors: - `inst-catch-audit`
   1. [x] - `ph-1` - Emit structured log describing audit persistence failure - `inst-log-audit-failure`
4. [x] - `ph-1` - **RETURN** audit completed - `inst-return-audit`
<!-- fdd-id-content -->

## D. States (FDL)

### Access Token State Machine

- [x] **ID**: `fdd-fdd-example-feature-platform-core-state-access-token`

<!-- fdd-id-content -->
**States**: ACTIVE, EXPIRED, REVOKED
**Initial State**: ACTIVE

**Transitions**:
1. [x] - `ph-1` - **FROM** ACTIVE **TO** EXPIRED **WHEN** current time is after token expiry - `inst-active-to-expired`
2. [x] - `ph-1` - **FROM** ACTIVE **TO** REVOKED **WHEN** a revocation event is recorded for the token - `inst-active-to-revoked`
3. [x] - `ph-1` - **FROM** EXPIRED **TO** EXPIRED **WHEN** token is presented again - `inst-expired-stays-expired`
4. [x] - `ph-1` - **FROM** REVOKED **TO** REVOKED **WHEN** token is presented again - `inst-revoked-stays-revoked`
<!-- fdd-id-content -->

## E. Technical Details

### Database Schema

The platform core is responsible for defining and operating the persistence boundary. The authoritative entity shapes are referenced from the machine-readable schemas.

- Schema sources:
  - Task: [task.json](../../../schemas/task.json)
  - Comment: [comment.json](../../../schemas/comment.json)
  - Team: [team.json](../../../schemas/team.json)
  - User: [user.json](../../../schemas/user.json)

Operational assumptions for SQLite:
- The database is a local file controlled by the deployment environment.
- Single-writer contention is expected and must be handled as a first-class scenario.
- Transactions are short-lived and bounded to reduce lock time.

SQLite responsibilities in this feature:
- Database file location configuration.
- Connection pool configuration.
- Migrations execution order and idempotency.
- Transaction boundaries and retry/backoff on busy/locked.

Minimal persistence concerns for platform-core:
- Token storage for issued tokens and revocation.
- Audit/event tables if required for operational needs.
- Migration metadata table managed by migration tool.

### API Endpoints

This feature defines and enforces the API boundary but does not define business endpoints beyond what is necessary for platform concerns.

Contract source:
- [openapi.yaml](../../../api/openapi.yaml)

Platform-core owned endpoint groups:
- Token issuance:
  - `POST /auth/tokens`

Platform-core cross-cutting endpoint behavior:
- Request validation:
  - Requests conform to OpenAPI schema before reaching application services.
- Response validation intent:
  - Responses conform to OpenAPI schema for success and error shapes.
- Error response envelope:
  - `code` and `message` fields are stable and documented.

### Security

**Authorization**:
- Authorization is evaluated at the application boundary.
- Every operation must have an explicit authorization decision.
- Deny-by-default for operations without explicit access rules.

**Access Control**:
- Identity is established from bearer token.
- The principal includes user id and any derived context.
- Business-domain features define specific access rules, while platform-core provides:
  - Authentication enforcement.
  - A policy evaluation hook.
  - Standard error mapping for access failures.

Token policy:
- Tokens are presented as bearer tokens.
- Tokens may be revoked.
- Tokens may expire.
- Token storage includes metadata required for verification.

Secrets handling:
- Sensitive material is not logged.
- Passwords are never stored in plaintext.
- Token values are treated as sensitive.

### Error Handling

The platform defines a consistent error contract and maps internal failures to API errors.

Error response shape:
- `code`: stable error identifier suitable for client branching.
- `message`: safe human-readable message.

Error categories and mapping:

| Error Type | Handling Approach |
|------------|-------------------|
| Invalid request shape | 400 with error `code` and `message` |
| Missing/invalid token | 401 with error `code` and `message` |
| Permission denied | 403 with error `code` and `message` |
| Resource not found | 404 with error `code` and `message` |
| Conflict | 409 with error `code` and `message` |
| SQLite busy/locked | 503 after bounded retries |
| Unexpected error | 500 with safe `code` and generic message |

Operational visibility for failures:
- All failures include a request identifier in logs.
- Error codes are consistent across endpoints.
- Busy/locked scenarios are specifically logged and counted.

### Runtime & Operations

Deployment model:
- Single on-prem binary.
- Environment-based configuration.
- Local filesystem access for SQLite database file.

Configuration (illustrative categories, not code):
- Server:
  - Host and port.
  - Request timeouts.
- Database:
  - SQLite file path.
  - Connection pool size.
  - Busy timeout.
  - Retry budget and backoff schedule.
- Auth:
  - Token TTL.
  - Token hashing and storage strategy.
- Observability:
  - Log level.
  - Request ID propagation.
  - Metrics enablement.

Logging requirements:
- Structured logs.
- Consistent fields:
  - request id
  - actor identity (when available)
  - endpoint
  - outcome
  - latency bucket
- Sensitive data excluded.

Metrics requirements:
- Request count per endpoint.
- Request latency distribution.
- Error count per error code.
- SQLite busy/locked counts.

Failure modes handled:
- SQLite busy/locked.
- Token store read failure.
- Migration failure on startup.
- Invalid OpenAPI contract usage by client.

### Cross-Feature Boundaries

This feature provides primitives and boundaries used by `feature-business-domain`:
- HTTP boundary and request/response mapping.
- Authentication enforcement.
- Authorization policy interface.
- Transaction helpers and error mapping.

This feature does not define business invariants and does not define feature-level flows for tasks, teams, assignments, or comments.

## F. Requirements

### OpenAPI Contract Enforcement

- [x] **ID**: `fdd-fdd-example-feature-platform-core-req-openapi-enforcement`

<!-- fdd-id-content -->
**Status**: ✅ IMPLEMENTED
**Description**: The system MUST validate incoming requests against the OpenAPI contract at the API boundary and MUST map responses to the documented response shapes.

**References**:
- [Authenticated API Call Flow](#authenticated-api-call-flow)
- [Map Domain and Infrastructure Errors to API Errors](#map-domain-and-infrastructure-errors-to-api-errors)

**Implements**:
- `fdd-fdd-example-feature-platform-core-flow-authenticated-call`
- `fdd-fdd-example-feature-platform-core-algo-map-errors`

**Phases**:
- [x] `ph-1`: Contract validation and standard error envelope support.

**Tests Covered**:
- `fdd-fdd-example-feature-platform-core-test-openapi-requests`

**Acceptance Criteria**:
- Requests that do not match OpenAPI request schema result in a 400 response with the standard error shape.
- Error responses include a stable `code` and safe `message`.
<!-- fdd-id-content -->

### Token Issuance and Verification

- [x] **ID**: `fdd-fdd-example-feature-platform-core-req-token-auth`

<!-- fdd-id-content -->
**Status**: ✅ IMPLEMENTED
**Description**: The system MUST issue bearer tokens through the token issuance endpoint and MUST verify bearer tokens for protected endpoints.

**References**:
- [Token Issuance Flow](#token-issuance-flow)
- [Verify Bearer Token](#verify-bearer-token)
- [Access Token State Machine](#access-token-state-machine)

**Implements**:
- `fdd-fdd-example-feature-platform-core-flow-token-issuance`
- `fdd-fdd-example-feature-platform-core-algo-verify-bearer-token`
- `fdd-fdd-example-feature-platform-core-state-access-token`

**Phases**:
- [x] `ph-1`: Token issuance and verification with expiry and revocation checks.

**Tests Covered**:
- `fdd-fdd-example-feature-platform-core-test-token-issuance`
- `fdd-fdd-example-feature-platform-core-test-token-expired`

**Acceptance Criteria**:
- Invalid credentials result in 401 at token issuance.
- Missing or invalid bearer tokens result in 401 for protected endpoints.
<!-- fdd-id-content -->

### SQLite Transaction Handling

- [x] **ID**: `fdd-fdd-example-feature-platform-core-req-sqlite-tx-handling`

<!-- fdd-id-content -->
**Status**: ✅ IMPLEMENTED
**Description**: The system MUST execute write operations within transactions and MUST handle SQLite busy/locked errors using bounded retries and a clear transient error response.

**References**:
- [Database Busy Handling Flow](#database-busy-handling-flow)
- [Execute SQLite Transaction with Bounded Retry](#execute-sqlite-transaction-with-bounded-retry)

**Implements**:
- `fdd-fdd-example-feature-platform-core-flow-db-busy-handling`
- `fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry`

**Phases**:
- [x] `ph-1`: Transaction helper and retry policy for busy/locked.

**Tests Covered**:
- `fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry`

**Acceptance Criteria**:
- Under brief contention, retries succeed within configured budget.
- After budget exhaustion, the system returns 503 with the standard error shape.
<!-- fdd-id-content -->

## G. Testing Scenarios

### OpenAPI Request Validation

- [x] **ID**: `fdd-fdd-example-feature-platform-core-test-openapi-requests`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-platform-core-req-openapi-enforcement`

**Steps**:
1. [x] - `ph-1` - Prepare a request that violates the OpenAPI request schema - `inst-prepare-invalid-request`
2. [x] - `ph-1` - Send request to a documented endpoint - `inst-send-invalid-request`
3. [x] - `ph-1` - Verify response is 400 with standard error shape - `inst-verify-400-shape`
<!-- fdd-id-content -->

### Token Issuance Success

- [x] **ID**: `fdd-fdd-example-feature-platform-core-test-token-issuance`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-platform-core-req-token-auth`

**Steps**:
1. [x] - `ph-1` - Prepare a valid token issuance request - `inst-prepare-token-request`
2. [x] - `ph-1` - Send request to `POST /auth/tokens` - `inst-send-token-request`
3. [x] - `ph-1` - Verify 201 response contains bearer token fields - `inst-verify-token-response`
<!-- fdd-id-content -->

### Token Expired Rejection

- [x] **ID**: `fdd-fdd-example-feature-platform-core-test-token-expired`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-platform-core-req-token-auth`

**Steps**:
1. [x] - `ph-1` - Obtain an access token and let it expire - `inst-obtain-and-expire`
2. [x] - `ph-1` - Call a protected endpoint with the expired token - `inst-call-with-expired`
3. [x] - `ph-1` - Verify 401 response with standard error shape - `inst-verify-401-expired`
<!-- fdd-id-content -->

### SQLite Busy Retry

- [x] **ID**: `fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-platform-core-req-sqlite-tx-handling`

**Steps**:
1. [x] - `ph-1` - Induce SQLite lock contention for a write operation - `inst-induce-contention`
2. [x] - `ph-1` - Send a request that performs a write - `inst-send-write`
3. [x] - `ph-1` - Verify retry behavior stays within configured budget - `inst-verify-retry-budget`
4. [x] - `ph-1` - Verify success or 503 with standard error shape depending on contention duration - `inst-verify-outcome`
<!-- fdd-id-content -->
