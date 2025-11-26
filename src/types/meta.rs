//! Types for standard HTML meta tags (Phase 1)

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Standard HTML meta tags extracted from a web page
///
/// These are the foundation tags that virtually 100% of websites use.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MetaTags {
    // Basic meta tags
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub author: Option<String>,
    pub generator: Option<String>,

    // Links
    pub canonical: Option<String>,
    pub alternate: Vec<AlternateLink>,
    pub feeds: Vec<FeedLink>,
    pub shortlink: Option<String>,

    // Additional link types (Phase 8)
    pub icon: Option<String>,             // Favicon
    pub apple_touch_icon: Option<String>, // Apple touch icon for iOS
    pub manifest: Option<String>,         // PWA manifest
    pub prev: Option<String>,             // Previous page (pagination)
    pub next: Option<String>,             // Next page (pagination)

    // Robots directives
    pub robots: Option<RobotsDirective>,
    pub googlebot: Option<RobotsDirective>,

    // Viewport and mobile
    pub viewport: Option<String>,
    pub theme_color: Option<String>,

    // Language and charset
    pub charset: Option<String>,
    pub language: Option<String>,

    // Additional common meta
    pub application_name: Option<String>,
    pub referrer: Option<String>,

    // Site verification (Phase 6)
    pub google_site_verification: Option<String>,
    pub google_signin_client_id: Option<String>,
    pub msvalidate_01: Option<String>, // Bing Webmaster Tools
    pub yandex_verification: Option<String>,
    pub p_domain_verify: Option<String>, // Pinterest
    pub facebook_domain_verification: Option<String>,

    // Analytics (Phase 6)
    pub google_analytics: Option<String>,
    pub fb_app_id: Option<String>,
    pub fb_pages: Option<String>,

    // PWA meta (Phase 8)
    pub mobile_web_app_capable: Option<String>, // "yes" or "no" - Chrome/Android

    // Apple mobile meta (Phase 8)
    pub apple_mobile_web_app_capable: Option<String>, // "yes" or "no"
    pub apple_mobile_web_app_status_bar_style: Option<String>, // "default", "black", "black-translucent"
    pub apple_mobile_web_app_title: Option<String>,

    // Mobile App Links (Phase 8)
    pub apple_itunes_app: Option<String>, // app-id=123456789, affiliate-data=...
    pub google_play_app: Option<String>,  // app-id=com.example.android
    pub format_detection: Option<String>, // telephone=no, email=no, address=no

    // Microsoft/Windows meta (Phase 8)
    pub msapplication_tile_color: Option<String>, // Tile color for Windows 8+ start screen
    pub msapplication_tile_image: Option<String>, // Tile image URL
    pub msapplication_config: Option<String>,     // browserconfig.xml URL
}

/// Alternate link (for translations, mobile versions, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternateLink {
    pub href: String,
    pub hreflang: Option<String>,
    pub media: Option<String>,
    pub r#type: Option<String>,
}

/// Feed link (RSS, Atom)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedLink {
    pub href: String,
    pub title: Option<String>,
    pub r#type: String, // "application/rss+xml" or "application/atom+xml"
}

/// Robots directive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RobotsDirective {
    pub index: Option<bool>,      // true = index, false = noindex
    pub follow: Option<bool>,     // true = follow, false = nofollow
    pub archive: Option<bool>,    // true = archive, false = noarchive
    pub snippet: Option<bool>,    // true = snippet, false = nosnippet
    pub translate: Option<bool>,  // true = translate, false = notranslate
    pub imageindex: Option<bool>, // true = imageindex, false = noimageindex
    pub raw: String,              // Original content attribute value
}

