use crate::error::MermaidError;
use crate::theme::MermaidTheme;

use std::collections::BTreeMap;

const SEQUENCE_EVENT_ROW_HEIGHT: f64 = 44.0;
const SEQUENCE_FRAGMENT_HEADER_HEIGHT: f64 = 28.0;
const SEQUENCE_FRAGMENT_FOOTER_HEIGHT: f64 = 12.0;
const SEQUENCE_FRAGMENT_INSET_X: f64 = 18.0;
const SEQUENCE_FRAGMENT_MARGIN_X: f64 = 20.0;
const SEQUENCE_FRAGMENT_TAB_PADDING_X: f64 = 10.0;
const SEQUENCE_FRAGMENT_STROKE: &str = "#d7c8f8";
const SEQUENCE_FRAGMENT_TEXT: &str = "#4c3f6f";

pub fn render_sequence_diagram_to_svg(
    mermaid_source: &str,
    theme: &MermaidTheme,
) -> Result<String, MermaidError> {
    let diagram = parse_sequence_diagram(mermaid_source)?;

    let header_y = 16.0_f64;
    let box_margin = 10.0_f64;
    let edge_pad = 10.0_f64;

    let box_w = diagram
        .participants
        .iter()
        .map(|p| estimate_label_box_width(&p.label))
        .fold(100.0_f64, f64::max);
    let spacing = box_w + 30.0;
    let left_margin = box_w / 2.0 + edge_pad;

    let n = diagram.participants.len().max(1);
    let width =
        (left_margin + spacing * (n.saturating_sub(1) as f64) + box_w / 2.0 + edge_pad).max(360.0);
    let max_label_lines = diagram
        .participants
        .iter()
        .map(|p| p.label.split("<br/>").count())
        .max()
        .unwrap_or(1);
    let header_h = if max_label_lines > 1 {
        44.0_f64
    } else {
        32.0_f64
    };

    let events_top = header_y + header_h + 28.0;
    let (event_y_positions, fragment_layouts, content_bottom) =
        layout_sequence_events(&diagram.events, events_top);
    let footer_h = header_h;
    let footer_y = content_bottom + box_margin;
    let height = (footer_y + footer_h + header_y).max(220.0);

    let mut x_for: BTreeMap<&str, f64> = BTreeMap::new();
    for (idx, participant) in diagram.participants.iter().enumerate() {
        x_for.insert(participant.id.as_str(), left_margin + idx as f64 * spacing);
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

    for participant in &diagram.participants {
        let x = x_for
            .get(participant.id.as_str())
            .copied()
            .unwrap_or(left_margin);
        let box_x = x - box_w / 2.0;

        svg.push_str(&format!(
            "<rect x=\"{box_x:.3}\" y=\"{header_y:.3}\" width=\"{box_w:.3}\" height=\"{header_h:.3}\" rx=\"3\" ry=\"3\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
            theme.node_fill, theme.node_stroke
        ));
        svg.push_str(&render_participant_label(
            x,
            header_y + header_h / 2.0,
            &participant.label,
            &theme.text_color,
        ));

        let y0 = header_y + header_h;
        let y1 = footer_y;
        svg.push_str(&format!(
            "<line x1=\"{x:.3}\" y1=\"{y0:.3}\" x2=\"{x:.3}\" y2=\"{y1:.3}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            theme.edge_color
        ));

        svg.push_str(&format!(
            "<rect x=\"{box_x:.3}\" y=\"{footer_y:.3}\" width=\"{box_w:.3}\" height=\"{footer_h:.3}\" rx=\"3\" ry=\"3\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
            theme.node_fill, theme.node_stroke
        ));
        svg.push_str(&render_participant_label(
            x,
            footer_y + footer_h / 2.0,
            &participant.label,
            &theme.text_color,
        ));
    }

    if let Some((min_participant_x, max_participant_x)) = participant_span(&diagram, &x_for) {
        for fragment in &fragment_layouts {
            render_fragment(
                &mut svg,
                fragment,
                min_participant_x,
                max_participant_x,
                theme,
            );
        }
    }

    for (idx, ev) in diagram.events.iter().enumerate() {
        let Some(y) = event_y_positions.get(idx).copied().flatten() else {
            continue;
        };

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

                if (x1 - x2).abs() < 1.0 {
                    let loop_w = 40.0_f64;
                    let loop_h = SEQUENCE_EVENT_ROW_HEIGHT * 0.55;
                    let xr = x1 + loop_w;
                    let ye = y + loop_h;
                    svg.push_str(&format!(
                        "<path d=\"M {x1:.3},{y:.3} L {xr:.3},{y:.3} L {xr:.3},{ye:.3} L {x1:.3},{ye:.3}\" stroke=\"{}\" stroke-width=\"1.5\" fill=\"none\"{dash} marker-end=\"url(#seq_arrow)\"/>",
                        theme.edge_color
                    ));
                    if !text.is_empty() {
                        svg.push_str(&format!(
                            "<text x=\"{tx:.3}\" y=\"{ty:.3}\" text-anchor=\"start\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{}\">{}</text>",
                            theme.text_color,
                            escape_xml(text),
                            tx = x1 + 4.0,
                            ty = y - 6.0
                        ));
                    }
                } else {
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
            SequenceEvent::FragmentStart { .. }
            | SequenceEvent::FragmentElse { .. }
            | SequenceEvent::FragmentEnd => {}
        }
    }

    svg.push_str("</svg>");
    Ok(svg)
}

#[derive(Debug, Clone)]
struct SequenceParticipant {
    id: String,
    label: String,
    aliases: Vec<String>,
}

#[derive(Debug, Clone)]
struct SequenceDiagram {
    participants: Vec<SequenceParticipant>,
    events: Vec<SequenceEvent>,
}

impl SequenceParticipant {
    fn matches(&self, reference: &str) -> bool {
        self.id == reference
            || self.label == reference
            || self.aliases.iter().any(|alias| alias == reference)
    }
}

#[derive(Debug, Clone, Copy)]
enum SequenceFragmentKind {
    Alt,
    Loop,
    Opt,
}

impl SequenceFragmentKind {
    fn keyword(self) -> &'static str {
        match self {
            Self::Alt => "alt",
            Self::Loop => "loop",
            Self::Opt => "opt",
        }
    }
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
    FragmentStart {
        kind: SequenceFragmentKind,
        label: String,
    },
    FragmentElse {
        label: String,
    },
    FragmentEnd,
}

