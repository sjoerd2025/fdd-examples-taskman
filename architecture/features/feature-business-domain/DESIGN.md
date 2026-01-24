# Feature: Business Domain

## A. Feature Context

**Feature ID**: `fdd-fdd-example-feature-business-domain`
**Status**: ✅ IMPLEMENTED

### 1. Overview

This feature defines the business rules and behaviors for tasks, assignments, due dates, comments, and teams. It implements the domain and application-layer invariants behind the REST API, while delegating HTTP boundary concerns, authentication, and SQLite infrastructure to `feature-platform-core`.

### 2. Purpose

Provide consistent, testable domain behavior for managing tasks in personal and team contexts. The goal is to keep the business rules isolated from adapters, and to make the OpenAPI contract executable by implementing the required operations and invariants.

### 3. Actors

- `fdd-fdd-example-actor-individual-user` - Creates and manages personal tasks.
- `fdd-fdd-example-actor-team-member` - Works with team-scoped tasks, receives assignments, and collaborates via comments.
- `fdd-fdd-example-actor-team-admin` - Manages teams and membership roles.
- `fdd-fdd-example-actor-external-client` - Calls the API on behalf of a user.

### 4. References

- Overall Design: [DESIGN.md](../../DESIGN.md)
- Business Context: [BUSINESS.md](../../BUSINESS.md)
- Dependency: `fdd-fdd-example-feature-platform-core` (`ph-1`)
- API contract: [api/openapi.yaml](../../../api/openapi.yaml)
- Schemas: [schemas/](../../../schemas/)

## B. Actor Flows (FDL)

### Task Management (Create, Update, Delete, List)

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-flow-task-management`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-individual-user`

**Success Scenarios**:
- Create a personal task.
- Update a task (status, due date, assignee, text).
- Delete a task.
- List tasks filtered by team or assignee.

**Error Scenarios**:
- Invalid request data is rejected.
- Task not found.
- Access denied.

**Steps**:
1. [ ] - `ph-1` - External client sends `POST /tasks` with a title and optional fields - `inst-create-task-request`
2. [ ] - `ph-1` - System checks the user has permission to create the task in the requested scope - `inst-authorize-create`
3. [ ] - `ph-1` - **IF** task data is invalid - `inst-if-invalid-create`
   1. [ ] - `ph-1` - **RETURN** an error response with a client-safe message - `inst-return-invalid-create`
4. [ ] - `ph-1` - System creates task with initial status `to-do` - `inst-create-task`
5. [ ] - `ph-1` - System returns the created task - `inst-return-created-task`
6. [ ] - `ph-1` - External client sends `PATCH /tasks/{task_id}` with updates - `inst-update-task-request`
7. [ ] - `ph-1` - System checks the user has access to the target task - `inst-authorize-update`
8. [ ] - `ph-1` - **IF** task does not exist - `inst-if-not-found`
   1. [ ] - `ph-1` - **RETURN** a not-found error - `inst-return-not-found`
9. [ ] - `ph-1` - System applies updates while enforcing state and field invariants - `inst-apply-updates`
10. [ ] - `ph-1` - System returns the updated task - `inst-return-updated-task`
11. [ ] - `ph-1` - External client sends `DELETE /tasks/{task_id}` - `inst-delete-task-request`
12. [ ] - `ph-1` - System checks the user has permission to delete the task - `inst-authorize-delete`
13. [ ] - `ph-1` - System deletes the task - `inst-delete-task`
14. [ ] - `ph-1` - **RETURN** a success response - `inst-return-delete-success`
15. [ ] - `ph-1` - External client sends `GET /tasks` with optional `team_id` and `assignee_user_id` filters - `inst-list-tasks-request`
16. [ ] - `ph-1` - System returns only tasks visible to the user under access rules - `inst-return-visible-tasks`
<!-- fdd-id-content -->

