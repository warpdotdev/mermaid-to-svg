use std::collections::HashMap;

use super::{FlowchartLayoutOptions, LayoutEngine, LayoutNode};
use crate::ast::{Edge, EdgeStyle, FlowchartGraph, GraphDirection, Node, NodeShape, Statement};
use crate::config::{FlowchartConfig, RenderConfig};
use crate::layout::{compute_layout, compute_layout_with_config};
use crate::parser::parse_mermaid;

#[test]
fn test_single_node_layout() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Start".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let result = compute_layout(&graph);

    assert_eq!(result.nodes.len(), 1);
    assert!(result.nodes.contains_key("A"));

    let node = &result.nodes["A"];
    assert_eq!(node.label, "Start");
    assert!(node.width > 0.0);
    assert!(node.height > 0.0);
}

#[test]
fn test_flowchart_spacing_config_increases_rank_distance() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Edge(Edge {
            from: "A".to_string(),
            to: "B".to_string(),
            label: None,
            style: EdgeStyle::Arrow,
        })],
    };

    let default_layout = compute_layout(&graph);
    let config = RenderConfig {
        flowchart: FlowchartConfig {
            rank_spacing: Some(140),
            ..Default::default()
        },
        ..Default::default()
    };
    let configured_layout = compute_layout_with_config(&graph, &config);

    let default_delta = default_layout.nodes["B"].y - default_layout.nodes["A"].y;
    let configured_delta = configured_layout.nodes["B"].y - configured_layout.nodes["A"].y;

    assert!(configured_delta > default_delta);
}

#[test]
fn test_flowchart_padding_and_wrapping_config_affect_node_size() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Long label that wraps across several rendered lines".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let default_layout = compute_layout(&graph);
    let config = RenderConfig {
        flowchart: FlowchartConfig {
            padding: Some(4),
            wrapping_width: Some(70),
            ..Default::default()
        },
        ..Default::default()
    };
    let configured_layout = compute_layout_with_config(&graph, &config);

    let default_node = &default_layout.nodes["A"];
    let configured_node = &configured_layout.nodes["A"];

    assert!(configured_node.height > default_node.height);
    assert!(configured_node.width < default_node.width);
}
#[test]
fn test_flowchart_font_size_config_affects_node_and_edge_label_size() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Font".to_string()),
            shape: NodeShape::Rectangle,
        })],
    };

    let default_layout = compute_layout(&graph);
    let config = RenderConfig {
        font_size: Some("32px".to_string()),
        ..Default::default()
    };
    let configured_layout = compute_layout_with_config(&graph, &config);

    let default_node = &default_layout.nodes["A"];
    let configured_node = &configured_layout.nodes["A"];

    assert!(configured_node.width > default_node.width);
    assert!(configured_node.height > default_node.height);

    let default_engine = LayoutEngine::new(&graph);
    let configured_engine =
        LayoutEngine::new_with_options(&graph, FlowchartLayoutOptions::from_render_config(&config));
    let (default_label_width, default_label_height) = default_engine
        .edge_label_dimensions("Edge")
        .expect("label should have dimensions");
    let (configured_label_width, configured_label_height) = configured_engine
        .edge_label_dimensions("Edge")
        .expect("label should have dimensions");

    assert!(configured_label_width > default_label_width);
    assert!(configured_label_height > default_label_height);
}

#[test]
fn test_two_node_edge_layout() {
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

    let result = compute_layout(&graph);

    assert_eq!(result.nodes.len(), 2);
    assert_eq!(result.edges.len(), 1);

    let node_a = &result.nodes["A"];
    let node_b = &result.nodes["B"];
    assert!(node_b.y > node_a.y);
}

#[test]
fn test_edges_to_subgraphs_do_not_create_rendered_nodes() {
    let graph = parse_mermaid(
        r#"flowchart TD
    A[Start]
    subgraph PIPE["Pipeline"]
        B[Step]
    end
    A --> PIPE
    PIPE --> C[Done]"#,
    )
    .unwrap();

    let result = compute_layout(&graph);

    assert!(!result.nodes.contains_key("PIPE"));
    assert!(result
        .subgraphs
        .iter()
        .any(|subgraph| subgraph.id == "PIPE"));
    assert!(result
        .edges
        .iter()
        .any(|edge| edge.from == "A" && edge.to == "PIPE"));
    assert!(result
        .edges
        .iter()
        .any(|edge| edge.from == "PIPE" && edge.to == "C"));
}

