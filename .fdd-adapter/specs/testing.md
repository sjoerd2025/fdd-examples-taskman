---
fdd: true
type: adapter-spec
name: Testing
version: 1.0
purpose: Define test locations, execution commands, and traceability expectations
---

# Testing

## Test Locations

- Integration/E2E tests: `tests/`

## Execution

- Run all tests:
  - `cargo test`

## Traceability

- Tests should include traceability tags as comments where applicable:
  - `// @fdd-test:{test-id}:ph-{N}`

## Sources

- `tests/` directory
- `.fdd/requirements/feature-changes-structure.md`

## Validation Checklist

- [ ] Test command specified
- [ ] Test locations listed
- [ ] Traceability requirement stated
