#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

MERMAID_VERSION="11.12.2"
OUT_DIR="$PROJECT_DIR/third_party/mermaid-js/$MERMAID_VERSION"

if ! command -v npm >/dev/null 2>&1; then
  echo "error: npm is required (Node.js)." >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

if [ -f "$OUT_DIR/package.json" ]; then
  echo "Already present: $OUT_DIR"
  exit 0
fi

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

cd "$tmp_dir"

tarball="$(npm pack "mermaid@$MERMAID_VERSION" --silent)"

mkdir -p extract

tar -xzf "$tarball" -C extract

# npm pack produces a top-level "package/" directory
cp -R extract/package/* "$OUT_DIR/"

echo "Vendored mermaid@$MERMAID_VERSION into: $OUT_DIR"