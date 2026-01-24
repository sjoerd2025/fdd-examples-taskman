# ADR-0001: Initial Architecture for On-Prem Task Management API

**Date**: 2026-01-23

**Status**: Proposed

**ADR ID**: `fdd-fdd-example-adr-initial-architecture`

## Context and Problem Statement

We need to define the initial architecture and core technology choices for a Task Management API that supports individual users and teams. The system is intended to run on-premises, which constrains dependency choices and favors operational simplicity.

This decision must establish a coherent baseline for implementation and future evolution, including persistence, primary implementation language, and API contract approach.

## Decision Drivers

* On-prem deployment requirement and operational simplicity
* Ability to ship as a single deployable unit early (modular monolith)
* Clear separation of domain logic from infrastructure (maintainability)
* Strong correctness and safety properties for a networked multi-user system
* Stable, tool-friendly API contract for clients and integrations

## Considered Options

* Modular monolith using Hexagonal (Ports & Adapters) architecture, implemented in Rust, with SQLite persistence and OpenAPI as the API contract
* Layered monolith implemented in a dynamic language (e.g., Python/Node.js) with PostgreSQL as the database and OpenAPI as the API contract
* Microservices from the start, implemented in Rust, with PostgreSQL and service-to-service APIs (gRPC/OpenAPI hybrid)

## Decision Outcome

Chosen option: "Modular monolith using Hexagonal (Ports & Adapters) architecture, implemented in Rust, with SQLite persistence and OpenAPI as the API contract", because it fits on-prem constraints, keeps early development and deployment simple, and still provides clear seams (ports) to evolve adapters or split components later.

### Consequences

* Good, because SQLite provides a low-operational-overhead on-prem storage option and enables shipping a single binary with a local database file.
* Good, because Rust provides strong safety guarantees and predictable performance for an API server handling concurrent requests.
* Good, because an OpenAPI-first contract makes integrations easier (documentation, client generation, validation) and provides a stable boundary.
* Bad, because SQLite has practical concurrency limitations (single-writer) that may require careful transaction boundaries and retry/backoff under write contention.
* Bad, because Rust development velocity may be lower initially compared to some higher-level stacks, especially for rapid prototyping.

## Related Design Elements

**Actors**:
* `fdd-fdd-example-actor-external-client` - Integrates via the REST API using an OpenAPI-described contract.
* `fdd-fdd-example-actor-team-admin` - Drives team-level workflows that require consistent authorization and data integrity.

**Capabilities**:
* `fdd-fdd-example-capability-task-management` - Requires stable persistence, clear API boundaries, and consistent business invariants.
* `fdd-fdd-example-capability-team-management` - Requires role-based access control and reliable membership persistence.

**Requirements**:
* `fdd-fdd-example-req-authenticate-api` - Needs a consistent auth boundary at the API adapter and enforcement in application services.

**Principles**:
* `fdd-fdd-example-principle-domain-isolation` - Motivates Hexagonal boundaries and adapter-based integration.
* `fdd-fdd-example-principle-on-prem-simplicity` - Motivates SQLite and a modular monolith over cloud-managed dependencies.
