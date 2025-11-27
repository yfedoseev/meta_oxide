//! Web App Manifest extractor
//!
//! Extracts and parses Web App Manifest files for Progressive Web Apps (PWAs).
//! Handles link discovery and JSON parsing with URL resolution.

use crate::errors::{MicroformatError, Result};
use crate::extractors::common::{html_utils, url_utils};
use crate::types::manifest::{ManifestDiscovery, WebAppManifest};

#[cfg(test)]
mod tests;

/// Extract manifest link from HTML
///
/// Finds `<link rel="manifest" href="...">` and resolves the URL
///
/// # Arguments
/// * `html` - The HTML content to extract from
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<ManifestDiscovery>` - Discovery result with href or error
///
/// # Example
/// ```rust
/// use meta_oxide::extractors::manifest;
///
/// let html = r#"<link rel="manifest" href="/manifest.json">"#;
/// let discovery = manifest::extract_link(html, Some("https://example.com")).unwrap();
/// assert_eq!(discovery.href, Some("https://example.com/manifest.json".to_string()));
/// ```
pub fn extract_link(html: &str, base_url: Option<&str>) -> Result<ManifestDiscovery> {
    let doc = html_utils::parse_html(html);

    // Find <link rel="manifest" href="...">
    let selector = html_utils::create_selector("link[rel=manifest][href]")?;

    if let Some(link) = doc.select(&selector).next() {
        if let Some(href) = html_utils::get_attr(&link, "href") {
            // Resolve URL if base_url is provided
            let resolved = if let Some(base) = base_url {
                url_utils::resolve_url(Some(base), &href).map_err(MicroformatError::InvalidUrl)?
            } else {
                href
            };

            return Ok(ManifestDiscovery { href: Some(resolved), manifest: None });
        }
    }

    // No manifest link found
    Ok(ManifestDiscovery::default())
}

/// Parse manifest JSON content
///
/// Parses the JSON manifest and resolves all relative URLs
///
/// # Arguments
/// * `json` - The manifest JSON content
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<WebAppManifest>` - Parsed manifest or error
///
/// # Example
/// ```rust
/// use meta_oxide::extractors::manifest;
///
/// let json = r#"{"name": "My App", "start_url": "/"}"#;
/// let manifest = manifest::parse_manifest(json, Some("https://example.com")).unwrap();
/// assert_eq!(manifest.name, Some("My App".to_string()));
/// assert_eq!(manifest.start_url, Some("https://example.com/".to_string()));
/// ```
pub fn parse_manifest(json: &str, base_url: Option<&str>) -> Result<WebAppManifest> {
    let mut manifest: WebAppManifest = serde_json::from_str(json)
        .map_err(|e| MicroformatError::ParseError(format!("Invalid manifest JSON: {}", e)))?;

    // Resolve relative URLs in the manifest
    if let Some(base) = base_url {
        // Resolve start_url
        if let Some(ref start_url) = manifest.start_url {
            if let Ok(resolved) = url_utils::resolve_url(Some(base), start_url) {
                manifest.start_url = Some(resolved);
            }
        }

        // Resolve scope
        if let Some(ref scope) = manifest.scope {
            if let Ok(resolved) = url_utils::resolve_url(Some(base), scope) {
                manifest.scope = Some(resolved);
            }
        }

        // Resolve icon URLs
        for icon in &mut manifest.icons {
            if let Ok(resolved) = url_utils::resolve_url(Some(base), &icon.src) {
                icon.src = resolved;
            }
        }

        // Resolve screenshot URLs
        for screenshot in &mut manifest.screenshots {
            if let Ok(resolved) = url_utils::resolve_url(Some(base), &screenshot.src) {
                screenshot.src = resolved;
            }
        }

        // Resolve shortcut URLs and icons
        for shortcut in &mut manifest.shortcuts {
            if let Ok(resolved) = url_utils::resolve_url(Some(base), &shortcut.url) {
                shortcut.url = resolved;
            }
            for icon in &mut shortcut.icons {
                if let Ok(resolved) = url_utils::resolve_url(Some(base), &icon.src) {
                    icon.src = resolved;
                }
            }
        }

        // Resolve related application URLs
        for app in &mut manifest.related_applications {
            if let Some(ref url) = app.url {
                if let Ok(resolved) = url_utils::resolve_url(Some(base), url) {
                    app.url = Some(resolved);
                }
            }
        }
    }

    Ok(manifest)
}