#[derive(Debug, Clone)]
struct SequenceFragmentLayout {
    kind: SequenceFragmentKind,
    label: String,
    depth: usize,
    start_y: f64,
    end_y: f64,
    else_markers: Vec<SequenceElseMarker>,
}

#[derive(Debug, Clone)]
struct SequenceElseMarker {
    separator_y: f64,
    label: String,
}

struct OpenSequenceFragment {
    kind: SequenceFragmentKind,
    label: String,
    depth: usize,
    start_y: f64,
    else_markers: Vec<SequenceElseMarker>,
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

    let mut participants: Vec<SequenceParticipant> = Vec::new();
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
            let participant = parse_participant_declaration(rest, line_no)?;
            register_participant(&mut participants, participant);
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
                Some((a, b)) => (
                    resolve_participant_ref(&mut participants, a.trim()),
                    resolve_participant_ref(&mut participants, b.trim()),
                ),
                None => {
                    let participant = resolve_participant_ref(&mut participants, who);
                    (participant.clone(), participant)
                }
            };

            events.push(SequenceEvent::NoteOver {
                from,
                to,
                text: text.to_string(),
            });
            continue;
        }

        if let Some(msg) = parse_message_line(line) {
            let from = resolve_participant_ref(&mut participants, &msg.from);
            let to = resolve_participant_ref(&mut participants, &msg.to);

            events.push(SequenceEvent::Message {
                from,
                to,
                text: msg.text,
                dashed: msg.dashed,
            });
            continue;
        }

        if let Some(fragment) = parse_fragment_line(line) {
            events.push(fragment);
            continue;
        }

        return Err(MermaidError::ParseError {
            line: line_no,
            message: format!("Unrecognized sequenceDiagram line: {line}"),
        });
    }

    if participants.is_empty() {
        participants.push(SequenceParticipant {
            id: "Participant".to_string(),
            label: "Participant".to_string(),
            aliases: Vec::new(),
        });
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

fn parse_fragment_line(line: &str) -> Option<SequenceEvent> {
    let trimmed = line.trim();

    if let Some(label) = trimmed.strip_prefix("alt") {
        return Some(SequenceEvent::FragmentStart {
            kind: SequenceFragmentKind::Alt,
            label: label.trim().to_string(),
        });
    }

    if let Some(label) = trimmed.strip_prefix("loop") {
        return Some(SequenceEvent::FragmentStart {
            kind: SequenceFragmentKind::Loop,
            label: label.trim().to_string(),
        });
    }

    if let Some(label) = trimmed.strip_prefix("opt") {
        return Some(SequenceEvent::FragmentStart {
            kind: SequenceFragmentKind::Opt,
            label: label.trim().to_string(),
        });
    }

    if let Some(label) = trimmed.strip_prefix("else") {
        return Some(SequenceEvent::FragmentElse {
            label: label.trim().to_string(),
        });
    }

    if trimmed == "end" {
        return Some(SequenceEvent::FragmentEnd);
    }

    None
}

fn parse_participant_declaration(
    raw: &str,
    line_no: usize,
) -> Result<SequenceParticipant, MermaidError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(MermaidError::ParseError {
            line: line_no,
            message: "Expected participant name".to_string(),
        });
    }

    let (id, label, aliases) = match raw.split_once(" as ") {
        Some((lhs, rhs)) => {
            let lhs = normalize_participant_token(lhs);
            let rhs = normalize_participant_token(rhs);
            if lhs.is_empty() || rhs.is_empty() {
                return Err(MermaidError::ParseError {
                    line: line_no,
                    message: "Expected participant name".to_string(),
                });
            }
            let (id, label) = choose_participant_id_and_label(&lhs, &rhs);
            let mut aliases = Vec::new();
            push_unique_alias(&mut aliases, lhs);
            push_unique_alias(&mut aliases, rhs);
            (id, label, aliases)
        }
        None => {
            let name = normalize_participant_token(raw);
            if name.is_empty() {
                return Err(MermaidError::ParseError {
                    line: line_no,
                    message: "Expected participant name".to_string(),
                });
            }
            (name.clone(), name, Vec::new())
        }
    };

    Ok(SequenceParticipant { id, label, aliases })
}

