# Feature: First-party Web UI

## A. Feature Context

**Feature ID**: `fdd-fdd-example-feature-first-party-web-ui`
**Status**: 🔄 IN_PROGRESS

### 1. Overview

This feature delivers a first-party web user interface for task management. The UI is implemented as a React (TypeScript) single-page application (SPA) and is served as static assets by the backend for on-prem deployment. All data access is performed through the existing REST API described by OpenAPI.

### 2. Purpose

This feature exists to provide an out-of-the-box, modern, fast UI that enables users to manage tasks, teams, and comments without building a custom client. It reduces adoption friction while preserving an API-first architecture and enforcing authorization server-side.

### 3. Actors

- `fdd-fdd-example-actor-individual-user` - Uses UI to manage personal tasks and view task state.
- `fdd-fdd-example-actor-team-member` - Uses UI to work on team tasks, update status/due dates, and collaborate via comments.
- `fdd-fdd-example-actor-team-admin` - Uses UI to manage teams and membership visibility.
- `fdd-fdd-example-actor-first-party-web-ui` - The client application that calls the API using bearer tokens.

### 4. References

- Overall Design: [DESIGN.md](../../DESIGN.md)
- Dependencies:
  - [feature-platform-core](../feature-platform-core/)
  - [feature-business-domain](../feature-business-domain/)

## B. Actor Flows (FDL)

### Authenticate in Web UI

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-flow-authenticate`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- User obtains a bearer token and reaches the authenticated task list.

**Error Scenarios**:
- Invalid credentials causes an authentication error and no token is stored.
- Expired or revoked token causes re-authentication and token clearing.

**Steps**:
1. [x] - `ph-1` - User opens the web UI entry page - `inst-open-entry`
2. [x] - `ph-1` - UI shows sign-in form - `inst-show-login-form`
3. [x] - `ph-1` - User enters email and password - `inst-enter-credentials`
4. [x] - `ph-1` - User submits sign-in request - `inst-submit-login`
5. [x] - `ph-1` - UI calls token issuance endpoint - `inst-call-issue-token`
6. [x] - `ph-1` - **IF** token issuance returns an authentication error: - `inst-if-auth-failed`
   1. [x] - `ph-1` - UI shows an error message and keeps user on sign-in screen - `inst-show-auth-error`
7. [x] - `ph-1` - **ELSE**: - `inst-else-auth-ok`
   1. [x] - `ph-1` - UI stores bearer token locally for subsequent API calls - `inst-store-token`
   2. [x] - `ph-1` - UI transitions to the initial authenticated screen - `inst-enter-app`
8. [x] - `ph-1` - **RETURN** authenticated UI session established - `inst-return-authenticated`
<!-- fdd-id-content -->

### Browse and Filter Tasks

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- User sees a list of tasks and can filter by team, status, and assignee.

**Error Scenarios**:
- Authorization error results in a clear message and the UI prevents showing restricted data.
- Transient backend error results in retry affordance without losing filter state.

**Steps**:
1. [x] - `ph-1` - User navigates to task list screen - `inst-open-task-list`
2. [x] - `ph-1` - UI loads current filters from URL or local UI state - `inst-load-filter-state`
3. [x] - `ph-1` - UI prepares list request parameters from filters - `inst-build-list-query`
4. [x] - `ph-1` - UI calls task listing endpoint - `inst-call-list-tasks`
5. [x] - `ph-1` - **IF** API returns authorization error: - `inst-if-authz-list-denied`
   1. [x] - `ph-1` - UI shows access denied message and does not render task data - `inst-show-list-denied`
   2. [x] - `ph-1` - **RETURN** list flow ended - `inst-return-list-denied`
6. [x] - `ph-1` - **ELSE IF** API returns transient error: - `inst-else-if-list-transient`
   1. [x] - `ph-1` - UI shows retry affordance and preserves filters - `inst-show-list-retry`
   2. [x] - `ph-1` - **RETURN** list flow ended - `inst-return-list-retry`
7. [x] - `ph-1` - **ELSE**: - `inst-else-list-ok`
   1. [x] - `ph-1` - UI renders task list results - `inst-render-task-list`
8. [x] - `ph-1` - User changes a filter (team, status, or assignee) - `inst-change-filter`
9. [x] - `ph-1` - UI updates filter state and refreshes list - `inst-refresh-list`
10. [x] - `ph-1` - **RETURN** task list shown with selected filters - `inst-return-list-shown`
<!-- fdd-id-content -->

### Create or Edit Task

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-flow-edit-task`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- User creates a task and sees it appear in the list.
- User edits an existing task and sees changes reflected.

