#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$ROOT_DIR/bin"
JAVA_OUT_DIR="$ROOT_DIR/java/out"
SAVE_FILE="$ROOT_DIR/save/progress.json"

echo "Cleaning build artifacts..."
rm -rf "$BIN_DIR" "$JAVA_OUT_DIR"

echo "Removing saved progress..."
rm -f "$SAVE_FILE"

echo "Clean complete."
