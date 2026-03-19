use super::*;

use crate::fixtures::FixtureCase;

use dagre_rust::{GraphConfig, GraphEdge, GraphNode};
use graphlib_rust::{Graph, GraphOption};

use std::collections::{HashMap, HashSet};

fn samples_root() -> String {
    format!(
        "{}/samples",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn fixture_mermaid_path(case: &FixtureCase) -> String {
    format!("{}/{}", samples_root(), case.mermaid_asset_path())
}

fn fixture_reference_svg_path(case: &FixtureCase) -> String {
    format!(
        "{}/{}/reference/{}.svg",
        samples_root(),
        case.diagram_type,
        case.name
    )
}

fn flowchart_case(name: &str) -> FixtureCase {
    FixtureCase {
        diagram_type: "flowchart".to_string(),
        name: name.to_string(),
    }
}

#[test]
fn parse_reference_svg_smoke() {
    let case = flowchart_case("01_basic_flowchart");
    let svg_path = fixture_reference_svg_path(&case);

    let svg = std::fs::read_to_string(svg_path).expect("reference svg should be readable");
    let geom = extract_geometry_from_reference_svg(&svg, 12).expect("should parse geometry");
    let json = geom.to_json_pretty().expect("should serialize to json");

    assert!(!geom.nodes.is_empty());
    assert!(!geom.edges.is_empty());
    assert!(json.contains("\"nodes\""));
    assert!(json.contains("\"edges\""));
    assert!(json.contains("\"clusters\""));
}

#[test]
fn reference_edge_id_parsing() {
    let (start, end) = parse_edge_id("L_A_B_0");
    assert_eq!(start.as_deref(), Some("A"));
    assert_eq!(end.as_deref(), Some("B"));
}

#[test]
fn topology_matches_reference_svg_for_01_basic_flowchart() {
    let case = flowchart_case("01_basic_flowchart");
    let mermaid_path = fixture_mermaid_path(&case);
    let reference_svg_path = fixture_reference_svg_path(&case);

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let actual_node_ids: HashSet<&str> = actual.nodes.iter().map(|n| n.id.as_str()).collect();
    let reference_node_ids: HashSet<&str> = reference.nodes.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(actual_node_ids, reference_node_ids);

    fn edge_counts(geom: &SvgGeometry) -> HashMap<(String, String), usize> {
        let mut counts: HashMap<(String, String), usize> = HashMap::new();
        for e in &geom.edges {
            let (Some(from), Some(to)) = (e.start.as_ref(), e.end.as_ref()) else {
                continue;
            };
            *counts.entry((from.clone(), to.clone())).or_insert(0) += 1;
        }
        counts
    }

    assert_eq!(edge_counts(&actual), edge_counts(&reference));
}

#[test]
fn node_centers_match_reference_svg_for_01_basic_flowchart() {
    let case = flowchart_case("01_basic_flowchart");
    let mermaid_path = fixture_mermaid_path(&case);
    let reference_svg_path = fixture_reference_svg_path(&case);

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let actual_by_id: HashMap<&str, &SvgNode> =
        actual.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    for r in &reference.nodes {
        let a = actual_by_id
            .get(r.id.as_str())
            .copied()
            .expect("node should exist");

        let dx = (a.center_x - r.center_x).abs();
        let dy = (a.center_y - r.center_y).abs();
        assert!(dx <= 3.0, "node {} dx too large: {dx}", r.id);
        assert!(dy <= 3.0, "node {} dy too large: {dy}", r.id);
    }
}

#[test]
fn edge_routes_match_reference_svg_for_01_basic_flowchart() {
    let case = flowchart_case("01_basic_flowchart");
    let mermaid_path = fixture_mermaid_path(&case);
    let reference_svg_path = fixture_reference_svg_path(&case);

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let act_list = actual_edges.get(&key).expect("edge pair should exist");
        assert_eq!(act_list.len(), ref_list.len());

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let act_edge = act_list.get(idx).copied().expect("edge index should exist");
            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            assert!(d <= 3.0, "edge {:?}[{idx}] hausdorff too large: {d}", key);
        }
    }
}

#[test]
fn node_centers_match_reference_svg_for_07_horizontal() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path =
        format!("{base_dir}/samples/flowchart/mermaid/07_horizontal.mmd");
    let reference_svg_path =
        format!("{base_dir}/samples/flowchart/reference/07_horizontal.svg");

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    for r in &reference.nodes {
        let a = actual
            .nodes
            .iter()
            .find(|n| n.id == r.id)
            .expect("node should exist");

        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        let dist = (dx * dx + dy * dy).sqrt();
        assert!(dist <= 8.0, "node {} dist too large: {dist}", r.id);
    }
}

