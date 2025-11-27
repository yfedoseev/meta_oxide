//! Phase 1: Standard HTML Meta Tags (100% adoption)
//!
//! Extracts basic meta tags that virtually all websites use.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::meta::{AlternateLink, FeedLink, MetaTags, RobotsDirective};

#[cfg(test)]
mod tests;

/// Extract all standard meta tags from HTML
///
/// # Arguments
/// * `html` - The HTML content
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<MetaTags>` - Extracted meta tags or error
pub fn extract(html: &str, base_url: Option<&str>) -> Result<MetaTags> {
    let document = html_utils::parse_html(html);
    let mut meta = MetaTags::default();

    // Extract title
    if let Ok(selector) = html_utils::create_selector("title") {
        meta.title = document.select(&selector).next().and_then(|e| html_utils::extract_text(&e));
    }

    // Extract charset
    if let Ok(selector) = html_utils::create_selector("meta[charset]") {
        meta.charset =
            document.select(&selector).next().and_then(|e| html_utils::get_attr(&e, "charset"));
    }

    // Extract charset from Content-Type
    if meta.charset.is_none() {
        if let Ok(selector) = html_utils::create_selector(r#"meta[http-equiv="Content-Type"]"#) {
            meta.charset = document
                .select(&selector)
                .next()
                .and_then(|e| html_utils::get_attr(&e, "content"))
                .and_then(|content| {
                    // Extract charset from "text/html; charset=UTF-8"
                    content.split("charset=").nth(1).map(|s| s.trim().to_string())
                });
        }
    }

    // Extract language from html tag
    if let Ok(selector) = html_utils::create_selector("html[lang]") {
        meta.language =
            document.select(&selector).next().and_then(|e| html_utils::get_attr(&e, "lang"));
    }

    // Extract meta name tags
    if let Ok(selector) = html_utils::create_selector("meta[name][content]") {
        for element in document.select(&selector) {
            if let (Some(name), Some(content)) =
                (html_utils::get_attr(&element, "name"), html_utils::get_attr(&element, "content"))
            {
                let content = content.trim().to_string();
                if content.is_empty() {
                    continue;
                }

                match name.to_lowercase().as_str() {
                    "description" => meta.description = Some(content),
                    "keywords" => {
                        meta.keywords = Some(
                            content
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect(),
                        );
                    }
                    "author" => meta.author = Some(content),
                    "generator" => meta.generator = Some(content),
                    "viewport" => meta.viewport = Some(content),
                    "theme-color" => meta.theme_color = Some(content),
                    "application-name" => meta.application_name = Some(content),
                    "referrer" => meta.referrer = Some(content),
                    "robots" => meta.robots = Some(RobotsDirective::parse(&content)),
                    "googlebot" => meta.googlebot = Some(RobotsDirective::parse(&content)),
                    // Site verification tags (Phase 6)
                    "google-site-verification" => meta.google_site_verification = Some(content),
                    "google-signin-client_id" => meta.google_signin_client_id = Some(content),
                    "msvalidate.01" => meta.msvalidate_01 = Some(content),
                    "yandex-verification" => meta.yandex_verification = Some(content),
                    "p:domain_verify" => meta.p_domain_verify = Some(content),
                    "facebook-domain-verification" => {
                        meta.facebook_domain_verification = Some(content)
                    }
                    // Analytics tags (Phase 6)
                    "google-analytics" => meta.google_analytics = Some(content),
                    // PWA meta tags (Phase 8)
                    "mobile-web-app-capable" => meta.mobile_web_app_capable = Some(content),
                    // Apple mobile meta tags (Phase 8)
                    "apple-mobile-web-app-capable" => {
                        meta.apple_mobile_web_app_capable = Some(content)
                    }
                    "apple-mobile-web-app-status-bar-style" => {
                        meta.apple_mobile_web_app_status_bar_style = Some(content)
                    }
                    "apple-mobile-web-app-title" => meta.apple_mobile_web_app_title = Some(content),
                    // Mobile App Links (Phase 8)
                    "apple-itunes-app" => meta.apple_itunes_app = Some(content),
                    "google-play-app" => meta.google_play_app = Some(content),
                    "format-detection" => meta.format_detection = Some(content),
                    // Microsoft/Windows meta tags (Phase 8)
                    "msapplication-tilecolor" => meta.msapplication_tile_color = Some(content),
                    "msapplication-tileimage" => meta.msapplication_tile_image = Some(content),
                    "msapplication-config" => meta.msapplication_config = Some(content),
                    _ => {}
                }
            }
        }
    }

    // Extract link tags
    if let Ok(selector) = html_utils::create_selector("link[rel][href]") {
        for element in document.select(&selector) {
            if let (Some(rel), Some(href)) =
                (html_utils::get_attr(&element, "rel"), html_utils::get_attr(&element, "href"))
            {
                let resolved_href = url_utils::resolve_url(base_url, &href).unwrap_or(href.clone());

                match rel.to_lowercase().as_str() {
                    "canonical" => {
                        if meta.canonical.is_none() {
                            meta.canonical = Some(resolved_href);
                        }
                    }
                    "shortlink" => {
                        meta.shortlink = Some(resolved_href);
                    }
                    "icon" => {
                        if meta.icon.is_none() {
                            meta.icon = Some(resolved_href);
                        }
                    }
                    "apple-touch-icon" => {
                        if meta.apple_touch_icon.is_none() {
                            meta.apple_touch_icon = Some(resolved_href);
                        }
                    }
                    "manifest" => {
                        meta.manifest = Some(resolved_href);
                    }
                    "prev" => {
                        meta.prev = Some(resolved_href);
                    }
                    "next" => {
                        meta.next = Some(resolved_href);
                    }
                    "alternate" => {
                        // Check if it's a feed or translation
                        let link_type = html_utils::get_attr(&element, "type");

                        if let Some(ref t) = link_type {
                            if t.contains("rss") || t.contains("atom") {
                                // It's a feed
                                meta.feeds.push(FeedLink {
                                    href: resolved_href,
                                    title: html_utils::get_attr(&element, "title"),
                                    r#type: t.clone(),
                                });
                                continue;
                            }
                        }

                        // It's an alternate link (translation/mobile/etc.)
                        meta.alternate.push(AlternateLink {
                            href: resolved_href,
                            hreflang: html_utils::get_attr(&element, "hreflang"),
                            media: html_utils::get_attr(&element, "media"),
                            r#type: link_type,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    // Extract meta property tags (for Facebook, etc.)
    if let Ok(selector) = html_utils::create_selector("meta[property][content]") {
        for element in document.select(&selector) {
            if let (Some(property), Some(content)) = (
                html_utils::get_attr(&element, "property"),
                html_utils::get_attr(&element, "content"),
            ) {
                let content = content.trim().to_string();
                if content.is_empty() {
                    continue;
                }

                match property.to_lowercase().as_str() {
                    "fb:app_id" => meta.fb_app_id = Some(content),
                    "fb:pages" => meta.fb_pages = Some(content),
                    _ => {}
                }
            }
        }
    }

    Ok(meta)
}
