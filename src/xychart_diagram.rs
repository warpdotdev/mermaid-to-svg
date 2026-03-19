use crate::error::MermaidError;
use crate::theme::MermaidTheme;

const CHART_WIDTH: f64 = 700.0;
const CHART_HEIGHT: f64 = 500.0;

const CHART_TITLE_FONT_SIZE: f64 = 20.0;
const CHART_TITLE_PADDING: f64 = 10.0;

const AXIS_LABEL_FONT_SIZE: f64 = 14.0;
const AXIS_LABEL_PADDING: f64 = 5.0;

const AXIS_TICK_LENGTH: f64 = 5.0;
const AXIS_TICK_WIDTH: f64 = 2.0;

const AXIS_LINE_WIDTH: f64 = 2.0;

const DEFAULT_TICK_COUNT: usize = 10;

const DEFAULT_AXIS_COLOR: &str = "#131300";

pub fn render_xychart_diagram_to_svg(
    mermaid_source: &str,
    theme: &MermaidTheme,
) -> Result<String, MermaidError> {
    let chart = parse_xychart(mermaid_source)?;

    let x_ticks = d3_ticks(chart.x_axis_min, chart.x_axis_max, DEFAULT_TICK_COUNT);
    let y_ticks = d3_ticks(chart.y_axis_min, chart.y_axis_max, DEFAULT_TICK_COUNT);

    let x_tick_labels: Vec<String> = x_ticks.iter().map(|v| format_tick(*v)).collect();
    let y_tick_labels: Vec<String> = y_ticks.iter().map(|v| format_tick(*v)).collect();

    let label_text_height = approx_text_height(AXIS_LABEL_FONT_SIZE);
    let x_label_max_width = x_tick_labels
        .iter()
        .map(|s| approx_text_width(s, AXIS_LABEL_FONT_SIZE))
        .fold(0.0, f64::max);
    let y_label_max_width = y_tick_labels
        .iter()
        .map(|s| approx_text_width(s, AXIS_LABEL_FONT_SIZE))
        .fold(0.0, f64::max);

    let title_height = if chart.title.is_empty() {
        0.0
    } else {
        approx_text_height(CHART_TITLE_FONT_SIZE) + 2.0 * CHART_TITLE_PADDING
    };

    let x_axis_height =
        AXIS_LINE_WIDTH + AXIS_TICK_LENGTH + (label_text_height + 2.0 * AXIS_LABEL_PADDING);

    let y_axis_width =
        AXIS_LINE_WIDTH + AXIS_TICK_LENGTH + (y_label_max_width + 2.0 * AXIS_LABEL_PADDING);

    let plot_x = y_axis_width;
    let plot_y = title_height;
    let plot_w = CHART_WIDTH - plot_x;
    let plot_h = CHART_HEIGHT - plot_y - x_axis_height;

    let x_outer_padding = (x_label_max_width / 2.0).min(0.2 * plot_w);
    let y_outer_padding = (label_text_height / 2.0).min(0.2 * plot_h);

    let x0 = plot_x + x_outer_padding;
    let x1 = plot_x + plot_w - x_outer_padding;
    let y_top = plot_y + y_outer_padding;
    let y_bottom = plot_y + plot_h - y_outer_padding;

    let mut svg = String::new();

    svg.push_str(&format!(
        "<svg aria-roledescription=\"xychart\" role=\"graphics-document document\" viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" style=\"max-width: {CHART_WIDTH}px; background-color: {};\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"100%\" id=\"my-svg\">",
        theme.background
    ));

    svg.push_str("<g/><g class=\"main\">");
    svg.push_str(&format!(
        "<rect fill=\"{}\" class=\"background\" height=\"{CHART_HEIGHT}\" width=\"{CHART_WIDTH}\"/>",
        theme.background
    ));

    if !chart.title.is_empty() {
        let title_y = title_height / 2.0;
        let title_x = CHART_WIDTH / 2.0;
        svg.push_str("<g class=\"chart-title\">");
        svg.push_str(&format!(
            "<text transform=\"translate({title_x}, {title_y}) rotate(0)\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-size=\"{CHART_TITLE_FONT_SIZE}\" fill=\"{DEFAULT_AXIS_COLOR}\" y=\"0\" x=\"0\">{}</text>",
            escape_xml(&chart.title)
        ));
        svg.push_str("</g>");
    }

    svg.push_str("<g class=\"plot\">");
    if !chart.line_data.is_empty() {
        let points = compute_line_points(&chart, x0, x1, y_top, y_bottom);
        let d = points_to_path_d(&points);
        let stroke = theme.node_fill.as_str();
        svg.push_str("<g class=\"line-plot-0\">");
        svg.push_str(&format!(
            "<path stroke-width=\"2\" stroke=\"{stroke}\" fill=\"none\" d=\"{d}\"/>",
        ));
        svg.push_str("</g>");
    }
    svg.push_str("</g>");

    let bottom_axis_y = plot_y + plot_h;
    svg.push_str("<g class=\"bottom-axis\">");
    svg.push_str("<g class=\"axis-line\">");
    svg.push_str(&format!(
        "<path stroke-width=\"{AXIS_LINE_WIDTH}\" stroke=\"{DEFAULT_AXIS_COLOR}\" fill=\"none\" d=\"M {plot_x},{y} L {CHART_WIDTH},{y}\"/>",
        y = bottom_axis_y + AXIS_LINE_WIDTH / 2.0,
    ));
    svg.push_str("</g>");

    svg.push_str("<g class=\"label\">");
    for (tick_value, tick_label) in x_ticks.iter().zip(x_tick_labels.iter()) {
        let x = scale_linear(*tick_value, chart.x_axis_min, chart.x_axis_max, x0, x1);
        let y = bottom_axis_y + AXIS_LABEL_PADDING + AXIS_TICK_LENGTH + AXIS_LINE_WIDTH;
        svg.push_str(&format!(
            "<text transform=\"translate({x}, {y}) rotate(0)\" text-anchor=\"middle\" dominant-baseline=\"text-before-edge\" font-size=\"{AXIS_LABEL_FONT_SIZE}\" fill=\"{DEFAULT_AXIS_COLOR}\" y=\"0\" x=\"0\">{}</text>",
            escape_xml(tick_label)
        ));
    }
    svg.push_str("</g>");

    svg.push_str("<g class=\"ticks\">");
    let tick_y0 = bottom_axis_y + AXIS_LINE_WIDTH;
    let tick_y1 = tick_y0 + AXIS_TICK_LENGTH;
    for tick_value in &x_ticks {
        let x = scale_linear(*tick_value, chart.x_axis_min, chart.x_axis_max, x0, x1);
        svg.push_str(&format!(
            "<path stroke-width=\"{AXIS_TICK_WIDTH}\" stroke=\"{DEFAULT_AXIS_COLOR}\" fill=\"none\" d=\"M {x},{tick_y0} L {x},{tick_y1}\"/>",
        ));
    }
    svg.push_str("</g>");
    svg.push_str("</g>");

    svg.push_str("<g class=\"left-axis\">");
    svg.push_str("<g class=\"axisl-line\">");
    let axis_x = y_axis_width - AXIS_LINE_WIDTH / 2.0;
    svg.push_str(&format!(
        "<path stroke-width=\"{AXIS_LINE_WIDTH}\" stroke=\"{DEFAULT_AXIS_COLOR}\" fill=\"none\" d=\"M {axis_x},{plot_y} L {axis_x},{y1}\"/>",
        y1 = plot_y + plot_h,
    ));
    svg.push_str("</g>");

    svg.push_str("<g class=\"label\">");
    let label_x = y_axis_width - AXIS_LABEL_PADDING - AXIS_TICK_LENGTH - AXIS_LINE_WIDTH;
    for (tick_value, tick_label) in y_ticks.iter().zip(y_tick_labels.iter()) {
        let y = scale_linear(
            *tick_value,
            chart.y_axis_min,
            chart.y_axis_max,
            y_bottom,
            y_top,
        );
        svg.push_str(&format!(
            "<text transform=\"translate({label_x}, {y}) rotate(0)\" text-anchor=\"end\" dominant-baseline=\"middle\" font-size=\"{AXIS_LABEL_FONT_SIZE}\" fill=\"{DEFAULT_AXIS_COLOR}\" y=\"0\" x=\"0\">{}</text>",
            escape_xml(tick_label)
        ));
    }
    svg.push_str("</g>");

    svg.push_str("<g class=\"ticks\">");
    let tick_x0 = y_axis_width - AXIS_LINE_WIDTH;
    let tick_x1 = tick_x0 - AXIS_TICK_LENGTH;
    for tick_value in &y_ticks {
        let y = scale_linear(
            *tick_value,
            chart.y_axis_min,
            chart.y_axis_max,
            y_bottom,
            y_top,
        );
        svg.push_str(&format!(
            "<path stroke-width=\"{AXIS_TICK_WIDTH}\" stroke=\"{DEFAULT_AXIS_COLOR}\" fill=\"none\" d=\"M {tick_x0},{y} L {tick_x1},{y}\"/>",
        ));
    }
    svg.push_str("</g>");
    svg.push_str("</g>");

    svg.push_str("</g><g class=\"mermaid-tmp-group\"/></svg>");

    Ok(svg)
}

