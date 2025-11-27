//! Comprehensive tests for RDFa extractor

use super::*;
use crate::types::rdfa::RdfaValue;

// Basic extraction tests

#[test]
fn test_rdfa_basic_vocab_typeof_property() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Person">
            <span property="name">Jane Doe</span>
            <span property="jobTitle">Professor</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].vocab, Some("https://schema.org/".to_string()));
    assert_eq!(result[0].type_of, Some(vec!["Person".to_string()]));
    assert_eq!(result[0].properties.len(), 2);
}

#[test]
fn test_rdfa_schema_org_person() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Person">
            <span property="name">Jane Doe</span>
            <span property="email">jane@example.com</span>
            <a property="url" href="https://example.com/jane">Profile</a>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    let item = &result[0];
    assert!(item.properties.contains_key("name"));
    assert!(item.properties.contains_key("email"));
    assert!(item.properties.contains_key("url"));
}

#[test]
fn test_rdfa_schema_org_product() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Product">
            <span property="name">Widget</span>
            <span property="description">A great widget</span>
            <span property="brand">ACME</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].type_of, Some(vec!["Product".to_string()]));
    assert_eq!(result[0].properties.len(), 3);
}

#[test]
fn test_rdfa_schema_org_article() {
    let html = r#"
        <article vocab="https://schema.org/" typeof="Article">
            <h1 property="headline">Test Article</h1>
            <span property="author">John Smith</span>
            <time property="datePublished" content="2025-01-01">January 1, 2025</time>
        </article>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].type_of, Some(vec!["Article".to_string()]));
}

// Value type tests

#[test]
fn test_rdfa_literal_value() {
    let html = r#"<div typeof="Thing"><span property="name">Test</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("name").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, "Test"),
        _ => panic!("Expected literal value"),
    }
}

#[test]
fn test_rdfa_resource_value_href() {
    let html = r#"<div typeof="Thing"><a property="url" href="https://example.com">Link</a></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("url").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_resource_value_src() {
    let html =
        r#"<div typeof="Thing"><img property="image" src="https://example.com/image.jpg" /></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("image").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com/image.jpg"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_resource_value_resource_attr() {
    let html = r#"<div typeof="Thing"><span property="sameAs" resource="https://example.com/thing">Thing</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("sameAs").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com/thing"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_content_override() {
    let html = r#"<div typeof="Thing"><span property="name" content="Actual Name">Display Name</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("name").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, "Actual Name"),
        _ => panic!("Expected literal value"),
    }
}

#[test]
fn test_rdfa_datatype_handling() {
    let html = r#"<div typeof="Thing"><span property="age" datatype="xsd:integer">25</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("age").unwrap()[0] {
        RdfaValue::TypedLiteral { value, datatype } => {
            assert_eq!(value, "25");
            assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#integer");
        }
        _ => panic!("Expected typed literal"),
    }
}

#[test]
fn test_rdfa_datatype_with_content() {
    let html = r#"<div typeof="Thing"><span property="price" content="19.99" datatype="xsd:decimal">$19.99</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("price").unwrap()[0] {
        RdfaValue::TypedLiteral { value, datatype } => {
            assert_eq!(value, "19.99");
            assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#decimal");
        }
        _ => panic!("Expected typed literal"),
    }
}

