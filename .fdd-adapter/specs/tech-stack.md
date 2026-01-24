---
fdd: true
type: adapter-spec
name: Tech Stack
version: 1.0
purpose: Define project runtime tech stack and supported tooling
---

# Tech Stack

## Languages

- Rust (edition 2021)
- TypeScript

## Frameworks / Runtime

- Axum 0.7
- Tokio 1.x
- React (first-party web UI)

## Data / Persistence

- SQLite
- SQLx 0.7 (sqlite runtime)

## Serialization

- serde / serde_json
- serde_yaml

## Observability

- tracing
- tracing-subscriber

## Sources

- `Cargo.toml`
- `architecture/ADR/general/0001-fdd-fdd-example-adr-initial-architecture.md`
- `architecture/DESIGN.md`

## Validation Checklist

- [ ] Language is listed with version/edition
- [ ] Major frameworks and database are listed
- [ ] Each major item has a source reference