#[derive(Debug, Clone)]
struct XyChart {
    title: String,
    x_axis_min: f64,
    x_axis_max: f64,
    y_axis_min: f64,
    y_axis_max: f64,
    line_data: Vec<f64>,
}

fn parse_xychart(input: &str) -> Result<XyChart, MermaidError> {
    let mut found_header = false;

    let mut title = String::new();
    let mut x_axis_min: Option<f64> = None;
    let mut x_axis_max: Option<f64> = None;
    let mut y_axis_min: Option<f64> = None;
    let mut y_axis_max: Option<f64> = None;
    let mut line_data: Vec<f64> = Vec::new();

    for (idx, raw) in input.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if !found_header {
            if line.split_whitespace().next() != Some("xychart-beta") {
                return Err(MermaidError::ParseError {
                    line: line_no,
                    message: "Expected 'xychart-beta' declaration".to_string(),
                });
            }
            found_header = true;
            continue;
        }

        if let Some(rest) = line.strip_prefix("title ") {
            title = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = line.strip_prefix("x-axis ") {
            let (min, max) = parse_axis_range(rest.trim(), line_no)?;
            x_axis_min = Some(min);
            x_axis_max = Some(max);
            continue;
        }

        if let Some(rest) = line.strip_prefix("y-axis ") {
            let (min, max) = parse_axis_range(rest.trim(), line_no)?;
            y_axis_min = Some(min);
            y_axis_max = Some(max);
            continue;
        }

        if let Some(rest) = line.strip_prefix("line") {
            let values = parse_bracketed_number_list(rest.trim(), line_no)?;
            line_data = values;
            continue;
        }
    }

    if !found_header {
        return Err(MermaidError::ParseError {
            line: 1,
            message: "Expected 'xychart-beta' declaration".to_string(),
        });
    }

    let (x_axis_min, x_axis_max) = (x_axis_min.unwrap_or(0.0), x_axis_max.unwrap_or(0.0));
    let (y_axis_min, y_axis_max) = (y_axis_min.unwrap_or(0.0), y_axis_max.unwrap_or(0.0));

    if line_data.is_empty() {
        return Err(MermaidError::ParseError {
            line: 1,
            message: "xychart requires at least one plot".to_string(),
        });
    }

    Ok(XyChart {
        title,
        x_axis_min,
        x_axis_max,
        y_axis_min,
        y_axis_max,
        line_data,
    })
}

