# Technical Design: fdd-example

## A. Architecture Overview

### Architectural Vision

The Task Management API is designed as an on-prem deployable backend system that serves both individual users and teams. The design prioritizes a clean separation between business logic and infrastructure concerns, enabling evolution of the system (e.g., swapping persistence, adding integrations) without rewriting core domain rules.

We adopt a Hexagonal (Ports & Adapters) approach implemented as a modular monolith. The HTTP API, persistence, and integrations are treated as adapters around a central application/domain core. This supports fast iteration early, while keeping clear seams for future extraction into separate services if scale or organizational needs demand it.

The system exposes a REST API described by an OpenAPI contract. The contract is treated as the authoritative boundary for clients and enables tooling (client generation, documentation, validation) and a stable integration surface.

The system also includes a first-party web user interface implemented as a React (TypeScript) single-page application. For operational simplicity in on-prem environments, the backend serves the UI as static assets while the UI uses the same OpenAPI-described REST API for all data access.

### Architecture Layers

| Layer | Responsibility | Technology |
|-------|---------------|------------|
| Presentation | First-party Web UI + HTTP REST API, request validation, auth entry points, response mapping | React (TypeScript) SPA, Rust (Axum), OpenAPI-driven contracts |
| Application | Use-case orchestration, transactional boundaries, invariants enforcement | Rust application services |
| Domain | Business entities and rules (tasks, assignments, comments, teams) | Rust domain module |
| Infrastructure | Persistence, migrations, background jobs, external integrations | SQLite, SQLx, on-prem deployment |

## B. Requirements & Principles

### B.1: Functional Requirements

#### FR-001: Manage Tasks

**ID**: `fdd-fdd-example-req-manage-tasks`

<!-- fdd-id-content -->
**Priority**: HIGH
**Capabilities**: `fdd-fdd-example-capability-task-management`
**Use Cases**: `fdd-fdd-example-usecase-create-task`
**Actors**: `fdd-fdd-example-actor-individual-user`, `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-external-client`

The system must support task creation, retrieval, update, and deletion, including basic state transitions across the lifecycle (to-do, in-progress, done). Tasks must support optional descriptions and optional association with a team and/or assignee.
<!-- fdd-id-content -->

#### FR-002: Assign Tasks

**ID**: `fdd-fdd-example-req-assign-tasks`

<!-- fdd-id-content -->
**Priority**: HIGH
**Capabilities**: `fdd-fdd-example-capability-task-assignment`
**Use Cases**: `fdd-fdd-example-usecase-assign-task`
**Actors**: `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-team-admin`, `fdd-fdd-example-actor-external-client`

The system must support assigning tasks to team members, reassigning tasks, unassigning tasks, and querying tasks by assignee. Assignment operations must validate that the assignee is a member of the relevant team when a task is team-scoped.
<!-- fdd-id-content -->

#### FR-003: Schedule Due Dates

**ID**: `fdd-fdd-example-req-schedule-due-dates`

<!-- fdd-id-content -->
**Priority**: MEDIUM
**Capabilities**: `fdd-fdd-example-capability-due-date-scheduling`
**Actors**: `fdd-fdd-example-actor-individual-user`, `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-external-client`

The system must allow setting, updating, and clearing due dates for tasks, and enable listing/filtering tasks based on due date criteria.
<!-- fdd-id-content -->

#### FR-004: Comment on Tasks

**ID**: `fdd-fdd-example-req-task-comments`

<!-- fdd-id-content -->
**Priority**: HIGH
**Capabilities**: `fdd-fdd-example-capability-comments-collaboration`
**Use Cases**: `fdd-fdd-example-usecase-add-comment`
**Actors**: `fdd-fdd-example-actor-individual-user`, `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-external-client`

The system must support adding and listing comments for a task, including audit metadata (author and timestamps). Comment creation must verify that the author has access to the task.
<!-- fdd-id-content -->

#### FR-005: Manage Teams

**ID**: `fdd-fdd-example-req-manage-teams`

<!-- fdd-id-content -->
**Priority**: MEDIUM
**Capabilities**: `fdd-fdd-example-capability-team-management`
**Actors**: `fdd-fdd-example-actor-team-admin`, `fdd-fdd-example-actor-external-client`

The system must support creating teams and managing team membership and roles, including an admin role that can manage membership and settings.
<!-- fdd-id-content -->

#### FR-006: Authenticate API Access

**ID**: `fdd-fdd-example-req-authenticate-api`

<!-- fdd-id-content -->
**Priority**: HIGH
**Capabilities**: `fdd-fdd-example-capability-task-management`, `fdd-fdd-example-capability-team-management`, `fdd-fdd-example-capability-task-management-web-ui`
**Actors**: `fdd-fdd-example-actor-individual-user`, `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-team-admin`, `fdd-fdd-example-actor-external-client`, `fdd-fdd-example-actor-first-party-web-ui`