**Error Scenarios**:
- Validation error results in field-level feedback.
- Token is invalid and user is redirected to sign-in.

**Steps**:
1. [ ] - `ph-2` - User opens task editor for a new or existing task - `inst-open-editor`
2. [ ] - `ph-2` - **IF** editing an existing task: - `inst-if-edit-existing`
   1. [ ] - `ph-2` - UI calls task get endpoint - `inst-call-get-task`
   2. [ ] - `ph-2` - UI renders fields populated from API response - `inst-render-populated-editor`
3. [ ] - `ph-2` - **ELSE**: - `inst-else-new-task`
   1. [ ] - `ph-2` - UI renders empty editor with defaults - `inst-render-empty-editor`
4. [ ] - `ph-2` - User edits fields (title, description, status, due date) - `inst-edit-fields`
5. [ ] - `ph-2` - User submits the editor - `inst-submit-editor`
6. [ ] - `ph-2` - UI prepares request body based on editor state - `inst-build-editor-body`
7. [ ] - `ph-2` - **IF** creating a task: - `inst-if-create-task`
   1. [ ] - `ph-2` - UI calls task create endpoint - `inst-call-create-task`
8. [ ] - `ph-2` - **ELSE**: - `inst-else-update-task`
   1. [ ] - `ph-2` - UI calls task update endpoint - `inst-call-update-task`
9. [ ] - `ph-2` - **IF** API returns validation error: - `inst-if-edit-validation-error`
   1. [ ] - `ph-2` - UI shows field-level errors and keeps editor state - `inst-show-field-errors`
   2. [ ] - `ph-2` - **RETURN** editor remains open - `inst-return-editor-open`
10. [ ] - `ph-2` - **ELSE IF** API returns authentication error: - `inst-else-if-edit-auth-error`
    1. [ ] - `ph-2` - UI clears token and redirects to sign-in - `inst-clear-token-redirect`
    2. [ ] - `ph-2` - **RETURN** user re-authentication required - `inst-return-reauth`
11. [ ] - `ph-2` - **ELSE**: - `inst-else-edit-ok`
    1. [ ] - `ph-2` - UI shows success confirmation - `inst-show-save-success`
    2. [ ] - `ph-2` - UI returns to task list and refreshes - `inst-return-to-list-refresh`
12. [ ] - `ph-2` - **RETURN** task persisted and visible to user - `inst-return-edit-done`
<!-- fdd-id-content -->

