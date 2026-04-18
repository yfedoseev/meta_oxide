use super::*;

#[test]
fn meta_and_link_both_captured() {
    let html = r#"
        <meta name="c2pa:claim_generator" content="Adobe Photoshop 26.0">
        <meta name="c2pa:producer" content="AP News">
        <meta name="c2pa:ai_action" content="ai-generated">
        <meta name="c2pa:format" content="image/jpeg">
        <link rel="c2pa-manifest" href="/manifests/a.c2pa">
    "#;
    let c = extract(html, Some("https://ap.example")).unwrap();
    assert_eq!(c.claim_generator.as_deref(), Some("Adobe Photoshop 26.0"));
    assert_eq!(c.producer.as_deref(), Some("AP News"));
    assert_eq!(c.ai_action.as_deref(), Some("ai-generated"));
    assert_eq!(c.format.as_deref(), Some("image/jpeg"));
    assert_eq!(c.manifest_url.as_deref(), Some("https://ap.example/manifests/a.c2pa"));
}

#[test]
fn content_credentials_rel_alias() {
    let html = r#"<link rel="contentcredentials" href="https://example.com/m.c2pa">"#;
    let c = extract(html, None).unwrap();
    assert_eq!(c.manifest_url.as_deref(), Some("https://example.com/m.c2pa"));
}

#[test]
fn other_bucket_captures_unknown() {
    let html = r#"<meta name="c2pa:ingredient_count" content="3">"#;
    let c = extract(html, None).unwrap();
    assert_eq!(c.other.get("ingredient_count").map(String::as_str), Some("3"));
}

#[test]
fn empty_when_absent() {
    let html = r#"<meta name="description" content="hi">"#;
    assert!(extract(html, None).unwrap().is_empty());
}
