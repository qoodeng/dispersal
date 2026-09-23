#!/usr/bin/env bash
# One command that must pass before any push: format, lint, tests, end-to-end smoke study.
set -euo pipefail
cd "$(dirname "$0")"

echo "== Rust: fmt, clippy, tests"
(cd coarse && cargo fmt --check && cargo clippy --release --all-targets -- -D warnings && cargo test --release)

echo "== Python: ruff"
uvx ruff format --check analysis
uvx ruff check analysis

echo "== Grid is reproducible from its checksummed source"
before=$(sha256sum data/grid.json | cut -d' ' -f1)
uv run python analysis/prepare_grid.py > /dev/null
after=$(sha256sum data/grid.json | cut -d' ' -f1)
[ "$before" = "$after" ] || { echo "data/grid.json changed when regenerated"; exit 1; }

echo "== Smoke study (small, separate work directory)"
cargo build --release --manifest-path coarse/Cargo.toml -q
DISPERSAL_STUDY_WORK=runtime/smoke uv run python analysis/study.py prepare --sims 60 --seed 1 > /dev/null
DISPERSAL_STUDY_WORK=runtime/smoke uv run python analysis/study.py simulate --dt 25 > /dev/null
DISPERSAL_STUDY_WORK=runtime/smoke uv run python analysis/study.py analyze --report-dir runtime/smoke > /dev/null
echo "all checks passed"