#[test]
fn test_split_edges_route_through_target_lane_top_to_bottom() {
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
                label: Some("Left".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "C".to_string(),
                label: Some("Right".to_string()),
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

    let result = compute_layout(&graph);

    let node_b = &result.nodes["B"];
    let node_c = &result.nodes["C"];

    let edge_ab = result.edges.iter().find(|e| e.from == "A" && e.to == "B");
    let Some(edge_ab) = edge_ab else {
        panic!("Expected edge A -> B");
    };
    assert!(
        edge_ab.points.len() >= 2,
        "Edge A->B should have at least start and end points"
    );
    let start_ab = edge_ab
        .points
        .first()
        .expect("Edge should have start point");
    let end_ab = edge_ab.points.last().expect("Edge should have end point");
    assert!(
        end_ab.1 > start_ab.1,
        "Edge should go downward (end y > start y)"
    );

    let edge_ac = result.edges.iter().find(|e| e.from == "A" && e.to == "C");
    let Some(edge_ac) = edge_ac else {
        panic!("Expected edge A -> C");
    };
    assert!(
        edge_ac.points.len() >= 2,
        "Edge A->C should have at least start and end points"
    );
    let start_ac = edge_ac
        .points
        .first()
        .expect("Edge should have start point");
    let end_ac = edge_ac.points.last().expect("Edge should have end point");
    assert!(
        end_ac.1 > start_ac.1,
        "Edge should go downward (end y > start y)"
    );

    assert!(
        (start_ab.0 - start_ac.0).abs() > 1.0 || node_b.x == node_c.x,
        "Edges from same node should start at different x positions (port spreading) unless targets are aligned"
    );
}

#[test]
fn test_merge_edges_route_through_source_lane_top_to_bottom() {
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
                label: Some("Left".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "C".to_string(),
                label: Some("Right".to_string()),
                shape: NodeShape::Rectangle,
            }),
            Statement::Node(Node {
                id: "D".to_string(),
                label: Some("End".to_string()),
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
            Statement::Edge(Edge {
                from: "B".to_string(),
                to: "D".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "C".to_string(),
                to: "D".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
        ],
    };

    let result = compute_layout(&graph);

    let node_a = &result.nodes["A"];
    let node_b = &result.nodes["B"];
    let node_c = &result.nodes["C"];
    let node_d = &result.nodes["D"];

    assert!(node_b.y > node_a.y, "B should be below A");
    assert!(node_c.y > node_a.y, "C should be below A");
    assert!(node_d.y > node_b.y, "D should be below B");
    assert!(node_d.y > node_c.y, "D should be below C");

    let edge_bd = result.edges.iter().find(|e| e.from == "B" && e.to == "D");
    let Some(edge_bd) = edge_bd else {
        panic!("Expected edge B -> D");
    };
    assert!(
        edge_bd.points.len() >= 2,
        "Edge B->D should have at least start and end points"
    );

    let edge_cd = result.edges.iter().find(|e| e.from == "C" && e.to == "D");
    let Some(edge_cd) = edge_cd else {
        panic!("Expected edge C -> D");
    };
    assert!(
        edge_cd.points.len() >= 2,
        "Edge C->D should have at least start and end points"
    );
}

#[test]
fn test_horizontal_layout() {
    let graph = FlowchartGraph {
        direction: GraphDirection::LeftToRight,
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

    let result = compute_layout(&graph);

    let node_a = &result.nodes["A"];
    let node_b = &result.nodes["B"];
    assert!(node_b.x > node_a.x);
}

#[test]
fn test_diamond_shape_sizing() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Node(Node {
            id: "A".to_string(),
            label: Some("Decision".to_string()),
            shape: NodeShape::Diamond,
        })],
    };

    let result = compute_layout(&graph);
    let node = &result.nodes["A"];
    assert_eq!(node.width, node.height);
}

#[test]
fn test_implicit_node_creation() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![Statement::Edge(Edge {
            from: "A".to_string(),
            to: "B".to_string(),
            label: None,
            style: EdgeStyle::Arrow,
        })],
    };

    let result = compute_layout(&graph);
    assert_eq!(result.nodes.len(), 2);
    assert!(result.nodes.contains_key("A"));
    assert!(result.nodes.contains_key("B"));
}