// Nested item tests

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_nested_typeof() {
    let html = r#"
        <div typeof="Person">
            <span property="name">Jane Doe</span>
            <div property="address" typeof="PostalAddress">
                <span property="streetAddress">123 Main St</span>
                <span property="addressLocality">Springfield</span>
            </div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    match &result[0].properties.get("address").unwrap()[0] {
        RdfaValue::Item(item) => {
            assert_eq!(item.type_of, Some(vec!["PostalAddress".to_string()]));
            assert!(item.properties.contains_key("streetAddress"));
            assert!(item.properties.contains_key("addressLocality"));
        }
        _ => panic!("Expected nested item"),
    }
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items - requires architectural changes to prevent infinite recursion
fn test_rdfa_deeply_nested() {
    let html = r#"
        <div typeof="Organization">
            <span property="name">ACME Corp</span>
            <div property="address" typeof="PostalAddress">
                <span property="streetAddress">123 Main St</span>
                <div property="geo" typeof="GeoCoordinates">
                    <span property="latitude">40.7128</span>
                    <span property="longitude">-74.0060</span>
                </div>
            </div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    match &result[0].properties.get("address").unwrap()[0] {
        RdfaValue::Item(address) => match &address.properties.get("geo").unwrap()[0] {
            RdfaValue::Item(geo) => {
                assert_eq!(geo.type_of, Some(vec!["GeoCoordinates".to_string()]));
                assert!(geo.properties.contains_key("latitude"));
            }
            _ => panic!("Expected nested geo item"),
        },
        _ => panic!("Expected nested address item"),
    }
}

// Edge case tests

#[test]
fn test_rdfa_empty_html() {
    let result = extract("", None).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_rdfa_no_rdfa_markup() {
    let html = r#"<div><p>Just regular HTML</p></div>"#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_rdfa_malformed_attributes() {
    let html = r#"<div typeof=""><span property="">Test</span></div>"#;
    let result = extract(html, None).unwrap();
    // Should handle gracefully - empty typeof means no types
    assert_eq!(result.len(), 1);
    assert!(result[0].type_of.is_none() || result[0].type_of == Some(vec![]));
}

#[test]
fn test_rdfa_missing_vocab() {
    let html = r#"<div typeof="Person"><span property="name">Jane</span></div>"#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].vocab.is_none());
}

#[test]
fn test_rdfa_property_without_typeof() {
    let html = r#"<div vocab="https://schema.org/"><span property="name">Test</span></div>"#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    // Should still extract the property even without typeof
    assert!(result[0].properties.contains_key("name"));
}

#[test]
fn test_rdfa_whitespace_in_typeof() {
    let html = r#"<div typeof="  Person   Employee  "><span property="name">Jane</span></div>"#;
    let result = extract(html, None).unwrap();
    let types = result[0].type_of.as_ref().unwrap();
    assert_eq!(types.len(), 2);
    assert!(types.contains(&"Person".to_string()));
    assert!(types.contains(&"Employee".to_string()));
}

#[test]
fn test_rdfa_empty_property_value() {
    let html = r#"<div typeof="Thing"><span property="name"></span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("name").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, ""),
        _ => panic!("Expected literal value"),
    }
}

// Multiple items tests

#[test]
fn test_rdfa_multiple_items_on_page() {
    let html = r#"
        <div typeof="Person"><span property="name">Jane</span></div>
        <div typeof="Person"><span property="name">John</span></div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_rdfa_multiple_vocabs_on_page() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Person">
            <span property="name">Jane</span>
        </div>
        <div vocab="http://xmlns.com/foaf/0.1/" typeof="Person">
            <span property="name">John</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].vocab, Some("https://schema.org/".to_string()));
    assert_eq!(result[1].vocab, Some("http://xmlns.com/foaf/0.1/".to_string()));
}

// URL resolution tests

#[test]
fn test_rdfa_resolve_relative_url_href() {
    let html = r#"<div typeof="Thing"><a property="url" href="/page">Link</a></div>"#;
    let result = extract(html, Some("https://example.com")).unwrap();
    match &result[0].properties.get("url").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com/page"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_resolve_relative_url_src() {
    let html = r#"<div typeof="Thing"><img property="image" src="/images/photo.jpg" /></div>"#;
    let result = extract(html, Some("https://example.com")).unwrap();
    match &result[0].properties.get("image").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com/images/photo.jpg"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_resolve_about_url() {
    let html = r#"<div typeof="Person" about="/jane"><span property="name">Jane</span></div>"#;
    let result = extract(html, Some("https://example.com")).unwrap();
    assert_eq!(result[0].about, Some("https://example.com/jane".to_string()));
}