#[test]
fn edge_routes_match_reference_svg_for_07_horizontal() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path =
        format!("{base_dir}/samples/flowchart/mermaid/07_horizontal.mmd");
    let reference_svg_path =
        format!("{base_dir}/samples/flowchart/reference/07_horizontal.svg");

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let act_list = actual_edges.get(&key).expect("edge pair should exist");
        assert_eq!(act_list.len(), ref_list.len());

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let act_edge = act_list.get(idx).copied().expect("edge index should exist");
            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            assert!(d <= 8.0, "edge {:?}[{idx}] hausdorff too large: {d}", key);
        }
    }
}

#[test]
#[ignore]
fn debug_node_position_deltas_for_01_basic_flowchart() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/01_basic_flowchart.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/01_basic_flowchart.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let actual_by_id: HashMap<&str, &SvgNode> =
        actual.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let reference_by_id: HashMap<&str, &SvgNode> =
        reference.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let mut max_dist: f64 = 0.0;
    let mut sum_dist: f64 = 0.0;
    let mut n = 0_usize;

    let mut ids: Vec<&str> = reference_by_id.keys().copied().collect();
    ids.sort();

    for id in ids {
        let Some(r) = reference_by_id.get(id).copied() else {
            continue;
        };
        let Some(a) = actual_by_id.get(id).copied() else {
            continue;
        };

        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        let dist = (dx * dx + dy * dy).sqrt();

        max_dist = max_dist.max(dist);
        sum_dist += dist;
        n += 1;

        println!("{id}: dx={dx:.1} dy={dy:.1} dist={dist:.1}");
    }

    let avg_dist = if n == 0 { 0.0 } else { sum_dist / n as f64 };
    println!("centroid shift: dx0={dx0:.1} dy0={dy0:.1}");
    println!("node deltas: max_dist={max_dist:.1} avg_dist={avg_dist:.1} n={n}");
}

#[test]
#[ignore]
fn debug_edge_hausdorff_for_01_basic_flowchart() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/01_basic_flowchart.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/01_basic_flowchart.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let Some(act_list) = actual_edges.get(&key) else {
            println!("missing actual edges for {:?}", key);
            continue;
        };

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let Some(act_edge) = act_list.get(idx) else {
                println!("missing actual edge idx={idx} for {:?}", key);
                continue;
            };

            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            println!("{:?}[{idx}] hausdorff={d:.2}", key);
        }
    }
}

#[test]
#[ignore]
fn debug_parity_for_06_subgraphs() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path =
        format!("{base_dir}/samples/flowchart/mermaid/06_subgraphs.mmd");
    let reference_svg_path =
        format!("{base_dir}/samples/flowchart/reference/06_subgraphs.svg");

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let mut max_node_dist: f64 = 0.0;
    for r in &reference.nodes {
        let a = actual
            .nodes
            .iter()
            .find(|n| n.id == r.id)
            .expect("node should exist");
        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        max_node_dist = max_node_dist.max((dx * dx + dy * dy).sqrt());
    }

    println!("node max dist: {max_node_dist:.2}");

    let actual_clusters: HashMap<&str, &SvgCluster> =
        actual.clusters.iter().map(|c| (c.id.as_str(), c)).collect();
    let reference_clusters: HashMap<&str, &SvgCluster> = reference
        .clusters
        .iter()
        .map(|c| (c.id.as_str(), c))
        .collect();

    let mut cluster_ids: Vec<&str> = reference_clusters.keys().copied().collect();
    cluster_ids.sort();

    for cid in cluster_ids {
        let Some(r) = reference_clusters.get(cid).copied() else {
            continue;
        };
        let Some(a) = actual_clusters.get(cid).copied() else {
            println!("missing actual cluster: {cid}");
            continue;
        };

        let dx = (a.x + dx0) - r.x;
        let dy = (a.y + dy0) - r.y;
        let dw = a.width - r.width;
        let dh = a.height - r.height;

        println!("cluster {cid}: dx={dx:.2} dy={dy:.2} dw={dw:.2} dh={dh:.2}");
    }

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let Some(act_list) = actual_edges.get(&key) else {
            println!("missing actual edges for {:?}", key);
            continue;
        };

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let Some(act_edge) = act_list.get(idx) else {
                println!("missing actual edge idx={idx} for {:?}", key);
                continue;
            };

            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            println!("{:?}[{idx}] hausdorff={d:.2}", key);
        }
    }
}

#[test]
#[ignore]
fn debug_parity_for_26_nested_subgraphs() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/26_nested_subgraphs.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/26_nested_subgraphs.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout_no_subgraph_centering(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    debug_print_parity(&actual, &reference);
}

#[test]
#[ignore]
fn debug_parity_for_26_nested_subgraphs_ported() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/26_nested_subgraphs.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/26_nested_subgraphs.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::mermaid_port::compute_layout_ported(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    debug_print_parity(&actual, &reference);
}

