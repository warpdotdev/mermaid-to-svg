use std::collections::HashMap;

use crate::ast::{EdgeStyle, FlowchartGraph, GraphDirection, NodeShape};
use crate::layout::{LayoutEdge, LayoutNode, LayoutResult, LayoutSubgraph};
use crate::text_wrap::{
    measure_wrapped_lines, wrap_text_lines, DEFAULT_CHAR_WIDTH, DEFAULT_WRAP_WIDTH,
};
use dagre_rust::layout::layout as dagre_layout;
use dagre_rust::{GraphConfig, GraphEdge, GraphNode};
use graphlib_rust::Graph;

use super::cluster_adjust::{adjust_clusters_and_edges, ExtractedCluster};
use super::{flow_data, flow_db};

const FLOWCHART_PADDING: f64 = 15.0;
const EDGE_LABEL_PADDING: f64 = 2.0;
const SUBGRAPH_PADDING: f64 = 8.0;

#[derive(Debug, Clone)]
struct NodeMeta {
    label: String,
    shape: NodeShape,
    width: f64,
    height: f64,
    fill_color: Option<String>,
    stroke_color: Option<String>,
    is_group: bool,
    title: Option<String>,
}

#[derive(Debug, Clone)]
struct EdgeMeta {
    label: Option<String>,
    style: EdgeStyle,
}

#[derive(Debug, Default)]
struct LocalLayout {
    nodes: HashMap<String, LayoutNode>,
    edges: Vec<LayoutEdge>,
    subgraphs: Vec<LayoutSubgraph>,
    width: f64,
    height: f64,
}

pub fn compute_layout_ported(flowchart: &FlowchartGraph) -> LayoutResult {
    let db = flow_db::from_flowchart_graph(flowchart);
    let data = flow_data::get_data(&db);

    let rankdir = match db.direction {
        GraphDirection::TopToBottom => "tb",
        GraphDirection::BottomToTop => "bt",
        GraphDirection::LeftToRight => "lr",
        GraphDirection::RightToLeft => "rl",
    };

    let mut node_meta: HashMap<String, NodeMeta> = HashMap::new();
    for node in &data.nodes {
        let (fill_color, stroke_color) =
            node.styles.iter().fold((None, None), |mut acc, (k, v)| {
                if k == "fill" {
                    acc.0 = Some(v.clone());
                }
                if k == "stroke" {
                    acc.1 = Some(v.clone());
                }
                acc
            });

        let (width, height) = if node.is_group {
            (0.0, 0.0)
        } else {
            measure_node(&node.label, node.shape)
        };

        node_meta.insert(
            node.id.clone(),
            NodeMeta {
                label: node.label.clone(),
                shape: node.shape,
                width,
                height,
                fill_color,
                stroke_color,
                is_group: node.is_group,
                title: if node.is_group {
                    Some(node.label.clone())
                } else {
                    None
                },
            },
        );
    }

    let mut edge_meta: HashMap<(String, String), EdgeMeta> = HashMap::new();

    let mut g: Graph<GraphConfig, GraphNode, GraphEdge> =
        Graph::new(Some(graphlib_rust::GraphOption {
            directed: Some(true),
            multigraph: Some(true),
            compound: Some(true),
        }));
    g.set_graph(GraphConfig {
        rankdir: Some(rankdir.to_string()),
        nodesep: Some(50.0),
        ranksep: Some(50.0),
        marginx: Some(8.0),
        marginy: Some(8.0),
        ..Default::default()
    });

    for node in &data.nodes {
        if node.is_group {
            g.set_node(
                node.id.clone(),
                Some(GraphNode {
                    width: 0.0,
                    height: 0.0,
                    padding: Some(SUBGRAPH_PADDING as f32),
                    ..Default::default()
                }),
            );
        } else {
            let Some(meta) = node_meta.get(&node.id) else {
                continue;
            };
            g.set_node(
                node.id.clone(),
                Some(GraphNode {
                    width: meta.width as f32,
                    height: meta.height as f32,
                    ..Default::default()
                }),
            );
        }

        if let Some(parent_id) = &node.parent_id {
            let _ = g.set_parent(&node.id, Some(parent_id.clone()));
        }
    }

    for edge in &data.edges {
        let mut edge_label = GraphEdge {
            labelpos: Some("c".to_string()),
            ..Default::default()
        };
        if let Some(label) = &edge.label {
            if let Some((width, height)) = edge_label_dimensions(label) {
                edge_label.width = Some(width as f32);
                edge_label.height = Some(height as f32);
            }
        }

        edge_meta.insert(
            (edge.start.clone(), edge.end.clone()),
            EdgeMeta {
                label: edge.label.clone(),
                style: edge.style,
            },
        );

        let _ = g.set_edge(&edge.start, &edge.end, Some(edge_label), None);
    }

    let mut extracted = adjust_clusters_and_edges(&mut g);

    let layout = layout_recursive(&mut g, &mut extracted, &node_meta, &edge_meta);

    LayoutResult {
        nodes: layout.nodes,
        edges: layout.edges,
        subgraphs: layout.subgraphs,
        width: layout.width,
        height: layout.height,
    }
}

