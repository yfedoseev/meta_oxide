# Makefile for meta_oxide — the Universal Metadata Extraction Library.
#
# Common development tasks across the Rust core, Python (maturin),
# Node.js / WASM (npm), and Java / C# bindings.

.PHONY: help dev install \
        test test-lib test-all test-proptest test-snapshot test-coverage test-verbose \
        build build-wheel build-wheels-all build-node build-wasm \
        bench fmt fmt-check lint clippy doc \
        check check-rust check-node check-all \
        semver-check release-notes \
        clean

# --- Development ---------------------------------------------------------

# Fast Python rebuild (editable install via maturin develop)
dev:
	maturin develop --features python

# Release-mode install (optimised extension in the active venv)
install:
	maturin develop --release --features python

# --- Testing --------------------------------------------------------------

test: test-all

# Library-only tests — the fast gate (~0.1s)
test-lib:
	cargo test --lib --offline

# Full test suite: lib + integration + snapshot + proptest (25 × 256 cases)
test-all:
	cargo test --offline --tests

# Proptest soak — default 256 cases per property; override with PROPTEST_CASES=N
test-proptest:
	cargo test --test proptest_parser --offline

# Insta snapshot tests — use `cargo insta review` to approve diffs
test-snapshot:
	cargo test --test snapshot_fixtures --offline

test-verbose:
	cargo test --offline --tests -- --nocapture

# Line coverage — requires `cargo install cargo-llvm-cov`
test-coverage:
	cargo llvm-cov --lib --offline --summary-only

# --- Building -------------------------------------------------------------

build:
	cargo build --release

build-wheel:
	maturin build --release --features python

# Multi-Python-version wheel build (requires each interpreter on PATH)
build-wheels-all:
	maturin build --release --features python \
	  --interpreter python3.9 python3.10 python3.11 python3.12 python3.13

build-node:
	cd bindings/node && npm ci && npm run build

build-wasm:
	cd bindings/wasm && npm ci && npm run build

bench:
	cargo bench --offline

# --- Code quality ---------------------------------------------------------

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint: clippy

clippy:
	cargo clippy --all-targets --offline -- -D warnings

doc:
	cargo doc --no-deps --offline

# --- Composite gates ------------------------------------------------------

check-rust: fmt-check clippy test-all

check-node:
	cd bindings/node && npm ci && npm run build && npx tsc --noEmit
	cd bindings/wasm && npm ci

check: check-rust

# Everything — what CI runs
check-all: check-rust check-node semver-check

# SemVer compatibility check against the latest crates.io release.
# Requires `cargo install cargo-semver-checks`.
semver-check:
	cargo semver-checks check-release

# Preview release notes locally (reads CHANGELOG.md for the given version)
release-notes:
	@[ -n "$(VERSION)" ] || (echo "Usage: make release-notes VERSION=0.1.3"; exit 1)
	.github/scripts/extract-release-notes.sh $(VERSION)
	@echo ""
	@echo "=== release-title.txt ==="
	@cat release-title.txt
	@echo ""
	@echo "=== release-notes.md ==="
	@cat release-notes.md

# --- Cleanup --------------------------------------------------------------

clean:
	cargo clean
	rm -rf target/ bindings/node/target/ bindings/wasm/pkg/ bindings/wasm/pkg-node/ bindings/wasm/pkg-bundler/
	rm -rf bindings/node/index.js bindings/node/index.d.ts bindings/node/*.node
	rm -rf dist/ *.egg-info/
	find . -name '__pycache__' -type d -prune -exec rm -rf {} + 2>/dev/null || true

# --- Help -----------------------------------------------------------------

help:
	@echo "meta_oxide — Makefile targets"
	@echo ""
	@echo "Development:"
	@echo "  make dev              Fast Python rebuild (maturin develop)"
	@echo "  make install          Release Python install"
	@echo ""
	@echo "Testing:"
	@echo "  make test             Run the full test suite (alias of test-all)"
	@echo "  make test-lib         Lib tests only (fast)"
	@echo "  make test-proptest    Property-based invariants (PROPTEST_CASES=N to crank)"
	@echo "  make test-snapshot    Insta snapshot tests"
	@echo "  make test-coverage    Line coverage via cargo-llvm-cov"
	@echo ""
	@echo "Build:"
	@echo "  make build            Cargo release build"
	@echo "  make build-wheel      Build a Python wheel (current interpreter)"
	@echo "  make build-wheels-all Wheels for Python 3.9..3.13"
	@echo "  make build-node       Build Node.js binding"
	@echo "  make build-wasm       Build WASM binding"
	@echo "  make bench            Criterion benchmarks"
	@echo ""
	@echo "Quality:"
	@echo "  make fmt / fmt-check  Format Rust code / check only"
	@echo "  make clippy           Run Clippy with -D warnings"
	@echo "  make doc              Build API docs"
	@echo "  make semver-check     cargo-semver-checks vs. latest crates.io"
	@echo ""
	@echo "Gates:"
	@echo "  make check-rust       fmt + clippy + tests"
	@echo "  make check-node       Node + WASM binding build + typecheck"
	@echo "  make check-all        Everything CI runs"
	@echo ""
	@echo "Release:"
	@echo "  make release-notes VERSION=0.1.3"
	@echo "                        Preview the GitHub release body for VERSION"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean            Remove all build artefacts"