#[test]
#[ignore]
fn debug_layout_result_positions_for_26_nested_subgraphs_no_centering() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/26_nested_subgraphs.mmd"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout_no_subgraph_centering(&graph);

    for id in ["E", "A", "B", "C", "D", "F"] {
        let node = layout.nodes.get(id).expect("node should exist");
        println!(
            "{id}: x={:.3} y={:.3} w={:.3} h={:.3}",
            node.x, node.y, node.width, node.height
        );
    }

    let mut subgraphs = layout.subgraphs.clone();
    subgraphs.sort_by(|a, b| a.id.cmp(&b.id));
    for sg in subgraphs {
        println!(
            "{}: x={:.3} y={:.3} w={:.3} h={:.3}",
            sg.id, sg.x, sg.y, sg.width, sg.height
        );
    }
}

#[test]
#[ignore]
fn debug_reference_svg_positions_for_26_nested_subgraphs() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/26_nested_subgraphs.svg"
    );

    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    for id in ["E", "A", "B", "C", "D", "F"] {
        let node = reference
            .nodes
            .iter()
            .find(|n| n.id == id)
            .expect("node should exist");
        println!(
            "{id}: x={:.3} y={:.3} w={:.3} h={:.3}",
            node.center_x, node.center_y, node.width, node.height
        );
    }

    let mut clusters = reference.clusters.clone();
    clusters.sort_by(|a, b| a.id.cmp(&b.id));
    for c in clusters {
        println!(
            "{}: x={:.3} y={:.3} w={:.3} h={:.3}",
            c.id, c.x, c.y, c.width, c.height
        );
    }
}

#[test]
#[ignore]
fn debug_parity_for_26_nested_subgraphs_with_centering() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/26_nested_subgraphs.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/26_nested_subgraphs.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    debug_print_parity(&actual, &reference);
}

#[test]
#[ignore]
fn debug_parity_summary_for_all_samples() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_dir = format!("{base_dir}/samples/flowchart/mermaid");
    let reference_dir = format!("{base_dir}/samples/flowchart/reference");

    let mut entries: Vec<_> = std::fs::read_dir(&mermaid_dir)
        .expect("mermaid samples dir should be readable")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s == "mmd")
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut worst: Option<(String, f64, f64, f64)> = None;

    for entry in entries {
        let path = entry.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let mermaid = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("mermaid sample should be readable: {path:?}"));
        let reference_svg_path = format!("{reference_dir}/{stem}.svg");
        let reference_svg = std::fs::read_to_string(&reference_svg_path)
            .unwrap_or_else(|_| panic!("reference svg should be readable: {reference_svg_path}"));

        let graph = crate::parser::parse_mermaid(&mermaid)
            .unwrap_or_else(|_| panic!("mermaid should parse: {stem}"));
        let layout = crate::layout::compute_layout(&graph);

        let actual = extract_geometry_from_layout_result(&layout);
        let reference = extract_geometry_from_reference_svg(&reference_svg, 12)
            .unwrap_or_else(|_| panic!("should parse reference geometry: {stem}"));

        let (ref_cx, ref_cy) = centroid(&reference.nodes);
        let (act_cx, act_cy) = centroid(&actual.nodes);
        let dx0 = ref_cx - act_cx;
        let dy0 = ref_cy - act_cy;

        let mut node_max_dist: f64 = 0.0;
        for r in &reference.nodes {
            let Some(a) = actual.nodes.iter().find(|n| n.id == r.id) else {
                node_max_dist = f64::INFINITY;
                break;
            };
            let dx = (a.center_x + dx0) - r.center_x;
            let dy = (a.center_y + dy0) - r.center_y;
            node_max_dist = node_max_dist.max((dx * dx + dy * dy).sqrt());
        }

        let mut edge_max_hausdorff: f64 = 0.0;
        let actual_edges = group_edges_by_pair(&actual);
        let reference_edges = group_edges_by_pair(&reference);
        for (key, ref_list) in reference_edges {
            let Some(act_list) = actual_edges.get(&key) else {
                edge_max_hausdorff = f64::INFINITY;
                break;
            };

            for (idx, ref_edge) in ref_list.iter().enumerate() {
                let Some(act_edge) = act_list.get(idx) else {
                    edge_max_hausdorff = f64::INFINITY;
                    break;
                };

                let shifted = shift_points(&act_edge.points, dx0, dy0);
                let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
                edge_max_hausdorff = edge_max_hausdorff.max(d);
            }
        }

        let mut cluster_max_delta: f64 = 0.0;
        let actual_clusters: HashMap<&str, &SvgCluster> =
            actual.clusters.iter().map(|c| (c.id.as_str(), c)).collect();
        for r in &reference.clusters {
            let Some(a) = actual_clusters.get(r.id.as_str()).copied() else {
                cluster_max_delta = f64::INFINITY;
                break;
            };
            let dx = ((a.x + dx0) - r.x).abs();
            let dy = ((a.y + dy0) - r.y).abs();
            let dw = (a.width - r.width).abs();
            let dh = (a.height - r.height).abs();
            cluster_max_delta = cluster_max_delta.max(dx.max(dy).max(dw).max(dh));
        }

        println!(
            "{stem}: node_max={:.2} edge_max={:.2} cluster_max={:.2}",
            node_max_dist, edge_max_hausdorff, cluster_max_delta
        );

        let update_worst = match &worst {
            Some((_, w_node, w_edge, w_cluster)) => {
                node_max_dist > *w_node
                    || edge_max_hausdorff > *w_edge
                    || cluster_max_delta > *w_cluster
            }
            None => true,
        };

        if update_worst {
            worst = Some((
                stem.to_string(),
                node_max_dist,
                edge_max_hausdorff,
                cluster_max_delta,
            ));
        }
    }

    if let Some((name, node, edge, cluster)) = worst {
        println!("worst: {name} node_max={node:.2} edge_max={edge:.2} cluster_max={cluster:.2}");
    }
}

