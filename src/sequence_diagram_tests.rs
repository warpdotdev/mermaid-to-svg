use super::*;

const KEVIN_DIAGRAM: &str = r#"sequenceDiagram
    participant W as File Watcher
    participant L as LocalRepoMetadataModel<br/>(remote server)
    participant N as Network Layer<br/>(future, out of scope)
    participant R as RemoteRepoMetadataModel<br/>(client)

    W->>L: BulkFilesystemWatcherEvent
    L->>L: compute_file_tree_mutations() [bg thread]
    L->>L: apply_file_tree_mutations() [main thread]
    alt emit_incremental_updates = true
        L->>L: generate_repo_metadata_update()
        L-->>N: emit IncrementalUpdateReady { update }
        N-->>R: (transport — protobuf over SSH)
        R->>R: apply_incremental_update(update)
        R->>R: emit FileTreeEntryUpdated
    end"#;

#[test]
fn parses_aliased_participants_correctly() {
    let diagram = parse_sequence_diagram(KEVIN_DIAGRAM).expect("should parse");

    assert_eq!(
        diagram.participants.len(),
        4,
        "should have exactly 4 participants, not 8"
    );

    assert_eq!(diagram.participants[0].id, "W");
    assert_eq!(diagram.participants[0].label, "File Watcher");

    assert_eq!(diagram.participants[1].id, "L");
    assert_eq!(
        diagram.participants[1].label,
        "LocalRepoMetadataModel<br/>(remote server)"
    );

    assert_eq!(diagram.participants[2].id, "N");
    assert_eq!(diagram.participants[3].id, "R");
}

#[test]
fn renders_aliased_participants_without_br_in_svg() {
    let theme = crate::theme::MermaidTheme::default();
    let svg =
        render_sequence_diagram_to_svg(KEVIN_DIAGRAM, &theme).expect("should render without error");

    assert!(!svg.contains("<br/>"), "SVG must not contain literal <br/>");
    assert!(
        !svg.contains("W as File Watcher"),
        "SVG must not show the alias declaration as a label"
    );
    assert!(
        svg.contains("File Watcher"),
        "SVG should show the display label 'File Watcher'"
    );
    assert!(
        svg.contains("<tspan"),
        "SVG should use tspan elements for multi-line participant labels"
    );
}