The system must require authentication for all protected API endpoints and must issue personal access tokens for API clients. Authorization rules must ensure users can only access tasks and teams they have permissions for.
<!-- fdd-id-content -->

#### FR-007: Provide First-party Web UI

**ID**: `fdd-fdd-example-req-task-management-web-ui`

<!-- fdd-id-content -->
**Priority**: HIGH
**Capabilities**: `fdd-fdd-example-capability-task-management-web-ui`
**Use Cases**: `fdd-fdd-example-usecase-manage-tasks-web-ui`
**Actors**: `fdd-fdd-example-actor-individual-user`, `fdd-fdd-example-actor-team-member`, `fdd-fdd-example-actor-team-admin`, `fdd-fdd-example-actor-first-party-web-ui`

The system must provide a first-party web user interface that allows users to authenticate and manage tasks, teams, and comments using the same REST API contract described by OpenAPI.
<!-- fdd-id-content -->

### B.2: Design Principles

#### Stable Contracts First

**ID**: `fdd-fdd-example-principle-stable-contracts`

<!-- fdd-id-content -->
The OpenAPI specification is the authoritative contract for external integrations. Changes to API surface must be versioned and reviewed to avoid breaking clients.
<!-- fdd-id-content -->

#### API Contract–Driven UI

**ID**: `fdd-fdd-example-principle-api-contract-driven-ui`

<!-- fdd-id-content -->
The first-party web UI must use OpenAPI as the source of truth for request/response shapes and error envelopes. UI changes must not rely on undocumented backend behavior or implicit fields.
<!-- fdd-id-content -->

#### UI as a First-class Adapter

**ID**: `fdd-fdd-example-principle-ui-as-adapter`

<!-- fdd-id-content -->
The first-party web UI is an external adapter (client) to the backend ports. Domain rules and authorization are enforced server-side; the UI may optimize user experience but must not duplicate or weaken access-control logic.
<!-- fdd-id-content -->

#### Domain Logic Isolated

**ID**: `fdd-fdd-example-principle-domain-isolation`

<!-- fdd-id-content -->
Business rules and invariants must live in the domain/application core and must not depend on HTTP frameworks or database libraries. Adapters translate external representations to domain concepts.
<!-- fdd-id-content -->

#### Least Privilege by Default

**ID**: `fdd-fdd-example-principle-least-privilege`

<!-- fdd-id-content -->
Authorization is enforced at the application boundary for every operation. Team membership and roles determine access; defaults should minimize accidental data exposure.
<!-- fdd-id-content -->

#### Operational Simplicity (On-Prem)

**ID**: `fdd-fdd-example-principle-on-prem-simplicity`

<!-- fdd-id-content -->
The system should be easy to deploy and operate in on-prem environments, with minimal external dependencies and clear observability signals.
<!-- fdd-id-content -->

### B.3: Constraints

#### On-Prem Deployment

**ID**: `fdd-fdd-example-constraint-on-prem-deployment`

<!-- fdd-id-content -->
**ADRs**: `fdd-fdd-example-adr-initial-architecture`

The system must be deployable on-premises without requiring managed cloud services. Operational assumptions should not depend on cloud-only primitives.
<!-- fdd-id-content -->

#### SQLite as Primary Database

**ID**: `fdd-fdd-example-constraint-sqlite-storage`

<!-- fdd-id-content -->
**ADRs**: `fdd-fdd-example-adr-initial-architecture`

SQLite is the primary persistence technology. Data access patterns should consider SQLite limitations (single-writer, file-based storage) and apply appropriate concurrency and transaction strategies.
<!-- fdd-id-content -->

#### Rust Implementation

**ID**: `fdd-fdd-example-constraint-rust-implementation`

<!-- fdd-id-content -->
**ADRs**: `fdd-fdd-example-adr-initial-architecture`

The implementation language is Rust. System design must align with Rust’s strengths (memory safety, explicit concurrency) and ecosystem choices.
<!-- fdd-id-content -->

## C. Technical Architecture

### C.1: Component Model

```mermaid
graph TD
  External[External Client] -->|HTTP REST| Api[HTTP API Adapter]
  Web[First-party Web UI (SPA)] -->|HTTP REST| Api
  Api -->|Static assets| Web
  Api -->|Commands/Queries| App[Application Services]
  App --> Domain[Domain Model]
  App --> Repo[Persistence Port]
  Repo --> Sqlite[SQLite Adapter]
  App --> Auth[AuthN/AuthZ]
  App --> Notif[Notification/Events (internal)]
```