#[test]
#[ignore]
fn debug_parity_summary_for_all_samples_ported() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_dir = format!("{base_dir}/samples/flowchart/mermaid");
    let reference_dir = format!("{base_dir}/samples/flowchart/reference");

    let mut entries: Vec<_> = std::fs::read_dir(&mermaid_dir)
        .expect("mermaid samples dir should be readable")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s == "mmd")
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut worst: Option<(String, f64, f64, f64)> = None;

    for entry in entries {
        let path = entry.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let mermaid = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("mermaid sample should be readable: {path:?}"));
        let reference_svg_path = format!("{reference_dir}/{stem}.svg");
        let reference_svg = std::fs::read_to_string(&reference_svg_path)
            .unwrap_or_else(|_| panic!("reference svg should be readable: {reference_svg_path}"));

        let graph = crate::parser::parse_mermaid(&mermaid)
            .unwrap_or_else(|_| panic!("mermaid should parse: {stem}"));
        let layout = crate::mermaid_port::compute_layout_ported(&graph);

        let actual = extract_geometry_from_layout_result(&layout);
        let reference = extract_geometry_from_reference_svg(&reference_svg, 12)
            .unwrap_or_else(|_| panic!("should parse reference geometry: {stem}"));

        let (ref_cx, ref_cy) = centroid(&reference.nodes);
        let (act_cx, act_cy) = centroid(&actual.nodes);
        let dx0 = ref_cx - act_cx;
        let dy0 = ref_cy - act_cy;

        let mut node_max_dist: f64 = 0.0;
        for r in &reference.nodes {
            let Some(a) = actual.nodes.iter().find(|n| n.id == r.id) else {
                node_max_dist = f64::INFINITY;
                break;
            };
            let dx = (a.center_x + dx0) - r.center_x;
            let dy = (a.center_y + dy0) - r.center_y;
            node_max_dist = node_max_dist.max((dx * dx + dy * dy).sqrt());
        }

        let mut edge_max_hausdorff: f64 = 0.0;
        let actual_edges = group_edges_by_pair(&actual);
        let reference_edges = group_edges_by_pair(&reference);
        for (key, ref_list) in reference_edges {
            let Some(act_list) = actual_edges.get(&key) else {
                edge_max_hausdorff = f64::INFINITY;
                break;
            };

            for (idx, ref_edge) in ref_list.iter().enumerate() {
                let Some(act_edge) = act_list.get(idx) else {
                    edge_max_hausdorff = f64::INFINITY;
                    break;
                };

                let shifted = shift_points(&act_edge.points, dx0, dy0);
                let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
                edge_max_hausdorff = edge_max_hausdorff.max(d);
            }
        }

        let mut cluster_max_delta: f64 = 0.0;
        let actual_clusters: HashMap<&str, &SvgCluster> =
            actual.clusters.iter().map(|c| (c.id.as_str(), c)).collect();
        for r in &reference.clusters {
            let Some(a) = actual_clusters.get(r.id.as_str()).copied() else {
                cluster_max_delta = f64::INFINITY;
                break;
            };
            let dx = ((a.x + dx0) - r.x).abs();
            let dy = ((a.y + dy0) - r.y).abs();
            let dw = (a.width - r.width).abs();
            let dh = (a.height - r.height).abs();
            cluster_max_delta = cluster_max_delta.max(dx.max(dy).max(dw).max(dh));
        }

        println!(
            "{stem}: node_max={:.2} edge_max={:.2} cluster_max={:.2}",
            node_max_dist, edge_max_hausdorff, cluster_max_delta
        );

        let update_worst = match &worst {
            Some((_, w_node, w_edge, w_cluster)) => {
                node_max_dist > *w_node
                    || edge_max_hausdorff > *w_edge
                    || cluster_max_delta > *w_cluster
            }
            None => true,
        };

        if update_worst {
            worst = Some((
                stem.to_string(),
                node_max_dist,
                edge_max_hausdorff,
                cluster_max_delta,
            ));
        }
    }

    if let Some((name, node, edge, cluster)) = worst {
        println!("worst: {name} node_max={node:.2} edge_max={edge:.2} cluster_max={cluster:.2}");
    }
}

