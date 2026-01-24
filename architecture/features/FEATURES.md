# Features: fdd-example

**Status Overview**: 3 features total (2 implemented, 1 in development, 0 design ready, 0 in design, 0 not started)

**Meaning**:
- ⏳ NOT_STARTED
- 📝 IN_DESIGN
- 📘 DESIGN_READY
- 🔄 IN_DEVELOPMENT
- ✅ IMPLEMENTED

---

### 1. [fdd-fdd-example-feature-platform-core](feature-platform-core/) ✅ HIGH

- **Purpose**: Core platform foundation: HTTP API adapter, authn/authz boundary, SQLite persistence adapter, and operational runtime concerns.
- **Status**: IMPLEMENTED
- **Depends On**: None
- **Blocks**: [feature-business-domain](feature-business-domain/)
- **Scope**:
  - API adapter (OpenAPI contract as source of truth, request validation, consistent error mapping)
  - Authentication and authorization boundary (bearer token issuance and verification, permission checks)
  - Persistence adapter for SQLite (migrations, transactions, handling busy/locked cases)
  - Runtime & operations (configuration, structured logging, basic metrics)
- **Requirements Covered**:
  - `fdd-fdd-example-req-authenticate-api`
  - `fdd-fdd-example-nfr-security`
  - `fdd-fdd-example-nfr-runtime-operations`
  - `fdd-fdd-example-nfr-reliability`
  - `fdd-fdd-example-nfr-performance`
  - `fdd-fdd-example-nfr-scalability`
- **Principles Covered**:
  - `fdd-fdd-example-principle-stable-contracts`
  - `fdd-fdd-example-principle-domain-isolation`
  - `fdd-fdd-example-principle-on-prem-simplicity`
- **Constraints Affected**:
  - `fdd-fdd-example-constraint-on-prem-deployment`
  - `fdd-fdd-example-constraint-sqlite-storage`
  - `fdd-fdd-example-constraint-rust-implementation`
- **Phases**:
  - `ph-1`: ✅ IMPLEMENTED — Establish platform scaffolding and boundaries (API/auth/persistence/runtime)

---

### 2. [fdd-fdd-example-feature-business-domain](feature-business-domain/) ✅ HIGH

- **Purpose**: Business logic implementation for tasks, assignments, due dates, comments, and teams.
- **Status**: IMPLEMENTED
- **Depends On**: [feature-platform-core](feature-platform-core/)
- **Blocks**: None
- **Scope**:
  - Tasks CRUD and lifecycle status transitions
  - Task assignment rules (including membership checks for team-scoped tasks)
  - Due date scheduling (set/update/clear and query support)
  - Comments and collaboration (access checks, author metadata)
  - Teams and membership management (admin/member roles)
- **Requirements Covered**:
  - `fdd-fdd-example-req-manage-tasks`
  - `fdd-fdd-example-req-assign-tasks`
  - `fdd-fdd-example-req-schedule-due-dates`
  - `fdd-fdd-example-req-task-comments`
  - `fdd-fdd-example-req-manage-teams`
- **Principles Covered**:
  - `fdd-fdd-example-principle-domain-isolation`
  - `fdd-fdd-example-principle-least-privilege`
- **Constraints Affected**:
  - `fdd-fdd-example-constraint-sqlite-storage`
- **Phases**:
  - `ph-1`: ✅ IMPLEMENTED — Implement core domain behaviors and invariants for tasks and teams
    - **Depends On**:
      - [feature-platform-core](feature-platform-core/) `ph-1`

---

### 3. [fdd-fdd-example-feature-first-party-web-ui](feature-first-party-web-ui/) 🔄 HIGH

- **Purpose**: First-party Web UI (React + TypeScript) for managing tasks, teams, and comments, served as static assets by the backend.
- **Status**: DONE
- **Depends On**: [feature-platform-core](feature-platform-core/), [feature-business-domain](feature-business-domain/)
- **Blocks**: None
- **Scope**:
  - React (TypeScript) SPA for tasks/teams/comments
  - Authentication in UI via bearer tokens issued by the API
  - Contract-driven integration based on OpenAPI (`api/openapi.yaml`)
  - Static asset build and delivery served by the backend for on-prem deployment
  - UI performance work (caching, fast initial render, responsive interactions)
- **Requirements Covered**:
  - `fdd-fdd-example-req-task-management-web-ui`
  - `fdd-fdd-example-nfr-ui-performance`
- **Principles Covered**:
  - `fdd-fdd-example-principle-api-contract-driven-ui`
  - `fdd-fdd-example-principle-ui-as-adapter`
- **Constraints Affected**:
  - `fdd-fdd-example-constraint-on-prem-deployment`
- **Phases**:
  - `ph-1`: ✅ IMPLEMENTED — UI scaffolding, static build, and serving assets via backend
  - `ph-2`: ⏳ NOT_STARTED — Task screens (list/detail/edit) and comments integration
  - `ph-3`: ⏳ NOT_STARTED — Teams screens and UX/performance improvements
