#!/usr/bin/env bash

# Build and run tests for rembg-rs
# Usage: ./test-build.sh
# This script builds the project in release mode (with cli feature) and then
# runs test.sh which processes images in test_input/ using models/.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"

echo "🛠️  Building rembg-rs (release, cli feature)"
# Build release with cli; if you only need the library, remove --features cli
cargo build --release --features cli

echo "✅ Build finished"

echo "▶️  Running test.sh"
# Ensure test.sh is executable
chmod +x "$REPO_ROOT/test.sh"

# Run tests
"$REPO_ROOT/test.sh"

echo "✅ Test run completed"
