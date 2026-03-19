use super::*;
use crate::theme::MermaidTheme;

fn default_theme() -> MermaidTheme {
    MermaidTheme::default()
}

#[test]
fn test_basic_journey_parses() {
    let input = r#"journey
    title My working day

    section Go to work
      Make tea: 5: Me
      Go upstairs: 3: Me

    section Go home
      Go downstairs: 5: Me
      Sit down: 5: Me
"#;
    let result = render_journey_diagram_to_svg(input, &default_theme());
    assert!(
        result.is_ok(),
        "Failed to render basic journey: {:?}",
        result.err()
    );
    let svg = result.unwrap();

    // Check title
    assert!(svg.contains("My working day"));

    // Check sections
    assert!(svg.contains("Go to work"));
    assert!(svg.contains("Go home"));

    // Check tasks
    assert!(svg.contains("Make tea"));
    assert!(svg.contains("Go upstairs"));
    assert!(svg.contains("Go downstairs"));
    assert!(svg.contains("Sit down"));

    // Check actor legend
    assert!(svg.contains("Me"));

    // Check structural elements
    assert!(svg.contains("arrowhead"));
    assert!(svg.contains("class=\"face\""));
    assert!(svg.contains("class=\"mouth\""));
    assert!(svg.contains("task-line"));
    assert!(svg.contains("journey-section"));
    assert!(svg.contains("section-type-0"));
    assert!(svg.contains("section-type-1"));
    assert!(svg.contains("task-type-0"));
    assert!(svg.contains("task-type-1"));
}

#[test]
fn test_journey_face_types() {
    let input = r#"journey
    title Faces
    section Test
      Happy: 5: Me
      Neutral: 3: Me
      Sad: 1: Me
"#;
    let result = render_journey_diagram_to_svg(input, &default_theme());
    assert!(result.is_ok());
    let svg = result.unwrap();

    // Happy face has smile arc path
    assert!(svg.contains("class=\"mouth\" d=\"M"));

    // Neutral face has line mouth
    assert!(svg.contains("<line class=\"mouth\""));
}

#[test]
fn test_journey_multiple_actors() {
    let input = r#"journey
    title Multi actor
    section Work
      Code: 5: Alice, Bob
"#;
    let result = render_journey_diagram_to_svg(input, &default_theme());
    assert!(result.is_ok());
    let svg = result.unwrap();

    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    // Both actors should have legend entries
    assert!(svg.contains("actor-0"));
    assert!(svg.contains("actor-1"));
}

#[test]
fn test_journey_no_title() {
    let input = r#"journey
    section Work
      Code: 5: Me
"#;
    let result = render_journey_diagram_to_svg(input, &default_theme());
    assert!(result.is_ok());
    let svg = result.unwrap();

    // Should not have extra title vertical space
    assert!(!svg.contains("font-weight=\"bold\" y=\"25\""));
}

#[test]
fn test_format_num() {
    assert_eq!(format_num(7.5), "7.5");
    assert_eq!(format_num(6.818181818181818), "6.818");
    assert_eq!(format_num(15.0), "15");
    assert_eq!(format_num(1.100), "1.1");
}

#[test]
fn test_smile_arc_path() {
    let path = generate_smile_arc(7.5, 6.818181818181818);
    // Should match the reference SVG arc
    assert!(path.starts_with("M6.818,0A6.818,6.818,0,1,1,-6.818,0L-7.5,0A7.5,7.5,0,1,0,7.5,0Z"));
}

#[test]
fn test_journey_svg_is_valid_xml() {
    // Regression test: unescaped double-quotes in font-family attributes
    // previously broke XML parsing, causing renderers to show blank SVGs.
    let input = r#"journey
    title My working day
    section Go to work
      Make tea: 5: Me
      Go upstairs: 3: Me
    section Go home
      Go downstairs: 5: Me
"#;
    let svg = render_journey_diagram_to_svg(input, &default_theme()).unwrap();

    // The SVG must not contain unescaped double-quotes inside attribute values.
    // Specifically, font-family values must use single quotes.
    assert!(
        !svg.contains("font-family=\"\"trebuchet"),
        "font-family attribute contains unescaped double-quotes"
    );
    assert!(
        !svg.contains("font-family: \"Open Sans\""),
        "inline style font-family contains unescaped double-quotes"
    );

    // Title must have a visible fill color (not empty)
    assert!(
        !svg.contains("fill=\"\" "),
        "title text has empty fill attribute"
    );

    // foreignObject must have requiredExtensions for proper switch fallback
    assert!(
        svg.contains("requiredExtensions=\"http://www.w3.org/1999/xhtml\""),
        "foreignObject missing requiredExtensions attribute"
    );
}
