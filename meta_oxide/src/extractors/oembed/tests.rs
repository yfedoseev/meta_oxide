//! Tests for oEmbed endpoint discovery

use super::*;

#[test]
fn test_oembed_json_endpoint() {
    let html = r#"<link rel="alternate" type="application/json+oembed" href="https://example.com/oembed?format=json&url=...">"#;
    let discovery = extract(html, None).unwrap();
    assert_eq!(discovery.json_endpoints.len(), 1);
    assert_eq!(discovery.json_endpoints[0].href, "https://example.com/oembed?format=json&url=...");
    assert!(matches!(discovery.json_endpoints[0].format, OEmbedFormat::Json));
}

#[test]
fn test_oembed_xml_endpoint() {
    let html = r#"<link rel="alternate" type="text/xml+oembed" href="https://example.com/oembed?format=xml&url=...">"#;
    let discovery = extract(html, None).unwrap();
    assert_eq!(discovery.xml_endpoints.len(), 1);
    assert_eq!(discovery.xml_endpoints[0].href, "https://example.com/oembed?format=xml&url=...");
    assert!(matches!(discovery.xml_endpoints[0].format, OEmbedFormat::Xml));
}

#[test]
fn test_oembed_with_title() {
    let html = r#"<link rel="alternate" type="application/json+oembed" href="https://example.com/oembed" title="Example oEmbed">"#;
    let discovery = extract(html, None).unwrap();
    assert_eq!(discovery.json_endpoints.len(), 1);
    assert_eq!(discovery.json_endpoints[0].title, Some("Example oEmbed".to_string()));
}

#[test]
fn test_oembed_multiple_endpoints() {
    let html = r#"
        <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed.json">
        <link rel="alternate" type="text/xml+oembed" href="https://example.com/oembed.xml">
    "#;
    let discovery = extract(html, None).unwrap();
    assert_eq!(discovery.json_endpoints.len(), 1);
    assert_eq!(discovery.xml_endpoints.len(), 1);
}

#[test]
fn test_oembed_youtube_style() {
    let html = r#"<link rel="alternate" type="application/json+oembed" href="https://www.youtube.com/oembed?format=json&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DdQw4w9WgXcQ" title="Rick Astley - Never Gonna Give You Up">"#;
    let discovery = extract(html, None).unwrap();
    assert!(discovery.has_endpoints());
    assert_eq!(discovery.json_endpoints.len(), 1);
    assert!(discovery.json_endpoints[0].href.contains("youtube.com/oembed"));
}

#[test]
fn test_oembed_url_resolution() {
    let html = r#"<link rel="alternate" type="application/json+oembed" href="/oembed?url=...">"#;
    let discovery = extract(html, Some("https://example.com")).unwrap();
    assert_eq!(discovery.json_endpoints.len(), 1);
    assert_eq!(discovery.json_endpoints[0].href, "https://example.com/oembed?url=...");
}

#[test]
fn test_oembed_empty() {
    let html = r#"<html><head><title>No oEmbed</title></head></html>"#;
    let discovery = extract(html, None).unwrap();
    assert!(!discovery.has_endpoints());
    assert!(discovery.json_endpoints.is_empty());
    assert!(discovery.xml_endpoints.is_empty());
}

#[test]
fn test_oembed_preferred_endpoints() {
    let html = r#"
        <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed1.json">
        <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed2.json">
        <link rel="alternate" type="text/xml+oembed" href="https://example.com/oembed.xml">
    "#;
    let discovery = extract(html, None).unwrap();
    assert_eq!(discovery.json_endpoints.len(), 2);
    assert_eq!(discovery.xml_endpoints.len(), 1);

    // Test preferred endpoint helpers
    assert_eq!(discovery.preferred_json().unwrap().href, "https://example.com/oembed1.json");
    assert_eq!(discovery.preferred_xml().unwrap().href, "https://example.com/oembed.xml");
}

#[test]
fn test_oembed_no_href() {
    let html = r#"<link rel="alternate" type="application/json+oembed">"#;
    let discovery = extract(html, None).unwrap();
    assert!(discovery.json_endpoints.is_empty());
}

#[test]
fn test_oembed_no_type() {
    let html = r#"<link rel="alternate" href="https://example.com/oembed">"#;
    let discovery = extract(html, None).unwrap();
    assert!(discovery.json_endpoints.is_empty());
}

#[test]
fn test_oembed_wrong_rel() {
    let html = r#"<link rel="stylesheet" type="application/json+oembed" href="https://example.com/oembed">"#;
    let discovery = extract(html, None).unwrap();
    assert!(discovery.json_endpoints.is_empty());
}

#[test]
fn test_oembed_empty_href() {
    let html = r#"<link rel="alternate" type="application/json+oembed" href="">"#;
    let discovery = extract(html, None).unwrap();
    assert!(discovery.json_endpoints.is_empty());
}
