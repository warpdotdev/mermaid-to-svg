mod ast;
mod block_diagram;
mod c4_diagram;
mod class_diagram;
mod er_diagram;
mod error;
pub mod fixtures;
mod gantt_diagram;
mod gitgraph_diagram;
mod info_diagram;
mod journey_diagram;
mod kanban_diagram;
mod layout;
mod mermaid_port;
mod mindmap_diagram;
mod packet_diagram;
mod parser;
mod pie_diagram;
mod quadrant_diagram;
mod radar_diagram;
mod requirement_diagram;
mod sankey_diagram;
mod sequence_diagram;
mod state_diagram;
mod svg_renderer;
mod text_wrap;
mod theme;
mod timeline_diagram;
mod xychart_diagram;

#[cfg(test)]
mod reference_svg;

pub use error::MermaidError;
pub use theme::MermaidTheme;

pub fn render_mermaid_to_svg(
    mermaid_source: &str,
    theme: Option<&MermaidTheme>,
) -> Result<String, MermaidError> {
    let default_theme = MermaidTheme::default();
    let theme = theme.unwrap_or(&default_theme);

    let diagram_type = first_diagram_type_token(mermaid_source);

    if diagram_type == Some("erDiagram") {
        return er_diagram::render_er_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("classDiagram") {
        return class_diagram::render_class_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("mindmap") {
        return mindmap_diagram::render_mindmap_to_svg(mermaid_source, theme);
    }

    if matches!(diagram_type, Some("stateDiagram") | Some("stateDiagram-v2")) {
        let graph = state_diagram::parse_state_diagram(mermaid_source)?;
        let layout_result = layout::compute_layout(&graph);
        let svg = svg_renderer::render(&layout_result, theme);
        return Ok(svg);
    }

    if diagram_type == Some("pie") {
        return pie_diagram::render_pie_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("gantt") {
        return gantt_diagram::render_gantt_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("requirementDiagram") {
        return requirement_diagram::render_requirement_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("info") {
        return info_diagram::render_info_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("packet-beta") {
        return packet_diagram::render_packet_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("block-beta") {
        return block_diagram::render_block_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("radar-beta") {
        return radar_diagram::render_radar_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("sankey-beta") {
        return sankey_diagram::render_sankey_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("sequenceDiagram") {
        return sequence_diagram::render_sequence_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("gitGraph") {
        return gitgraph_diagram::render_gitgraph_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("timeline") {
        return timeline_diagram::render_timeline_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("journey") {
        return journey_diagram::render_journey_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("kanban") {
        return kanban_diagram::render_kanban_diagram_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("quadrantChart") {
        return quadrant_diagram::render_quadrant_chart_to_svg(mermaid_source, theme);
    }

    if diagram_type == Some("xychart-beta") {
        return xychart_diagram::render_xychart_diagram_to_svg(mermaid_source, theme);
    }

    if matches!(
        diagram_type,
        Some("C4Context")
            | Some("C4Container")
            | Some("C4Component")
            | Some("C4Dynamic")
            | Some("C4Deployment")
    ) {
        return c4_diagram::render_c4_diagram_to_svg(mermaid_source, theme);
    }

    let is_flowchart = matches!(diagram_type, Some("graph") | Some("flowchart"));
    if is_flowchart && mermaid_port::is_enabled() {
        return mermaid_port::render_mermaid_to_svg_ported(mermaid_source, theme);
    }

    let graph = parser::parse_mermaid(mermaid_source)?;
    let layout_result = layout::compute_layout(&graph);
    let svg = svg_renderer::render(&layout_result, theme);

    Ok(svg)
}

fn first_diagram_type_token(input: &str) -> Option<&str> {
    input
        .lines()
        .map(|l| l.trim())
        .find(|l| !l.is_empty() && !l.starts_with("%%"))
        .and_then(|l| l.split_whitespace().next())
}

pub fn is_mermaid_diagram(lang: &str) -> bool {
    let lang_lower = lang.to_lowercase();
    lang_lower == "mermaid" || lang_lower.starts_with("mermaid ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mermaid_diagram() {
        assert!(is_mermaid_diagram("mermaid"));
        assert!(is_mermaid_diagram("Mermaid"));
        assert!(is_mermaid_diagram("MERMAID"));
        assert!(is_mermaid_diagram("mermaid "));
        assert!(!is_mermaid_diagram("rust"));
        assert!(!is_mermaid_diagram(""));
    }

    #[test]
    fn test_simple_flowchart() {
        let mermaid = r#"graph TD
    A[Start] --> B[End]"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_empty_flowchart_does_not_panic() {
        let result = render_mermaid_to_svg("graph TD\n", None);

        assert!(result.is_ok());
        if let Ok(svg) = result {
            assert!(svg.contains("<svg"));
            assert!(svg.contains("</svg>"));
        }
    }

    #[test]
    fn test_flowchart_with_theme() {
        let mermaid = r#"graph LR
    A --> B"#;

        let theme = MermaidTheme::dark();
        let result = render_mermaid_to_svg(mermaid, Some(&theme));
        assert!(result.is_ok());
    }

    #[test]
    fn test_flowchart_with_decision() {
        let mermaid = r#"graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Action 1]
    B -->|No| D[Action 2]
    C --> E[End]
    D --> E"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_flowchart_decodes_html_entities_before_svg_escaping() {
        let mermaid = r#"graph TD
    A["shared_ptr&lt;Connection&gt; &amp; weak_ptr&lt;Player&gt;"]"#;

        let svg = render_mermaid_to_svg(mermaid, None).expect("should render");
        assert!(svg.contains("shared_ptr&lt;Connection&gt;"));
        assert!(svg.contains("weak_ptr&lt;Player&gt;"));
        assert!(!svg.contains("&amp;lt;Connection&amp;gt;"));
        assert!(!svg.contains("&amp;amp;"));
    }
    #[test]
    fn test_flowchart_renders_escaped_newline_label_as_multiple_lines() {
        let mermaid = r#"graph TD
    A["Source\nTarget"]"#;

        let svg = render_mermaid_to_svg(mermaid, None).expect("should render");
        assert!(svg.contains(">Source</tspan>"));
        assert!(svg.contains(">Target</tspan>"));
        assert!(!svg.contains("Source\\nTarget"));
    }

    #[test]
    fn test_invalid_mermaid() {
        let mermaid = "not a valid mermaid diagram";
        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_er_diagram() {
        let mermaid = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE_ITEM : contains

    CUSTOMER {
        string name
        string custNumber
    }

    ORDER {
        int orderNumber
        date orderDate
    }

    LINE_ITEM {
        int quantity
        float price
    }"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_packet_diagram() {
        let mermaid = r#"packet-beta
0-3: "Header"
4-7: "Payload"
8: "CRC""#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("packetBlock"));
    }

    #[test]
    fn test_simple_info_diagram() {
        let mermaid = "info";

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("v11.12.2"));
    }

    #[test]
    fn test_simple_class_diagram() {
        let mermaid = r#"classDiagram
    class Animal
    class Duck

    Animal : +int age
    Animal <|-- Duck"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_state_diagram() {
        let mermaid = r#"stateDiagram-v2
    [*] --> Idle
    Idle --> Working : start
    Working --> Idle : done
    Working --> [*]"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_pie_diagram() {
        let mermaid = r#"pie
    title Pets adopted by volunteers
    \"Dogs\" : 386
    \"Cats\" : 85
    \"Rats\" : 15"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_gantt_diagram() {
        let mermaid = r#"gantt
    title Simple Gantt
    dateFormat  YYYY-MM-DD

    section Build
    Setup        :a1, 2026-01-01, 2d
    Implement    :a2, after a1, 5d
    Test         :a3, after a2, 3d"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_sequence_diagram() {
        let mermaid = r#"sequenceDiagram
    participant Alice
    participant Bob

    Alice->>Bob: Hello
    Note over Alice,Bob: Hello back
    Bob-->>Alice: Hi"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_timeline_diagram() {
        let mermaid = r#"timeline
    title History of Social Platforms
    2002 : LinkedIn
    2004 : Facebook
    2006 : Twitter"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_journey_diagram() {
        let mermaid = r#"journey
    title My working day

    section Go to work
      Make tea: 5: Me
      Go upstairs: 3: Me

    section Go home
      Go downstairs: 5: Me"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_mindmap_diagram() {
        let mermaid = r#"mindmap
  root((mindmap))
    Origins
      Long history
    Tooling
      Mermaid
      Rust"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_quadrant_chart() {
        let mermaid = r#"quadrantChart
    title Reach and engagement
    x-axis Low Reach --> High Reach
    y-axis Low Engagement --> High Engagement
    quadrant-1 High impact
    quadrant-2 Viral
    quadrant-3 Niche
    quadrant-4 Broad but shallow
    \"Post A\": [0.3, 0.6]
    \"Post B\": [0.8, 0.2]"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_simple_xychart() {
        let mermaid = r#"xychart-beta
    title Demo
    x-axis 0 --> 10
    y-axis 0 --> 100
    line [5, 10, 20, 40]"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Demo"));
    }

    #[test]
    fn test_simple_sankey() {
        let mermaid = r#"sankey-beta
    A,B,10
    B,C,5
    B,D,5"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("linearGradient"));
    }

    #[test]
    fn test_simple_radar() {
        let mermaid = r#"radar-beta
axis A, B, C
curve Series1 { 1, 2, 3 }"#;

        let result = render_mermaid_to_svg(mermaid, None);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("radarGraticule"));
        assert!(svg.contains("Series1"));
    }

    #[test]
    fn test_simple_requirement_diagram() {
        let mermaid = r#"requirementDiagram
direction LR

requirement req1 {
    id: 1
    text: \"The system shall do something\"
    risk: high
    verifyMethod: test
}

element el1 {
    type: \"Subsystem\"
    docref: \"DOC-1\"
}

req1 - satisfies -> el1"#;

        let result = render_mermaid_to_svg(mermaid, None);
        let svg = match result {
            Ok(svg) => svg,
            Err(err) => panic!("expected ok result, got error: {err}"),
        };
        assert!(svg.contains("requirementDiagram"));
        assert!(svg.contains("req1"));
        assert!(svg.contains("el1"));
        assert!(svg.contains("&lt;&lt;satisfies&gt;&gt;") || svg.contains("<<satisfies>>"));
    }

    #[test]
    fn test_simple_kanban_diagram() {
        let mermaid = r#"kanban
Todo
    Task 1
    Task 2
Doing
    Task 3"#;

        let result = render_mermaid_to_svg(mermaid, None);
        let svg = match result {
            Ok(svg) => svg,
            Err(err) => panic!("expected ok result, got error: {err}"),
        };
        assert!(svg.contains("aria-roledescription=\"kanban\""));
        assert!(svg.contains("Todo"));
        assert!(svg.contains("Task 1"));
        assert!(svg.contains("Doing"));
        assert!(svg.contains("Task 3"));
    }

    #[test]
    fn test_simple_block_diagram() {
        let mermaid = r#"block-beta
A[\"A\"] --> B[\"B\"]"#;

        let result = render_mermaid_to_svg(mermaid, None);
        let svg = match result {
            Ok(svg) => svg,
            Err(err) => panic!("expected ok result, got error: {err}"),
        };
        assert!(svg.contains("aria-roledescription=\"block\""));
        assert!(svg.contains("id=\"A\""));
        assert!(svg.contains("id=\"B\""));
    }

    #[test]
    fn test_simple_gitgraph_diagram() {
        let mermaid = r#"gitGraph
    commit
    commit
    branch develop
    checkout develop
    commit
    checkout main
    merge develop"#;

        let result = render_mermaid_to_svg(mermaid, None);
        let svg = match result {
            Ok(svg) => svg,
            Err(err) => panic!("expected ok result, got error: {err}"),
        };
        assert!(svg.contains("aria-roledescription=\"gitGraph\""));
        assert!(svg.contains("main"));
        assert!(svg.contains("develop"));
    }
}