fn choose_participant_id_and_label(lhs: &str, rhs: &str) -> (String, String) {
    let lhs_has_whitespace = lhs.chars().any(char::is_whitespace);
    let rhs_has_whitespace = rhs.chars().any(char::is_whitespace);

    match (lhs_has_whitespace, rhs_has_whitespace) {
        (true, false) => (rhs.to_string(), lhs.to_string()),
        (false, true) => (lhs.to_string(), rhs.to_string()),
        _ if lhs.len() <= rhs.len() => (lhs.to_string(), rhs.to_string()),
        _ => (rhs.to_string(), lhs.to_string()),
    }
}

fn normalize_participant_token(token: &str) -> String {
    token
        .trim()
        .trim_matches(|ch| matches!(ch, '"' | '\''))
        .trim()
        .to_string()
}

fn register_participant(list: &mut Vec<SequenceParticipant>, participant: SequenceParticipant) {
    let match_index = list.iter().position(|existing| {
        existing.matches(&participant.id)
            || existing.matches(&participant.label)
            || participant
                .aliases
                .iter()
                .any(|alias| existing.matches(alias))
    });

    match match_index {
        Some(index) => {
            let existing = &mut list[index];
            if existing.label == existing.id && participant.label != participant.id {
                existing.label = participant.label.clone();
            }
            push_unique_alias(&mut existing.aliases, participant.id.clone());
            push_unique_alias(&mut existing.aliases, participant.label.clone());
            for alias in participant.aliases {
                push_unique_alias(&mut existing.aliases, alias);
            }
        }
        None => list.push(participant),
    }
}

fn resolve_participant_ref(list: &mut Vec<SequenceParticipant>, reference: &str) -> String {
    let reference = normalize_participant_token(reference);

    if let Some(participant) = list
        .iter()
        .find(|participant| participant.matches(&reference))
    {
        return participant.id.clone();
    }

    let participant = SequenceParticipant {
        id: reference.clone(),
        label: reference,
        aliases: Vec::new(),
    };
    let id = participant.id.clone();
    list.push(participant);
    id
}

fn push_unique_alias(aliases: &mut Vec<String>, value: String) {
    if !value.is_empty() && !aliases.iter().any(|alias| alias == &value) {
        aliases.push(value);
    }
}