### View and Add Comment

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-flow-comments`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- User views comment list for a task.
- User adds a comment and sees it appended.

**Error Scenarios**:
- Access denied results in a clear message and comments are not displayed.

**Steps**:
1. [ ] - `ph-2` - User opens a task details view - `inst-open-task-details`
2. [ ] - `ph-2` - UI calls comments list endpoint for the task - `inst-call-list-comments`
3. [ ] - `ph-2` - **IF** API returns authorization error: - `inst-if-comments-denied`
   1. [ ] - `ph-2` - UI shows access denied message and hides comments - `inst-hide-comments-denied`
   2. [ ] - `ph-2` - **RETURN** comments flow ended - `inst-return-comments-denied`
4. [ ] - `ph-2` - **ELSE**: - `inst-else-comments-ok`
   1. [ ] - `ph-2` - UI renders existing comments - `inst-render-comments`
5. [ ] - `ph-2` - User enters comment text - `inst-enter-comment`
6. [ ] - `ph-2` - User submits comment - `inst-submit-comment`
7. [ ] - `ph-2` - UI calls comment create endpoint - `inst-call-create-comment`
8. [ ] - `ph-2` - **IF** API returns validation error: - `inst-if-comment-validation`
   1. [ ] - `ph-2` - UI shows inline error and keeps text - `inst-show-comment-error`
   2. [ ] - `ph-2` - **RETURN** comment not created - `inst-return-comment-not-created`
9. [ ] - `ph-2` - **ELSE**: - `inst-else-comment-ok`
   1. [ ] - `ph-2` - UI appends new comment to list and clears input - `inst-append-comment`
10. [ ] - `ph-2` - **RETURN** comments displayed and editable - `inst-return-comments-done`
<!-- fdd-id-content -->

### Manage Teams in UI

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-flow-manage-teams`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-admin`

**Success Scenarios**:
- Team admin creates a team and then sees it in the team list.

**Error Scenarios**:
- Authorization error prevents access to team admin actions.

**Steps**:
1. [ ] - `ph-3` - Admin opens teams screen - `inst-open-teams-screen`
2. [ ] - `ph-3` - UI calls teams list endpoint - `inst-call-list-teams`
3. [ ] - `ph-3` - UI renders team list - `inst-render-team-list`
4. [ ] - `ph-3` - Admin opens create team dialog - `inst-open-create-team`
5. [ ] - `ph-3` - Admin enters team name and submits - `inst-submit-team-create`
6. [ ] - `ph-3` - UI calls team create endpoint - `inst-call-create-team`
7. [ ] - `ph-3` - **IF** API returns authorization error: - `inst-if-team-create-denied`
   1. [ ] - `ph-3` - UI shows access denied message and keeps team list - `inst-show-team-denied`
   2. [ ] - `ph-3` - **RETURN** team creation not performed - `inst-return-team-create-denied`
8. [ ] - `ph-3` - **ELSE**: - `inst-else-team-created`
   1. [ ] - `ph-3` - UI refreshes team list and shows the new team - `inst-refresh-teams`
9. [ ] - `ph-3` - **RETURN** teams managed through UI - `inst-return-teams-managed`
<!-- fdd-id-content -->

## C. Algorithms (FDL)

### Prepare API Request from UI

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request`

<!-- fdd-id-content -->
**Input**: method, path, query, body, token
**Output**: prepared request with headers and serialized body

**Steps**:
1. [x] - `ph-1` - Initialize request with method and path - `inst-init-request`
2. [x] - `ph-1` - **IF** query parameters exist: - `inst-if-query`
   1. [x] - `ph-1` - Attach query parameters to request URL - `inst-attach-query`
3. [x] - `ph-1` - **IF** token exists: - `inst-if-token`
   1. [x] - `ph-1` - Attach Authorization header with bearer token - `inst-attach-auth-header`
4. [x] - `ph-1` - **IF** body exists: - `inst-if-body`
   1. [x] - `ph-1` - Serialize body according to OpenAPI contract expectations - `inst-serialize-body`
   2. [x] - `ph-1` - Attach Content-Type header and body - `inst-attach-body`
5. [x] - `ph-1` - **RETURN** prepared request - `inst-return-prepared`
<!-- fdd-id-content -->

### Handle API Error Envelope

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error`

<!-- fdd-id-content -->
**Input**: response status, error envelope (code, message)
**Output**: UI action (display message, redirect, or retry affordance)

**Steps**:
1. [x] - `ph-1` - **MATCH** status: - `inst-match-status`
   - [x] - `ph-1` - **CASE** 400: - `inst-case-400`
     1. [x] - `ph-1` - Display validation message near relevant form input when possible - `inst-show-400`
     2. [x] - `ph-1` - **RETURN** keep user on current screen - `inst-return-keep-screen`
   - [x] - `ph-1` - **CASE** 401: - `inst-case-401`
     1. [x] - `ph-1` - Clear stored token - `inst-clear-token`
     2. [x] - `ph-1` - Redirect user to sign-in - `inst-redirect-login`
     3. [x] - `ph-1` - **RETURN** re-authentication required - `inst-return-reauth-required`
   - [x] - `ph-1` - **CASE** 403: - `inst-case-403`
     1. [x] - `ph-1` - Display access denied message - `inst-show-403`
     2. [x] - `ph-1` - **RETURN** do not display restricted content - `inst-return-no-content`
   - [x] - `ph-1` - **CASE** 409: - `inst-case-409`
     1. [x] - `ph-1` - Display conflict message and suggest refreshing data - `inst-show-409`
     2. [x] - `ph-1` - **RETURN** user action required - `inst-return-user-action`
   - [x] - `ph-1` - **DEFAULT**: - `inst-default-status`
     1. [x] - `ph-1` - Display generic error and offer retry - `inst-show-generic`
     2. [x] - `ph-1` - **RETURN** retry allowed - `inst-return-retry`
<!-- fdd-id-content -->

### Apply UI Caching Strategy

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching`