#[test]
#[ignore]
fn debug_parity_for_49_dense_connections() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path = format!(
        "{base_dir}/samples/flowchart/mermaid/49_dense_connections.mmd"
    );
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/49_dense_connections.svg"
    );

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    debug_print_parity(&actual, &reference);
}

#[test]
#[ignore]
fn debug_dagre_direct_positions_for_49_dense_connections() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/49_dense_connections.svg"
    );

    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let mut g: Graph<GraphConfig, GraphNode, GraphEdge> = Graph::new(Some(GraphOption {
        directed: Some(true),
        multigraph: Some(true),
        compound: Some(false),
    }));

    g.set_graph(GraphConfig {
        rankdir: Some("tb".to_string()),
        nodesep: Some(50.0),
        ranksep: Some(50.0),
        edgesep: Some(20.0),
        marginx: Some(8.0),
        marginy: Some(8.0),
        ..Default::default()
    });

    for n in &reference.nodes {
        g.set_node(
            n.id.clone(),
            Some(GraphNode {
                width: n.width as f32,
                height: n.height as f32,
                ..Default::default()
            }),
        );
    }

    for e in &reference.edges {
        let (Some(from), Some(to)) = (e.start.as_ref(), e.end.as_ref()) else {
            continue;
        };
        let _ = g.set_edge(from, to, Some(GraphEdge::default()), None);
    }

    dagre_rust::layout::layout(&mut g);

    for id in ["A", "B", "C", "D", "E", "F", "G"] {
        let node = g.node(&id.to_string()).expect("node should exist");
        println!("direct {id}: x={:.2} y={:.2}", node.x, node.y);
    }

    let mut actual_nodes: Vec<SvgNode> = Vec::new();
    for id in g.nodes() {
        let Some(node) = g.node(&id) else {
            continue;
        };
        if node.dummy.is_some() {
            continue;
        }
        actual_nodes.push(SvgNode {
            id,
            center_x: node.x as f64,
            center_y: node.y as f64,
            width: node.width as f64,
            height: node.height as f64,
        });
    }

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual_nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let mut max_node_dist: f64 = 0.0;
    for r in &reference.nodes {
        let Some(a) = actual_nodes.iter().find(|n| n.id == r.id) else {
            max_node_dist = f64::INFINITY;
            break;
        };
        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        max_node_dist = max_node_dist.max((dx * dx + dy * dy).sqrt());
    }

    println!("direct dagre node max dist: {max_node_dist:.2}");

    for r in &reference.nodes {
        let Some(a) = actual_nodes.iter().find(|n| n.id == r.id) else {
            continue;
        };
        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        let dist = (dx * dx + dy * dy).sqrt();
        println!("{}: dx={dx:.2} dy={dy:.2} dist={dist:.2}", r.id);
    }
}

#[test]
#[ignore]
fn debug_dagre_layering_for_49_dense_connections() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let reference_svg_path = format!(
        "{base_dir}/samples/flowchart/reference/49_dense_connections.svg"
    );

    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let mut g: Graph<GraphConfig, GraphNode, GraphEdge> = Graph::new(Some(GraphOption {
        directed: Some(true),
        multigraph: Some(true),
        compound: Some(false),
    }));

    g.set_graph(GraphConfig {
        rankdir: Some("tb".to_string()),
        nodesep: Some(50.0),
        ranksep: Some(50.0),
        edgesep: Some(20.0),
        marginx: Some(8.0),
        marginy: Some(8.0),
        ..Default::default()
    });

    for n in &reference.nodes {
        g.set_node(
            n.id.clone(),
            Some(GraphNode {
                width: n.width as f32,
                height: n.height as f32,
                ..Default::default()
            }),
        );
    }

    for e in &reference.edges {
        let (Some(from), Some(to)) = (e.start.as_ref(), e.end.as_ref()) else {
            continue;
        };
        let _ = g.set_edge(from, to, Some(GraphEdge::default()), None);
    }

    dagre_rust::layout::layout(&mut g);

    let layering = dagre_rust::layout::util::build_layer_matrix(&g);
    for (rank, layer) in layering.iter().enumerate() {
        println!("rank {rank}:");
        for v in layer {
            let node = g.node(v).expect("node should exist");
            println!(
                "  {v}: order={:?} dummy={:?} x={:.2} y={:.2}",
                node.order, node.dummy, node.x, node.y
            );
        }
    }
}