fn parse_axis_range(s: &str, line: usize) -> Result<(f64, f64), MermaidError> {
    let Some((left, right)) = s.split_once("-->") else {
        return Err(MermaidError::ParseError {
            line,
            message: format!("Invalid axis range: {s}"),
        });
    };

    let left = left.trim();
    let right = right.trim();

    let (min_str, _) = left
        .rsplit_once(' ')
        .map(|(a, b)| (b.trim(), a.trim()))
        .unwrap_or((left, ""));

    let min: f64 = min_str.parse().map_err(|_| MermaidError::ParseError {
        line,
        message: format!("Invalid axis min: {min_str}"),
    })?;
    let max: f64 = right.parse().map_err(|_| MermaidError::ParseError {
        line,
        message: format!("Invalid axis max: {right}"),
    })?;

    Ok((min, max))
}

fn parse_bracketed_number_list(s: &str, line: usize) -> Result<Vec<f64>, MermaidError> {
    let Some(start) = s.find('[') else {
        return Err(MermaidError::ParseError {
            line,
            message: format!("Invalid plot data: {s}"),
        });
    };
    let Some(end) = s.rfind(']') else {
        return Err(MermaidError::ParseError {
            line,
            message: format!("Invalid plot data: {s}"),
        });
    };

    let inner = &s[start + 1..end];
    let mut out = Vec::new();

    for part in inner.split(',') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        let v: f64 = p.parse().map_err(|_| MermaidError::ParseError {
            line,
            message: format!("Invalid plot value: {p}"),
        })?;
        out.push(v);
    }

    Ok(out)
}

