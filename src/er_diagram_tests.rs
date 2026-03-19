use super::*;

#[test]
fn test_parse_basic_er_diagram() {
    let input = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE_ITEM : contains

    CUSTOMER {
        string name
        string custNumber
    }

    ORDER {
        int orderNumber
        date orderDate
    }

    LINE_ITEM {
        int quantity
        float price
    }"#;

    let diagram = parse_er_diagram(input).unwrap();
    assert_eq!(diagram.entities.len(), 3);
    assert!(diagram.entities.contains_key("CUSTOMER"));
    assert!(diagram.entities.contains_key("ORDER"));
    assert!(diagram.entities.contains_key("LINE_ITEM"));
    assert_eq!(diagram.relationships.len(), 2);

    let customer = &diagram.entities["CUSTOMER"];
    assert_eq!(customer.attributes.len(), 2);
    assert_eq!(customer.attributes[0].attr_type, "string");
    assert_eq!(customer.attributes[0].name, "name");

    let rel0 = &diagram.relationships[0];
    assert_eq!(rel0.entity_a, "CUSTOMER");
    assert_eq!(rel0.entity_b, "ORDER");
    assert_eq!(rel0.role, "places");
    assert_eq!(rel0.rel_spec.card_a, Cardinality::OnlyOne);
    assert_eq!(rel0.rel_spec.card_b, Cardinality::ZeroOrMore);
    assert_eq!(rel0.rel_spec.rel_type, Identification::Identifying);

    let rel1 = &diagram.relationships[1];
    assert_eq!(rel1.entity_a, "ORDER");
    assert_eq!(rel1.entity_b, "LINE_ITEM");
    assert_eq!(rel1.role, "contains");
    assert_eq!(rel1.rel_spec.card_a, Cardinality::OnlyOne);
    assert_eq!(rel1.rel_spec.card_b, Cardinality::OneOrMore);
}

#[test]
fn test_render_basic_er_diagram() {
    let input = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE_ITEM : contains

    CUSTOMER {
        string name
        string custNumber
    }

    ORDER {
        int orderNumber
        date orderDate
    }

    LINE_ITEM {
        int quantity
        float price
    }"#;

    let theme = MermaidTheme::default();
    let result = render_er_diagram_to_svg(input, &theme);
    assert!(result.is_ok());
    let svg = result.unwrap();

    // Basic SVG structure
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("erDiagram"));

    // Entity names present
    assert!(svg.contains("CUSTOMER"));
    assert!(svg.contains("ORDER"));
    assert!(svg.contains("LINE_ITEM"));

    // Attributes present
    assert!(svg.contains("string"));
    assert!(svg.contains("name"));
    assert!(svg.contains("custNumber"));
    assert!(svg.contains("orderNumber"));

    // Relationship labels present
    assert!(svg.contains("places"));
    assert!(svg.contains("contains"));

    // ER markers present
    assert!(svg.contains("onlyOneStart"));
    assert!(svg.contains("onlyOneEnd"));
    assert!(svg.contains("zeroOrMoreStart"));
    assert!(svg.contains("zeroOrMoreEnd"));
    assert!(svg.contains("oneOrMoreEnd"));

    // Has divider lines
    assert!(svg.contains("divider"));

    // Has entity box class
    assert!(svg.contains("entityBox"));
}

#[test]
fn test_parse_non_identifying_relationship() {
    let input = r#"erDiagram
    A }o..o{ B : rel"#;

    let diagram = parse_er_diagram(input).unwrap();
    let rel = &diagram.relationships[0];
    assert_eq!(rel.rel_spec.rel_type, Identification::NonIdentifying);
    assert_eq!(rel.rel_spec.card_a, Cardinality::ZeroOrMore);
    assert_eq!(rel.rel_spec.card_b, Cardinality::ZeroOrMore);
}

#[test]
fn test_entity_without_attributes() {
    let input = r#"erDiagram
    A ||--|| B : links"#;

    let diagram = parse_er_diagram(input).unwrap();
    assert_eq!(diagram.entities.len(), 2);
    assert!(diagram.entities["A"].attributes.is_empty());
    assert!(diagram.entities["B"].attributes.is_empty());

    let theme = MermaidTheme::default();
    let result = render_er_diagram_to_svg(input, &theme);
    assert!(result.is_ok());
}

#[test]
fn test_cardinality_parsing() {
    assert_eq!(parse_cardinality("||", 1).unwrap(), Cardinality::OnlyOne);
    assert_eq!(parse_cardinality("|o", 1).unwrap(), Cardinality::ZeroOrOne);
    assert_eq!(parse_cardinality("o|", 1).unwrap(), Cardinality::ZeroOrOne);
    assert_eq!(parse_cardinality("|{", 1).unwrap(), Cardinality::OneOrMore);
    assert_eq!(parse_cardinality("}|", 1).unwrap(), Cardinality::OneOrMore);
    assert_eq!(parse_cardinality("o{", 1).unwrap(), Cardinality::ZeroOrMore);
    assert_eq!(parse_cardinality("}o", 1).unwrap(), Cardinality::ZeroOrMore);
}