#[test]
#[ignore]
fn debug_dagre_direct_positions_for_26_nested_subgraphs() {
    let mut g: Graph<GraphConfig, GraphNode, GraphEdge> = Graph::new(Some(GraphOption {
        directed: Some(true),
        multigraph: Some(true),
        compound: Some(true),
    }));

    g.set_graph(GraphConfig {
        rankdir: Some("tb".to_string()),
        nodesep: Some(50.0),
        ranksep: Some(50.0),
        edgesep: Some(20.0),
        ..Default::default()
    });

    for id in ["Outer", "Inner1", "Inner2"] {
        g.set_node(
            id.to_string(),
            Some(GraphNode {
                width: 0.0,
                height: 0.0,
                padding: Some(8.0),
                ..Default::default()
            }),
        );
    }

    let node_sizes: [(&str, f32); 6] = [
        ("A", 109.8125),
        ("B", 110.3125),
        ("C", 110.828125),
        ("D", 111.078125),
        ("E", 119.75),
        ("F", 109.875),
    ];

    for (id, width) in node_sizes {
        g.set_node(
            id.to_string(),
            Some(GraphNode {
                width,
                height: 49.0,
                ..Default::default()
            }),
        );
    }

    let _ = g.set_parent(&"Inner1".to_string(), Some("Outer".to_string()));
    let _ = g.set_parent(&"Inner2".to_string(), Some("Outer".to_string()));

    let _ = g.set_parent(&"A".to_string(), Some("Inner1".to_string()));
    let _ = g.set_parent(&"B".to_string(), Some("Inner1".to_string()));
    let _ = g.set_parent(&"C".to_string(), Some("Inner2".to_string()));
    let _ = g.set_parent(&"D".to_string(), Some("Inner2".to_string()));

    let _ = g.set_edge(
        &"A".to_string(),
        &"B".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"B".to_string(),
        &"C".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"C".to_string(),
        &"D".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"E".to_string(),
        &"A".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"D".to_string(),
        &"F".to_string(),
        Some(GraphEdge::default()),
        None,
    );

    dagre_rust::layout::layout(&mut g);

    for id in ["E", "A", "B", "C", "D", "F"] {
        let node = g.node(&id.to_string()).expect("node should exist");
        println!("{id}: x={:.3} y={:.3}", node.x, node.y);
    }

    for id in ["Inner1", "Inner2", "Outer"] {
        let node = g.node(&id.to_string()).expect("cluster should exist");
        println!(
            "{id}: x={:.3} y={:.3} w={:.3} h={:.3}",
            node.x, node.y, node.width, node.height
        );
    }
}

