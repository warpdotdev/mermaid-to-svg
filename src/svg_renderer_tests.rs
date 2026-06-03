use crate::ast::{Edge, EdgeStyle, FlowchartGraph, GraphDirection, Node, NodeShape, Statement};
use crate::config::{FlowchartConfig, RenderConfig};
use crate::layout::{compute_layout, LayoutEdge, LayoutResult};
use crate::svg_renderer::{render, render_with_config};
use crate::theme::MermaidTheme;
use std::collections::HashMap;

#[test]
fn test_basic_svg_structure() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Test".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let theme = MermaidTheme::default();
    let svg = render(&layout, &theme);

    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("Test"));
}
#[test]
fn test_svg_background_is_rasterizable_geometry() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Test".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let mut theme = MermaidTheme::default();
    theme.background = "#010203".to_string();
    let svg = render(&layout, &theme);

    assert!(svg.contains("style=\"background-color: #010203;\""));
    assert!(svg.contains(r##"<rect x="0" y="0" width=""##));
    assert!(svg.contains(r##"fill="#010203" stroke="none"/>"##));
}

#[test]
fn test_render_with_config_applies_font_family_and_size() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Configured font".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let config = RenderConfig {
        font_family: Some("Inter".to_string()),
        font_size: Some("22px".to_string()),
        ..Default::default()
    };
    let svg = render_with_config(&layout, &MermaidTheme::default(), &config);

    assert!(svg.contains("font-family=\"Inter\""));
    assert!(svg.contains("font-size=\"22\""));
}

#[test]
fn test_render_with_config_linear_curve_uses_line_segments() {
    let layout = LayoutResult {
        nodes: HashMap::new(),
        edges: vec![LayoutEdge {
            from: "A".to_string(),
            to: "B".to_string(),
            label: None,
            style: EdgeStyle::Line,
            points: vec![(0.0, 0.0), (40.0, 30.0), (80.0, 0.0)],
            label_pos: None,
        }],
        subgraphs: Vec::new(),
        width: 100.0,
        height: 60.0,
    };
    let config = RenderConfig {
        flowchart: FlowchartConfig {
            curve: Some("linear".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let svg = render_with_config(&layout, &MermaidTheme::default(), &config);

    assert!(svg.contains("d=\"M0.0,0.0L40.0,30.0L80.0,0.0\""));
}

#[test]
fn test_rectangle_rendering() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Rectangle".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("<rect"));
    assert!(svg.contains("Rectangle"));
}

#[test]
fn test_diamond_rendering() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Decision".to_string()),
            shape: NodeShape::Diamond,
        })],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("<polygon"));
    assert!(svg.contains("Decision"));
}

#[test]
fn test_circle_rendering() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Circle".to_string()),
            shape: NodeShape::Circle,
        })],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("<circle"));
    assert!(svg.contains("Circle"));
}

#[test]
fn test_edge_rendering() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![
            Statement::Node(Node {
                id: "A".to_string(),
                label: Some("Start".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "B".to_string(),
                label: Some("End".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Edge(Edge {
                from: "A".to_string(),
                to: "B".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
        ],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("<path"));
    assert!(svg.contains("marker-end"));
}

#[test]
fn test_edge_label_rendering() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Edge(Edge {
            from: "A".to_string(),
            to: "B".to_string(),
            label: Some("Yes".to_string()),
            style: EdgeStyle::Arrow,
        })],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("Yes"));
}

#[test]
fn test_edge_rounding_uses_bezier_curves() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![
            Statement::Node(Node {
                id: "A".to_string(),
                label: Some("a".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "B".to_string(),
                label: Some("b".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "C".to_string(),
                label: Some("c".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Edge(Edge {
                from: "A".to_string(),
                to: "B".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "A".to_string(),
                to: "C".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
        ],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(svg.contains("C"));
}

#[test]
fn test_label_position_at_midpoint_along_path() {
    let points = vec![(0.0, 0.0), (0.0, 100.0), (100.0, 100.0)];
    let (x, y) = super::SvgRenderer::label_position(&points);
    assert!((x - 0.0).abs() < 0.001);
    assert!((y - 100.0).abs() < 0.001);
}

#[test]
fn test_dark_theme() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Dark".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let theme = MermaidTheme::dark();
    let svg = render(&layout, &theme);

    assert!(svg.contains(&theme.background));
    assert!(svg.contains(&theme.node_fill));
}

#[test]
fn test_xml_escaping() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("<script>alert('xss')</script>".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let layout = compute_layout(&graph);
    let svg = render(&layout, &MermaidTheme::default());

    assert!(!svg.contains("<script>"));
    assert!(svg.contains("&lt;script&gt;"));
}
