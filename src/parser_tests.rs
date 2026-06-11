use super::*;
use crate::ast::{EdgeStyle, GraphDirection, NodeShape};

#[test]
fn test_parse_graph_direction_td() {
    let input = "graph TD\n    A --> B";
    let result = parse_mermaid(input).unwrap();
    assert_eq!(result.direction, GraphDirection::TopToBottom);
}

#[test]
fn test_parse_graph_direction_lr() {
    let input = "graph LR\n    A --> B";
    let result = parse_mermaid(input).unwrap();
    assert_eq!(result.direction, GraphDirection::LeftToRight);
}

#[test]
fn test_parse_flowchart_keyword() {
    let input = "flowchart TD\n    A --> B";
    let result = parse_mermaid(input).unwrap();
    assert_eq!(result.direction, GraphDirection::TopToBottom);
}

#[test]
fn test_parse_simple_edge() {
    let input = "graph TD\n    A --> B";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].from, "A");
    assert_eq!(edges[0].to, "B");
    assert_eq!(edges[0].style, EdgeStyle::Arrow);
}

#[test]
fn test_parse_edge_with_label() {
    let input = "graph TD\n    A -->|Yes| B";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].label, Some("Yes".to_string()));
}

#[test]
fn test_parse_escaped_newline_edge_label() {
    let input = r#"graph TD
    A -->|Yes\ncontinue| B"#;
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].label, Some("Yes\ncontinue".to_string()));
}
#[test]
fn test_parse_node_with_label() {
    let input = "graph TD\n    A[Start] --> B[End]";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].id, "A");
    assert_eq!(nodes[0].label, Some("Start".to_string()));
    assert_eq!(nodes[0].shape, NodeShape::Rectangle);
}
#[test]
fn test_parse_quoted_html_break_labels() {
    let input = "graph TD\n    A[\"Source<br/>Target\"]";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].label, Some("Source\nTarget".to_string()));
}

#[test]
fn test_parse_escaped_newline_node_label() {
    let input = r#"graph TD
    A["Source\nTarget"]"#;
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].label, Some("Source\nTarget".to_string()));
}
#[test]
fn test_parse_html_entities_in_labels() {
    let input = "graph TD\n    A[\"shared_ptr&lt;Connection&gt; &amp; weak_ptr&lt;Player&gt;\"]";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0].label,
        Some("shared_ptr<Connection> & weak_ptr<Player>".to_string())
    );
}

#[test]
fn test_parse_dotted_arrow_with_dot_delimited_label() {
    let input = "graph TD\n    Room -.weak.-> PlayerA";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].from, "Room");
    assert_eq!(edges[0].to, "PlayerA");
    assert_eq!(edges[0].style, EdgeStyle::DottedArrow);
    assert_eq!(edges[0].label, Some("weak".to_string()));
}

#[test]
fn test_parse_dash_dash_quoted_label_with_parens() {
    let input = "graph TD\n    E -- \"Reprompt()\" --> SY[synthetic]";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].from, "E");
    assert_eq!(edges[0].to, "SY");
    assert_eq!(edges[0].style, EdgeStyle::Arrow);
    assert_eq!(edges[0].label, Some("Reprompt()".to_string()));
}

#[test]
fn test_parse_dash_dash_label_preserves_node_shapes() {
    let input = "graph LR\n    A[start] -- label one --> B(end)";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();
    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].id, "A");
    assert_eq!(nodes[0].label, Some("start".to_string()));
    assert_eq!(nodes[0].shape, NodeShape::Rectangle);
    assert_eq!(nodes[1].id, "B");
    assert_eq!(nodes[1].label, Some("end".to_string()));
    assert_eq!(nodes[1].shape, NodeShape::RoundedRectangle);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].label, Some("label one".to_string()));
    assert_eq!(edges[0].style, EdgeStyle::Arrow);
}

#[test]
fn test_parse_labeled_thick_and_line_edges() {
    let input = "graph LR\n    A --> B\n    B == slow ==> C\n    C -- plain --- D";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 3);
    assert_eq!(edges[1].from, "B");
    assert_eq!(edges[1].to, "C");
    assert_eq!(edges[1].style, EdgeStyle::ThickArrow);
    assert_eq!(edges[1].label, Some("slow".to_string()));
    assert_eq!(edges[2].from, "C");
    assert_eq!(edges[2].to, "D");
    assert_eq!(edges[2].style, EdgeStyle::Line);
    assert_eq!(edges[2].label, Some("plain".to_string()));
}

#[test]
fn test_parse_rounded_node() {
    let input = "graph TD\n    A(Rounded)";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].shape, NodeShape::RoundedRectangle);
}

#[test]
fn test_parse_diamond_node() {
    let input = "graph TD\n    A{Decision}";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].shape, NodeShape::Diamond);
}

