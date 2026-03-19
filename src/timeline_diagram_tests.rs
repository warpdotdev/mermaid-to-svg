use super::*;

#[test]
fn parse_basic_timeline() {
    let input = r#"timeline
    title History of Social Platforms
    2002 : LinkedIn
    2004 : Facebook
    2006 : Twitter
"#;
    let result = parse_timeline_diagram(input).unwrap();
    assert_eq!(result.title.as_deref(), Some("History of Social Platforms"));
    assert_eq!(result.tasks.len(), 3);
    assert_eq!(result.tasks[0].period, "2002");
    assert_eq!(result.tasks[0].events, vec!["LinkedIn"]);
    assert_eq!(result.tasks[1].period, "2004");
    assert_eq!(result.tasks[1].events, vec!["Facebook"]);
    assert_eq!(result.tasks[2].period, "2006");
    assert_eq!(result.tasks[2].events, vec!["Twitter"]);
    assert!(result.sections.is_empty());
}

#[test]
fn parse_timeline_with_sections() {
    let input = r#"timeline
    title Timeline
    section Early
    2001 : Event A
    2002 : Event B
    section Late
    2010 : Event C
"#;
    let result = parse_timeline_diagram(input).unwrap();
    assert_eq!(result.sections, vec!["Early", "Late"]);
    assert_eq!(result.tasks.len(), 3);
    assert_eq!(result.tasks[0].section.as_deref(), Some("Early"));
    assert_eq!(result.tasks[2].section.as_deref(), Some("Late"));
}

#[test]
fn render_basic_timeline_produces_horizontal_layout() {
    let input = r#"timeline
    title History of Social Platforms
    2002 : LinkedIn
    2004 : Facebook
    2006 : Twitter
"#;
    let theme = MermaidTheme::default();
    let svg = render_timeline_diagram_to_svg(input, &theme).unwrap();

    // Should contain SVG header
    assert!(svg.contains("<svg"));
    assert!(svg.contains("timeline"));

    // Should contain title
    assert!(svg.contains("History of Social Platforms"));

    // Should contain task nodes
    assert!(svg.contains("2002"));
    assert!(svg.contains("2004"));
    assert!(svg.contains("2006"));

    // Should contain event nodes
    assert!(svg.contains("LinkedIn"));
    assert!(svg.contains("Facebook"));
    assert!(svg.contains("Twitter"));

    // Should contain arrowhead definition
    assert!(svg.contains("marker"));
    assert!(svg.contains("arrowhead"));

    // Should contain dashed connectors
    assert!(svg.contains("stroke-dasharray"));

    // Should contain horizontal arrow
    assert!(svg.contains("lineWrapper"));

    // Should have section CSS classes for multicolor
    assert!(svg.contains("section-"));

    // Should contain node background paths
    assert!(svg.contains("node-bkg"));

    // Should NOT contain bullet circles (old list layout)
    assert!(!svg.contains("<circle cx=\"28"));
}

#[test]
fn render_timeline_without_title() {
    let input = r#"timeline
    2020 : Pandemic
    2021 : Vaccines
"#;
    let theme = MermaidTheme::default();
    let svg = render_timeline_diagram_to_svg(input, &theme).unwrap();
    assert!(svg.contains("Pandemic"));
    assert!(svg.contains("Vaccines"));
    assert!(svg.contains("taskWrapper"));
}
