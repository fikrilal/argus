#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== [1/4] Checking Rust code formatting (cargo fmt) ==="
cargo fmt --check

echo "=== [2/4] Running Clippy linter (cargo clippy) ==="
cargo clippy --all-targets --all-features -- -D warnings

echo "=== [3/4] Running automated test suite (cargo test) ==="
cargo test

echo "=== [4/4] Building release binary (cargo build --release) ==="
cargo build --release

echo ""
echo "All verification gates passed cleanly! ✓"
