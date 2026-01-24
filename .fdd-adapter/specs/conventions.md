---
fdd: true
type: adapter-spec
name: Conventions
version: 1.0
purpose: Define coding style, naming, and repository conventions
---

# Conventions

## Language

- Rust 2021 edition

## File / Module Structure

- Application code: `src/`
- Integration/E2E tests: `tests/`

## FDD Tagging Conventions (Code)

- Use paired tags:
  - `// fdd-begin {id}:ph-{N}:inst-{id}`
  - `// fdd-end   {id}:ph-{N}:inst-{id}`
- Always include `:ph-{N}` postfix on feature-scoped tags.

## Sources

- `src/*.rs` (existing tag usage)
- `.fdd/requirements/FDL.md`
- `.fdd/requirements/feature-changes-structure.md`

## Validation Checklist

- [ ] Code tagging rules included
- [ ] Directory conventions included