#[test]
#[ignore]
fn debug_edge_dummy_chain_for_e_to_a_in_26_nested_subgraphs() {
    let mut g: Graph<GraphConfig, GraphNode, GraphEdge> = Graph::new(Some(GraphOption {
        directed: Some(true),
        multigraph: Some(true),
        compound: Some(true),
    }));

    g.set_graph(GraphConfig {
        rankdir: Some("tb".to_string()),
        nodesep: Some(50.0),
        ranksep: Some(50.0),
        edgesep: Some(20.0),
        ..Default::default()
    });

    for id in ["Outer", "Inner1", "Inner2"] {
        g.set_node(
            id.to_string(),
            Some(GraphNode {
                width: 0.0,
                height: 0.0,
                padding: Some(8.0),
                ..Default::default()
            }),
        );
    }

    let node_sizes: [(&str, f32); 6] = [
        ("A", 109.8125),
        ("B", 110.3125),
        ("C", 110.828125),
        ("D", 111.078125),
        ("E", 119.75),
        ("F", 109.875),
    ];

    for (id, width) in node_sizes {
        g.set_node(
            id.to_string(),
            Some(GraphNode {
                width,
                height: 49.0,
                ..Default::default()
            }),
        );
    }

    let _ = g.set_parent(&"Inner1".to_string(), Some("Outer".to_string()));
    let _ = g.set_parent(&"Inner2".to_string(), Some("Outer".to_string()));

    let _ = g.set_parent(&"A".to_string(), Some("Inner1".to_string()));
    let _ = g.set_parent(&"B".to_string(), Some("Inner1".to_string()));
    let _ = g.set_parent(&"C".to_string(), Some("Inner2".to_string()));
    let _ = g.set_parent(&"D".to_string(), Some("Inner2".to_string()));

    let _ = g.set_edge(
        &"A".to_string(),
        &"B".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"B".to_string(),
        &"C".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"C".to_string(),
        &"D".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"E".to_string(),
        &"A".to_string(),
        Some(GraphEdge::default()),
        None,
    );
    let _ = g.set_edge(
        &"D".to_string(),
        &"F".to_string(),
        Some(GraphEdge::default()),
        None,
    );

    dagre_rust::layout::make_space_for_edge_labels(&mut g);
    dagre_rust::layout::remove_self_edges(&mut g);
    dagre_rust::layout::acyclic::run(&mut g);
    dagre_rust::layout::nesting_graph::run(&mut g);

    let mut nc_graph = dagre_rust::layout::util::as_non_compound_graph(&mut g);
    dagre_rust::layout::rank::rank(&mut nc_graph);
    dagre_rust::layout::util::transfer_node_edge_labels(&nc_graph, &mut g);

    dagre_rust::layout::inject_edge_label_proxies(&mut g);
    dagre_rust::layout::util::remove_empty_ranks(&mut g);
    dagre_rust::layout::nesting_graph::cleanup(&mut g);
    dagre_rust::layout::util::normalize_ranks(&mut g);
    dagre_rust::layout::assign_rank_min_max(&mut g);
    dagre_rust::layout::remove_edge_label_proxies(&mut g);

    dagre_rust::layout::normalize::run(&mut g);
    dagre_rust::layout::parent_dummy_chains::parent_dummy_chains(&mut g);

    for v in g.nodes() {
        let node = g.node(&v).expect("node should exist");
        if node.dummy.as_deref() != Some("edge") {
            continue;
        }
        let Some(edge_obj) = node.edge_obj.as_ref() else {
            continue;
        };
        if edge_obj.v != "E" || edge_obj.w != "A" {
            continue;
        }

        let parent = g.parent(&v).cloned().unwrap_or_default();
        println!("{v}: rank={:?} parent={parent}", node.rank);
    }

    for id in ["E", "A", "Outer", "Inner1", "Inner2"] {
        let node = g.node(&id.to_string()).expect("node should exist");
        let parent = g.parent(&id.to_string()).cloned().unwrap_or_default();
        println!(
            "{id}: rank={:?} parent={parent} min_rank={:?} max_rank={:?}",
            node.rank, node.min_rank, node.max_rank
        );
    }
}

fn debug_print_parity(actual: &SvgGeometry, reference: &SvgGeometry) {
    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let mut deltas: Vec<(&str, f64, f64, f64)> = Vec::new();

    for r in &reference.nodes {
        let a = actual
            .nodes
            .iter()
            .find(|n| n.id == r.id)
            .expect("node should exist");
        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let dw = a.width - r.width;
        let dh = a.height - r.height;
        if dw.abs() > 0.5 || dh.abs() > 0.5 {
            println!("{id} size: dw={dw:.2} dh={dh:.2}", id = r.id);
        }
        deltas.push((r.id.as_str(), dx, dy, dist));
    }

    deltas.sort_by(|a, b| b.3.total_cmp(&a.3));

    let max_node_dist = deltas.first().map(|d| d.3).unwrap_or(0.0);
    println!("node max dist: {max_node_dist:.2}");

    for (id, dx, dy, dist) in deltas {
        println!("{id}: dx={dx:.2} dy={dy:.2} dist={dist:.2}");
    }

    let actual_clusters: HashMap<&str, &SvgCluster> =
        actual.clusters.iter().map(|c| (c.id.as_str(), c)).collect();
    let reference_clusters: HashMap<&str, &SvgCluster> = reference
        .clusters
        .iter()
        .map(|c| (c.id.as_str(), c))
        .collect();

    let mut cluster_ids: Vec<&str> = reference_clusters.keys().copied().collect();
    cluster_ids.sort();

    for cid in cluster_ids {
        let Some(r) = reference_clusters.get(cid).copied() else {
            continue;
        };
        let Some(a) = actual_clusters.get(cid).copied() else {
            println!("missing actual cluster: {cid}");
            continue;
        };

        let dx = (a.x + dx0) - r.x;
        let dy = (a.y + dy0) - r.y;
        let dw = a.width - r.width;
        let dh = a.height - r.height;

        println!("cluster {cid}: dx={dx:.2} dy={dy:.2} dw={dw:.2} dh={dh:.2}");
    }

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let Some(act_list) = actual_edges.get(&key) else {
            println!("missing actual edges for {:?}", key);
            continue;
        };

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let Some(act_edge) = act_list.get(idx) else {
                println!("missing actual edge idx={idx} for {:?}", key);
                continue;
            };

            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            println!("{:?}[{idx}] hausdorff={d:.2}", key);
        }
    }
}

