---
fdd: true
type: adapter-spec
name: Build & Deploy
version: 1.0
purpose: Define build commands and runtime expectations
---

# Build & Deploy

## Build

- Debug build:
  - `cargo build`

## Formatting

- Check formatting:
  - `cargo fmt --check`

## Lint

- Clippy:
  - `cargo clippy --all-targets --all-features`

## Run

- Local run:
  - `cargo run`

## Sources

- `Cargo.toml`
- `src/main.rs` (runtime wiring)

## Validation Checklist

- [ ] Build command specified
- [ ] Formatting command specified
- [ ] Lint command specified
- [ ] Commands match Rust toolchain