fn participant_span(diagram: &SequenceDiagram, x_for: &BTreeMap<&str, f64>) -> Option<(f64, f64)> {
    let mut xs = diagram
        .participants
        .iter()
        .filter_map(|participant| x_for.get(participant.id.as_str()).copied());
    let first = xs.next()?;
    let mut min_x = first;
    let mut max_x = first;
    for x in xs {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
    }
    Some((min_x, max_x))
}

fn layout_sequence_events(
    events: &[SequenceEvent],
    events_top: f64,
) -> (Vec<Option<f64>>, Vec<SequenceFragmentLayout>, f64) {
    let mut event_y_positions = vec![None; events.len()];
    let mut fragment_layouts = Vec::new();
    let mut open_fragments: Vec<OpenSequenceFragment> = Vec::new();
    let mut cursor_y = events_top;

    for (idx, event) in events.iter().enumerate() {
        match event {
            SequenceEvent::Message { .. } | SequenceEvent::NoteOver { .. } => {
                event_y_positions[idx] = Some(cursor_y + SEQUENCE_EVENT_ROW_HEIGHT / 2.0);
                cursor_y += SEQUENCE_EVENT_ROW_HEIGHT;
            }
            SequenceEvent::FragmentStart { kind, label } => {
                open_fragments.push(OpenSequenceFragment {
                    kind: *kind,
                    label: label.clone(),
                    depth: open_fragments.len(),
                    start_y: cursor_y,
                    else_markers: Vec::new(),
                });
                cursor_y += SEQUENCE_FRAGMENT_HEADER_HEIGHT;
            }
            SequenceEvent::FragmentElse { label } => {
                if let Some(fragment) = open_fragments.last_mut() {
                    fragment.else_markers.push(SequenceElseMarker {
                        separator_y: cursor_y,
                        label: label.clone(),
                    });
                }
                cursor_y += SEQUENCE_FRAGMENT_HEADER_HEIGHT;
            }
            SequenceEvent::FragmentEnd => {
                if let Some(fragment) = open_fragments.pop() {
                    fragment_layouts.push(SequenceFragmentLayout {
                        kind: fragment.kind,
                        label: fragment.label,
                        depth: fragment.depth,
                        start_y: fragment.start_y,
                        end_y: cursor_y + SEQUENCE_FRAGMENT_FOOTER_HEIGHT,
                        else_markers: fragment.else_markers,
                    });
                    cursor_y += SEQUENCE_FRAGMENT_FOOTER_HEIGHT;
                }
            }
        }
    }

    while let Some(fragment) = open_fragments.pop() {
        fragment_layouts.push(SequenceFragmentLayout {
            kind: fragment.kind,
            label: fragment.label,
            depth: fragment.depth,
            start_y: fragment.start_y,
            end_y: cursor_y + SEQUENCE_FRAGMENT_FOOTER_HEIGHT,
            else_markers: fragment.else_markers,
        });
        cursor_y += SEQUENCE_FRAGMENT_FOOTER_HEIGHT;
    }

    fragment_layouts.sort_by(|a, b| {
        a.depth
            .cmp(&b.depth)
            .then_with(|| a.start_y.total_cmp(&b.start_y))
    });

    (event_y_positions, fragment_layouts, cursor_y)
}