#[test]
fn test_cyclic_graph_back_edge_detection() {
    let graph = FlowchartGraph {
        direction: GraphDirection::LeftToRight,
        statements: vec![
            Statement::Edge(Edge {
                from: "Idle".to_string(),
                to: "Running".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "Running".to_string(),
                to: "Paused".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "Paused".to_string(),
                to: "Running".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "Running".to_string(),
                to: "Stopped".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "Paused".to_string(),
                to: "Stopped".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "Stopped".to_string(),
                to: "Idle".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
        ],
    };

    let result = compute_layout(&graph);

    let idle = &result.nodes["Idle"];
    let running = &result.nodes["Running"];
    let paused = &result.nodes["Paused"];
    let stopped = &result.nodes["Stopped"];

    assert!(running.x > idle.x);
    assert!(paused.x > running.x);
    assert!(stopped.x > running.x);
}
#[test]
fn test_cyclic_flowchart_weak_edges_do_not_define_main_rank_flow() {
    let graph = parse_mermaid(
        r#"graph TD
    Hub["Hub<br/>players_: unordered_map&lt;Connection*, shared_ptr&lt;Player&gt;&gt;"]
    PlayerA["Player A<br/>conn_: shared_ptr&lt;Connection&gt;<br/>room_: shared_ptr&lt;Room&gt;"]
    Room["Room<br/>host: weak_ptr&lt;Player&gt;<br/>guest: weak_ptr&lt;Player&gt;"]
    ConnA["Connection A (fd=4)"]

    Hub -->|owns| PlayerA
    PlayerA -->|owns| ConnA
    PlayerA -->|shares| Room
    Room -.weak.-> PlayerA"#,
    )
    .expect("diagram should parse");

    let result = compute_layout(&graph);

    let hub = &result.nodes["Hub"];
    let player_a = &result.nodes["PlayerA"];
    let room = &result.nodes["Room"];
    let conn_a = &result.nodes["ConnA"];
    let weak_edge = result
        .edges
        .iter()
        .find(|edge| edge.from == "Room" && edge.to == "PlayerA")
        .expect("expected weak back edge");

    assert!(player_a.y > hub.y);
    assert!(room.y > player_a.y);
    assert!(conn_a.y > player_a.y);
    assert_eq!(weak_edge.style, EdgeStyle::DottedArrow);
    assert_eq!(weak_edge.label.as_deref(), Some("weak"));
    assert!(weak_edge
        .points
        .first()
        .zip(weak_edge.points.last())
        .is_some_and(|(start, end)| end.1 < start.1));
}

#[test]
fn test_vertical_back_edge_uses_local_side_lane() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![],
    };
    let engine = LayoutEngine::new(&graph);
    let from = LayoutNode {
        id: "Room".to_string(),
        x: 120.0,
        y: 200.0,
        width: 80.0,
        height: 40.0,
        shape: NodeShape::Rectangle,
        label: "Room".to_string(),
        fill_color: None,
        stroke_color: None,
    };
    let to = LayoutNode {
        id: "PlayerA".to_string(),
        x: 40.0,
        y: 80.0,
        width: 80.0,
        height: 40.0,
        shape: NodeShape::Rectangle,
        label: "Player A".to_string(),
        fill_color: None,
        stroke_color: None,
    };
    let far_obstacle = LayoutNode {
        id: "FarObstacle".to_string(),
        x: 400.0,
        y: 140.0,
        width: 80.0,
        height: 220.0,
        shape: NodeShape::Rectangle,
        label: "Far obstacle".to_string(),
        fill_color: None,
        stroke_color: None,
    };
    let all_nodes = HashMap::from([
        (from.id.clone(), from.clone()),
        (to.id.clone(), to.clone()),
        (far_obstacle.id.clone(), far_obstacle.clone()),
    ]);

    let points = engine.compute_back_edge_points(&from, &to, true, &all_nodes);
    let max_x = points
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    let expected_side_x = from.x + from.width / 2.0 + 30.0;

    assert!((max_x - expected_side_x).abs() < 0.001);
    assert!(max_x < far_obstacle.x - far_obstacle.width / 2.0);
}
#[test]
fn test_longest_path_ranking() {
    let graph = FlowchartGraph {
        direction: GraphDirection::TopToBottom,
        statements: vec![
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
            Statement::Edge(Edge {
                from: "B".to_string(),
                to: "D".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
            Statement::Edge(Edge {
                from: "C".to_string(),
                to: "D".to_string(),
                label: None,
                style: EdgeStyle::Arrow,
            }),
        ],
    };

    let result = compute_layout(&graph);

    let node_a = &result.nodes["A"];
    let node_b = &result.nodes["B"];
    let node_c = &result.nodes["C"];
    let node_d = &result.nodes["D"];

    assert!((node_b.y - node_c.y).abs() < 0.001);
    assert!(node_d.y > node_b.y);
    assert!(node_a.y < node_b.y);
}
