---
fdd: true
type: adapter-spec
name: Domain Model
version: 1.0
purpose: Define authoritative domain schema format and locations
---

# Domain Model

## Authoritative Format

- JSON Schema (Draft 2020-12)

## Locations

- Schemas directory: `schemas/`

## Files

- `schemas/task.json`
- `schemas/comment.json`
- `schemas/team.json`
- `schemas/user.json`

## Usage Rules

- Treat `schemas/` as the source of truth for entity shape.
- Feature DESIGN documents must reference schemas rather than re-defining entity shapes.

## Sources

- `schemas/*.json`
- `architecture/DESIGN.md` (Section C.2)

## Validation Checklist

- [ ] Schema format stated explicitly
- [ ] Schema locations listed
- [ ] At least one example schema file listed
