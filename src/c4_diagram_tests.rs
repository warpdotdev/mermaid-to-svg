use super::*;

#[test]
fn test_parse_basic_c4_context() {
    let input = r#"C4Context
title System Context diagram for Internet Banking System
Person(customer, "Banking Customer", "A customer of the bank")
System(banking, "Internet Banking System", "Allows customers to view information about their bank accounts")
Rel(customer, banking, "Uses")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.c4_type, "C4Context");
    assert_eq!(
        diagram.title,
        "System Context diagram for Internet Banking System"
    );
    assert_eq!(diagram.shapes.len(), 2);
    assert_eq!(diagram.shapes[0].alias, "customer");
    assert_eq!(diagram.shapes[0].type_c4, "person");
    assert_eq!(diagram.shapes[0].label, "Banking Customer");
    assert_eq!(diagram.shapes[0].descr, "A customer of the bank");
    assert_eq!(diagram.shapes[1].alias, "banking");
    assert_eq!(diagram.shapes[1].type_c4, "system");
    assert_eq!(diagram.rels.len(), 1);
    assert_eq!(diagram.rels[0].from, "customer");
    assert_eq!(diagram.rels[0].to, "banking");
    assert_eq!(diagram.rels[0].label, "Uses");
}

#[test]
fn test_parse_c4_container() {
    let input = r#"C4Container
title Container diagram for Internet Banking System
Container(web_app, "Web Application", "Java, Spring MVC", "Delivers the static content")
Container(api, "API Application", "Java, Docker", "Provides banking functionality")
ContainerDb(db, "Database", "Oracle", "Stores user data")
Rel(web_app, api, "Makes API calls to", "JSON/HTTPS")
Rel(api, db, "Reads from and writes to", "JDBC")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.c4_type, "C4Container");
    assert_eq!(diagram.shapes.len(), 3);
    assert_eq!(diagram.shapes[0].type_c4, "container");
    assert_eq!(diagram.shapes[0].techn, "Java, Spring MVC");
    assert_eq!(diagram.shapes[2].type_c4, "container_db");
    assert_eq!(diagram.rels.len(), 2);
    assert_eq!(diagram.rels[1].techn, "JDBC");
}

#[test]
fn test_parse_c4_with_boundary() {
    let input = r#"C4Context
Person(user, "User")
Enterprise_Boundary(eb, "Enterprise") {
    System(sys, "System")
}
Rel(user, sys, "Uses")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.shapes.len(), 2);
    assert_eq!(diagram.shapes[0].parent_boundary, "global");
    assert_eq!(diagram.shapes[1].parent_boundary, "eb");
    assert_eq!(diagram.boundaries.len(), 2); // global + eb
    assert_eq!(diagram.boundaries[1].alias, "eb");
    assert_eq!(diagram.boundaries[1].label, "Enterprise");
}

#[test]
fn test_parse_c4_external() {
    let input = r#"C4Context
Person_Ext(ext_user, "External User")
System_Ext(ext_sys, "External System", "Third party")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.shapes[0].type_c4, "external_person");
    assert_eq!(diagram.shapes[1].type_c4, "external_system");
}

#[test]
fn test_render_basic_c4_produces_svg() {
    let input = r#"C4Context
title System Context diagram for Internet Banking System
Person(customer, "Banking Customer", "A customer of the bank")
System(banking, "Internet Banking System", "Allows customers to view information about their bank accounts")
Rel(customer, banking, "Uses")
"#;
    let theme = MermaidTheme::default();
    let svg = render_c4_diagram_to_svg(input, &theme).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("aria-roledescription=\"c4\""));
    assert!(svg.contains("Banking Customer"));
    assert!(svg.contains("Internet Banking System"));
    assert!(svg.contains("Uses"));
    assert!(svg.contains("person-man"));
    assert!(svg.contains("#08427B")); // person bg color
    assert!(svg.contains("#1168BD")); // system bg color
}

#[test]
fn test_render_c4_with_title() {
    let input = r#"C4Context
title My Title
Person(p, "Person")
"#;
    let theme = MermaidTheme::default();
    let svg = render_c4_diagram_to_svg(input, &theme).unwrap();
    assert!(svg.contains("My Title"));
}

#[test]
fn test_split_c4_args() {
    let args = split_c4_args(r#"customer, "Banking Customer", "A customer of the bank""#);
    assert_eq!(
        args,
        vec!["customer", "Banking Customer", "A customer of the bank"]
    );
}

#[test]
fn test_split_c4_args_no_quotes() {
    let args = split_c4_args("a, b, c");
    assert_eq!(args, vec!["a", "b", "c"]);
}

#[test]
fn test_invalid_c4_header() {
    let input = "notAC4Diagram\n";
    let result = parse_c4_diagram(input);
    assert!(result.is_err());
}

#[test]
fn test_birel_parsing() {
    let input = r#"C4Context
Person(a, "A")
System(b, "B")
BiRel(a, b, "Bidirectional")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.rels[0].rel_type, "birel");
    assert_eq!(diagram.rels[0].label, "Bidirectional");
}

#[test]
fn test_c4_component_diagram() {
    let input = r#"C4Component
Component(comp1, "Component 1", "Java", "Does something")
Component_Ext(comp2, "External Component", "Python", "External dependency")
Rel(comp1, comp2, "Calls")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.c4_type, "C4Component");
    assert_eq!(diagram.shapes[0].type_c4, "component");
    assert_eq!(diagram.shapes[1].type_c4, "external_component");
}

#[test]
fn test_c4_dynamic_diagram() {
    let input = r#"C4Dynamic
Person(user, "User")
System(sys, "System")
Rel(user, sys, "Makes request")
"#;
    let diagram = parse_c4_diagram(input).unwrap();
    assert_eq!(diagram.c4_type, "C4Dynamic");
}
