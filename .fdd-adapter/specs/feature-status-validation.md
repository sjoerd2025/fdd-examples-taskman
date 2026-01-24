---
fdd: true
type: adapter-spec
name: Feature Status Validation
version: 1.0
purpose: Define code quality gates for marking a feature as implemented
---

# Feature Status Validation

## Prohibited in Production Code

- `unimplemented!()`
- `panic!()` used as control flow
- Bare `.unwrap()` / `.expect()` in non-test code unless justified

## Prohibited in Feature Implementation Code

- Incomplete-work markers in code areas tagged with feature `@fdd-change` / `@fdd-flow` / `@fdd-algo` / `@fdd-state`

## Tests

- No `#[ignore]` tests for feature scenarios unless there is a documented reason.

## Sources

- `.fdd/workflows/feature-code-validate.md` (workflow requirements)

## Validation Checklist

- [ ] Prohibited markers listed
- [ ] Test ignore policy stated
