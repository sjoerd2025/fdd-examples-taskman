---
fdd: true
type: adapter-spec
name: Architecture Patterns
version: 1.0
purpose: Describe architecture patterns and how code is organized
---

# Architecture Patterns

## Primary Pattern

- Modular monolith using Hexagonal (Ports & Adapters)

## Practical Indicators in This Repo

- HTTP boundary: `src/app.rs` (axum routes) + request extraction
- Auth boundary: `src/auth.rs`
- Persistence adapter: `src/db.rs` (sqlx/sqlite)
- Cross-cutting policies: `src/error.rs`, `src/tx.rs`, `src/audit.rs`

## Sources

- `architecture/ADR/general/0001-fdd-fdd-example-adr-initial-architecture.md`
- `architecture/DESIGN.md` (Section A, C.1)

## Validation Checklist

- [ ] At least one pattern listed
- [ ] Pattern mapped to concrete folders/files
- [ ] Sources referenced
