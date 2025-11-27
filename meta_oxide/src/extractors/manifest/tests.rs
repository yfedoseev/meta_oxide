//! Comprehensive tests for Web App Manifest extractor

use super::*;

// Link discovery tests

#[test]
fn test_manifest_link_discovery_basic() {
    let html = r#"<link rel="manifest" href="/manifest.json">"#;
    let result = extract_link(html, Some("https://example.com")).unwrap();
    assert_eq!(result.href, Some("https://example.com/manifest.json".to_string()));
    assert!(result.manifest.is_none());
}

#[test]
fn test_manifest_link_discovery_relative_url() {
    let html = r#"<link rel="manifest" href="./manifest.json">"#;
    let result = extract_link(html, Some("https://example.com/app/")).unwrap();
    assert_eq!(result.href, Some("https://example.com/app/manifest.json".to_string()));
}

#[test]
fn test_manifest_link_discovery_parent_dir() {
    let html = r#"<link rel="manifest" href="../manifest.json">"#;
    let result = extract_link(html, Some("https://example.com/app/page/")).unwrap();
    assert_eq!(result.href, Some("https://example.com/app/manifest.json".to_string()));
}

#[test]
fn test_manifest_link_discovery_absolute_url() {
    let html = r#"<link rel="manifest" href="https://cdn.example.com/manifest.json">"#;
    let result = extract_link(html, Some("https://example.com")).unwrap();
    assert_eq!(result.href, Some("https://cdn.example.com/manifest.json".to_string()));
}

#[test]
fn test_manifest_link_discovery_no_link() {
    let html = r#"<html><head><title>No Manifest</title></head></html>"#;
    let result = extract_link(html, None).unwrap();
    assert!(result.href.is_none());
}

#[test]
fn test_manifest_link_discovery_wrong_rel() {
    let html = r#"<link rel="stylesheet" href="/manifest.json">"#;
    let result = extract_link(html, None).unwrap();
    assert!(result.href.is_none());
}

#[test]
fn test_manifest_link_discovery_no_href() {
    let html = r#"<link rel="manifest">"#;
    let result = extract_link(html, None).unwrap();
    assert!(result.href.is_none());
}

#[test]
fn test_manifest_link_discovery_multiple_links() {
    let html = r#"
        <link rel="icon" href="/icon.png">
        <link rel="manifest" href="/manifest.json">
        <link rel="stylesheet" href="/style.css">
    "#;
    let result = extract_link(html, Some("https://example.com")).unwrap();
    assert_eq!(result.href, Some("https://example.com/manifest.json".to_string()));
}

// JSON parsing tests

#[test]
fn test_parse_manifest_full() {
    let json = r##"
{
    "name": "My Progressive Web App",
    "short_name": "MyPWA",
    "description": "A sample PWA",
    "start_url": "/",
    "display": "standalone",
    "theme_color": "#3367D6",
    "background_color": "#FFFFFF",
    "scope": "/app/"
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.name, Some("My Progressive Web App".to_string()));
    assert_eq!(result.short_name, Some("MyPWA".to_string()));
    assert_eq!(result.description, Some("A sample PWA".to_string()));
    assert_eq!(result.start_url, Some("https://example.com/".to_string()));
    assert_eq!(result.display, Some("standalone".to_string()));
    assert_eq!(result.theme_color, Some("#3367D6".to_string()));
    assert_eq!(result.background_color, Some("#FFFFFF".to_string()));
    assert_eq!(result.scope, Some("https://example.com/app/".to_string()));
}

#[test]
fn test_parse_manifest_minimal() {
    let json = r#"{"name": "Test App"}"#;
    let result = parse_manifest(json, None).unwrap();
    assert_eq!(result.name, Some("Test App".to_string()));
    assert!(result.short_name.is_none());
    assert!(result.icons.is_empty());
}

#[test]
fn test_parse_manifest_empty() {
    let json = r#"{}"#;
    let result = parse_manifest(json, None).unwrap();
    assert!(result.name.is_none());
}