### Task Assignment (Assign, Reassign, Unassign)

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-flow-task-assignment`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- Assign a team-scoped task to a team member.
- Clear assignee.

**Error Scenarios**:
- Assignee is not a member of the task team.
- User is not allowed to modify the task.

**Steps**:
1. [ ] - `ph-1` - External client sends `PATCH /tasks/{task_id}` with a new `assignee_user_id` or null - `inst-assignment-request`
2. [ ] - `ph-1` - System checks the user has access to modify the task - `inst-authorize-assignment`
3. [ ] - `ph-1` - **IF** task is personal (no team scope) and assignee is not the current user - `inst-if-personal-assignment-invalid`
   1. [ ] - `ph-1` - **RETURN** an access denied error - `inst-return-personal-assignment-denied`
4. [ ] - `ph-1` - **IF** task is team-scoped and assignee is not a team member - `inst-if-assignee-not-member`
   1. [ ] - `ph-1` - **RETURN** an invalid assignment error - `inst-return-assignee-not-member`
5. [ ] - `ph-1` - System updates task assignee - `inst-update-assignee`
6. [ ] - `ph-1` - **RETURN** the updated task - `inst-return-assigned-task`
<!-- fdd-id-content -->

### Due Date Scheduling (Set, Update, Clear)

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-flow-due-date-scheduling`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-individual-user`

**Success Scenarios**:
- Set a due date on a task.
- Clear a due date.

**Error Scenarios**:
- Due date format invalid.
- User cannot modify the task.

**Steps**:
1. [ ] - `ph-1` - External client sends `PATCH /tasks/{task_id}` with `due_at` value or null - `inst-due-date-request`
2. [ ] - `ph-1` - System checks the user has access to modify the task - `inst-authorize-due-date`
3. [ ] - `ph-1` - **IF** due date value is invalid - `inst-if-invalid-due-date`
   1. [ ] - `ph-1` - **RETURN** an invalid request error - `inst-return-invalid-due-date`
4. [ ] - `ph-1` - System updates due date - `inst-update-due-date`
5. [ ] - `ph-1` - **RETURN** the updated task - `inst-return-updated-due-date`
<!-- fdd-id-content -->

### Comments on Tasks (Add, List)

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-flow-task-comments`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-member`

**Success Scenarios**:
- Add a comment to a task.
- List comments for a task.

**Error Scenarios**:
- User has no access to the task.
- Comment body invalid.

**Steps**:
1. [ ] - `ph-1` - External client sends `POST /tasks/{task_id}/comments` with comment body - `inst-add-comment-request`
2. [ ] - `ph-1` - System checks the user has access to the task - `inst-authorize-comment-create`
3. [ ] - `ph-1` - **IF** comment body is invalid - `inst-if-invalid-comment-body`
   1. [ ] - `ph-1` - **RETURN** an invalid request error - `inst-return-invalid-comment-body`
4. [ ] - `ph-1` - System creates comment with author identity and timestamp - `inst-create-comment`
5. [ ] - `ph-1` - **RETURN** the created comment - `inst-return-created-comment`
6. [ ] - `ph-1` - External client sends `GET /tasks/{task_id}/comments` - `inst-list-comments-request`
7. [ ] - `ph-1` - System checks the user has access to the task - `inst-authorize-comment-list`
8. [ ] - `ph-1` - System returns the comment list - `inst-return-comments`
<!-- fdd-id-content -->

### Teams (Create, List)

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-flow-team-management`

<!-- fdd-id-content -->
**Actor**: `fdd-fdd-example-actor-team-admin`

**Success Scenarios**:
- Create a team.
- List teams visible to the user.

**Error Scenarios**:
- Team name invalid.
- User is not permitted.

**Steps**:
1. [ ] - `ph-1` - External client sends `POST /teams` with a team name - `inst-create-team-request`
2. [ ] - `ph-1` - System checks the user has permission to create a team - `inst-authorize-create-team`
3. [ ] - `ph-1` - **IF** team name is invalid - `inst-if-invalid-team-name`
   1. [ ] - `ph-1` - **RETURN** an invalid request error - `inst-return-invalid-team-name`
4. [ ] - `ph-1` - System creates team and records creator as admin in internal membership model - `inst-create-team`
5. [ ] - `ph-1` - **RETURN** the created team - `inst-return-created-team`
6. [ ] - `ph-1` - External client sends `GET /teams` - `inst-list-teams-request`
7. [ ] - `ph-1` - System returns only teams visible to the user - `inst-return-visible-teams`
<!-- fdd-id-content -->

## C. Algorithms (FDL)

### Authorize Task Access

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-authorize-task-access`

<!-- fdd-id-content -->
**Input**: principal, task
**Output**: allow or deny

**Steps**:
1. [ ] - `ph-1` - **IF** task is personal and principal is not the task owner - `inst-if-personal-deny`
2. [ ] - `ph-1` - **IF** condition is true **RETURN** deny - `inst-return-personal-deny`
3. [ ] - `ph-1` - **IF** task is team-scoped and principal is not a member of the team - `inst-if-team-deny`
4. [ ] - `ph-1` - **IF** condition is true **RETURN** deny - `inst-return-team-deny`
5. [ ] - `ph-1` - **RETURN** allow - `inst-return-allow`
<!-- fdd-id-content -->

### Validate Task Create Request

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-validate-create-task`

