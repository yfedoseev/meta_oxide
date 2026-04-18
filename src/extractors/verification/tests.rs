use super::*;

#[test]
fn extracts_known_platforms() {
    let html = r#"
        <meta name="google-site-verification" content="ABC123">
        <meta name="msvalidate.01" content="BING456">
        <meta name="facebook-domain-verification" content="fbv789">
        <meta name="yandex-verification" content="YX111">
        <meta name="p:domain_verify" content="PIN222">
        <meta name="bytedance-verification-code" content="BD333">
        <meta name="norton-safeweb-site-verification" content="NSW444">
    "#;
    let v = extract(html).unwrap();
    assert_eq!(v.google.as_deref(), Some("ABC123"));
    assert_eq!(v.bing.as_deref(), Some("BING456"));
    assert_eq!(v.facebook.as_deref(), Some("fbv789"));
    assert_eq!(v.yandex.as_deref(), Some("YX111"));
    assert_eq!(v.pinterest.as_deref(), Some("PIN222"));
    assert_eq!(v.tiktok.as_deref(), Some("BD333"));
    assert_eq!(v.norton.as_deref(), Some("NSW444"));
}

#[test]
fn unknown_verification_tags_go_to_other() {
    let html = r#"<meta name="example-site-verification" content="XYZ">"#;
    let v = extract(html).unwrap();
    assert_eq!(v.other.get("example-site-verification").map(String::as_str), Some("XYZ"));
}

#[test]
fn empty_when_none_present() {
    let html = r#"<meta name="description" content="hi">"#;
    assert!(extract(html).unwrap().is_empty());
}
