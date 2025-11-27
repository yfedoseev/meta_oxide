//! Tests for Dublin Core metadata extraction

use super::*;

#[test]
fn test_dublin_core_title() {
    let html = r#"<meta name="DC.title" content="My Document Title">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.title, Some("My Document Title".to_string()));
}

#[test]
fn test_dublin_core_creator() {
    let html = r#"<meta name="DC.creator" content="John Doe">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.creator, Some("John Doe".to_string()));
}

#[test]
fn test_dublin_core_subject() {
    let html = r#"<meta name="DC.subject" content="rust, metadata, extraction">"#;
    let dc = extract(html).unwrap();
    assert_eq!(
        dc.subject,
        Some(vec!["rust".to_string(), "metadata".to_string(), "extraction".to_string()])
    );
}

#[test]
fn test_dublin_core_description() {
    let html = r#"<meta name="DC.description" content="A comprehensive guide to metadata">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.description, Some("A comprehensive guide to metadata".to_string()));
}

#[test]
fn test_dublin_core_publisher() {
    let html = r#"<meta name="DC.publisher" content="Example Press">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.publisher, Some("Example Press".to_string()));
}

#[test]
fn test_dublin_core_date() {
    let html = r#"<meta name="DC.date" content="2024-01-15">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.date, Some("2024-01-15".to_string()));
}

#[test]
fn test_dublin_core_type() {
    let html = r#"<meta name="DC.type" content="Text">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.type_, Some("Text".to_string()));
}

#[test]
fn test_dublin_core_format() {
    let html = r#"<meta name="DC.format" content="text/html">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.format, Some("text/html".to_string()));
}

#[test]
fn test_dublin_core_identifier() {
    let html = r#"<meta name="DC.identifier" content="ISBN:123-456-789">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.identifier, Some("ISBN:123-456-789".to_string()));
}

#[test]
fn test_dublin_core_language() {
    let html = r#"<meta name="DC.language" content="en-US">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.language, Some("en-US".to_string()));
}

#[test]
fn test_dublin_core_rights() {
    let html = r#"<meta name="DC.rights" content="Copyright 2024 Example Corp">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.rights, Some("Copyright 2024 Example Corp".to_string()));
}

#[test]
fn test_dublin_core_lowercase_prefix() {
    let html = r#"<meta name="dc.title" content="Lowercase Prefix">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.title, Some("Lowercase Prefix".to_string()));
}

#[test]
fn test_dublin_core_dcterms_prefix() {
    let html = r#"<meta name="dcterms.title" content="DCTerms Title">"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.title, Some("DCTerms Title".to_string()));
}

#[test]
fn test_dublin_core_complete() {
    let html = r#"
        <html>
        <head>
            <meta name="DC.title" content="Complete Document">
            <meta name="DC.creator" content="Jane Smith">
            <meta name="DC.subject" content="technology, innovation">
            <meta name="DC.description" content="A complete example">
            <meta name="DC.publisher" content="Tech Publishers">
            <meta name="DC.date" content="2024-02-01">
            <meta name="DC.type" content="Article">
            <meta name="DC.format" content="text/html">
            <meta name="DC.identifier" content="DOI:10.1234/example">
            <meta name="DC.language" content="en">
            <meta name="DC.rights" content="CC-BY-4.0">
        </head>
        </html>
    "#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.title, Some("Complete Document".to_string()));
    assert_eq!(dc.creator, Some("Jane Smith".to_string()));
    assert_eq!(dc.subject, Some(vec!["technology".to_string(), "innovation".to_string()]));
    assert_eq!(dc.description, Some("A complete example".to_string()));
    assert_eq!(dc.publisher, Some("Tech Publishers".to_string()));
    assert_eq!(dc.date, Some("2024-02-01".to_string()));
    assert_eq!(dc.type_, Some("Article".to_string()));
    assert_eq!(dc.format, Some("text/html".to_string()));
    assert_eq!(dc.identifier, Some("DOI:10.1234/example".to_string()));
    assert_eq!(dc.language, Some("en".to_string()));
    assert_eq!(dc.rights, Some("CC-BY-4.0".to_string()));
}

#[test]
fn test_dublin_core_empty() {
    let html = r#"<html><head><title>No Dublin Core</title></head></html>"#;
    let dc = extract(html).unwrap();
    assert_eq!(dc.title, None);
    assert_eq!(dc.creator, None);
    assert_eq!(dc.description, None);
}

#[test]
fn test_dublin_core_contributor_list() {
    let html = r#"<meta name="DC.contributor" content="Alice, Bob, Charlie">"#;
    let dc = extract(html).unwrap();
    assert_eq!(
        dc.contributor,
        Some(vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()])
    );
}
