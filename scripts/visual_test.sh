#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SAMPLES_ROOT="$PROJECT_DIR/samples"
OUTPUT_DIR="$PROJECT_DIR/output"

# Pinned mermaid-cli + mermaid versions (matches scripts/generate_reference_svgs.sh)
MERMAID_CLI_VERSION="${MERMAID_CLI_VERSION:-11.4.2}"
MERMAID_VERSION="${MERMAID_VERSION:-11.12.2}"

TYPE_FILTER="${1:-}"

if ! command -v npx >/dev/null 2>&1; then
  echo "error: npx is required (Node.js)." >&2
  exit 1
fi

mkdir -p "$OUTPUT_DIR"

# Create a puppeteer config to pass --no-sandbox (required when running as root)
PUPPETEER_CONFIG="$OUTPUT_DIR/.puppeteer-config.json"
echo '{"args": ["--no-sandbox", "--disable-setuid-sandbox"]}' > "$PUPPETEER_CONFIG"

# Locate the pinned mermaid-cli binary once so we can:
# - avoid re-running npx installs per fixture
# - reuse the same browser/stack for both reference PNG generation and our SVG rasterization
MMDC_BIN=$(npx -y \
  -p "@mermaid-js/mermaid-cli@$MERMAID_CLI_VERSION" \
  -p "mermaid@$MERMAID_VERSION" \
  -c "which mmdc")
NPX_ROOT=$(dirname "$(dirname "$(dirname "$MMDC_BIN")")")
NPX_NODE_MODULES="$NPX_ROOT/node_modules"

# Build the render binary once.
echo "Building render_mermaid..."
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml" --bin render_mermaid -q
RENDER_BIN="$PROJECT_DIR/target/debug/render_mermaid"

if [ -n "$TYPE_FILTER" ]; then
  PATTERN="$SAMPLES_ROOT/$TYPE_FILTER/mermaid"'/*.mmd'
else
  PATTERN="$SAMPLES_ROOT"'/*/mermaid/*.mmd'
fi

# Process each sample.
for mmd_file in $PATTERN; do
  if [ ! -f "$mmd_file" ]; then
    continue
  fi

  diagram_type=$(basename "$(dirname "$(dirname "$mmd_file")")")
  name=$(basename "$mmd_file" .mmd)
  key="${diagram_type}_${name}"
  echo "Processing $key..."

  # Generate reference PNG with pinned mermaid-cli.
  "$MMDC_BIN" \
    -p "$SCRIPT_DIR/puppeteer-config.json" \
    -i "$mmd_file" \
    -o "$OUTPUT_DIR/ref_${key}.png" \
    -s 2 \
    -b white \
    -p "$PUPPETEER_CONFIG" \
    2>/dev/null

  # Generate our SVG (using the already-built binary).
  "$RENDER_BIN" "$mmd_file" > "$OUTPUT_DIR/our_${key}.svg"

  # Rasterize our SVG to PNG using Chromium (via puppeteer).
  # This avoids font/rasterization differences between mermaid-cli (Chromium) and rsvg-convert.
  NODE_PATH="$NPX_NODE_MODULES" node "$SCRIPT_DIR/rasterize_svg_with_puppeteer.js" \
    "$OUTPUT_DIR/our_${key}.svg" \
    "$OUTPUT_DIR/our_${key}.png" \
    2 \
    white

  echo "  Generated: ref_${key}.png, our_${key}.png"
done

echo ""
echo "Done! PNG pairs are in: $OUTPUT_DIR"
echo ""
