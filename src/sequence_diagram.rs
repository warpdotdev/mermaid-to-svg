use crate::error::MermaidError;
use crate::theme::MermaidTheme;

use std::collections::BTreeMap;

pub fn render_sequence_diagram_to_svg(
    mermaid_source: &str,
    theme: &MermaidTheme,
) -> Result<String, MermaidError> {
    let diagram = parse_sequence_diagram(mermaid_source)?;

    let header_h = 32.0_f64;
    let header_y = 16.0_f64;
    let left_margin = 80.0_f64;
    let spacing = 150.0_f64;
    let box_w = 120.0_f64;
    let box_margin = 10.0_f64;

    let n = diagram.participants.len().max(1);
    let width = (left_margin * 2.0 + spacing * (n.saturating_sub(1) as f64)).max(360.0);

    let row_h = 44.0_f64;
    let events_top = header_y + header_h + 28.0;
    // Reserve space for bottom participant boxes (mirrorActors)
    let footer_h = header_h;
    let content_bottom = events_top + diagram.events.len() as f64 * row_h;
    let footer_y = content_bottom + box_margin;
    let height = (footer_y + footer_h + header_y).max(220.0);

    let mut x_for: BTreeMap<&str, f64> = BTreeMap::new();
    for (idx, p) in diagram.participants.iter().enumerate() {
        x_for.insert(p.as_str(), left_margin + idx as f64 * spacing);
    }

    let mut svg = String::new();

    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">"
    ));
    svg.push_str(&format!(
        "<rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"{}\"/>",
        theme.background
    ));

    svg.push_str(
        "<defs><marker id=\"seq_arrow\" markerWidth=\"8\" markerHeight=\"8\" refX=\"7\" refY=\"4\" orient=\"auto\"><path d=\"M0,0 L8,4 L0,8 Z\" /></marker></defs>",
    );

    // Draw top participant boxes, lifelines, and bottom participant boxes
    for p in &diagram.participants {
        let x = x_for.get(p.as_str()).copied().unwrap_or(left_margin);
        let box_x = x - box_w / 2.0;

        // Top participant box
        svg.push_str(&format!(
            "<rect x=\"{box_x:.3}\" y=\"{header_y:.3}\" width=\"{box_w:.3}\" height=\"{header_h:.3}\" rx=\"3\" ry=\"3\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
            theme.node_fill, theme.node_stroke
        ));
        svg.push_str(&format!(
            "<text x=\"{x:.3}\" y=\"{ty:.3}\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"12\" fill=\"{}\">{}</text>",
            theme.text_color,
            escape_xml(p),
            ty = header_y + header_h / 2.0
        ));

        // Lifeline: solid line from bottom of top box to top of bottom box
        // Mermaid.js uses solid lines (stroke: #999, stroke-width: 0.5px) for lifelines
        let y0 = header_y + header_h;
        let y1 = footer_y;
        svg.push_str(&format!(
            "<line x1=\"{x:.3}\" y1=\"{y0:.3}\" x2=\"{x:.3}\" y2=\"{y1:.3}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            theme.edge_color
        ));

        // Bottom participant box (mirrorActors)
        svg.push_str(&format!(
            "<rect x=\"{box_x:.3}\" y=\"{footer_y:.3}\" width=\"{box_w:.3}\" height=\"{footer_h:.3}\" rx=\"3\" ry=\"3\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
            theme.node_fill, theme.node_stroke
        ));
        svg.push_str(&format!(
            "<text x=\"{x:.3}\" y=\"{ty:.3}\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"12\" fill=\"{}\">{}</text>",
            theme.text_color,
            escape_xml(p),
            ty = footer_y + footer_h / 2.0
        ));
    }

    for (idx, ev) in diagram.events.iter().enumerate() {
        let y = events_top + idx as f64 * row_h;

        match ev {
            SequenceEvent::Message {
                from,
                to,
                text,
                dashed,
            } => {
                let x1 = x_for.get(from.as_str()).copied().unwrap_or(left_margin);
                let x2 = x_for.get(to.as_str()).copied().unwrap_or(left_margin);

                let dash = if *dashed {
                    " stroke-dasharray=\"5,4\""
                } else {
                    ""
                };
                svg.push_str(&format!(
                    "<line x1=\"{x1:.3}\" y1=\"{y:.3}\" x2=\"{x2:.3}\" y2=\"{y:.3}\" stroke=\"{}\" stroke-width=\"1.5\" marker-end=\"url(#seq_arrow)\"{dash}/>",
                    theme.edge_color
                ));

                if !text.is_empty() {
                    let mx = (x1 + x2) / 2.0;
                    svg.push_str(&format!(
                        "<text x=\"{mx:.3}\" y=\"{ty:.3}\" text-anchor=\"middle\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>",
                        theme.text_color,
                        escape_xml(text),
                        ty = y - 6.0
                    ));
                }
            }
            SequenceEvent::NoteOver { from, to, text } => {
                let x1 = x_for.get(from.as_str()).copied().unwrap_or(left_margin);
                let x2 = x_for.get(to.as_str()).copied().unwrap_or(left_margin);

                let (lx, rx) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                let pad = 50.0_f64;
                let note_x = (lx - pad).max(8.0);
                let note_w = (rx - lx + pad * 2.0).max(120.0);
                let note_h = 26.0_f64;

                svg.push_str(&format!(
                    "<rect x=\"{note_x:.3}\" y=\"{ny:.3}\" width=\"{note_w:.3}\" height=\"{note_h:.3}\" rx=\"4\" ry=\"4\" fill=\"#fff2b0\" stroke=\"{}\" stroke-width=\"1\"/>",
                    theme.edge_color,
                    ny = y - note_h / 2.0
                ));

                svg.push_str(&format!(
                    "<text x=\"{x:.3}\" y=\"{y:.3}\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>",
                    theme.text_color,
                    escape_xml(text),
                    x = note_x + note_w / 2.0
                ));
            }
        }
    }

    svg.push_str("</svg>");
    Ok(svg)
}