impl RobotsDirective {
    /// Parse robots meta content into structured directive
    pub fn parse(content: &str) -> Self {
        let content_lower = content.to_lowercase();
        let directives: Vec<&str> = content_lower.split(',').map(|s| s.trim()).collect();

        let mut directive = RobotsDirective { raw: content.to_string(), ..Default::default() };

        for d in directives {
            match d {
                "index" => directive.index = Some(true),
                "noindex" => directive.index = Some(false),
                "follow" => directive.follow = Some(true),
                "nofollow" => directive.follow = Some(false),
                "archive" => directive.archive = Some(true),
                "noarchive" => directive.archive = Some(false),
                "snippet" => directive.snippet = Some(true),
                "nosnippet" => directive.snippet = Some(false),
                "translate" => directive.translate = Some(true),
                "notranslate" => directive.translate = Some(false),
                "imageindex" => directive.imageindex = Some(true),
                "noimageindex" => directive.imageindex = Some(false),
                "all" => {
                    directive.index = Some(true);
                    directive.follow = Some(true);
                }
                "none" => {
                    directive.index = Some(false);
                    directive.follow = Some(false);
                }
                _ => {} // Ignore unknown directives
            }
        }

        directive
    }
}

// Python conversion implementations
#[cfg(feature = "python")]
impl MetaTags {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(ref v) = self.title {
            dict.set_item("title", v).unwrap();
        }
        if let Some(ref v) = self.description {
            dict.set_item("description", v).unwrap();
        }
        if let Some(ref v) = self.keywords {
            dict.set_item("keywords", v.clone()).unwrap();
        }
        if let Some(ref v) = self.author {
            dict.set_item("author", v).unwrap();
        }
        if let Some(ref v) = self.canonical {
            dict.set_item("canonical", v).unwrap();
        }
        if let Some(ref v) = self.viewport {
            dict.set_item("viewport", v).unwrap();
        }
        if let Some(ref v) = self.charset {
            dict.set_item("charset", v).unwrap();
        }
        if let Some(ref v) = self.language {
            dict.set_item("language", v).unwrap();
        }
        if let Some(ref v) = self.theme_color {
            dict.set_item("theme_color", v).unwrap();
        }
        if let Some(ref v) = self.generator {
            dict.set_item("generator", v).unwrap();
        }
        if let Some(ref v) = self.application_name {
            dict.set_item("application_name", v).unwrap();
        }
        if let Some(ref v) = self.referrer {
            dict.set_item("referrer", v).unwrap();
        }
        if let Some(ref v) = self.shortlink {
            dict.set_item("shortlink", v).unwrap();
        }

        // Additional link types
        if let Some(ref v) = self.icon {
            dict.set_item("icon", v).unwrap();
        }
        if let Some(ref v) = self.apple_touch_icon {
            dict.set_item("apple_touch_icon", v).unwrap();
        }
        if let Some(ref v) = self.manifest {
            dict.set_item("manifest", v).unwrap();
        }
        if let Some(ref v) = self.prev {
            dict.set_item("prev", v).unwrap();
        }
        if let Some(ref v) = self.next {
            dict.set_item("next", v).unwrap();
        }

        // Site verification
        if let Some(ref v) = self.google_site_verification {
            dict.set_item("google_site_verification", v).unwrap();
        }
        if let Some(ref v) = self.google_signin_client_id {
            dict.set_item("google_signin_client_id", v).unwrap();
        }
        if let Some(ref v) = self.msvalidate_01 {
            dict.set_item("msvalidate_01", v).unwrap();
        }
        if let Some(ref v) = self.yandex_verification {
            dict.set_item("yandex_verification", v).unwrap();
        }
        if let Some(ref v) = self.p_domain_verify {
            dict.set_item("p_domain_verify", v).unwrap();
        }
        if let Some(ref v) = self.facebook_domain_verification {
            dict.set_item("facebook_domain_verification", v).unwrap();
        }

        // Analytics
        if let Some(ref v) = self.google_analytics {
            dict.set_item("google_analytics", v).unwrap();
        }
        if let Some(ref v) = self.fb_app_id {
            dict.set_item("fb_app_id", v).unwrap();
        }
        if let Some(ref v) = self.fb_pages {
            dict.set_item("fb_pages", v).unwrap();
        }

        // PWA
        if let Some(ref v) = self.mobile_web_app_capable {
            dict.set_item("mobile_web_app_capable", v).unwrap();
        }