#[test]
#[ignore]
fn debug_parity_for_07_horizontal() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let mermaid_path =
        format!("{base_dir}/samples/flowchart/mermaid/07_horizontal.mmd");
    let reference_svg_path =
        format!("{base_dir}/samples/flowchart/reference/07_horizontal.svg");

    let mermaid = std::fs::read_to_string(mermaid_path).expect("mermaid source should be readable");
    let reference_svg =
        std::fs::read_to_string(reference_svg_path).expect("reference svg should be readable");

    let graph = crate::parser::parse_mermaid(&mermaid).expect("mermaid should parse");
    let layout = crate::layout::compute_layout(&graph);

    let actual = extract_geometry_from_layout_result(&layout);
    let reference =
        extract_geometry_from_reference_svg(&reference_svg, 12).expect("should parse geometry");

    let (ref_cx, ref_cy) = centroid(&reference.nodes);
    let (act_cx, act_cy) = centroid(&actual.nodes);
    let dx0 = ref_cx - act_cx;
    let dy0 = ref_cy - act_cy;

    let mut max_node_dist: f64 = 0.0;
    for r in &reference.nodes {
        let a = actual
            .nodes
            .iter()
            .find(|n| n.id == r.id)
            .expect("node should exist");
        let dx = (a.center_x + dx0) - r.center_x;
        let dy = (a.center_y + dy0) - r.center_y;
        max_node_dist = max_node_dist.max((dx * dx + dy * dy).sqrt());
    }

    println!("node max dist: {max_node_dist:.2}");

    let actual_edges = group_edges_by_pair(&actual);
    let reference_edges = group_edges_by_pair(&reference);

    for (key, ref_list) in reference_edges {
        let Some(act_list) = actual_edges.get(&key) else {
            println!("missing actual edges for {:?}", key);
            continue;
        };

        for (idx, ref_edge) in ref_list.iter().enumerate() {
            let Some(act_edge) = act_list.get(idx) else {
                println!("missing actual edge idx={idx} for {:?}", key);
                continue;
            };

            let shifted = shift_points(&act_edge.points, dx0, dy0);
            let d = directed_hausdorff_distance(&ref_edge.points, &shifted);
            println!("{:?}[{idx}] hausdorff={d:.2}", key);
        }
    }
}

fn group_edges_by_pair(geom: &SvgGeometry) -> HashMap<(String, String), Vec<&SvgEdge>> {
    let mut out: HashMap<(String, String), Vec<&SvgEdge>> = HashMap::new();

    for e in &geom.edges {
        let (Some(from), Some(to)) = (e.start.as_ref(), e.end.as_ref()) else {
            continue;
        };
        out.entry((from.clone(), to.clone())).or_default().push(e);
    }

    for edges in out.values_mut() {
        edges.sort_by(|a, b| a.id.cmp(&b.id));
    }

    out
}

fn shift_points(points: &[(f64, f64)], dx: f64, dy: f64) -> Vec<(f64, f64)> {
    points.iter().map(|(x, y)| (x + dx, y + dy)).collect()
}

fn directed_hausdorff_distance(samples: &[(f64, f64)], polyline: &[(f64, f64)]) -> f64 {
    samples
        .iter()
        .map(|&p| point_polyline_distance(p, polyline))
        .fold(0.0_f64, f64::max)
}

fn point_polyline_distance(p: (f64, f64), polyline: &[(f64, f64)]) -> f64 {
    if polyline.len() < 2 {
        return f64::INFINITY;
    }

    let mut min_d = f64::INFINITY;
    for seg in polyline.windows(2) {
        let d = point_segment_distance(p, seg[0], seg[1]);
        min_d = min_d.min(d);
    }

    min_d
}

fn point_segment_distance(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let abx = b.0 - a.0;
    let aby = b.1 - a.1;

    let apx = p.0 - a.0;
    let apy = p.1 - a.1;

    let ab_len2 = abx * abx + aby * aby;
    if ab_len2 == 0.0 {
        let dx = p.0 - a.0;
        let dy = p.1 - a.1;
        return (dx * dx + dy * dy).sqrt();
    }

    let t = (apx * abx + apy * aby) / ab_len2;
    let t = t.clamp(0.0, 1.0);

    let proj = (a.0 + t * abx, a.1 + t * aby);
    let dx = p.0 - proj.0;
    let dy = p.1 - proj.1;
    (dx * dx + dy * dy).sqrt()
}

fn centroid(nodes: &[SvgNode]) -> (f64, f64) {
    let mut sx: f64 = 0.0;
    let mut sy: f64 = 0.0;
    let mut n: f64 = 0.0;

    for node in nodes {
        sx += node.center_x;
        sy += node.center_y;
        n += 1.0;
    }

    if n == 0.0 {
        (0.0, 0.0)
    } else {
        (sx / n, sy / n)
    }
}