fn layout_recursive(
    graph: &mut Graph<GraphConfig, GraphNode, GraphEdge>,
    extracted: &mut HashMap<String, ExtractedCluster>,
    node_meta: &HashMap<String, NodeMeta>,
    edge_meta: &HashMap<(String, String), EdgeMeta>,
) -> LocalLayout {
    let mut child_layouts: HashMap<String, LocalLayout> = HashMap::new();

    for (cluster_id, cluster) in extracted.iter_mut() {
        let layout = layout_recursive(
            &mut cluster.graph,
            &mut cluster.children,
            node_meta,
            edge_meta,
        );
        if let Some(node) = graph.node_mut(cluster_id) {
            node.width = layout.width as f32;
            node.height = layout.height as f32;
        }
        child_layouts.insert(cluster_id.clone(), layout);
    }

    dagre_layout(graph);

    let mut layout = extract_local_layout(graph, node_meta, edge_meta);

    for (cluster_id, mut child_layout) in child_layouts {
        let Some(cluster_node) = graph.node(&cluster_id) else {
            continue;
        };

        let dx = cluster_node.x as f64 - cluster_node.width as f64 / 2.0;
        let dy = cluster_node.y as f64 - cluster_node.height as f64 / 2.0;
        shift_layout(&mut child_layout, dx, dy);

        for (id, node) in child_layout.nodes {
            layout.nodes.insert(id, node);
        }
        layout.edges.extend(child_layout.edges);
        layout.subgraphs.extend(child_layout.subgraphs);
    }

    layout
}

fn extract_local_layout(
    graph: &Graph<GraphConfig, GraphNode, GraphEdge>,
    node_meta: &HashMap<String, NodeMeta>,
    edge_meta: &HashMap<(String, String), EdgeMeta>,
) -> LocalLayout {
    let mut nodes: HashMap<String, LayoutNode> = HashMap::new();
    let mut subgraphs: Vec<LayoutSubgraph> = Vec::new();

    for node_id in graph.nodes() {
        let Some(meta) = node_meta.get(&node_id) else {
            continue;
        };
        let Some(node) = graph.node(&node_id) else {
            continue;
        };

        if meta.is_group {
            let width = node.width as f64;
            let height = node.height as f64;
            if width <= 0.0 || height <= 0.0 {
                continue;
            }
            subgraphs.push(LayoutSubgraph {
                id: node_id.clone(),
                title: meta.title.clone(),
                x: node.x as f64 - width / 2.0,
                y: node.y as f64 - height / 2.0,
                width,
                height,
            });
        } else {
            nodes.insert(
                node_id.clone(),
                LayoutNode {
                    id: node_id.clone(),
                    x: node.x as f64,
                    y: node.y as f64,
                    width: meta.width,
                    height: meta.height,
                    shape: meta.shape,
                    label: meta.label.clone(),
                    fill_color: meta.fill_color.clone(),
                    stroke_color: meta.stroke_color.clone(),
                },
            );
        }
    }

    let mut edges: Vec<LayoutEdge> = Vec::new();
    for edge_obj in graph.edges() {
        let Some(edge_label) = graph.edge_with_obj(&edge_obj) else {
            continue;
        };

        let Some(meta) = edge_meta.get(&(edge_obj.v.clone(), edge_obj.w.clone())) else {
            continue;
        };

        let points: Vec<(f64, f64)> = edge_label
            .points
            .as_ref()
            .map(|pts| pts.iter().map(|p| (p.x as f64, p.y as f64)).collect())
            .unwrap_or_default();

        let label_pos = meta.label.as_ref().and_then(|label| {
            if label.trim().is_empty() {
                None
            } else if edge_label.width.unwrap_or(0.0) > 0.0
                || edge_label.height.unwrap_or(0.0) > 0.0
            {
                Some((edge_label.x as f64, edge_label.y as f64))
            } else {
                None
            }
        });

        edges.push(LayoutEdge {
            from: edge_obj.v,
            to: edge_obj.w,
            label: meta.label.clone(),
            style: meta.style,
            points,
            label_pos,
        });
    }

    let graph_width = graph.graph().width as f64;
    let graph_height = graph.graph().height as f64;

    LocalLayout {
        nodes,
        edges,
        subgraphs,
        width: graph_width,
        height: graph_height,
    }
}

