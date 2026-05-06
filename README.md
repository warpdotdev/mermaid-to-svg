# mermaid_to_svg

A pure Rust crate for converting [Mermaid](https://mermaid.js.org/) diagram syntax to SVG.

## Architecture

```
Mermaid text → Parser → AST → Layout (dagre_rust) → SVG Renderer → SVG string
```

We use `dagre_rust` (a Rust port of dagre.js, the same layout engine mermaid.js uses) for graph layout, then render SVG directly. This gives us:

- **Pure Rust** — No external runtime dependencies (no Node.js, no headless browser)
- **Same layout algorithms as mermaid.js** — dagre_rust provides node positioning and edge routing
- **Reasonable scope** — We focus on parsing and rendering, delegating layout to dagre

## Supported Diagram Types

### Flowcharts (primary)

- Graph direction: `graph TD`, `graph LR`, `graph TB`, `graph RL`, `graph BT`
- Node shapes: rectangle, rounded, stadium/pill, diamond, hexagon, asymmetric/flag, subroutine, cylinder, circle
- Edge types: arrow, line, dotted, thick — all with optional labels
- Subgraphs: `subgraph title ... end`
- Basic inline styling: `style A fill:#f9f,stroke:#333`

### Experimental / In Progress

These render something but are not yet at mermaid.js parity:

- `erDiagram`, `classDiagram`, `stateDiagram` / `stateDiagram-v2`
- `sequenceDiagram`, `mindmap`, `timeline`, `journey`
- `quadrantChart`, `pie`, `gantt`

## Usage

```rust
use mermaid_to_svg::{render_mermaid_to_svg, MermaidTheme};

let mermaid = r#"
graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Action 1]
    B -->|No| D[Action 2]
"#;

// With default theme
let svg = render_mermaid_to_svg(mermaid, None)?;

// With custom theme
let theme = MermaidTheme {
    background: "#1e1e1e".into(),
    node_fill: "#2d2d2d".into(),
    node_stroke: "#888888".into(),
    text_color: "#ffffff".into(),
    edge_color: "#888888".into(),
};
let svg = render_mermaid_to_svg(mermaid, Some(&theme))?;
```

### CLI

```bash
# From a file
cargo run --bin render_mermaid -- diagram.mmd > output.svg

# From stdin
echo 'graph TD; A-->B' | cargo run --bin render_mermaid > output.svg
```

## Visual Comparison

See [docs/visual-comparison.md](docs/visual-comparison.md) for a side-by-side comparison of our rendering output against the canonical mermaid-cli across 80 sample diagrams.

See [docs/agent-visual-verification.md](docs/agent-visual-verification.md) for the agent prompt and visual verification loop used to iterate against Mermaid reference output.

## Testing

### Unit Tests

```bash
cargo test --lib
```

### Full Test Suite

```bash
cargo test
```

### Visual Comparison

To visually compare rendering output against the canonical mermaid CLI:

```bash
./scripts/visual_compare.sh
```

This generates a static HTML page at `output/comparison.html` showing our SVG output side-by-side with the mermaid CLI reference for each sample diagram.

### Snapshot Tests

Snapshot tests use `insta`. To update snapshots:

```bash
cargo insta review
```

### Regenerating Reference SVGs

```bash
./scripts/generate_reference_svgs.sh
```

## Module Structure

- `lib.rs` — Public API
- `parser.rs` — Mermaid flowchart syntax parser
- `ast.rs` — Abstract syntax tree types
- `layout.rs` — Uses dagre_rust for graph layout
- `svg_renderer.rs` — Converts layout result to SVG
- `theme.rs` — Theme colors for light/dark mode support
- `error.rs` — Error types

## License

MIT — see [LICENSE](LICENSE).

This project includes code derived from mermaid.js, dagre.js, and dagre_rust.
See [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES) for details.

Warp requires contributors to sign a contributor license agreement (CLA) before their contributions can be merged. You can read and sign our CLA at https://cla.warp.dev.