#[test]
fn test_parse_manifest_icons_array() {
    let json = r##"
{
    "name": "Test",
    "icons": [
        {
            "src": "/icon-192.png",
            "sizes": "192x192",
            "type": "image/png",
            "purpose": "any"
        },
        {
            "src": "/icon-512.png",
            "sizes": "512x512",
            "type": "image/png",
            "purpose": "maskable"
        }
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.icons.len(), 2);
    assert_eq!(result.icons[0].src, "https://example.com/icon-192.png");
    assert_eq!(result.icons[0].sizes, Some("192x192".to_string()));
    assert_eq!(result.icons[0].mime_type, Some("image/png".to_string()));
    assert_eq!(result.icons[0].purpose, Some("any".to_string()));
    assert_eq!(result.icons[1].src, "https://example.com/icon-512.png");
    assert_eq!(result.icons[1].purpose, Some("maskable".to_string()));
}

#[test]
fn test_parse_manifest_shortcuts() {
    let json = r##"
{
    "name": "Test",
    "shortcuts": [
        {
            "name": "New Item",
            "url": "/new",
            "description": "Create a new item"
        },
        {
            "name": "Search",
            "url": "/search",
            "icons": [{"src": "/search-icon.png", "sizes": "96x96"}]
        }
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.shortcuts.len(), 2);
    assert_eq!(result.shortcuts[0].name, "New Item");
    assert_eq!(result.shortcuts[0].url, "https://example.com/new");
    assert_eq!(result.shortcuts[0].description, Some("Create a new item".to_string()));
    assert_eq!(result.shortcuts[1].icons.len(), 1);
    assert_eq!(result.shortcuts[1].icons[0].src, "https://example.com/search-icon.png");
}

#[test]
fn test_parse_manifest_related_applications() {
    let json = r##"
{
    "name": "Test",
    "related_applications": [
        {
            "platform": "play",
            "url": "https://play.google.com/store/apps/details?id=com.example",
            "id": "com.example"
        },
        {
            "platform": "itunes",
            "url": "https://apps.apple.com/app/id123456789"
        }
    ],
    "prefer_related_applications": true
}
"##;
    let result = parse_manifest(json, None).unwrap();
    assert_eq!(result.related_applications.len(), 2);
    assert_eq!(result.related_applications[0].platform, "play");
    assert!(result.related_applications[0].url.is_some());
    assert_eq!(result.related_applications[0].id, Some("com.example".to_string()));
    assert_eq!(result.related_applications[1].platform, "itunes");
    assert_eq!(result.prefer_related_applications, Some(true));
}

#[test]
fn test_parse_manifest_screenshots() {
    let json = r##"
{
    "name": "Test",
    "screenshots": [
        {
            "src": "/screenshot1.png",
            "sizes": "540x720",
            "type": "image/png",
            "label": "Home screen"
        },
        {
            "src": "/screenshot2.png",
            "sizes": "540x720",
            "type": "image/png"
        }
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.screenshots.len(), 2);
    assert_eq!(result.screenshots[0].src, "https://example.com/screenshot1.png");
    assert_eq!(result.screenshots[0].label, Some("Home screen".to_string()));
    assert_eq!(result.screenshots[1].src, "https://example.com/screenshot2.png");
}

#[test]
fn test_parse_manifest_categories() {
    let json = r##"
{
    "name": "Test",
    "categories": ["productivity", "utilities", "lifestyle"]
}
"##;
    let result = parse_manifest(json, None).unwrap();
    assert_eq!(result.categories.len(), 3);
    assert!(result.categories.contains(&"productivity".to_string()));
    assert!(result.categories.contains(&"utilities".to_string()));
}

#[test]
fn test_parse_manifest_orientation_and_dir() {
    let json = r##"
{
    "name": "Test",
    "orientation": "portrait",
    "dir": "rtl",
    "lang": "ar"
}
"##;
    let result = parse_manifest(json, None).unwrap();
    assert_eq!(result.orientation, Some("portrait".to_string()));
    assert_eq!(result.dir, Some("rtl".to_string()));
    assert_eq!(result.lang, Some("ar".to_string()));
}

#[test]
fn test_parse_manifest_id_field() {
    let json = r##"
{
    "name": "Test",
    "id": "com.example.app"
}
"##;
    let result = parse_manifest(json, None).unwrap();
    assert_eq!(result.id, Some("com.example.app".to_string()));
}

// URL resolution tests

#[test]
fn test_resolve_start_url() {
    let json = r#"{"name": "Test", "start_url": "/app/"}"#;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.start_url, Some("https://example.com/app/".to_string()));
}

#[test]
fn test_resolve_scope() {
    let json = r#"{"name": "Test", "scope": "/app/"}"#;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.scope, Some("https://example.com/app/".to_string()));
}

#[test]
fn test_resolve_icon_urls() {
    let json = r##"
{
    "name": "Test",
    "icons": [
        {"src": "/icon.png", "sizes": "192x192"},
        {"src": "https://cdn.example.com/icon.png", "sizes": "512x512"}
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.icons[0].src, "https://example.com/icon.png");
    // Absolute URL should remain unchanged
    assert_eq!(result.icons[1].src, "https://cdn.example.com/icon.png");
}

#[test]
fn test_resolve_screenshot_urls() {
    let json = r##"
{
    "name": "Test",
    "screenshots": [
        {"src": "/screens/home.png", "sizes": "540x720"}
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.screenshots[0].src, "https://example.com/screens/home.png");
}

#[test]
fn test_resolve_shortcut_urls() {
    let json = r##"
{
    "name": "Test",
    "shortcuts": [
        {
            "name": "New",
            "url": "/new",
            "icons": [{"src": "/icons/new.png"}]
        }
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.shortcuts[0].url, "https://example.com/new");
    assert_eq!(result.shortcuts[0].icons[0].src, "https://example.com/icons/new.png");
}

// Error handling tests

#[test]
fn test_parse_manifest_invalid_json() {
    let json = r#"{"name": "Test", invalid}"#;
    let result = parse_manifest(json, None);
    assert!(result.is_err());
}

#[test]
fn test_parse_manifest_malformed_json() {
    let json = r#"not json at all"#;
    let result = parse_manifest(json, None);
    assert!(result.is_err());
}

#[test]
fn test_parse_manifest_extra_fields() {
    // JSON with unknown fields should be parsed successfully (ignored)
    let json = r##"
{
    "name": "Test",
    "unknown_field": "value",
    "another_unknown": 123
}
"##;
    let result = parse_manifest(json, None);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, Some("Test".to_string()));
}

// Real-world example tests

#[test]
fn test_real_world_pwa_manifest() {
    let json = r##"
{
    "name": "Twitter Progressive Web App",
    "short_name": "Twitter",
    "description": "It's what's happening",
    "start_url": "/",
    "display": "standalone",
    "theme_color": "#1DA1F2",
    "background_color": "#FFFFFF",
    "icons": [
        {
            "src": "/icons/icon-192x192.png",
            "sizes": "192x192",
            "type": "image/png",
            "purpose": "any maskable"
        },
        {
            "src": "/icons/icon-512x512.png",
            "sizes": "512x512",
            "type": "image/png"
        }
    ]
}
"##;
    let result = parse_manifest(json, Some("https://twitter.com")).unwrap();
    assert_eq!(result.name, Some("Twitter Progressive Web App".to_string()));
    assert_eq!(result.short_name, Some("Twitter".to_string()));
    assert_eq!(result.display, Some("standalone".to_string()));
    assert_eq!(result.icons.len(), 2);
}

#[test]
fn test_real_world_minimal_manifest() {
    let json = r##"
{
    "name": "Simple App",
    "icons": [
        {"src": "/icon.png", "sizes": "512x512", "type": "image/png"}
    ]
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.name, Some("Simple App".to_string()));
    assert_eq!(result.icons.len(), 1);
}

// Integration tests

#[test]
fn test_extract_full_workflow() {
    let html = r#"<link rel="manifest" href="/manifest.json">"#;
    let discovery = extract(html, Some("https://example.com")).unwrap();
    assert_eq!(discovery.href, Some("https://example.com/manifest.json".to_string()));
}

#[test]
fn test_manifest_with_all_fields() {
    let json = r##"
{
    "name": "Complete PWA",
    "short_name": "PWA",
    "description": "A complete progressive web app",
    "start_url": "/app/",
    "display": "fullscreen",
    "orientation": "landscape",
    "theme_color": "#000000",
    "background_color": "#FFFFFF",
    "scope": "/app/",
    "lang": "en-US",
    "dir": "ltr",
    "id": "com.example.pwa",
    "icons": [{"src": "/icon.png", "sizes": "512x512"}],
    "screenshots": [{"src": "/screen.png", "sizes": "540x720"}],
    "shortcuts": [{"name": "Home", "url": "/"}],
    "categories": ["productivity"],
    "related_applications": [{"platform": "play", "id": "com.example"}],
    "prefer_related_applications": false
}
"##;
    let result = parse_manifest(json, Some("https://example.com")).unwrap();
    assert_eq!(result.name, Some("Complete PWA".to_string()));
    assert_eq!(result.short_name, Some("PWA".to_string()));
    assert_eq!(result.display, Some("fullscreen".to_string()));
    assert_eq!(result.orientation, Some("landscape".to_string()));
    assert_eq!(result.lang, Some("en-US".to_string()));
    assert_eq!(result.dir, Some("ltr".to_string()));
    assert_eq!(result.id, Some("com.example.pwa".to_string()));
    assert!(!result.icons.is_empty());
    assert!(!result.screenshots.is_empty());
    assert!(!result.shortcuts.is_empty());
    assert!(!result.categories.is_empty());
    assert!(!result.related_applications.is_empty());
    assert_eq!(result.prefer_related_applications, Some(false));
}
