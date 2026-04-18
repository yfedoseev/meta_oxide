//! Mobile deep-linking metadata extractor.
//!
//! See [`crate::types::app_links`] for the output shape.

use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::types::app_links::{AppLinkPlatform, AppLinks, AppleItunesApp};
use scraper::Html;

#[cfg(test)]
mod tests;

/// Extract App Links / Smart App Banner metadata from an HTML string.
pub fn extract(html: &str, base_url: Option<&str>) -> Result<AppLinks> {
    extract_from_dom(&html_utils::parse_html(html), base_url)
}

/// Extract App Links from an already-parsed DOM.
pub fn extract_from_dom(document: &Html, _base_url: Option<&str>) -> Result<AppLinks> {
    let mut links = AppLinks::default();

    let Ok(selector) = html_utils::create_selector("meta[name][content], meta[property][content]")
    else {
        return Ok(links);
    };

    for element in document.select(&selector) {
        let name = html_utils::get_attr(&element, "name")
            .or_else(|| html_utils::get_attr(&element, "property"));
        let Some(name) = name else {
            continue;
        };
        let Some(content) = html_utils::get_attr(&element, "content") else {
            continue;
        };
        let content = content.trim().to_string();
        if content.is_empty() {
            continue;
        }

        if let Some(rest) = name.strip_prefix("al:") {
            apply_al_tag(&mut links, rest, &content);
            continue;
        }

        match name.as_str() {
            "apple-itunes-app" => {
                links.apple_itunes_app = Some(parse_apple_itunes(&content));
            }
            "google-play-app" => {
                let mut gp = links.google_play_app.clone().unwrap_or_default();
                gp.app_id = Some(parse_kv(&content, "app-id").unwrap_or(content.clone()));
                links.google_play_app = Some(gp);
            }
            "android-app-intent" => {
                let mut gp = links.google_play_app.clone().unwrap_or_default();
                gp.intent = Some(content);
                links.google_play_app = Some(gp);
            }
            _ => {}
        }
    }

    Ok(links)
}

fn apply_al_tag(links: &mut AppLinks, suffix: &str, content: &str) {
    // Suffix shapes:
    //   ios:url           → platform.url
    //   ios:app_name      → platform.app_name
    //   ios:app_store_id  → platform.app_store_id
    //   android:package   → platform.package
    //   android:class     → platform.class
    //   web:url           → links.web_url
    //   web:should_fallback → links.web_should_fallback
    //   windows_phone:url → windows.url  (plus `windows:*`, `windows_universal:*`)
    let Some((platform, key)) = suffix.split_once(':') else {
        return;
    };

    // Web is not really a platform — handle separately.
    if platform == "web" {
        match key {
            "url" => links.web_url = Some(content.to_string()),
            "should_fallback" => {
                links.web_should_fallback =
                    Some(matches!(content.to_ascii_lowercase().as_str(), "true" | "1" | "yes"));
            }
            _ => {}
        }
        return;
    }

    let slot: &mut Option<AppLinkPlatform> = match platform {
        "ios" => &mut links.ios,
        "ipad" => &mut links.ipad,
        "iphone" => &mut links.iphone,
        "android" => &mut links.android,
        "windows" | "windows_phone" | "windows_universal" => &mut links.windows,
        _ => return,
    };
    let p = slot.get_or_insert_with(AppLinkPlatform::default);
    match key {
        "url" => p.url = Some(content.to_string()),
        "app_name" => p.app_name = Some(content.to_string()),
        "app_store_id" => p.app_store_id = Some(content.to_string()),
        "package" => p.package = Some(content.to_string()),
        "class" => p.class = Some(content.to_string()),
        _ => {}
    }
}

fn parse_apple_itunes(content: &str) -> AppleItunesApp {
    let mut out = AppleItunesApp::default();
    for part in content.split(',') {
        let part = part.trim();
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        let value = value.trim().to_string();
        match key.trim() {
            "app-id" => out.app_id = Some(value),
            "app-argument" => out.app_argument = Some(value),
            "affiliate-data" => out.affiliate_data = Some(value),
            _ => {}
        }
    }
    out
}

fn parse_kv(content: &str, key: &str) -> Option<String> {
    content.split(',').find_map(|part| {
        let (k, v) = part.trim().split_once('=')?;
        (k.trim() == key).then(|| v.trim().to_string())
    })
}
