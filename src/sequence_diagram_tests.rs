use super::*;

const ALIASED_PARTICIPANTS_WITH_MULTILINE_LABELS_DIAGRAM: &str = r#"sequenceDiagram
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
    let diagram = parse_sequence_diagram(ALIASED_PARTICIPANTS_WITH_MULTILINE_LABELS_DIAGRAM)
        .expect("should parse");

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
        render_sequence_diagram_to_svg(ALIASED_PARTICIPANTS_WITH_MULTILINE_LABELS_DIAGRAM, &theme)
            .expect("should render without error");

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

#[test]
fn parses_sequence_diagram_with_opt_and_aliases() {
    let diagram = parse_sequence_diagram(
        r#"sequenceDiagram
    participant User
    participant WorkspaceView
    participant RightPanelView
    participant WorkingDirectoriesModel as WDModel
    participant CodeReviewView

    User->>WorkspaceView: toggle panel
    WorkspaceView->>RightPanelView: open_code_review(repo_path, diff_model, terminal)
    RightPanelView->>WDModel: get_code_review_view(pane_group_id)
    opt Old view exists
        RightPanelView->>CodeReviewView: on_close()
    end
    RightPanelView->>RightPanelView: create_code_review_view(repo)
    RightPanelView->>WDModel: store_code_review_view(pane_group_id, new_view)
    RightPanelView->>CodeReviewView: on_open(repo)"#,
    )
    .expect("sequence diagram should parse");

    assert_eq!(diagram.participants.len(), 5);
    assert!(diagram
        .participants
        .iter()
        .any(|participant| participant.id == "WDModel"
            && participant.label == "WorkingDirectoriesModel"));

    let wdmodel_messages = diagram
        .events
        .iter()
        .filter(|event| {
            matches!(
                event,
                SequenceEvent::Message { to, .. } if to == "WDModel"
            )
        })
        .count();
    assert_eq!(wdmodel_messages, 2);
}

#[test]
fn renders_sequence_diagram_with_opt_and_aliases() {
    let svg = render_sequence_diagram_to_svg(
        r#"sequenceDiagram
    participant WorkingDirectoriesModel as WDModel
    participant RightPanelView
    participant CodeReviewView

    RightPanelView->>WDModel: get_code_review_view(pane_group_id)
    opt Old view exists
        RightPanelView->>CodeReviewView: on_close()
    end"#,
        &MermaidTheme::default(),
    )
    .expect("sequence diagram should render");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("WorkingDirectoriesModel"));
    assert!(svg.contains("get_code_review_view(pane_group_id)"));
    assert!(!svg.contains(">WDModel<"));
}

#[test]
fn renders_sequence_fragments() {
    let svg = render_sequence_diagram_to_svg(
        r#"sequenceDiagram
    participant Alice
    participant Bob
    Alice->>Bob: Request
    alt success
        Bob-->>Alice: 200 OK
    else transient failure
        loop retry up to 3 times
            Alice->>Bob: Request
        end
    end"#,
        &MermaidTheme::default(),
    )
    .expect("sequence diagram should render");

    assert!(svg.contains(">alt<"));
    assert!(svg.contains("[success]"));
    assert!(svg.contains(">loop<"));
    assert!(svg.contains("[retry up to 3 times]"));
}

#[test]
fn renders_note_right_of_participant_with_escaped_semicolon() {
    let svg = render_sequence_diagram_to_svg(
        r#"sequenceDiagram
    participant S as warp-server
    Note right of S: previous segment already charged#59; new segment starts fresh"#,
        &MermaidTheme::default(),
    )
    .expect("sequence diagram should render");

    assert!(svg.contains("previous segment already charged; new segment starts fresh"));
    assert!(!svg.contains("#59;"));
}
