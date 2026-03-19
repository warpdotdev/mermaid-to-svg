#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SAMPLES_ROOT="$PROJECT_DIR/samples"

# Pinned mermaid-cli + mermaid versions used for generating canonical reference SVGs.
# If you bump either, regenerate all reference SVG fixtures.
MERMAID_CLI_VERSION="${MERMAID_CLI_VERSION:-11.4.2}"
MERMAID_VERSION="${MERMAID_VERSION:-11.12.2}"

TYPE_FILTER="${1:-}"

if ! command -v npx >/dev/null 2>&1; then
  echo "error: npx is required (Node.js)." >&2
  exit 1
fi

if [ -n "$TYPE_FILTER" ]; then
  PATTERN="$SAMPLES_ROOT/$TYPE_FILTER/mermaid"'/*.mmd'
else
  PATTERN="$SAMPLES_ROOT"'/*/mermaid/*.mmd'
fi
# Create a puppeteer config to pass --no-sandbox (required when running as root)
TMP_PUPPETEER_CONFIG=$(mktemp)
echo '{"args": ["--no-sandbox", "--disable-setuid-sandbox"]}' > "$TMP_PUPPETEER_CONFIG"
trap 'rm -f "$TMP_PUPPETEER_CONFIG"' EXIT

for mmd_file in $PATTERN; do
  if [ ! -f "$mmd_file" ]; then
    continue
  fi

  diagram_type=$(basename "$(dirname "$(dirname "$mmd_file")")") 
  name=$(basename "$mmd_file" .mmd)

  out_dir="$SAMPLES_ROOT/$diagram_type/reference"
  out_png="$out_dir/$name.png"

  mkdir -p "$out_dir"

  echo "Generating reference PNG: $diagram_type/$name"

  npx -y \
    -p "@mermaid-js/mermaid-cli@$MERMAID_CLI_VERSION" \
    -p "mermaid@$MERMAID_VERSION" \
    mmdc \
    -i "$mmd_file" \
    -o "$out_png" \
    -s 2 \
    -b white \
    -p "$TMP_PUPPETEER_CONFIG" \
    2>/dev/null

done

echo "Done."