<!-- fdd-id-content -->
**Input**: create_task_request
**Output**: valid or error

**Steps**:
1. [ ] - `ph-1` - **IF** title is empty **RETURN** error - `inst-if-title-empty`
2. [ ] - `ph-1` - **IF** title exceeds maximum length **RETURN** error - `inst-if-title-too-long`
3. [ ] - `ph-1` - **IF** due date is present but not a valid date-time **RETURN** error - `inst-if-due-invalid`
4. [ ] - `ph-1` - **RETURN** valid - `inst-return-valid`
<!-- fdd-id-content -->

### Apply Task Updates

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-apply-task-updates`

<!-- fdd-id-content -->
**Input**: task, update_task_request
**Output**: updated task or error

**Steps**:
1. [ ] - `ph-1` - **IF** update includes status change and transition is not allowed **RETURN** error - `inst-if-status-transition-invalid`
2. [ ] - `ph-1` - **IF** update includes a title that is empty **RETURN** error - `inst-if-update-title-empty`
3. [ ] - `ph-1` - Apply allowed field updates to the task - `inst-apply-fields`
4. [ ] - `ph-1` - **RETURN** updated task - `inst-return-updated-task`
<!-- fdd-id-content -->

### Validate Assignment

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-validate-assignment`

<!-- fdd-id-content -->
**Input**: principal, task, new_assignee_user_id
**Output**: valid or error

**Steps**:
1. [ ] - `ph-1` - **IF** new assignee is null **RETURN** valid - `inst-if-unassign`
2. [ ] - `ph-1` - **IF** task is personal and new assignee is not the principal **RETURN** deny - `inst-if-personal-assignee-deny`
3. [ ] - `ph-1` - **IF** task is team-scoped and new assignee is not in team membership model **RETURN** error - `inst-if-not-member`
4. [ ] - `ph-1` - **RETURN** valid - `inst-return-valid`
<!-- fdd-id-content -->

### Create Comment

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-create-comment`

<!-- fdd-id-content -->
**Input**: principal, task, comment_body
**Output**: comment or error

**Steps**:
1. [ ] - `ph-1` - **IF** comment body is empty **RETURN** error - `inst-if-body-empty`
2. [ ] - `ph-1` - **IF** principal does not have access to the task **RETURN** deny - `inst-if-no-access`
3. [ ] - `ph-1` - Create comment with task id, author id, body, created timestamp - `inst-create-comment`
4. [ ] - `ph-1` - **RETURN** comment - `inst-return-comment`
<!-- fdd-id-content -->

### Create Team

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-algo-create-team`

<!-- fdd-id-content -->
**Input**: principal, team_name
**Output**: team or error

**Steps**:
1. [ ] - `ph-1` - **IF** team name is empty **RETURN** error - `inst-if-name-empty`
2. [ ] - `ph-1` - Create team with generated id and name - `inst-create-team`
3. [ ] - `ph-1` - Record principal as admin in internal membership model - `inst-record-admin`
4. [ ] - `ph-1` - **RETURN** team - `inst-return-team`
<!-- fdd-id-content -->

## D. States (FDL)