fn compute_line_points(
    chart: &XyChart,
    x0: f64,
    x1: f64,
    y_top: f64,
    y_bottom: f64,
) -> Vec<(f64, f64)> {
    let n = chart.line_data.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        let x = scale_linear(chart.x_axis_min, chart.x_axis_min, chart.x_axis_max, x0, x1);
        let y = scale_linear(
            chart.line_data[0],
            chart.y_axis_min,
            chart.y_axis_max,
            y_bottom,
            y_top,
        );
        return vec![(x, y)];
    }

    let step = (chart.x_axis_max - chart.x_axis_min) / (n.saturating_sub(1) as f64);
    let mut points = Vec::with_capacity(n);

    for (i, y_val) in chart.line_data.iter().copied().enumerate() {
        let x_val = chart.x_axis_min + step * (i as f64);
        let x = scale_linear(x_val, chart.x_axis_min, chart.x_axis_max, x0, x1);
        let y = scale_linear(y_val, chart.y_axis_min, chart.y_axis_max, y_bottom, y_top);
        points.push((x, y));
    }

    points
}

fn points_to_path_d(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut d = String::new();
    if let Some((x, y)) = points.first().copied() {
        d.push_str(&format!("M{x},{y}"));
    }

    for &(x, y) in points.iter().skip(1) {
        d.push_str(&format!("L{x},{y}"));
    }

    d
}

fn scale_linear(
    value: f64,
    domain_min: f64,
    domain_max: f64,
    range_min: f64,
    range_max: f64,
) -> f64 {
    if (domain_max - domain_min).abs() < f64::EPSILON {
        return range_min;
    }

    let t = (value - domain_min) / (domain_max - domain_min);
    range_min + t * (range_max - range_min)
}

fn d3_ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    if count == 0 {
        return Vec::new();
    }
    if !start.is_finite() || !stop.is_finite() {
        return Vec::new();
    }
    if start == stop {
        return vec![start];
    }

    let reverse = stop < start;
    let (a, b) = if reverse {
        (stop, start)
    } else {
        (start, stop)
    };

    let step = tick_step(a, b, count as f64);
    if !step.is_finite() || step == 0.0 {
        return Vec::new();
    }

    let start0 = (a / step).ceil();
    let stop0 = (b / step).floor();

    let n = (stop0 - start0 + 1.0).max(0.0) as i64;
    let mut ticks = Vec::with_capacity(n as usize);

    for i in 0..n {
        ticks.push((start0 + i as f64) * step);
    }

    if reverse {
        ticks.reverse();
    }

    ticks
}

fn tick_step(start: f64, stop: f64, count: f64) -> f64 {
    let step0 = (stop - start).abs() / count.max(1.0);
    let step1 = 10.0_f64.powf(step0.log10().floor());
    let error = step0 / step1;

    let e10 = 50.0_f64.sqrt();
    let e5 = 10.0_f64.sqrt();
    let e2 = 2.0_f64.sqrt();

    let step = if error >= e10 {
        step1 * 10.0
    } else if error >= e5 {
        step1 * 5.0
    } else if error >= e2 {
        step1 * 2.0
    } else {
        step1
    };

    if stop < start {
        -step
    } else {
        step
    }
}

fn format_tick(value: f64) -> String {
    let rounded = value.round();
    if (value - rounded).abs() < 1e-9 {
        return format!("{:.0}", rounded);
    }

    let s = format!("{value:.6}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

fn approx_text_width(text: &str, font_size: f64) -> f64 {
    let n = text.chars().count() as f64;
    n * font_size * 0.525
}

fn approx_text_height(font_size: f64) -> f64 {
    (font_size * 1.15).round()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
