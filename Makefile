.PHONY: test build run validate

FDD_TOOL ?= .fdd/skills/fdd/scripts/fdd.py

build:
	npm --prefix ui run build
	cargo build

test:
	cargo test

run:
	@echo "Server running at http://127.0.0.1:8080"
	cargo run

validate:
	python3 $(FDD_TOOL) validate
