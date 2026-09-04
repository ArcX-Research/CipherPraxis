SHELL := /bin/bash

CARGO ?= cargo
PYTHON ?= python3
HOST ?= 127.0.0.1
PORT ?= 8787

.DEFAULT_GOAL := help

.PHONY: help fmt fmt-check lint lint-rust lint-scripts test content check build build-dev dev serve clean

help:
	@printf '%s\n' \
		'Cipher Praxis development commands:' \
		'  make fmt          Format Rust sources' \
		'  make fmt-check    Verify Rust formatting' \
		'  make lint         Run Rust and script linters' \
		'  make test         Run the workspace test suite' \
		'  make content      Validate the content catalog' \
		'  make check        Run the complete project check' \
		'  make build        Build the release site in dist/' \
		'  make build-dev    Build the development site in dist/' \
		'  make dev          Build, serve, watch, and reload' \
		'  make serve        Serve the existing dist/ directory' \
		'  make clean        Remove generated build output'

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint: lint-rust lint-scripts

lint-rust:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

lint-scripts:
	@for script in scripts/*.sh; do bash -n "$$script"; done
	@$(PYTHON) -c 'import ast, pathlib; [ast.parse(path.read_text(), filename=str(path)) for path in pathlib.Path("scripts").glob("*.py")]'
	@if command -v shellcheck >/dev/null 2>&1; then shellcheck scripts/*.sh; else printf '%s\n' 'shellcheck not installed; skipped'; fi

test:
	$(CARGO) test --workspace --all-features

content:
	$(CARGO) run --quiet -p praxis-core --features authoring --bin praxis-check -- content --strict

check:
	./scripts/check.sh

build:
	./scripts/build.sh release

build-dev:
	./scripts/build.sh dev

dev:
	./scripts/dev.sh --host "$(HOST)" --port "$(PORT)"

serve:
	$(PYTHON) scripts/serve.py --host "$(HOST)" --port "$(PORT)" --no-build

clean:
	$(CARGO) clean
	rm -rf "$(CURDIR)/dist"