fn shift_layout(layout: &mut LocalLayout, dx: f64, dy: f64) {
    for node in layout.nodes.values_mut() {
        node.x += dx;
        node.y += dy;
    }

    for edge in &mut layout.edges {
        for point in &mut edge.points {
            point.0 += dx;
            point.1 += dy;
        }
        if let Some((x, y)) = edge.label_pos.as_mut() {
            *x += dx;
            *y += dy;
        }
    }

    for sg in &mut layout.subgraphs {
        sg.x += dx;
        sg.y += dy;
    }
}

fn measure_node(label: &str, shape: NodeShape) -> (f64, f64) {
    let lines = wrap_text_lines(label, DEFAULT_WRAP_WIDTH, DEFAULT_CHAR_WIDTH);
    let (text_width, text_height) = measure_wrapped_lines(&lines, DEFAULT_CHAR_WIDTH);
    let padding = FLOWCHART_PADDING;

    match shape {
        NodeShape::Rectangle => (text_width + padding * 4.0, text_height + padding * 2.0),
        NodeShape::RoundedRectangle => (text_width + padding * 2.0, text_height + padding * 2.0),
        NodeShape::Subroutine => {
            let w = text_width + padding;
            let h = text_height + padding;
            (w + 16.0, h)
        }
        NodeShape::Asymmetric => {
            let w = text_width + padding;
            let h = text_height + padding;
            (w + h / 4.0, h)
        }
        NodeShape::Hexagon => {
            let h = text_height + padding;
            let w = text_width + padding * 2.5;
            (w * 7.0 / 6.0, h)
        }
        NodeShape::Diamond => {
            let w = text_width + padding;
            let h = text_height + padding;
            let s = w + h;
            (s, s)
        }
        NodeShape::Circle => {
            let diameter = text_width + padding;
            (diameter, diameter)
        }
        NodeShape::Stadium => {
            let h = text_height + padding;
            let w = text_width + h / 4.0 + padding;
            (w, h)
        }
        NodeShape::Cylinder => {
            let w = text_width + padding;
            let rx = w / 2.0;
            let ry = rx / (2.5 + w / 50.0);
            let h = text_height + ry + padding;
            (w, h + 2.0 * ry)
        }
    }
}

fn edge_label_dimensions(label: &str) -> Option<(f64, f64)> {
    if label.trim().is_empty() {
        return None;
    }

    let lines = wrap_text_lines(label, DEFAULT_WRAP_WIDTH, DEFAULT_CHAR_WIDTH);
    if lines.is_empty() {
        return None;
    }

    let (text_width, text_height) = measure_wrapped_lines(&lines, DEFAULT_CHAR_WIDTH);
    let width = text_width + EDGE_LABEL_PADDING * 2.0;
    let height = text_height + EDGE_LABEL_PADDING * 2.0;
    Some((width, height))
}
