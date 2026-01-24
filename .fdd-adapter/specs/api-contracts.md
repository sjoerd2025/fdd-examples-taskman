---
fdd: true
type: adapter-spec
name: API Contracts
version: 1.0
purpose: Define authoritative API contract format, location, and boundary rules
---

# API Contracts

## Authoritative Format

- OpenAPI 3.0.x

## Location

- Contract file: `api/openapi.yaml`

## Boundary Rules

- OpenAPI is the source of truth for request/response shapes.
- Feature DESIGN documents should reference endpoints and error schemas from OpenAPI rather than redefining them.

## Sources

- `api/openapi.yaml`
- `architecture/DESIGN.md` (Section C.3)
- `architecture/ADR/general/0001-fdd-fdd-example-adr-initial-architecture.md`

## Validation Checklist

- [ ] Contract format and version family stated
- [ ] Contract file path stated
- [ ] Boundary rules listed
