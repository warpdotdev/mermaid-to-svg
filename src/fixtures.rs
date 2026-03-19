#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixtureCase {
    pub diagram_type: String,
    pub name: String,
}

impl FixtureCase {
    pub fn mermaid_asset_path(&self) -> String {
        format!("{}/mermaid/{}.mmd", self.diagram_type, self.name)
    }

    pub fn reference_svg_asset_path(&self) -> String {
        format!("{}/reference/{}.png", self.diagram_type, self.name)
    }
}

pub fn parse_mermaid_asset_path(path: &str) -> Option<FixtureCase> {
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    match parts.as_slice() {
        [diagram_type, "mermaid", filename] => {
            let name = filename.strip_suffix(".mmd")?;
            Some(FixtureCase {
                diagram_type: (*diagram_type).to_string(),
                name: name.to_string(),
            })
        }
        ["mermaid", filename] => {
            let name = filename.strip_suffix(".mmd")?;
            Some(FixtureCase {
                diagram_type: "flowchart".to_string(),
                name: name.to_string(),
            })
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "fixtures_tests.rs"]
mod tests;
