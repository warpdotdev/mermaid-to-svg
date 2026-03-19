#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MermaidTheme {
    pub background: String,
    pub node_fill: String,
    pub node_stroke: String,
    pub text_color: String,
    pub edge_color: String,
    pub subgraph_fill: String,
    pub subgraph_stroke: String,
}

impl Default for MermaidTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl MermaidTheme {
    pub fn light() -> Self {
        Self {
            background: "#ffffff".to_string(),
            node_fill: "#ECECFF".to_string(),
            node_stroke: "#9370DB".to_string(),
            text_color: "#333333".to_string(),
            edge_color: "#333333".to_string(),
            subgraph_fill: "#ffffde".to_string(),
            subgraph_stroke: "#aaaa33".to_string(),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: "#1e1e1e".to_string(),
            node_fill: "#2d2d2d".to_string(),
            node_stroke: "#888888".to_string(),
            text_color: "#ffffff".to_string(),
            edge_color: "#888888".to_string(),
            subgraph_fill: "#3a3a20".to_string(),
            subgraph_stroke: "#888844".to_string(),
        }
    }
}