<!-- fdd-id-content -->
**Input**: asset request, prior cache metadata
**Output**: caching decision for asset and UI data

**Steps**:
1. [x] - `ph-1` - **IF** request targets static asset: - `inst-if-static-asset`
   1. [x] - `ph-1` - Use cache headers to avoid re-downloading unchanged bundles - `inst-use-cache-headers`
2. [x] - `ph-1` - **ELSE**: - `inst-else-api-data`
   1. [x] - `ph-1` - Keep UI state while requests are in flight to prevent flicker - `inst-keep-ui-state`
   2. [x] - `ph-1` - **IF** a refresh is requested: - `inst-if-refresh`
      1. [x] - `ph-1` - Re-fetch data and update UI after response - `inst-refetch-update`
3. [x] - `ph-1` - **RETURN** caching strategy applied - `inst-return-caching-applied`
<!-- fdd-id-content -->

## D. States (FDL)

### UI Session State Machine

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-state-ui-session`

<!-- fdd-id-content -->
**States**: ANONYMOUS, AUTHENTICATED, SESSION_EXPIRED
**Initial State**: ANONYMOUS

**Transitions**:
1. [x] - `ph-1` - **FROM** ANONYMOUS **TO** AUTHENTICATED **WHEN** token issuance succeeds - `inst-anon-to-auth`
2. [x] - `ph-1` - **FROM** AUTHENTICATED **TO** SESSION_EXPIRED **WHEN** API returns 401 for a protected request - `inst-auth-to-expired`
3. [x] - `ph-1` - **FROM** SESSION_EXPIRED **TO** ANONYMOUS **WHEN** UI clears token and returns to sign-in - `inst-expired-to-anon`
4. [x] - `ph-1` - **FROM** AUTHENTICATED **TO** ANONYMOUS **WHEN** user explicitly signs out - `inst-auth-to-anon`
<!-- fdd-id-content -->

## E. Technical Details

### 1. UI Architecture and Boundaries

The UI is a React (TypeScript) single-page application.

The UI is a client of the backend REST API and must treat the API contract as authoritative.

The UI must avoid embedding domain authorization rules beyond basic presentation logic.

The UI must keep concerns separated:

- Screen composition and navigation
- API communication
- Token lifecycle
- UI state and caching
- Error presentation

The UI must not redefine entity shapes.

The UI must consume shapes from the OpenAPI contract and the JSON Schema artifacts referenced in the overall design.

### 2. Deployment Model

For on-prem operational simplicity, the backend serves the UI as static assets.

The packaging intent is a single on-prem deployment surface.

The UI build output must be deterministic.

The backend must map a stable public path for UI assets.

The UI must be loadable directly from the backend origin.

The UI must avoid cross-origin requirements by default.

The UI must work without external CDNs.

### 3. Key Screens and Navigation

The UI includes the following user-facing screens:

- Sign-in screen
- Task list screen
- Task details screen
- Task editor screen
- Teams screen

Navigation behavior:

- Users start at sign-in when no valid token exists.
- After sign-in, users land on task list.
- Task selection leads to task details.
- Edit actions lead to task editor.
- Team admin can navigate to teams screen.

The UI must preserve list filter state across navigation.

The UI must preserve unsaved editor state until the user cancels or successfully saves.

### 4. API References

The UI must only use endpoints referenced by the overall design and OpenAPI.

Primary endpoints used by this feature:

- `POST /auth/tokens` - obtain access token for the UI session
- `GET /tasks` - list tasks
- `POST /tasks` - create task
- `GET /tasks/{task_id}` - get task
- `PATCH /tasks/{task_id}` - update task
- `DELETE /tasks/{task_id}` - delete task
- `GET /tasks/{task_id}/comments` - list comments
- `POST /tasks/{task_id}/comments` - create comment
- `GET /teams` - list teams
- `POST /teams` - create team

OpenAPI reference:

- See [openapi.yaml](../../../api/openapi.yaml)

### 5. Authentication and Token Handling

The UI obtains a bearer token by calling `POST /auth/tokens`.

The UI includes the bearer token in the Authorization header for all protected endpoints.

Token storage intent:

- The token must be stored in a way that allows the UI to function after navigation.
- The UI must be able to clear the token on sign-out and on authentication failure.

Token invalidation handling:

- A 401 response on protected endpoints transitions UI session to SESSION_EXPIRED.
- The UI clears the token and redirects to sign-in.

The UI must avoid logging tokens.

The UI must not expose tokens in URLs.

### 6. Authorization and Access Control

Authorization is enforced server-side.

The UI must respect access control outcomes:

- If a 403 occurs, the UI must not display restricted content.
- The UI must show an access denied message.

The UI must not attempt to derive membership rules.

The UI may hide or disable actions that are known to be unauthorized based on response outcomes.

### 7. Error Handling

The UI error-handling approach follows the backend error envelope shape.

The UI must present errors consistently and predictably.

The UI must not display internal errors with confusing technical details.

UI error categories:

- Authentication errors (401)
- Authorization errors (403)
- Validation errors (400)
- Conflict or concurrency errors (409)
- Transient errors (503 or generic 5xx)

Error handling decisions:

- Authentication errors require clearing token and redirecting to sign-in.
- Authorization errors require denying access to the specific view.
- Validation errors require showing user-correctable messages.
- Transient errors require retry affordance.

| Error Type | Handling Approach |
|------------|-------------------|
| Authentication error (401) | Clear token and redirect to sign-in |
| Authorization error (403) | Display access denied and hide restricted content |
| Validation error (400) | Display field-level errors and preserve form state |
| Transient error (5xx/503) | Display retry affordance and preserve UI state |

### 8. Performance and “Fast UI” Requirements

This feature must satisfy the overall NFR for UI performance.

Performance intentions in this feature:

- Keep initial render fast and provide immediate visual feedback.
- Avoid unnecessary full-screen loading states.
- Preserve state and avoid re-fetch loops.

Static asset delivery:

- Assets must be served with cache headers.
- The UI should avoid large initial downloads.

API interaction behavior:

- UI should show optimistic UI cues when appropriate (progress indicators).
- UI should keep screen responsive while requests run.

### 9. Accessibility and UX Quality

The UI must use consistent navigation and visual hierarchy.

The UI must provide clear error messages.

The UI must support keyboard navigation for primary flows.

The UI should use accessible labels for form fields.

### 10. Data Model References

The UI must treat the following schemas as authoritative for entity shape:

- [Task schema](../../../schemas/task.json)
- [Comment schema](../../../schemas/comment.json)
- [Team schema](../../../schemas/team.json)
- [User schema](../../../schemas/user.json)

The UI must not redefine these types.

The UI must map API responses to UI view models without changing semantics.

### 11. Non-functional Operational Considerations

The UI is served from the backend.

Operational implications:

- Backend deployments must include UI assets.
- Version alignment between UI and backend is required for contract compatibility.

The UI should be resilient to temporary network errors.

## F. Requirements

### Provide first-party Web UI for task management

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-req-provide-ui`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST provide a first-party web UI that enables authenticated users to manage tasks, teams, and comments through the existing REST API contract.