**Components**:
- **First-party Web UI (SPA)**: React (TypeScript) application served as static assets by the backend; uses the REST API for all reads/writes.
- **HTTP API Adapter**: Exposes REST endpoints, validates requests, maps to application commands/queries.
- **Application Services**: Orchestrates use cases, enforces transactional boundaries, applies authorization.
- **Domain Model**: Task/Team/Comment concepts and invariants.
- **SQLite Adapter**: Implements persistence ports using SQLite.
- **Auth**: Token issuance and request authentication, authorization checks.
- **Notification/Events**: Internal event emission for future integrations (e.g., reminders).

**Interactions**:
- HTTP API Adapter → Application Services: translates API calls into use-case invocations.
- Application Services → Domain Model: validates invariants and performs state changes.
- Application Services → SQLite Adapter: persists and queries domain state.

### C.2: Domain Model

**Technology**: JSON Schema
**Location**: [schemas/](../schemas/)

**Core Entities**:
- [Task](../schemas/task.json) - Unit of work with status, due date, optional team and assignee.
- [Comment](../schemas/comment.json) - Text comment attached to a task.
- [Team](../schemas/team.json) - Group boundary for collaboration and permissions.
- [User](../schemas/user.json) - Human identity with authentication context.

**Relationships**:
- Team → User: team membership with role (admin/member) managed by the system.
- Task → Team: tasks may be scoped to a team.
- Task → User: tasks may have an assignee.
- Comment → Task: comments belong to a task.
- Comment → User: comment author.

### C.3: API Contracts

**Technology**: REST/OpenAPI
**Location**: [openapi.yaml](../api/openapi.yaml)

**Endpoints Overview**:
- `POST /auth/tokens` - Create access token
- `GET /tasks` - List tasks
- `POST /tasks` - Create task
- `GET /tasks/{task_id}` - Get task
- `PATCH /tasks/{task_id}` - Update task
- `DELETE /tasks/{task_id}` - Delete task
- `GET /tasks/{task_id}/comments` - List comments
- `POST /tasks/{task_id}/comments` - Add comment
- `GET /teams` - List teams
- `POST /teams` - Create team

**Error Handling**:
- Use consistent error shape (code/message) for client-visible errors.
- Prefer 4xx for validation/authz errors, 5xx for unexpected failures.

**AuthN/AuthZ Entry Points**:
- Clients obtain a bearer token via `POST /auth/tokens`.
- All protected endpoints require bearer token authentication.

**Versioning Strategy**:
- API version is tracked in the OpenAPI `info.version` field and through backwards-compatible changes where possible.

### C.4: Non-Functional Requirements

#### NFR: Performance

**ID**: `fdd-fdd-example-nfr-performance`

<!-- fdd-id-content -->
- 95th percentile latency target < 200ms for typical CRUD requests under expected load.
- Pagination and filtering must be supported for list endpoints to avoid unbounded queries.
<!-- fdd-id-content -->

#### NFR: UI Performance

**ID**: `fdd-fdd-example-nfr-ui-performance`

<!-- fdd-id-content -->
- Static assets must be served with caching headers to enable repeat navigations without re-downloading unchanged bundles.
- On a typical workstation and LAN environment, initial UI load should render the first usable screen within 2 seconds.
- UI interactions that result in API calls should provide immediate feedback and keep the interface responsive while requests are in flight.
<!-- fdd-id-content -->

#### NFR: Scalability

**ID**: `fdd-fdd-example-nfr-scalability`

<!-- fdd-id-content -->
- Support teams up to 100 members with consistent access control.
- Favor read-optimized query patterns for list endpoints, while acknowledging SQLite single-writer constraints.
<!-- fdd-id-content -->

#### NFR: Reliability

**ID**: `fdd-fdd-example-nfr-reliability`

<!-- fdd-id-content -->
- Data durability is ensured by transactional writes and safe shutdown behavior.
- The system must recover cleanly from restarts without manual intervention.
<!-- fdd-id-content -->

#### NFR: Runtime & Operations

**ID**: `fdd-fdd-example-nfr-runtime-operations`

<!-- fdd-id-content -->
- Deployment: single on-prem binary with configuration via environment variables.
- Execution context: local server/VM with filesystem access for SQLite database file.
- Observability: structured logs with request IDs and error codes; basic metrics for request rates and latency.
- Failure modes: handle database busy/locked scenarios with bounded retries and clear errors.
<!-- fdd-id-content -->

#### NFR: Security

**ID**: `fdd-fdd-example-nfr-security`

<!-- fdd-id-content -->
- Authentication: bearer token required for protected endpoints.
- Authorization: enforce team membership and ownership checks for all task and team operations.
- Data protection: do not log sensitive data (passwords/tokens); store only hashed secrets.
- Security boundaries: separate auth concerns from domain logic and validate input at API boundary.
<!-- fdd-id-content -->

## D. Additional Context

This design intentionally keeps the initial system as a modular monolith to simplify deployment and early development. If future requirements demand horizontal scaling or independent deployment, components can be extracted behind the existing ports.
