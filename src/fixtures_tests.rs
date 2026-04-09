use std::path::PathBuf;

#[test]
fn all_mermaid_fixtures_have_reference_svg() {
    let samples_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("samples");

    let mut missing: Vec<String> = Vec::new();

    let Ok(type_entries) = std::fs::read_dir(&samples_root) else {
        panic!("samples root should be readable: {samples_root:?}");
    };

    for entry in type_entries {
        let Ok(entry) = entry else {
            continue;
        };
        let diagram_type_path = entry.path();
        if !diagram_type_path.is_dir() {
            continue;
        }

        let diagram_type = match diagram_type_path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue,
        };

        let mermaid_dir = diagram_type_path.join("mermaid");
        if !mermaid_dir.is_dir() {
            continue;
        }

        let reference_dir = diagram_type_path.join("reference");

        let Ok(mermaid_entries) = std::fs::read_dir(&mermaid_dir) else {
            continue;
        };

        for mmd_entry in mermaid_entries {
            let Ok(mmd_entry) = mmd_entry else {
                continue;
            };

            let path = mmd_entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("mmd") {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            let reference_png_path = reference_dir.join(format!("{stem}.png"));
            if !reference_png_path.is_file() {
                missing.push(format!("{diagram_type}/{stem}"));
            }
        }
    }

    if !missing.is_empty() {
        missing.sort();
        panic!(
            "Missing reference PNGs for fixtures:\n{}",
            missing.join("\n")
        );
    }
}