fn render_fragment(
    svg: &mut String,
    fragment: &SequenceFragmentLayout,
    min_participant_x: f64,
    max_participant_x: f64,
    theme: &MermaidTheme,
) {
    let inset = fragment.depth as f64 * SEQUENCE_FRAGMENT_INSET_X;
    let x = min_participant_x - SEQUENCE_FRAGMENT_MARGIN_X + inset;
    let width =
        (max_participant_x - min_participant_x) + SEQUENCE_FRAGMENT_MARGIN_X * 2.0 - inset * 2.0;
    let height = (fragment.end_y - fragment.start_y).max(SEQUENCE_FRAGMENT_HEADER_HEIGHT);
    let y = fragment.start_y;
    let tab_text = fragment.kind.keyword();
    let tab_text_width = tab_text.chars().count() as f64 * 6.5;
    let tab_width = tab_text_width + SEQUENCE_FRAGMENT_TAB_PADDING_X * 2.0 + 10.0;
    let tab_height = 18.0;
    let tab_x = x + 8.0;
    let tab_y = y + 4.0;
    let tab_body_width = (tab_width - 10.0).max(10.0);

    svg.push_str(&format!(
        "<rect x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\" fill=\"none\" stroke=\"{SEQUENCE_FRAGMENT_STROKE}\" stroke-width=\"1\" stroke-dasharray=\"3,3\"/>"
    ));

    svg.push_str(&format!(
        "<path d=\"M {tab_x:.3},{tab_y:.3} h {tab_body_width:.3} l 10,0 l 0,10 l -10,8 h -{tab_body_width:.3} z\" fill=\"{}\" stroke=\"{SEQUENCE_FRAGMENT_STROKE}\" stroke-width=\"1\"/>",
        theme.node_fill
    ));
    svg.push_str(&format!(
        "<text x=\"{text_x:.3}\" y=\"{text_y:.3}\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{SEQUENCE_FRAGMENT_TEXT}\">{}</text>",
        escape_xml(tab_text),
        text_x = tab_x + tab_body_width / 2.0,
        text_y = tab_y + tab_height / 2.0
    ));

    if !fragment.label.is_empty() {
        let label_x = (tab_x + tab_body_width + 18.0).min(x + width - 8.0);
        svg.push_str(&format!(
            "<text x=\"{text_x:.3}\" y=\"{text_y:.3}\" text-anchor=\"start\" dominant-baseline=\"central\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{}\">[{}]</text>",
            theme.text_color,
            escape_xml(&fragment.label),
            text_x = label_x,
            text_y = y + SEQUENCE_FRAGMENT_HEADER_HEIGHT / 2.0
        ));
    }

    for else_marker in &fragment.else_markers {
        svg.push_str(&format!(
            "<line x1=\"{x:.3}\" y1=\"{y:.3}\" x2=\"{x2:.3}\" y2=\"{y:.3}\" stroke=\"{SEQUENCE_FRAGMENT_STROKE}\" stroke-width=\"1\" stroke-dasharray=\"3,3\"/>",
            x = x,
            y = else_marker.separator_y,
            x2 = x + width
        ));
        if !else_marker.label.is_empty() {
            svg.push_str(&format!(
                "<text x=\"{text_x:.3}\" y=\"{text_y:.3}\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Trebuchet MS,Verdana,Arial,sans-serif\" font-size=\"11\" fill=\"{}\">[{}]</text>",
                theme.text_color,
                escape_xml(&else_marker.label),
                text_x = x + width / 2.0,
                text_y = else_marker.separator_y + SEQUENCE_FRAGMENT_HEADER_HEIGHT / 2.0
            ));
        }
    }
}

fn estimate_label_box_width(label: &str) -> f64 {
    let char_w = 7.2_f64;
    let padding = 16.0_f64;
    let text_w = label
        .split("<br/>")
        .map(|line| line.chars().count() as f64 * char_w)
        .fold(0.0_f64, f64::max);
    (text_w + padding).max(100.0)
}

fn render_participant_label(x: f64, cy: f64, label: &str, color: &str) -> String {
    let font = "Trebuchet MS,Verdana,Arial,sans-serif";
    let size = 12_f64;
    let lines: Vec<&str> = label.split("<br/>").collect();

    if lines.len() == 1 {
        return format!(
            "<text x=\"{x:.3}\" y=\"{cy:.3}\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"{font}\" font-size=\"{size}\" fill=\"{color}\">{}</text>",
            escape_xml(lines[0])
        );
    }

    let line_h = 15.0_f64;
    let total_h = line_h * lines.len() as f64;
    let y0 = cy - total_h / 2.0 + line_h / 2.0;

    let mut out = format!(
        "<text x=\"{x:.3}\" text-anchor=\"middle\" font-family=\"{font}\" font-size=\"{size}\" fill=\"{color}\">"
    );
    for (i, line_text) in lines.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!(
                "<tspan x=\"{x:.3}\" y=\"{y0:.3}\">{}</tspan>",
                escape_xml(line_text)
            ));
        } else {
            out.push_str(&format!(
                "<tspan x=\"{x:.3}\" dy=\"{line_h:.3}\">{}</tspan>",
                escape_xml(line_text)
            ));
        }
    }
    out.push_str("</text>");
    out
}

#[cfg(test)]
#[path = "sequence_diagram_tests.rs"]
mod tests;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
