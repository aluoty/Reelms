#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JAVA_OUT_DIR="$ROOT_DIR/java/out"

if [[ ! -d "$JAVA_OUT_DIR" ]]; then
  echo "Java classes not found. Run ./build.sh first."
  exit 1
fi

java -cp "$JAVA_OUT_DIR" com.fishinggame.Main
