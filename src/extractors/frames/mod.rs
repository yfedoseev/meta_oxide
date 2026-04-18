//! Farcaster Frames and Open Frames extractor.
//!
//! Parses `fc:frame:*` (Farcaster) and `of:*` (Open Frames) `<meta>` tags.
//! See [`crate::types::frames`] for the output shape.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::frames::{Frame, FrameButton};
use scraper::Html;

#[cfg(test)]
mod tests;

/// Extract Frame metadata from an HTML string.
pub fn extract(html: &str, base_url: Option<&str>) -> Result<Frame> {
    extract_from_dom(&html_utils::parse_html(html), base_url)
}

/// Extract Frame metadata from an already-parsed DOM.
pub fn extract_from_dom(document: &Html, base_url: Option<&str>) -> Result<Frame> {
    let mut frame = Frame::default();
    // index → partial button state; merged into frame.buttons at the end.
    let mut buttons: std::collections::BTreeMap<u8, FrameButton> =
        std::collections::BTreeMap::new();

    let Ok(selector) = html_utils::create_selector("meta[name][content], meta[property][content]")
    else {
        return Ok(frame);
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

        let (namespace, rest) = if let Some(r) = name.strip_prefix("fc:frame") {
            frame.has_farcaster = true;
            ("fc", r)
        } else if let Some(r) = name.strip_prefix("of:") {
            frame.has_open_frames = true;
            ("of", r)
        } else {
            continue;
        };

        // For `fc:frame` (no suffix) the version lands in `content`.
        // For `of:version` it's a named tag.
        match (namespace, rest) {
            ("fc", "") => frame.version = Some(content),
            ("fc", ":image") | ("of", "image") => frame.image = Some(resolve(base_url, &content)),
            ("fc", ":image:aspect_ratio") | ("of", "image:aspect_ratio") => {
                frame.image_aspect_ratio = Some(content)
            }
            ("fc", ":post_url") | ("of", "post_url") => {
                frame.post_url = Some(resolve(base_url, &content))
            }
            ("fc", ":input:text") | ("of", "input:text") => frame.input_text = Some(content),
            ("fc", ":state") | ("of", "state") => frame.state = Some(content),
            ("of", "version") => frame.version = Some(content),
            ("of", "refresh_period") => frame.refresh_period = content.parse().ok(),
            ("of", rest) if rest.starts_with("accepts:") => frame.accepts.push(content),
            // Buttons: fc:frame:button:N, fc:frame:button:N:action, :target, :post_url
            //          of:button:N, of:button:N:action, …
            ("fc", rest) if rest.starts_with(":button:") => {
                handle_button(&mut buttons, &rest[":button:".len()..], &content, base_url);
            }
            ("of", rest) if rest.starts_with("button:") => {
                handle_button(&mut buttons, &rest["button:".len()..], &content, base_url);
            }
            _ => {}
        }
    }

    frame.buttons = buttons.into_values().collect();
    Ok(frame)
}

fn handle_button(
    buttons: &mut std::collections::BTreeMap<u8, FrameButton>,
    suffix: &str,
    content: &str,
    base_url: Option<&str>,
) {
    // `suffix` is either `N` or `N:action` / `N:target` / `N:post_url`.
    let (index_str, sub) = match suffix.split_once(':') {
        Some((idx, s)) => (idx, Some(s)),
        None => (suffix, None),
    };
    let Ok(index) = index_str.parse::<u8>() else {
        return;
    };
    if !(1..=10).contains(&index) {
        return; // Spec allows 4; we're generous but cap to sane range.
    }
    let entry =
        buttons.entry(index).or_insert_with(|| FrameButton { index, ..FrameButton::default() });
    match sub {
        None => entry.label = Some(content.to_string()),
        Some("action") => entry.action = Some(content.to_string()),
        Some("target") => entry.target = Some(resolve(base_url, content)),
        Some("post_url") => entry.post_url = Some(resolve(base_url, content)),
        _ => {}
    }
}

fn resolve(base_url: Option<&str>, url: &str) -> String {
    url_utils::resolve_url(base_url, url).unwrap_or_else(|_| url.to_string())
}