/// Extract manifest link from HTML (alias for extract_link)
///
/// This is the main entry point for manifest extraction from HTML.
/// Use `parse_manifest` separately if you already have the JSON content.
///
/// # Arguments
/// * `html` - The HTML content to extract from
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<ManifestDiscovery>` - Discovery result with href
pub fn extract(html: &str, base_url: Option<&str>) -> Result<ManifestDiscovery> {
    extract_link(html, base_url)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_extract_link_basic() {
        let html = r#"<link rel="manifest" href="/manifest.json">"#;
        let result = extract_link(html, None).unwrap();
        // Without base URL, relative URLs may not resolve
        assert!(result.href.is_some());
    }

    #[test]
    fn test_extract_link_with_base_url() {
        let html = r#"<link rel="manifest" href="/manifest.json">"#;
        let result = extract_link(html, Some("https://example.com")).unwrap();
        assert_eq!(result.href, Some("https://example.com/manifest.json".to_string()));
    }

    #[test]
    fn test_extract_link_absolute_url() {
        let html = r#"<link rel="manifest" href="https://cdn.example.com/manifest.json">"#;
        let result = extract_link(html, Some("https://example.com")).unwrap();
        assert_eq!(result.href, Some("https://cdn.example.com/manifest.json".to_string()));
    }

    #[test]
    fn test_extract_link_no_manifest() {
        let html = r#"<link rel="stylesheet" href="/style.css">"#;
        let result = extract_link(html, None).unwrap();
        assert!(result.href.is_none());
    }

    #[test]
    fn test_extract_link_multiple_links() {
        let html = r#"
            <link rel="stylesheet" href="/style.css">
            <link rel="manifest" href="/manifest.json">
            <link rel="icon" href="/icon.png">
        "#;
        let result = extract_link(html, Some("https://example.com")).unwrap();
        assert_eq!(result.href, Some("https://example.com/manifest.json".to_string()));
    }

    #[test]
    fn test_parse_manifest_minimal() {
        let json = r#"{"name": "My App"}"#;
        let result = parse_manifest(json, None).unwrap();
        assert_eq!(result.name, Some("My App".to_string()));
    }

    #[test]
    fn test_parse_manifest_with_start_url() {
        let json = r#"{"name": "My App", "start_url": "/"}"#;
        let result = parse_manifest(json, Some("https://example.com")).unwrap();
        assert_eq!(result.start_url, Some("https://example.com/".to_string()));
    }

    #[test]
    fn test_parse_manifest_with_icons() {
        let json = r##"
{
    "name": "My App",
    "icons": [
        {"src": "/icon-192.png", "sizes": "192x192", "type": "image/png"},
        {"src": "/icon-512.png", "sizes": "512x512", "type": "image/png"}
    ]
}
"##;
        let result = parse_manifest(json, Some("https://example.com")).unwrap();
        assert_eq!(result.icons.len(), 2);
        assert_eq!(result.icons[0].src, "https://example.com/icon-192.png");
        assert_eq!(result.icons[1].src, "https://example.com/icon-512.png");
    }

    #[test]
    fn test_parse_manifest_invalid_json() {
        let json = r#"{"name": "My App", invalid}"#;
        let result = parse_manifest(json, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_manifest_empty_json() {
        let json = r#"{}"#;
        let result = parse_manifest(json, None).unwrap();
        assert!(result.name.is_none());
    }

    #[test]
    fn test_extract_wrapper() {
        let html = r#"<link rel="manifest" href="/manifest.json">"#;
        let result = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(result.href, Some("https://example.com/manifest.json".to_string()));
    }
}
