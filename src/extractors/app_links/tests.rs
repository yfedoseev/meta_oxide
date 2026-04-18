use super::*;

#[test]
fn extracts_facebook_app_links() {
    let html = r#"
        <meta property="al:ios:url" content="applinks://docs">
        <meta property="al:ios:app_store_id" content="12345">
        <meta property="al:ios:app_name" content="App Links">
        <meta property="al:android:url" content="applinks://docs">
        <meta property="al:android:package" content="org.applinks">
        <meta property="al:android:class" content="org.applinks.DocsActivity">
        <meta property="al:web:url" content="https://applinks.org/documentation">
        <meta property="al:web:should_fallback" content="true">
    "#;
    let l = extract(html, None).unwrap();
    let ios = l.ios.unwrap();
    assert_eq!(ios.url.as_deref(), Some("applinks://docs"));
    assert_eq!(ios.app_store_id.as_deref(), Some("12345"));
    let android = l.android.unwrap();
    assert_eq!(android.package.as_deref(), Some("org.applinks"));
    assert_eq!(l.web_url.as_deref(), Some("https://applinks.org/documentation"));
    assert_eq!(l.web_should_fallback, Some(true));
}

#[test]
fn extracts_apple_smart_app_banner() {
    let html = r#"<meta name="apple-itunes-app" content="app-id=123456, app-argument=https://example.com/p/1, affiliate-data=partnerId=30">"#;
    let l = extract(html, None).unwrap();
    let banner = l.apple_itunes_app.unwrap();
    assert_eq!(banner.app_id.as_deref(), Some("123456"));
    assert_eq!(banner.app_argument.as_deref(), Some("https://example.com/p/1"));
    assert!(banner.affiliate_data.is_some());
}

#[test]
fn extracts_google_play_app() {
    let html = r#"
        <meta name="google-play-app" content="app-id=com.example.app">
        <meta name="android-app-intent" content="intent://example.com/p/1#Intent;scheme=https;package=com.example.app;end">
    "#;
    let l = extract(html, None).unwrap();
    let gp = l.google_play_app.unwrap();
    assert_eq!(gp.app_id.as_deref(), Some("com.example.app"));
    assert!(gp.intent.as_deref().unwrap().contains("intent://"));
}

#[test]
fn empty_when_no_tags() {
    let html = r#"<meta name="description" content="x">"#;
    assert!(extract(html, None).unwrap().is_empty());
}

#[test]
fn windows_variants_merge() {
    let html = r#"
        <meta property="al:windows:url" content="ms-app://x">
        <meta property="al:windows_phone:app_name" content="WinApp">
    "#;
    let l = extract(html, None).unwrap();
    let w = l.windows.unwrap();
    assert_eq!(w.url.as_deref(), Some("ms-app://x"));
    assert_eq!(w.app_name.as_deref(), Some("WinApp"));
}
