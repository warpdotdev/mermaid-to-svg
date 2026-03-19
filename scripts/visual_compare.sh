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

# Build the render binary.
echo "Building render_mermaid..."
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml" --bin render_mermaid -q
RENDER_BIN="$PROJECT_DIR/target/debug/render_mermaid"

# Create a puppeteer config for mermaid-cli.
PUPPETEER_CONFIG="$OUTPUT_DIR/.puppeteer-config.json"
echo '{"args": ["--no-sandbox", "--disable-setuid-sandbox"]}' > "$PUPPETEER_CONFIG"

if [ -n "$TYPE_FILTER" ]; then
  PATTERN="$SAMPLES_ROOT/$TYPE_FILTER/mermaid"'/*.mmd'
else
  PATTERN="$SAMPLES_ROOT"'/*/mermaid/*.mmd'
fi

# Collect entries for HTML generation.
ENTRIES=()

for mmd_file in $PATTERN; do
  if [ ! -f "$mmd_file" ]; then
    continue
  fi

  diagram_type=$(basename "$(dirname "$(dirname "$mmd_file")")")
  name=$(basename "$mmd_file" .mmd)
  key="${diagram_type}_${name}"
  echo "Processing $key..."

  # Generate reference SVG with mermaid-cli.
  npx -y \
    -p "@mermaid-js/mermaid-cli@$MERMAID_CLI_VERSION" \
    -p "mermaid@$MERMAID_VERSION" \
    mmdc \
    -i "$mmd_file" \
    -o "$OUTPUT_DIR/ref_${key}.svg" \
    -b white \
    -p "$PUPPETEER_CONFIG" \
    2>/dev/null || echo "  Warning: mermaid-cli failed for $key"

  # Generate our SVG.
  "$RENDER_BIN" "$mmd_file" > "$OUTPUT_DIR/our_${key}.svg" 2>/dev/null || echo "  Warning: render_mermaid failed for $key"

  ENTRIES+=("$key")
  echo "  Done: $key"
done

# Generate the HTML comparison page.
HTML_FILE="$OUTPUT_DIR/comparison.html"
cat > "$HTML_FILE" << 'HEADER'
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Mermaid SVG Visual Comparison</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #f5f5f5; padding: 20px; }
  h1 { margin-bottom: 20px; }
  .nav { margin-bottom: 20px; display: flex; flex-wrap: wrap; gap: 6px; }
  .nav a { font-size: 12px; padding: 4px 8px; background: #e0e0e0; border-radius: 4px; text-decoration: none; color: #333; }
  .nav a:hover { background: #ccc; }
  .entry { margin-bottom: 40px; border: 1px solid #ddd; border-radius: 8px; background: #fff; padding: 16px; }
  .entry h2 { margin-bottom: 12px; font-size: 16px; color: #333; }
  .comparison { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
  .side { border: 1px solid #eee; border-radius: 4px; padding: 8px; }
  .side h3 { font-size: 13px; color: #666; margin-bottom: 8px; text-align: center; }
  .side img { max-width: 100%; height: auto; display: block; margin: 0 auto; }
  .source { margin-top: 12px; }
  .source summary { cursor: pointer; font-size: 13px; color: #666; }
  .source pre { margin-top: 8px; padding: 8px; background: #f8f8f8; border-radius: 4px; font-size: 12px; overflow-x: auto; }
</style>
</head>
<body>
<h1>Mermaid SVG Visual Comparison</h1>
<p style="margin-bottom:16px;color:#666;">Our rendering (left) vs mermaid-cli reference (right)</p>
<div class="nav">
HEADER

# Write nav links.
for key in "${ENTRIES[@]}"; do
  echo "  <a href=\"#${key}\">${key}</a>" >> "$HTML_FILE"
done

echo '</div>' >> "$HTML_FILE"

# Write each comparison entry.
for key in "${ENTRIES[@]}"; do
  # Read the source mermaid if available.
  diagram_type="${key%%_*}"
  name="${key#*_}"
  mmd_file="$SAMPLES_ROOT/$diagram_type/mermaid/${name}.mmd"

  cat >> "$HTML_FILE" << ENTRY
<div class="entry" id="${key}">
  <h2>${key}</h2>
  <div class="comparison">
    <div class="side">
      <h3>Ours (mermaid_to_svg)</h3>
      <img src="our_${key}.svg" alt="Our rendering of ${key}">
    </div>
    <div class="side">
      <h3>Reference (mermaid-cli)</h3>
      <img src="ref_${key}.svg" alt="Reference rendering of ${key}">
    </div>
  </div>
ENTRY

  if [ -f "$mmd_file" ]; then
    echo '  <details class="source"><summary>Show Mermaid source</summary><pre>' >> "$HTML_FILE"
    # HTML-escape the source.
    sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g' "$mmd_file" >> "$HTML_FILE"
    echo '</pre></details>' >> "$HTML_FILE"
  fi

  echo '</div>' >> "$HTML_FILE"
done

cat >> "$HTML_FILE" << 'FOOTER'
</body>
</html>
FOOTER

echo ""
echo "Done! Open the comparison page:"
echo "  open $HTML_FILE"
echo ""