### Task Status State Machine

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-state-task-status`

<!-- fdd-id-content -->
**States**: `to-do`, `in-progress`, `done`
**Initial State**: `to-do`

**Transitions**:
1. [ ] - `ph-1` - **FROM** `to-do` **TO** `in-progress` **WHEN** user starts work - `inst-start-work`
2. [ ] - `ph-1` - **FROM** `in-progress` **TO** `done` **WHEN** user completes work - `inst-complete-work`
3. [ ] - `ph-1` - **FROM** `done` **TO** `in-progress` **WHEN** user reopens work - `inst-reopen-work`
4. [ ] - `ph-1` - **FROM** `in-progress` **TO** `to-do` **WHEN** user moves task back - `inst-move-back`
<!-- fdd-id-content -->

## E. Technical Details

### 1. API Endpoints (from OpenAPI)

This feature implements the business logic behind the following endpoints defined in the OpenAPI contract:

- `GET /tasks` (filters: `team_id`, `assignee_user_id`)
- `POST /tasks`
- `GET /tasks/{task_id}`
- `PATCH /tasks/{task_id}`
- `DELETE /tasks/{task_id}`
- `GET /tasks/{task_id}/comments`
- `POST /tasks/{task_id}/comments`
- `GET /teams`
- `POST /teams`

No additional endpoints are introduced by this feature.

### 2. Domain Model Alignment

This feature aligns to the schemas referenced by the overall design:

- Task schema: `schemas/task.json` and OpenAPI schema `Task`
- Comment schema: `schemas/comment.json` and OpenAPI schema `Comment`
- Team schema: `schemas/team.json` and OpenAPI schema `Team`

The feature treats the OpenAPI schema constraints as mandatory business constraints:

- Task title has a minimum length of 1 and maximum length of 200.
- Comment body has a minimum length of 1 and maximum length of 4000.
- Task status is constrained to `to-do`, `in-progress`, `done`.

### 3. Identity and Authorization

Authentication and request-level principal extraction are provided by `feature-platform-core`.

The business-domain feature requires a stable user identity in `uuid` form for:

- `Task.assignee_user_id`
- `Comment.author_user_id`

This feature assumes that the principal can be mapped to a stable user id. If the token identity is email-based, the mapping must be stable and deterministic for the lifetime of the system.

Authorization rules for `ph-1`:

- Personal task (no `team_id`): only the owner can read/update/delete; assignment is only allowed to self.
- Team task (`team_id` present): any team member can read/update/comment; delete requires admin permissions.
- Assignment for team tasks requires assignee membership in the same team.

### 4. Team Membership Model

OpenAPI currently does not provide endpoints for managing team membership.

For `ph-1` the membership and role model is treated as an internal system concern used to enforce invariants:

- Team creator is recorded as team admin.
- Only team admin is allowed to change membership and roles.
- Only team members may access team-scoped tasks.

### 5. Persistence Boundaries

SQLite connection management and transaction handling are provided by `feature-platform-core`.

This feature will require persistence support for:

- Tasks
- Comments
- Teams
- Team membership roles (internal)

The exact table structures are treated as implementation details, but must support the invariants and queries described in flows and algorithms.

### 6. Error Handling

Standard error envelope and mapping to HTTP responses are provided by `feature-platform-core`.

This feature is responsible for producing domain-level errors that can be mapped into the standard categories:

- Not found errors (task, team)
- Invalid request errors (violating schema constraints)
- Access denied errors (authorization and membership checks)
- Conflict errors where state transition or constraints are violated

## F. Requirements

### Manage Tasks

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-req-manage-tasks`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST support creating, reading, updating, listing, and deleting tasks as defined by the OpenAPI contract, including task status transitions and optional due dates.