#[test]
fn test_parse_circle_node() {
    let input = "graph TD\n    A((Circle))";
    let result = parse_mermaid(input).unwrap();

    let nodes: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Node(n) => Some(n),
            _ => None,
        })
        .collect();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].shape, NodeShape::Circle);
}

#[test]
fn test_parse_dotted_edge() {
    let input = "graph TD\n    A -.-> B";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].style, EdgeStyle::DottedArrow);
}

#[test]
fn test_parse_thick_edge() {
    let input = "graph TD\n    A ==> B";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].style, EdgeStyle::ThickArrow);
}

#[test]
fn test_parse_line_no_arrow() {
    let input = "graph TD\n    A --- B";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].style, EdgeStyle::Line);
}

#[test]
fn test_parse_subgraph() {
    let input = r#"graph TD
    subgraph Group
        A --> B
    end"#;
    let result = parse_mermaid(input).unwrap();

    let subgraphs: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Subgraph(sg) => Some(sg),
            _ => None,
        })
        .collect();

    assert_eq!(subgraphs.len(), 1);
    assert_eq!(subgraphs[0].id, "Group");
}

#[test]
fn test_parse_escaped_newline_subgraph_title() {
    let input = r#"graph TD
    subgraph Group["Outer\nGroup"]
        A --> B
    end"#;
    let result = parse_mermaid(input).unwrap();

    let subgraphs: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Subgraph(sg) => Some(sg),
            _ => None,
        })
        .collect();

    assert_eq!(subgraphs.len(), 1);
    assert_eq!(subgraphs[0].title, Some("Outer\nGroup".to_string()));
}
#[test]
fn test_parse_style_statement() {
    let input = "graph TD\n    A --> B\n    style A fill:#f9f,stroke:#333";
    let result = parse_mermaid(input).unwrap();

    let styles: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Style(st) => Some(st),
            _ => None,
        })
        .collect();

    assert_eq!(styles.len(), 1);
    assert_eq!(styles[0].node_id, "A");
    assert_eq!(styles[0].properties.len(), 2);
}

#[test]
fn test_parse_comments() {
    let input = r#"graph TD
    %% This is a comment
    A --> B"#;
    let result = parse_mermaid(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_edge_chain() {
    let input = "graph TD\n    A --> B --> C";
    let result = parse_mermaid(input).unwrap();

    let edges: Vec<_> = result
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Edge(e) => Some(e),
            _ => None,
        })
        .collect();

    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].from, "A");
    assert_eq!(edges[0].to, "B");
    assert_eq!(edges[1].from, "B");
    assert_eq!(edges[1].to, "C");
}

#[test]
fn test_invalid_missing_direction() {
    let input = "graph\n    A --> B";
    let result = parse_mermaid(input);
    assert!(result.is_err());
}

#[test]
fn test_invalid_no_graph_keyword() {
    let input = "A --> B";
    let result = parse_mermaid(input);
    assert!(result.is_err());
}

#[test]
fn test_sequence_diagram_is_reported_as_unsupported() {
    let input = "sequenceDiagram\n    Alice->>Bob: Hi";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(
        err,
        MermaidError::UnsupportedDiagramType(ref t) if t == "sequenceDiagram"
    ));
}

#[test]
fn test_class_diagram_is_reported_as_unsupported() {
    let input = "classDiagram\n    class A";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(
        err,
        MermaidError::UnsupportedDiagramType(ref t) if t == "classDiagram"
    ));
}

#[test]
fn test_state_diagram_v2_is_reported_as_unsupported() {
    let input = "stateDiagram-v2\n    [*] --> Idle";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(
        err,
        MermaidError::UnsupportedDiagramType(ref t) if t == "stateDiagram-v2"
    ));
}

#[test]
fn test_er_diagram_is_reported_as_unsupported() {
    let input = "erDiagram\n    A ||--|| B : relates";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(
        err,
        MermaidError::UnsupportedDiagramType(ref t) if t == "erDiagram"
    ));
}

#[test]
fn test_gantt_is_reported_as_unsupported() {
    let input = "gantt\n    title A";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(err, MermaidError::UnsupportedDiagramType(ref t) if t == "gantt"));
}

#[test]
fn test_pie_is_reported_as_unsupported() {
    let input = "pie\n    title A\n    \"A\": 1";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(err, MermaidError::UnsupportedDiagramType(ref t) if t == "pie"));
}

#[test]
fn test_directives_are_skipped_when_detecting_diagram_type() {
    let input = "%%{init: {\"theme\": \"base\"}}%%\nsequenceDiagram\n    Alice->>Bob: Hi";
    let err = parse_mermaid(input).unwrap_err();
    assert!(matches!(
        err,
        MermaidError::UnsupportedDiagramType(ref t) if t == "sequenceDiagram"
    ));
}