#[derive(Debug, Clone)]
struct SequenceDiagram {
    participants: Vec<String>,
    events: Vec<SequenceEvent>,
}

#[derive(Debug, Clone)]
enum SequenceEvent {
    Message {
        from: String,
        to: String,
        text: String,
        dashed: bool,
    },
    NoteOver {
        from: String,
        to: String,
        text: String,
    },
}

fn parse_sequence_diagram(input: &str) -> Result<SequenceDiagram, MermaidError> {
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0_usize;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with("%%") {
            i += 1;
            continue;
        }

        if line.split_whitespace().next() == Some("sequenceDiagram") {
            i += 1;
            break;
        }

        return Err(MermaidError::ParseError {
            line: i + 1,
            message: "Expected 'sequenceDiagram' declaration".to_string(),
        });
    }

    let mut participants: Vec<String> = Vec::new();
    let mut events: Vec<SequenceEvent> = Vec::new();

    while i < lines.len() {
        let raw = lines[i];
        let line = raw.trim();
        let line_no = i + 1;
        i += 1;

        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if let Some(rest) = line.strip_prefix("participant ") {
            let name = rest.trim();
            if name.is_empty() {
                return Err(MermaidError::ParseError {
                    line: line_no,
                    message: "Expected participant name".to_string(),
                });
            }
            if !participants.iter().any(|p| p == name) {
                participants.push(name.to_string());
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("Note over ") {
            let Some((who, text)) = rest.split_once(':') else {
                return Err(MermaidError::ParseError {
                    line: line_no,
                    message: format!("Invalid Note syntax: {line}"),
                });
            };

            let who = who.trim();
            let text = text.trim();
            let (from, to) = match who.split_once(',') {
                Some((a, b)) => (a.trim().to_string(), b.trim().to_string()),
                None => (who.to_string(), who.to_string()),
            };

            add_participant(&mut participants, &from);
            add_participant(&mut participants, &to);

            events.push(SequenceEvent::NoteOver {
                from,
                to,
                text: text.to_string(),
            });
            continue;
        }

        if let Some(msg) = parse_message_line(line) {
            let from = msg.from.clone();
            let to = msg.to.clone();
            add_participant(&mut participants, &from);
            add_participant(&mut participants, &to);

            events.push(SequenceEvent::Message {
                from,
                to,
                text: msg.text,
                dashed: msg.dashed,
            });
            continue;
        }

        if matches!(
            line.split_whitespace().next(),
            Some("alt") | Some("else") | Some("loop") | Some("end")
        ) {
            continue;
        }

        return Err(MermaidError::ParseError {
            line: line_no,
            message: format!("Unrecognized sequenceDiagram line: {line}"),
        });
    }

    if participants.is_empty() {
        participants.push("Participant".to_string());
    }

    Ok(SequenceDiagram {
        participants,
        events,
    })
}

struct ParsedMessage {
    from: String,
    to: String,
    text: String,
    dashed: bool,
}

fn parse_message_line(line: &str) -> Option<ParsedMessage> {
    let (head, text) = line
        .split_once(':')
        .map_or((line, ""), |(a, b)| (a, b.trim()));

    let head = head.trim();
    let (op, dashed) = if head.contains("-->>") {
        ("-->>", true)
    } else if head.contains("->>") {
        ("->>", false)
    } else {
        return None;
    };

    let (from_raw, to_raw) = head.split_once(op)?;
    let from = from_raw.trim().to_string();
    let mut to = to_raw.trim().to_string();
    to = to.trim_start_matches(['+', '-']).to_string();

    Some(ParsedMessage {
        from,
        to,
        text: text.to_string(),
        dashed,
    })
}

fn add_participant(list: &mut Vec<String>, name: &str) {
    if !list.iter().any(|p| p == name) {
        list.push(name.to_string());
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