**References**:
- [Task Management (Create, Update, Delete, List)](#task-management-create-update-delete-list)
- [Apply Task Updates](#apply-task-updates)
- [Task Status State Machine](#task-status-state-machine)

**Implements**:
- `fdd-fdd-example-feature-business-domain-flow-task-management`
- `fdd-fdd-example-feature-business-domain-algo-validate-create-task`
- `fdd-fdd-example-feature-business-domain-algo-apply-task-updates`
- `fdd-fdd-example-feature-business-domain-state-task-status`

**Phases**:
- [ ] `ph-1`: task CRUD, listing, and status transitions

**Tests Covered**:
- `fdd-fdd-example-feature-business-domain-test-task-crud`

**Acceptance Criteria**:
- Creating a task with a valid title returns a task in status `to-do`.
- Updating a task enforces status transitions and rejects invalid updates.
<!-- fdd-id-content -->

### Assign Tasks

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-req-assign-tasks`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST support assigning and unassigning tasks as defined by the OpenAPI contract and MUST enforce team membership checks for team-scoped tasks.

**References**:
- [Task Assignment (Assign, Reassign, Unassign)](#task-assignment-assign-reassign-unassign)
- [Validate Assignment](#validate-assignment)

**Implements**:
- `fdd-fdd-example-feature-business-domain-flow-task-assignment`
- `fdd-fdd-example-feature-business-domain-algo-validate-assignment`

**Phases**:
- [ ] `ph-1`: assignment and membership validation

**Tests Covered**:
- `fdd-fdd-example-feature-business-domain-test-assignment-membership`

**Acceptance Criteria**:
- Assigning a team-scoped task to a non-member is rejected.
- Unassigning a task sets `assignee_user_id` to null.
<!-- fdd-id-content -->

### Schedule Due Dates

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-req-schedule-due-dates`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST support setting, updating, and clearing due dates for tasks as defined by the OpenAPI contract.

**References**:
- [Due Date Scheduling (Set, Update, Clear)](#due-date-scheduling-set-update-clear)
- [Apply Task Updates](#apply-task-updates)

**Implements**:
- `fdd-fdd-example-feature-business-domain-flow-due-date-scheduling`
- `fdd-fdd-example-feature-business-domain-algo-apply-task-updates`

**Phases**:
- [ ] `ph-1`: due date set, update, clear

**Tests Covered**:
- `fdd-fdd-example-feature-business-domain-test-due-date-update`

**Acceptance Criteria**:
- Updating a task with a valid due date stores and returns the updated due date.
- Clearing the due date sets `due_at` to null.
<!-- fdd-id-content -->

### Comment on Tasks

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-req-task-comments`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST support creating and listing comments for a task as defined by the OpenAPI contract and MUST enforce access checks for comment operations.

**References**:
- [Comments on Tasks (Add, List)](#comments-on-tasks-add-list)
- [Authorize Task Access](#authorize-task-access)
- [Create Comment](#create-comment)

**Implements**:
- `fdd-fdd-example-feature-business-domain-flow-task-comments`
- `fdd-fdd-example-feature-business-domain-algo-authorize-task-access`
- `fdd-fdd-example-feature-business-domain-algo-create-comment`

**Phases**:
- [ ] `ph-1`: comment create and list with access checks

**Tests Covered**:
- `fdd-fdd-example-feature-business-domain-test-comment-access`

**Acceptance Criteria**:
- A user without access to a task cannot add comments to that task.
- Comment body constraints are enforced.
<!-- fdd-id-content -->

### Manage Teams

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-req-manage-teams`

<!-- fdd-id-content -->
**Status**: ⏳ NOT_STARTED
**Description**: The system MUST support listing and creating teams as defined by the OpenAPI contract and MUST record the creator as team admin in the internal membership model.

**References**:
- [Teams (Create, List)](#teams-create-list)
- [Create Team](#create-team)

**Implements**:
- `fdd-fdd-example-feature-business-domain-flow-team-management`
- `fdd-fdd-example-feature-business-domain-algo-create-team`

**Phases**:
- [ ] `ph-1`: team create and list, internal creator role recording

**Tests Covered**:
- `fdd-fdd-example-feature-business-domain-test-team-create-list`

**Acceptance Criteria**:
- Creating a team with a valid name returns a team with an id.
- Listing teams returns only teams visible to the user.
<!-- fdd-id-content -->

## G. Testing Scenarios

### Task CRUD and Status Transitions

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-test-task-crud`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-business-domain-req-manage-tasks`

**Steps**:
1. [ ] - `ph-1` - Create a task with a valid title and verify status is `to-do` - `inst-create-valid`
2. [ ] - `ph-1` - Update task status to `in-progress` and verify transition is accepted - `inst-status-in-progress`
3. [ ] - `ph-1` - Update task status to `done` and verify transition is accepted - `inst-status-done`
4. [ ] - `ph-1` - Delete the task and verify it is no longer visible - `inst-delete-task`
<!-- fdd-id-content -->

### Assignment Membership Validation

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-test-assignment-membership`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-business-domain-req-assign-tasks`

**Steps**:
1. [ ] - `ph-1` - Create a team-scoped task and record team membership - `inst-setup-team-task`
2. [ ] - `ph-1` - Attempt assignment to a non-member and verify request is rejected - `inst-assign-non-member`
3. [ ] - `ph-1` - Assign to a valid member and verify assignee is updated - `inst-assign-member`
<!-- fdd-id-content -->

### Due Date Update and Clear

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-test-due-date-update`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-business-domain-req-schedule-due-dates`

**Steps**:
1. [ ] - `ph-1` - Create a task with no due date and verify due date is null - `inst-setup-no-due`
2. [ ] - `ph-1` - Update task with a valid due date and verify it is returned - `inst-set-due`
3. [ ] - `ph-1` - Clear due date and verify due date becomes null - `inst-clear-due`
<!-- fdd-id-content -->

### Comment Access Control

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-test-comment-access`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-business-domain-req-task-comments`

**Steps**:
1. [ ] - `ph-1` - Create a team task and ensure a non-member is outside the team - `inst-setup-non-member`
2. [ ] - `ph-1` - Attempt to add comment as non-member and verify it is rejected - `inst-non-member-comment`
3. [ ] - `ph-1` - Add comment as a member and verify it is created - `inst-member-comment`
4. [ ] - `ph-1` - List comments and verify the comment is present - `inst-list-comments`
<!-- fdd-id-content -->

### Team Create and List

- [ ] **ID**: `fdd-fdd-example-feature-business-domain-test-team-create-list`

<!-- fdd-id-content -->
**Validates**: `fdd-fdd-example-feature-business-domain-req-manage-teams`

**Steps**:
1. [ ] - `ph-1` - Create a team with a valid name and verify it is returned - `inst-create-team`
2. [ ] - `ph-1` - List teams and verify the created team is present - `inst-list-teams`
<!-- fdd-id-content -->
