use crate::ast::{EdgeStyle, GraphDirection, NodeShape};

use super::super::flow_db::from_flowchart_graph;
use super::super::flow_parser;
use super::get_data;

#[test]
fn flow_data_matches_expected_for_01_basic_flowchart() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!("{base_dir}/samples/flowchart/mermaid/01_basic_flowchart.mmd");
    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");

    let graph = flow_parser::parse_flowchart(&mermaid).expect("mermaid should parse");
    assert_eq!(graph.direction, GraphDirection::TopToBottom);

    let db = from_flowchart_graph(&graph);
    assert_eq!(db.direction, GraphDirection::TopToBottom);
    assert_eq!(db.vertices.len(), 3);
    assert_eq!(db.vertex_order, vec!["A", "B", "C"]);
    assert_eq!(db.edges.len(), 2);
    assert!(db.subgraphs.is_empty());

    let data = get_data(&db);
    assert_eq!(data.nodes.len(), 3);
    assert_eq!(data.edges.len(), 2);

    let node_a = data.nodes.iter().find(|n| n.id == "A").expect("A");
    let node_b = data.nodes.iter().find(|n| n.id == "B").expect("B");
    let node_c = data.nodes.iter().find(|n| n.id == "C").expect("C");

    assert_eq!(node_a.label, "Start");
    assert_eq!(node_a.shape, NodeShape::Rectangle);
    assert_eq!(node_a.parent_id.as_deref(), None);
    assert!(node_a.styles.is_empty());
    assert!(!node_a.is_group);

    assert_eq!(node_b.label, "Process");
    assert_eq!(node_b.shape, NodeShape::Rectangle);
    assert_eq!(node_b.parent_id.as_deref(), None);
    assert!(node_b.styles.is_empty());
    assert!(!node_b.is_group);

    assert_eq!(node_c.label, "End");
    assert_eq!(node_c.shape, NodeShape::Rectangle);
    assert_eq!(node_c.parent_id.as_deref(), None);
    assert!(node_c.styles.is_empty());
    assert!(!node_c.is_group);

    let edge_ab = data
        .edges
        .iter()
        .find(|e| e.start == "A" && e.end == "B")
        .expect("A->B");
    assert_eq!(edge_ab.label.as_deref(), None);
    assert_eq!(edge_ab.style, EdgeStyle::Arrow);

    let edge_bc = data
        .edges
        .iter()
        .find(|e| e.start == "B" && e.end == "C")
        .expect("B->C");
    assert_eq!(edge_bc.label.as_deref(), None);
    assert_eq!(edge_bc.style, EdgeStyle::Arrow);
}

#[test]
fn flow_data_tracks_subgraphs_for_06_subgraphs() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!("{base_dir}/samples/flowchart/mermaid/06_subgraphs.mmd");
    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");

    let graph = flow_parser::parse_flowchart(&mermaid).expect("mermaid should parse");
    assert_eq!(graph.direction, GraphDirection::TopToBottom);

    let db = from_flowchart_graph(&graph);
    assert_eq!(db.subgraphs.len(), 2);

    let frontend = db
        .subgraphs
        .iter()
        .find(|s| s.id == "Frontend")
        .expect("Frontend");
    assert_eq!(frontend.title.as_deref(), Some("Frontend"));
    assert_eq!(frontend.parent_id.as_deref(), None);

    let backend = db
        .subgraphs
        .iter()
        .find(|s| s.id == "Backend")
        .expect("Backend");
    assert_eq!(backend.title.as_deref(), Some("Backend"));
    assert_eq!(backend.parent_id.as_deref(), None);

    assert_eq!(
        db.node_to_subgraph.get("A").map(String::as_str),
        Some("Frontend")
    );
    assert_eq!(
        db.node_to_subgraph.get("B").map(String::as_str),
        Some("Frontend")
    );
    assert_eq!(
        db.node_to_subgraph.get("C").map(String::as_str),
        Some("Backend")
    );
    assert_eq!(
        db.node_to_subgraph.get("D").map(String::as_str),
        Some("Backend")
    );

    let data = get_data(&db);

    let group_frontend = data
        .nodes
        .iter()
        .find(|n| n.id == "Frontend")
        .expect("Frontend group");
    assert!(group_frontend.is_group);

    let group_backend = data
        .nodes
        .iter()
        .find(|n| n.id == "Backend")
        .expect("Backend group");
    assert!(group_backend.is_group);

    let node_a = data.nodes.iter().find(|n| n.id == "A").expect("A");
    let node_d = data.nodes.iter().find(|n| n.id == "D").expect("D");

    assert_eq!(node_a.parent_id.as_deref(), Some("Frontend"));
    assert_eq!(node_d.parent_id.as_deref(), Some("Backend"));
    assert!(!node_a.is_group);
    assert!(!node_d.is_group);
}
#[test]
fn flow_data_does_not_create_vertices_for_subgraph_edge_endpoints() {
    let mermaid = r#"flowchart TD
    A[Start]
    subgraph PIPE["Pipeline"]
        B[Step]
    end
    A --> PIPE
    PIPE --> C[Done]"#;

    let graph = flow_parser::parse_flowchart(mermaid).expect("mermaid should parse");
    let db = from_flowchart_graph(&graph);

    assert!(db.subgraphs.iter().any(|subgraph| subgraph.id == "PIPE"));
    assert!(!db.vertices.contains_key("PIPE"));

    let data = get_data(&db);
    let pipe_nodes: Vec<_> = data.nodes.iter().filter(|node| node.id == "PIPE").collect();
    assert_eq!(pipe_nodes.len(), 1);
    assert!(pipe_nodes[0].is_group);
}
