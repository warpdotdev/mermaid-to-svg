mod cluster_adjust;
mod dagre_layout_port;
mod flow_data;
mod flow_db;
mod flow_parser;

use crate::error::MermaidError;
use crate::theme::MermaidTheme;

pub fn render_mermaid_to_svg_ported(
    mermaid_source: &str,
    theme: &MermaidTheme,
) -> Result<String, MermaidError> {
    let graph = flow_parser::parse_flowchart(mermaid_source)?;
    let layout_result = dagre_layout_port::compute_layout_ported(&graph);
    Ok(crate::svg_renderer::render(&layout_result, theme))
}

pub fn is_enabled() -> bool {
    std::env::var_os("MERMAID_TO_SVG_USE_PORT").is_some()
}

#[allow(dead_code)]
pub(crate) fn compute_layout_ported(
    flowchart: &crate::ast::FlowchartGraph,
) -> crate::layout::LayoutResult {
    dagre_layout_port::compute_layout_ported(flowchart)
}
