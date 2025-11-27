//! Twitter Card extractor
//!
//! Extracts Twitter Card metadata used by Twitter/X for link previews.
//! Specification: https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/abouts-cards

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::social::{TwitterApp, TwitterCard, TwitterPlayer};

/// Extract Twitter Card metadata from HTML
///
/// # Arguments
/// * `html` - HTML content to parse
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<TwitterCard>` - Extracted Twitter Card data
pub fn extract(html: &str, base_url: Option<&str>) -> Result<TwitterCard> {
    let document = html_utils::parse_html(html);
    let mut card = TwitterCard::default();

    // Track player/app metadata
    let mut player_url: Option<String> = None;
    let mut player_width: Option<u32> = None;
    let mut player_height: Option<u32> = None;
    let mut player_stream: Option<String> = None;

    let mut app_data = TwitterApp::default();
    let mut has_app_data = false;

    // Extract meta tags with name="twitter:*"
    if let Ok(selector) = html_utils::create_selector("meta[name]") {
        for element in document.select(&selector) {
            if let (Some(name), Some(content)) =
                (html_utils::get_attr(&element, "name"), html_utils::get_attr(&element, "content"))
            {
                let content = content.trim().to_string();
                if content.is_empty() {
                    continue;
                }

                // Parse name attribute
                if let Some(prop) = name.strip_prefix("twitter:") {
                    match prop {
                        "card" => card.card = Some(content),
                        "title" => card.title = Some(content),
                        "description" => card.description = Some(content),
                        "image" => {
                            card.image =
                                Some(url_utils::resolve_url(base_url, &content).unwrap_or(content))
                        }
                        "site" => card.site = Some(content),
                        "creator" => card.creator = Some(content),

                        // Handle nested properties
                        _ if prop.starts_with("image:") => {
                            if &prop[6..] == "alt" {
                                card.image_alt = Some(content);
                            }
                        }
                        _ if prop.starts_with("site:") => {
                            if &prop[5..] == "id" {
                                card.site_id = Some(content);
                            }
                        }
                        _ if prop.starts_with("creator:") => {
                            if &prop[8..] == "id" {
                                card.creator_id = Some(content);
                            }
                        }
                        _ if prop.starts_with("player") => {
                            if prop == "player" {
                                player_url = Some(
                                    url_utils::resolve_url(base_url, &content).unwrap_or(content),
                                );
                            } else if let Some(subprop) = prop.strip_prefix("player:") {
                                match subprop {
                                    "width" => player_width = content.parse().ok(),
                                    "height" => player_height = content.parse().ok(),
                                    "stream" => {
                                        player_stream = Some(
                                            url_utils::resolve_url(base_url, &content)
                                                .unwrap_or(content),
                                        )
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ if prop.starts_with("app:") => {
                            has_app_data = true;
                            let subprop = &prop[4..];

                            if let Some(platform_prop) = subprop.strip_prefix("name:") {
                                match platform_prop {
                                    "iphone" => app_data.name_iphone = Some(content),
                                    "ipad" => app_data.name_ipad = Some(content),
                                    "googleplay" => app_data.name_googleplay = Some(content),
                                    _ => {}
                                }
                            } else if let Some(platform_prop) = subprop.strip_prefix("id:") {
                                match platform_prop {
                                    "iphone" => app_data.id_iphone = Some(content),
                                    "ipad" => app_data.id_ipad = Some(content),
                                    "googleplay" => app_data.id_googleplay = Some(content),
                                    _ => {}
                                }
                            } else if let Some(platform_prop) = subprop.strip_prefix("url:") {
                                match platform_prop {
                                    "iphone" => app_data.url_iphone = Some(content),
                                    "ipad" => app_data.url_ipad = Some(content),
                                    "googleplay" => app_data.url_googleplay = Some(content),
                                    _ => {}
                                }
                            } else if subprop == "country" {
                                app_data.country = Some(content);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Build player object if URL exists
    if let Some(url) = player_url {
        card.player = Some(TwitterPlayer {
            url,
            width: player_width,
            height: player_height,
            stream: player_stream,
        });
    }

    // Set app data if any was found
    if has_app_data {
        card.app = Some(app_data);
    }

    Ok(card)
}

/// Extract Twitter Card with fallback to Open Graph
///
/// Twitter recommends: If twitter:* tags are missing, fall back to og:* tags.
/// This function implements that recommendation.
///
/// # Arguments
/// * `html` - HTML content to parse
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<TwitterCard>` - Extracted Twitter Card data with OG fallback
pub fn extract_with_fallback(html: &str, base_url: Option<&str>) -> Result<TwitterCard> {
    let mut card = extract(html, base_url)?;

    // If critical Twitter fields are missing, try Open Graph
    if card.title.is_none() || card.description.is_none() || card.image.is_none() {
        let og = super::opengraph::extract(html, base_url)?;

        if card.title.is_none() {
            card.title = og.title;
        }
        if card.description.is_none() {
            card.description = og.description;
        }
        if card.image.is_none() {
            card.image = og.image;
        }
    }

    Ok(card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic() {
        let html = r#"<meta name="twitter:card" content="summary">"#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("summary".to_string()));
    }

    #[test]
    fn test_extract_empty() {
        let html = "";
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, None);
    }

    #[test]
    fn test_fallback_to_og() {
        let html = r#"
            <meta property="og:title" content="OG Title">
            <meta property="og:description" content="OG Description">
            <meta property="og:image" content="https://example.com/og-image.jpg">
        "#;
        let card = extract_with_fallback(html, None).unwrap();

        // Should fall back to OG values
        assert_eq!(card.title, Some("OG Title".to_string()));
        assert_eq!(card.description, Some("OG Description".to_string()));
        assert_eq!(card.image, Some("https://example.com/og-image.jpg".to_string()));
    }

    #[test]
    fn test_twitter_takes_precedence() {
        let html = r#"
            <meta name="twitter:title" content="Twitter Title">
            <meta property="og:title" content="OG Title">
        "#;
        let card = extract_with_fallback(html, None).unwrap();

        // Twitter tags should take precedence
        assert_eq!(card.title, Some("Twitter Title".to_string()));
    }

    #[test]
    fn test_partial_fallback() {
        let html = r#"
            <meta name="twitter:title" content="Twitter Title">
            <meta property="og:description" content="OG Description">
            <meta property="og:image" content="https://example.com/og-image.jpg">
        "#;
        let card = extract_with_fallback(html, None).unwrap();

        // Twitter title should be used, but fall back to OG for others
        assert_eq!(card.title, Some("Twitter Title".to_string()));
        assert_eq!(card.description, Some("OG Description".to_string()));
        assert_eq!(card.image, Some("https://example.com/og-image.jpg".to_string()));
    }
}
