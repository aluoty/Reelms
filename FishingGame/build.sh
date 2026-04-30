#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CPP_DIR="$ROOT_DIR/cpp"
JAVA_SRC_DIR="$ROOT_DIR/java/src"
JAVA_OUT_DIR="$ROOT_DIR/java/out"
BIN_DIR="$ROOT_DIR/bin"

mkdir -p "$BIN_DIR" "$JAVA_OUT_DIR"

echo "Building C++ fish engine..."
g++ -std=c++17 -O2 "$CPP_DIR/fish_engine.cpp" -o "$BIN_DIR/fish_engine"

echo "Compiling Java UI..."
javac -d "$JAVA_OUT_DIR" $(find "$JAVA_SRC_DIR" -name "*.java")

echo "Build complete."
