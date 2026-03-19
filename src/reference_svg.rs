use anyhow::{anyhow, Result};
use roxmltree::Document;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SvgGeometry {
    pub nodes: Vec<SvgNode>,
    pub edges: Vec<SvgEdge>,
    pub clusters: Vec<SvgCluster>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SvgNode {
    pub id: String,
    pub center_x: f64,
    pub center_y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SvgEdge {
    pub id: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SvgCluster {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
}

impl SvgGeometry {
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

pub fn extract_geometry_from_reference_svg(
    svg: &str,
    samples_per_cubic: usize,
) -> Result<SvgGeometry> {
    let doc = Document::parse(svg)?;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut clusters = Vec::new();

    for node in doc.descendants().filter(|n| n.is_element()) {
        match node.tag_name().name() {
            "g" => {
                if node_attr_contains(&node, "class", "node") {
                    let Some(id_attr) = node.attribute("id") else {
                        continue;
                    };
                    let Some((cx, cy)) = node.attribute("transform").and_then(parse_translate)
                    else {
                        continue;
                    };
                    let Some((width, height)) = bbox_from_node_group(&node) else {
                        continue;
                    };

                    nodes.push(SvgNode {
                        id: normalize_flowchart_node_id(id_attr),
                        center_x: cx,
                        center_y: cy,
                        width,
                        height,
                    });
                } else if node_attr_contains(&node, "class", "cluster") {
                    let Some(id_attr) = node.attribute("id") else {
                        continue;
                    };
                    let Some(rect) = node
                        .children()
                        .find(|c| c.is_element() && c.tag_name().name() == "rect")
                    else {
                        continue;
                    };
                    let (Some(x_attr), Some(y_attr), Some(w_attr), Some(h_attr)) = (
                        rect.attribute("x"),
                        rect.attribute("y"),
                        rect.attribute("width"),
                        rect.attribute("height"),
                    ) else {
                        continue;
                    };
                    let (Ok(x), Ok(y), Ok(width), Ok(height)) = (
                        x_attr.parse::<f64>(),
                        y_attr.parse::<f64>(),
                        w_attr.parse::<f64>(),
                        h_attr.parse::<f64>(),
                    ) else {
                        continue;
                    };

                    let label = extract_cluster_label(&node);

                    clusters.push(SvgCluster {
                        id: id_attr.to_string(),
                        x,
                        y,
                        width,
                        height,
                        label,
                    });
                }
            }
            "path" => {
                let Some(id_attr) = node.attribute("id") else {
                    continue;
                };
                if !id_attr.starts_with("L_") {
                    continue;
                }
                let Some(d) = node.attribute("d") else {
                    continue;
                };
                let (start, end) = parse_edge_id(id_attr);
                let points = sample_path_d(d, samples_per_cubic)?;

                edges.push(SvgEdge {
                    id: id_attr.to_string(),
                    start,
                    end,
                    points,
                });
            }
            _ => {}
        }
    }

    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| a.id.cmp(&b.id));
    clusters.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(SvgGeometry {
        nodes,
        edges,
        clusters,
    })
}

pub fn extract_geometry_from_layout_result(layout: &crate::layout::LayoutResult) -> SvgGeometry {
    let mut nodes: Vec<SvgNode> = layout
        .nodes
        .values()
        .map(|n| SvgNode {
            id: n.id.clone(),
            center_x: n.x,
            center_y: n.y,
            width: n.width,
            height: n.height,
        })
        .collect();

    let mut edges: Vec<SvgEdge> = layout
        .edges
        .iter()
        .enumerate()
        .map(|(idx, e)| SvgEdge {
            id: format!("edge_{idx}_{from}_{to}", from = e.from, to = e.to),
            start: Some(e.from.clone()),
            end: Some(e.to.clone()),
            points: e.points.clone(),
        })
        .collect();

    let mut clusters: Vec<SvgCluster> = layout
        .subgraphs
        .iter()
        .map(|sg| SvgCluster {
            id: sg.id.clone(),
            x: sg.x,
            y: sg.y,
            width: sg.width,
            height: sg.height,
            label: sg.title.clone(),
        })
        .collect();

    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| a.id.cmp(&b.id));
    clusters.sort_by(|a, b| a.id.cmp(&b.id));

    SvgGeometry {
        nodes,
        edges,
        clusters,
    }
}

fn node_attr_contains(node: &roxmltree::Node<'_, '_>, attr: &str, needle: &str) -> bool {
    node.attribute(attr)
        .is_some_and(|v| v.split_whitespace().any(|c| c == needle))
}

fn extract_cluster_label(cluster_group: &roxmltree::Node<'_, '_>) -> Option<String> {
    let label_group = cluster_group
        .children()
        .find(|c| c.is_element() && node_attr_contains(c, "class", "cluster-label"))?;

    let mut parts = Vec::new();
    for tspan in label_group
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "tspan")
    {
        let Some(text) = tspan.text() else {
            continue;
        };
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        parts.push(text.to_string());
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn normalize_flowchart_node_id(svg_node_id: &str) -> String {
    let without_prefix = svg_node_id
        .strip_prefix("flowchart-")
        .unwrap_or(svg_node_id);
    let Some((head, tail)) = without_prefix.rsplit_once('-') else {
        return without_prefix.to_string();
    };
    if tail.chars().all(|c| c.is_ascii_digit()) {
        head.to_string()
    } else {
        without_prefix.to_string()
    }
}

fn parse_edge_id(id: &str) -> (Option<String>, Option<String>) {
    let Some(rest) = id.strip_prefix("L_") else {
        return (None, None);
    };
    let parts: Vec<&str> = rest.split('_').collect();
    if parts.len() < 3 {
        return (None, None);
    }
    let end = parts[parts.len() - 2].to_string();
    let start = parts[..parts.len() - 2].join("_");
    (Some(start), Some(end))
}

fn parse_translate(transform: &str) -> Option<(f64, f64)> {
    let t = transform.trim();
    let inner = t.strip_prefix("translate(")?.strip_suffix(')')?;
    let parts: Vec<&str> = inner
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() != 2 {
        return None;
    }
    let x: f64 = parts[0].parse().ok()?;
    let y: f64 = parts[1].parse().ok()?;
    Some((x, y))
}

fn bbox_from_node_group(node_group: &roxmltree::Node<'_, '_>) -> Option<(f64, f64)> {
    for child in node_group.descendants().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "rect" => {
                let w: f64 = child.attribute("width")?.parse().ok()?;
                let h: f64 = child.attribute("height")?.parse().ok()?;
                return Some((w, h));
            }
            "circle" => {
                let r: f64 = child.attribute("r")?.parse().ok()?;
                return Some((2.0 * r, 2.0 * r));
            }
            "ellipse" => {
                let rx: f64 = child.attribute("rx")?.parse().ok()?;
                let ry: f64 = child.attribute("ry")?.parse().ok()?;
                return Some((2.0 * rx, 2.0 * ry));
            }
            "polygon" => {
                let points = child.attribute("points")?;
                let bbox = bbox_from_points_attr(points)?;
                return Some((bbox.2 - bbox.0, bbox.3 - bbox.1));
            }
            _ => {}
        }
    }

    None
}

fn bbox_from_points_attr(points: &str) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for pair in points.split_whitespace() {
        let (x_str, y_str) = pair.split_once(',')?;
        let x: f64 = x_str.parse().ok()?;
        let y: f64 = y_str.parse().ok()?;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if min_x.is_infinite() {
        None
    } else {
        Some((min_x, min_y, max_x, max_y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PathCmd {
    MoveTo,
    LineTo,
    CubicTo,
}

fn sample_path_d(d: &str, samples_per_cubic: usize) -> Result<Vec<(f64, f64)>> {
    let cmds = tokenize_path_d(d)?;
    if cmds.is_empty() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    let mut current = (0.0_f64, 0.0_f64);

    for cmd in cmds {
        match cmd {
            ParsedCmd::MoveTo(p) => {
                current = p;
                out.push(p);
            }
            ParsedCmd::LineTo(p) => {
                out.push(p);
                current = p;
            }
            ParsedCmd::CubicTo { c1, c2, p } => {
                let n = samples_per_cubic.max(2);
                for i in 1..n {
                    let t = i as f64 / (n as f64 - 1.0);
                    out.push(cubic_bezier(current, c1, c2, p, t));
                }
                current = p;
            }
        }
    }

    Ok(out)
}

fn cubic_bezier(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    t: f64,
) -> (f64, f64) {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    let x = uuu * p0.0 + 3.0 * uu * t * p1.0 + 3.0 * u * tt * p2.0 + ttt * p3.0;
    let y = uuu * p0.1 + 3.0 * uu * t * p1.1 + 3.0 * u * tt * p2.1 + ttt * p3.1;
    (x, y)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Cmd(PathCmd),
    Number(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParsedCmd {
    MoveTo((f64, f64)),
    LineTo((f64, f64)),
    CubicTo {
        c1: (f64, f64),
        c2: (f64, f64),
        p: (f64, f64),
    },
}

fn tokenize_path_d(d: &str) -> Result<Vec<ParsedCmd>> {
    let tokens = lex_path_d(d)?;
    let mut out = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let Token::Cmd(cmd) = tokens[i] else {
            return Err(anyhow!("Expected command token"));
        };
        i += 1;

        match cmd {
            PathCmd::MoveTo => {
                let (x, y, next) = read_pair(&tokens, i)?;
                i = next;
                out.push(ParsedCmd::MoveTo((x, y)));
            }
            PathCmd::LineTo => {
                let (x, y, next) = read_pair(&tokens, i)?;
                i = next;
                out.push(ParsedCmd::LineTo((x, y)));
            }
            PathCmd::CubicTo => {
                let (x1, y1, next1) = read_pair(&tokens, i)?;
                let (x2, y2, next2) = read_pair(&tokens, next1)?;
                let (x, y, next3) = read_pair(&tokens, next2)?;
                i = next3;
                out.push(ParsedCmd::CubicTo {
                    c1: (x1, y1),
                    c2: (x2, y2),
                    p: (x, y),
                });
            }
        }
    }

    Ok(out)
}

fn read_pair(tokens: &[Token], start: usize) -> Result<(f64, f64, usize)> {
    let Token::Number(x) = tokens
        .get(start)
        .copied()
        .ok_or_else(|| anyhow!("Missing x"))?
    else {
        return Err(anyhow!("Expected x number"));
    };
    let Token::Number(y) = tokens
        .get(start + 1)
        .copied()
        .ok_or_else(|| anyhow!("Missing y"))?
    else {
        return Err(anyhow!("Expected y number"));
    };
    Ok((x, y, start + 2))
}

fn lex_path_d(d: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let bytes = d.as_bytes();

    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            'M' => {
                tokens.push(Token::Cmd(PathCmd::MoveTo));
                i += 1;
            }
            'L' => {
                tokens.push(Token::Cmd(PathCmd::LineTo));
                i += 1;
            }
            'C' => {
                tokens.push(Token::Cmd(PathCmd::CubicTo));
                i += 1;
            }
            ',' | ' ' | '\n' | '\t' | '\r' => {
                i += 1;
            }
            '-' | '.' | '0'..='9' => {
                let (num, next) = read_number(d, i)?;
                tokens.push(Token::Number(num));
                i = next;
            }
            _ => {
                return Err(anyhow!("Unsupported path char: {c}"));
            }
        }
    }

    Ok(tokens)
}

fn read_number(s: &str, start: usize) -> Result<(f64, usize)> {
    let bytes = s.as_bytes();
    let mut end = start;

    while end < bytes.len() {
        let c = bytes[end] as char;
        if c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+' {
            end += 1;
            continue;
        }
        break;
    }

    let num: f64 = s[start..end].parse()?;
    Ok((num, end))
}

#[cfg(test)]
#[path = "reference_svg_tests.rs"]
mod tests;