**References**:
- [Authenticate in Web UI](#authenticate-in-web-ui)
- [Browse and Filter Tasks](#browse-and-filter-tasks)
- [Create or Edit Task](#create-or-edit-task)
- [View and Add Comment](#view-and-add-comment)
- [Manage Teams in UI](#manage-teams-in-ui)

**Implements**:
- `fdd-fdd-example-feature-first-party-web-ui-flow-authenticate`
- `fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks`
- `fdd-fdd-example-feature-first-party-web-ui-flow-edit-task`
- `fdd-fdd-example-feature-first-party-web-ui-flow-comments`
- `fdd-fdd-example-feature-first-party-web-ui-flow-manage-teams`

**Phases**:
- [x] `ph-1`: Provide sign-in, token storage, and initial task list screen
- [ ] `ph-2`: Provide task editor and comments integration
- [ ] `ph-3`: Provide teams screen and admin UI for team creation

**Tests Covered**:
- `fdd-fdd-example-feature-first-party-web-ui-test-authenticate`
- `fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks`
- `fdd-fdd-example-feature-first-party-web-ui-test-edit-task`

**Acceptance Criteria**:
- A user can sign in and access the task list using a bearer token.
- A user can create or update a task and see the updated state reflected in the UI.
- A user can view and create comments for a task when authorized.
<!-- fdd-id-content -->

### Enforce contract-driven UI behavior

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-req-contract-driven-ui`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The UI MUST treat OpenAPI as the source of truth for request and response shapes, and MUST handle the standard error envelope consistently for all API calls.

**References**:
- [Handle API Error Envelope](#handle-api-error-envelope)
- [API References](#4-api-references)

**Implements**:
- `fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request`
- `fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error`

**Phases**:
- [ ] `ph-1`: Provide a shared API request preparation and shared error handling behavior

**Tests Covered**:
- `fdd-fdd-example-feature-first-party-web-ui-test-error-envelope`

**Acceptance Criteria**:
- For 400 responses, the UI shows user-correctable messages and preserves input.
- For 401 responses, the UI clears token and redirects to sign-in.
<!-- fdd-id-content -->

### Meet UI performance and responsiveness targets

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-req-ui-performance`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The UI MUST provide a responsive experience and SHOULD use caching for static assets and stable UI state to enable fast navigation and predictable behavior.

**References**:
- [Apply UI Caching Strategy](#apply-ui-caching-strategy)
- [Performance and “Fast UI” Requirements](#8-performance-and-fast-ui-requirements)

**Implements**:
- `fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching`

**Phases**:
- [x] `ph-1`: Ensure asset caching and responsive loading states
- [ ] `ph-3`: Improve UX responsiveness and reduce unnecessary re-fetching

**Tests Covered**:
- `fdd-fdd-example-feature-first-party-web-ui-test-ui-performance`

**Acceptance Criteria**:
- The UI provides clear loading feedback without blocking navigation unnecessarily.
- Static assets are cacheable and repeat page loads do not require re-downloading unchanged assets.
<!-- fdd-id-content -->

## G. Testing Scenarios

### Authenticate and establish UI session

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-test-authenticate`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-first-party-web-ui-req-provide-ui`

**Steps**:
1. [x] - `ph-1` - Start with no stored token and open UI entry page - `inst-start-anon`
2. [x] - `ph-1` - Submit valid credentials and obtain bearer token - `inst-submit-valid-credentials`
3. [x] - `ph-1` - Verify UI transitions to task list and uses token for API calls - `inst-verify-task-list-entry`
<!-- fdd-id-content -->

### Browse tasks with filters and retry

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-first-party-web-ui-req-provide-ui`

**Steps**:
1. [x] - `ph-1` - Open task list with an existing token - `inst-open-with-token`
2. [x] - `ph-1` - Apply team and status filters and request list - `inst-apply-filters`
3. [x] - `ph-1` - Verify task list renders and filter state is preserved - `inst-verify-render-preserve`
<!-- fdd-id-content -->

### Create or update a task through the UI

- [ ] **ID**: `fdd-fdd-example-feature-first-party-web-ui-test-edit-task`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-first-party-web-ui-req-provide-ui`

**Steps**:
1. [ ] - `ph-2` - Open task editor and enter valid task fields - `inst-open-editor-enter-fields`
2. [ ] - `ph-2` - Submit editor and persist task via API - `inst-submit-editor-persist`
3. [ ] - `ph-2` - Verify UI shows success and refreshed list contains updates - `inst-verify-updated-list`
<!-- fdd-id-content -->

### Handle error envelope consistently

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-test-error-envelope`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-first-party-web-ui-req-contract-driven-ui`

**Steps**:
1. [x] - `ph-1` - Trigger a validation error from an API call and observe UI message - `inst-trigger-400`
2. [x] - `ph-1` - Trigger an authentication error and verify token is cleared and sign-in is shown - `inst-trigger-401`
3. [x] - `ph-1` - Trigger an authorization error and verify restricted content is not shown - `inst-trigger-403`
<!-- fdd-id-content -->

### Meet UI performance expectations

- [x] **ID**: `fdd-fdd-example-feature-first-party-web-ui-test-ui-performance`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-first-party-web-ui-req-ui-performance`

**Steps**:
1. [x] - `ph-1` - Load UI with cached static assets and confirm no unnecessary re-download - `inst-check-asset-cache`
2. [x] - `ph-1` - Navigate between screens and confirm UI remains responsive during requests - `inst-check-responsive`
3. [ ] - `ph-3` - Refresh list data and confirm UI preserves state and avoids flicker - `inst-check-refresh-behavior`
<!-- fdd-id-content -->