#[test]
fn test_rdfa_absolute_url_unchanged() {
    let html =
        r#"<div typeof="Thing"><a property="url" href="https://other.com/page">Link</a></div>"#;
    let result = extract(html, Some("https://example.com")).unwrap();
    match &result[0].properties.get("url").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "https://other.com/page"),
        _ => panic!("Expected resource value"),
    }
}

// Real-world example tests

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_real_world_person() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Person">
            <span property="name">Dr. Jane Smith</span>
            <span property="jobTitle">Chief Technology Officer</span>
            <div property="worksFor" typeof="Organization">
                <span property="name">Tech Corp</span>
            </div>
            <a property="url" href="https://example.com/jane">Profile</a>
            <a property="email" href="mailto:jane@example.com">jane@example.com</a>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    let item = &result[0];
    assert_eq!(item.vocab, Some("https://schema.org/".to_string()));
    assert_eq!(item.type_of, Some(vec!["Person".to_string()]));
    assert!(item.properties.contains_key("name"));
    assert!(item.properties.contains_key("jobTitle"));
    assert!(item.properties.contains_key("worksFor"));
    assert!(item.properties.contains_key("url"));
    assert!(item.properties.contains_key("email"));
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_real_world_event() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Event">
            <h1 property="name">Tech Conference 2025</h1>
            <time property="startDate" content="2025-06-01T09:00">June 1, 2025</time>
            <time property="endDate" content="2025-06-03T17:00">June 3, 2025</time>
            <div property="location" typeof="Place">
                <span property="name">Convention Center</span>
                <div property="address" typeof="PostalAddress">
                    <span property="streetAddress">123 Main St</span>
                    <span property="addressLocality">San Francisco</span>
                    <span property="addressRegion">CA</span>
                </div>
            </div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].type_of, Some(vec!["Event".to_string()]));
    assert!(result[0].properties.contains_key("name"));
    assert!(result[0].properties.contains_key("startDate"));
    assert!(result[0].properties.contains_key("location"));
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_real_world_breadcrumb() {
    let html = r#"
        <ol vocab="https://schema.org/" typeof="BreadcrumbList">
            <li property="itemListElement" typeof="ListItem">
                <a property="item" href="/"><span property="name">Home</span></a>
                <meta property="position" content="1" />
            </li>
            <li property="itemListElement" typeof="ListItem">
                <a property="item" href="/products"><span property="name">Products</span></a>
                <meta property="position" content="2" />
            </li>
        </ol>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].type_of, Some(vec!["BreadcrumbList".to_string()]));
    assert!(result[0].properties.contains_key("itemListElement"));
}

// Unicode and special character tests

#[test]
fn test_rdfa_unicode_in_values() {
    let html = r#"<div typeof="Person"><span property="name">José García</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("name").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, "José García"),
        _ => panic!("Expected literal value"),
    }
}

#[test]
fn test_rdfa_emoji_in_values() {
    let html = r#"<div typeof="Thing"><span property="description">Amazing! 🎉</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("description").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, "Amazing! 🎉"),
        _ => panic!("Expected literal value"),
    }
}

#[test]
fn test_rdfa_html_entities() {
    let html = r#"<div typeof="Thing"><span property="name">AT&amp;T</span></div>"#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("name").unwrap()[0] {
        RdfaValue::Literal(s) => assert_eq!(s, "AT&T"),
        _ => panic!("Expected literal value"),
    }
}

// Complex structure tests

#[test]
fn test_rdfa_multiple_properties_same_name() {
    let html = r#"
        <div typeof="Person">
            <span property="telephone">555-1234</span>
            <span property="telephone">555-5678</span>
            <span property="telephone">555-9999</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result[0].properties.get("telephone").unwrap().len(), 3);
}