        // Apple mobile
        if let Some(ref v) = self.apple_mobile_web_app_capable {
            dict.set_item("apple_mobile_web_app_capable", v).unwrap();
        }
        if let Some(ref v) = self.apple_mobile_web_app_status_bar_style {
            dict.set_item("apple_mobile_web_app_status_bar_style", v).unwrap();
        }
        if let Some(ref v) = self.apple_mobile_web_app_title {
            dict.set_item("apple_mobile_web_app_title", v).unwrap();
        }

        // Mobile App Links
        if let Some(ref v) = self.apple_itunes_app {
            dict.set_item("apple_itunes_app", v).unwrap();
        }
        if let Some(ref v) = self.google_play_app {
            dict.set_item("google_play_app", v).unwrap();
        }
        if let Some(ref v) = self.format_detection {
            dict.set_item("format_detection", v).unwrap();
        }

        // Microsoft/Windows
        if let Some(ref v) = self.msapplication_tile_color {
            dict.set_item("msapplication_tile_color", v).unwrap();
        }
        if let Some(ref v) = self.msapplication_tile_image {
            dict.set_item("msapplication_tile_image", v).unwrap();
        }
        if let Some(ref v) = self.msapplication_config {
            dict.set_item("msapplication_config", v).unwrap();
        }

        // Complex types as dictionaries
        if let Some(ref robots) = self.robots {
            dict.set_item("robots", robots.to_py_dict(py)).unwrap();
        }
        if let Some(ref googlebot) = self.googlebot {
            dict.set_item("googlebot", googlebot.to_py_dict(py)).unwrap();
        }

        // Lists
        if !self.alternate.is_empty() {
            let alternates: Vec<_> = self.alternate.iter().map(|a| a.to_py_dict(py)).collect();
            dict.set_item("alternate", alternates).unwrap();
        }
        if !self.feeds.is_empty() {
            let feeds: Vec<_> = self.feeds.iter().map(|f| f.to_py_dict(py)).collect();
            dict.set_item("feeds", feeds).unwrap();
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl RobotsDirective {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        dict.set_item("raw", &self.raw).unwrap();
        if let Some(v) = self.index {
            dict.set_item("index", v).unwrap();
        }
        if let Some(v) = self.follow {
            dict.set_item("follow", v).unwrap();
        }
        if let Some(v) = self.archive {
            dict.set_item("archive", v).unwrap();
        }
        if let Some(v) = self.snippet {
            dict.set_item("snippet", v).unwrap();
        }
        if let Some(v) = self.translate {
            dict.set_item("translate", v).unwrap();
        }
        if let Some(v) = self.imageindex {
            dict.set_item("imageindex", v).unwrap();
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl AlternateLink {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("href", &self.href).unwrap();
        if let Some(ref v) = self.hreflang {
            dict.set_item("hreflang", v).unwrap();
        }
        if let Some(ref v) = self.media {
            dict.set_item("media", v).unwrap();
        }
        if let Some(ref v) = self.r#type {
            dict.set_item("type", v).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl FeedLink {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("href", &self.href).unwrap();
        dict.set_item("type", &self.r#type).unwrap();
        if let Some(ref v) = self.title {
            dict.set_item("title", v).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robots_directive_parse_index() {
        let directive = RobotsDirective::parse("index, follow");
        assert_eq!(directive.index, Some(true));
        assert_eq!(directive.follow, Some(true));
    }

    #[test]
    fn test_robots_directive_parse_noindex() {
        let directive = RobotsDirective::parse("noindex, nofollow");
        assert_eq!(directive.index, Some(false));
        assert_eq!(directive.follow, Some(false));
    }

    #[test]
    fn test_robots_directive_parse_all() {
        let directive = RobotsDirective::parse("all");
        assert_eq!(directive.index, Some(true));
        assert_eq!(directive.follow, Some(true));
    }

    #[test]
    fn test_robots_directive_parse_none() {
        let directive = RobotsDirective::parse("none");
        assert_eq!(directive.index, Some(false));
        assert_eq!(directive.follow, Some(false));
    }

    #[test]
    fn test_robots_directive_parse_mixed() {
        let directive = RobotsDirective::parse("noindex, follow, nosnippet");
        assert_eq!(directive.index, Some(false));
        assert_eq!(directive.follow, Some(true));
        assert_eq!(directive.snippet, Some(false));
    }
}
