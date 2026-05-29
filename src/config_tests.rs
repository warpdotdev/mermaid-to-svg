use super::*;
use crate::theme::{MermaidThemePreset, MermaidThemeVariables};

#[test]
fn parse_valid_frontmatter_returns_body_and_config() {
    let source = r##"---
title: Demo
config:
  theme: dark
  layout: elk
  look: handDrawn
  securityLevel: loose
  fontFamily: Inter
  fontSize: 16px
  themeVariables:
    background: "#010203"
    primaryColor: "#111111"
    primaryBorderColor: "#222222"
    primaryTextColor: "#333333"
    lineColor: "#444444"
    clusterBkg: "#555555"
    clusterBorder: "#666666"
  flowchart:
    curve: basis
    htmlLabels: false
    nodeSpacing: 12
    rankSpacing: "34"
    padding: 8
    diagramPadding: 9
    wrappingWidth: 120
    useMaxWidth: true
    defaultRenderer: dagre-wrapper
---
flowchart TD
  A --> B
"##;

    let parsed = parse_mermaid_frontmatter(source);

    assert_eq!(parsed.body, "flowchart TD\n  A --> B\n");
    assert_eq!(
        parsed
            .frontmatter
            .as_ref()
            .and_then(|frontmatter| frontmatter.title.as_ref()),
        Some(&"Demo".to_string())
    );
    assert_eq!(parsed.config.theme, Some(MermaidThemePreset::Dark));
    assert_eq!(parsed.config.layout, Some("elk".to_string()));
    assert_eq!(parsed.config.look, Some("handDrawn".to_string()));
    assert_eq!(parsed.config.security_level, Some("loose".to_string()));
    assert_eq!(parsed.config.font_family, Some("Inter".to_string()));
    assert_eq!(parsed.config.font_size, Some("16px".to_string()));
    assert_eq!(
        parsed.config.theme_variables.background,
        Some("#010203".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.node_fill,
        Some("#111111".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.node_stroke,
        Some("#222222".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.text_color,
        Some("#333333".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.edge_color,
        Some("#444444".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.subgraph_fill,
        Some("#555555".to_string())
    );
    assert_eq!(
        parsed.config.theme_variables.subgraph_stroke,
        Some("#666666".to_string())
    );
    assert_eq!(parsed.config.flowchart.curve, Some("basis".to_string()));
    assert_eq!(parsed.config.flowchart.html_labels, Some(false));
    assert_eq!(parsed.config.flowchart.node_spacing, Some(12));
    assert_eq!(parsed.config.flowchart.rank_spacing, Some(34));
    assert_eq!(parsed.config.flowchart.padding, Some(8));
    assert_eq!(parsed.config.flowchart.diagram_padding, Some(9));
    assert_eq!(parsed.config.flowchart.wrapping_width, Some(120));
    assert_eq!(parsed.config.flowchart.use_max_width, Some(true));
    assert_eq!(
        parsed.config.flowchart.default_renderer,
        Some("dagre-wrapper".to_string())
    );
}

#[test]
fn malformed_yaml_frontmatter_is_stripped_without_config() {
    let source = "---\nconfig:\n  theme: [dark\n---\ngraph TD\nA --> B\n";

    let parsed = parse_mermaid_frontmatter(source);

    assert_eq!(parsed.body, "graph TD\nA --> B\n");
    assert_eq!(parsed.config, RenderConfig::default());
    assert_eq!(parsed.frontmatter, Some(MermaidFrontmatter::default()));
}

#[test]
fn supported_theme_names_map_to_presets() {
    let cases = [
        ("default", MermaidThemePreset::Default),
        ("base", MermaidThemePreset::Base),
        ("dark", MermaidThemePreset::Dark),
        ("forest", MermaidThemePreset::Forest),
        ("neutral", MermaidThemePreset::Neutral),
    ];

    for (name, preset) in cases {
        assert_eq!(MermaidThemePreset::parse(name), Some(preset));
        assert!(preset.to_theme().background.starts_with('#'));
    }

    assert_eq!(MermaidThemePreset::parse("unknown"), None);
}

#[test]
fn theme_variable_aliases_map_to_mermaid_theme_fields() {
    let cases = [
        ("background", "background"),
        ("primaryColor", "node_fill"),
        ("mainBkg", "node_fill"),
        ("primaryBorderColor", "node_stroke"),
        ("nodeBorder", "node_stroke"),
        ("primaryTextColor", "text_color"),
        ("nodeTextColor", "text_color"),
        ("textColor", "text_color"),
        ("lineColor", "edge_color"),
        ("defaultLinkColor", "edge_color"),
        ("clusterBkg", "subgraph_fill"),
        ("clusterBorder", "subgraph_stroke"),
    ];

    for (alias, field) in cases {
        let mut variables = MermaidThemeVariables::default();

        assert!(variables.apply_mermaid_alias(alias, "#abcdef".to_string()));

        match field {
            "background" => assert_eq!(variables.background, Some("#abcdef".to_string())),
            "node_fill" => assert_eq!(variables.node_fill, Some("#abcdef".to_string())),
            "node_stroke" => assert_eq!(variables.node_stroke, Some("#abcdef".to_string())),
            "text_color" => assert_eq!(variables.text_color, Some("#abcdef".to_string())),
            "edge_color" => assert_eq!(variables.edge_color, Some("#abcdef".to_string())),
            "subgraph_fill" => assert_eq!(variables.subgraph_fill, Some("#abcdef".to_string())),
            "subgraph_stroke" => {
                assert_eq!(variables.subgraph_stroke, Some("#abcdef".to_string()))
            }
            _ => panic!("unexpected field {field}"),
        }
    }
}

#[test]
fn theme_variables_apply_over_selected_preset() {
    let source = r##"---
config:
  theme: forest
  themeVariables:
    background: "#101010"
    mainBkg: "#202020"
---
graph TD
A --> B
"##;

    let parsed = parse_mermaid_frontmatter(source);
    let theme = match parsed.config.to_mermaid_theme() {
        Some(theme) => theme,
        None => panic!("expected frontmatter theme"),
    };

    assert_eq!(theme.background, "#101010");
    assert_eq!(theme.node_fill, "#202020");
    assert_eq!(theme.node_stroke, "#13540c");
}

#[test]
fn unsupported_keys_are_ignored() {
    let source = r##"---
config:
  theme: unknown
  madeUpTopLevel: true
  themeVariables:
    notAThemeVariable: "#123456"
  flowchart:
    unsupportedFlowchartKey: value
---
graph TD
A --> B
"##;

    let parsed = parse_mermaid_frontmatter(source);

    assert_eq!(parsed.body, "graph TD\nA --> B\n");
    assert_eq!(parsed.config.theme, None);
    assert!(parsed.config.theme_variables.is_empty());
    assert_eq!(parsed.config.flowchart, FlowchartConfig::default());
}