#[test]
fn test_rdfa_mixed_value_types() {
    let html = r#"
        <div typeof="Thing">
            <span property="name">Test</span>
            <a property="url" href="https://example.com">Link</a>
            <span property="count" datatype="xsd:integer">42</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result[0].properties.len(), 3);
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_nested_with_vocab_inheritance() {
    let html = r#"
        <div vocab="https://schema.org/" typeof="Organization">
            <span property="name">ACME</span>
            <div property="founder" typeof="Person">
                <span property="name">John Doe</span>
            </div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    // Vocab should be inherited by nested item
    assert_eq!(result[0].vocab, Some("https://schema.org/".to_string()));
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_sibling_items() {
    let html = r#"
        <div vocab="https://schema.org/">
            <div typeof="Person"><span property="name">Jane</span></div>
            <div typeof="Person"><span property="name">John</span></div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    // Should find both sibling items
    assert_eq!(result.len(), 2);
}

// Prefix and CURIE tests

#[test]
fn test_rdfa_prefix_definition() {
    let html = r#"
        <div prefix="myns: http://example.com/ns#" typeof="myns:Thing">
            <span property="myns:name">Test</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    // CURIE should be expanded
    assert_eq!(result[0].type_of, Some(vec!["http://example.com/ns#Thing".to_string()]));
}

#[test]
fn test_rdfa_curie_expansion() {
    let html = r#"
        <div typeof="schema:Person">
            <span property="schema:name">Jane Doe</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    // Default schema prefix should expand
    assert_eq!(result[0].type_of, Some(vec!["https://schema.org/Person".to_string()]));
    assert!(result[0].properties.contains_key("https://schema.org/name"));
}

#[test]
fn test_rdfa_default_vocab() {
    let html = r#"
        <div typeof="foaf:Person">
            <span property="foaf:name">Jane</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    // foaf prefix should be available by default
    assert_eq!(result[0].type_of, Some(vec!["http://xmlns.com/foaf/0.1/Person".to_string()]));
}

#[test]
fn test_rdfa_multiple_prefixes() {
    let html = r#"
        <div prefix="ns1: http://example.com/ns1# ns2: http://example.com/ns2#"
             typeof="ns1:Type">
            <span property="ns2:prop">Value</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result[0].type_of, Some(vec!["http://example.com/ns1#Type".to_string()]));
    assert!(result[0].properties.contains_key("http://example.com/ns2#prop"));
}

#[test]
fn test_rdfa_datatype_with_prefix() {
    let html = r#"
        <div typeof="Thing">
            <span property="age" datatype="xsd:integer">30</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("age").unwrap()[0] {
        RdfaValue::TypedLiteral { value, datatype } => {
            assert_eq!(value, "30");
            // xsd prefix should be expanded
            assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#integer");
        }
        _ => panic!("Expected typed literal"),
    }
}

#[test]
fn test_rdfa_resource_with_prefix() {
    let html = r#"
        <div prefix="ex: http://example.com/"
             typeof="Thing">
            <a property="sameAs" resource="ex:thing/123">Link</a>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    match &result[0].properties.get("sameAs").unwrap()[0] {
        RdfaValue::Resource(uri) => assert_eq!(uri, "http://example.com/thing/123"),
        _ => panic!("Expected resource value"),
    }
}

#[test]
fn test_rdfa_about_with_prefix() {
    let html = r#"
        <div prefix="ex: http://example.com/" typeof="Person" about="ex:person/jane">
            <span property="name">Jane</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result[0].about, Some("http://example.com/person/jane".to_string()));
}

#[test]
#[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
fn test_rdfa_nested_with_prefixes() {
    let html = r#"
        <div prefix="ex: http://example.com/" typeof="ex:Organization">
            <span property="ex:name">ACME</span>
            <div property="ex:founder" typeof="ex:Person">
                <span property="ex:name">John</span>
            </div>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].type_of, Some(vec!["http://example.com/Organization".to_string()]));
}

#[test]
fn test_rdfa_mixed_prefixes_and_full_uris() {
    let html = r#"
        <div typeof="schema:Person">
            <span property="https://schema.org/name">Jane</span>
            <span property="schema:jobTitle">Engineer</span>
        </div>
    "#;
    let result = extract(html, None).unwrap();
    // Both CURIE and full URI should work
    assert!(result[0].properties.contains_key("https://schema.org/name"));
    assert!(result[0].properties.contains_key("https://schema.org/jobTitle"));
}
