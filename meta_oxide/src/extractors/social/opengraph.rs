//! Open Graph Protocol extractor
//!
//! Extracts Open Graph metadata used by Facebook, LinkedIn, WhatsApp, Slack, Discord.
//! Specification: https://ogp.me/

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::social::{OgArticle, OgAudio, OgBook, OgImage, OgProfile, OgVideo, OpenGraph};

/// Extract Open Graph metadata from HTML
///
/// # Arguments
/// * `html` - HTML content to parse
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<OpenGraph>` - Extracted Open Graph data
pub fn extract(html: &str, base_url: Option<&str>) -> Result<OpenGraph> {
    let document = html_utils::parse_html(html);
    let mut og = OpenGraph::default();

    // Track current image/video/audio for structured properties
    let mut current_image: Option<OgImage> = None;
    let mut current_video: Option<OgVideo> = None;
    let mut current_audio: Option<OgAudio> = None;

    // Article metadata accumulator
    let mut article_data = OgArticle::default();
    let mut has_article_data = false;

    // Book metadata accumulator
    let mut book_data = OgBook::default();
    let mut has_book_data = false;

    // Profile metadata accumulator
    let mut profile_data = OgProfile::default();
    let mut has_profile_data = false;

    // Extract meta tags with property="og:*" or property="article:*" etc.
    if let Ok(selector) = html_utils::create_selector("meta[property]") {
        for element in document.select(&selector) {
            if let (Some(property), Some(content)) = (
                html_utils::get_attr(&element, "property"),
                html_utils::get_attr(&element, "content"),
            ) {
                let content = content.trim().to_string();
                if content.is_empty() {
                    continue;
                }

                // Parse property name
                if let Some(prop) = property.strip_prefix("og:") {
                    match prop {
                        "title" => og.title = Some(content),
                        "type" => og.r#type = Some(content),
                        "url" => {
                            og.url =
                                Some(url_utils::resolve_url(base_url, &content).unwrap_or(content))
                        }
                        "image" => {
                            // Save previous image if exists
                            if let Some(img) = current_image.take() {
                                og.images.push(img);
                            }

                            let resolved_url = url_utils::resolve_url(base_url, &content)
                                .unwrap_or(content.clone());

                            // First image becomes the primary image
                            if og.image.is_none() {
                                og.image = Some(resolved_url.clone());
                            }

                            // Start new image
                            current_image =
                                Some(OgImage { url: resolved_url, ..Default::default() });
                        }
                        "description" => og.description = Some(content),
                        "site_name" => og.site_name = Some(content),
                        "locale" => og.locale = Some(content),

                        // Handle nested properties
                        _ if prop.starts_with("image:") => {
                            if let Some(ref mut img) = current_image {
                                match &prop[6..] {
                                    "secure_url" => img.secure_url = Some(content),
                                    "type" => img.r#type = Some(content),
                                    "width" => img.width = content.parse().ok(),
                                    "height" => img.height = content.parse().ok(),
                                    "alt" => img.alt = Some(content),
                                    _ => {}
                                }
                            }
                        }
                        _ if prop.starts_with("video:") => match &prop[6..] {
                            "secure_url" => {
                                if let Some(ref mut video) = current_video {
                                    video.secure_url = Some(content);
                                }
                            }
                            "type" => {
                                if let Some(ref mut video) = current_video {
                                    video.r#type = Some(content);
                                }
                            }
                            "width" => {
                                if let Some(ref mut video) = current_video {
                                    video.width = content.parse().ok();
                                }
                            }
                            "height" => {
                                if let Some(ref mut video) = current_video {
                                    video.height = content.parse().ok();
                                }
                            }
                            _ => {}
                        },
                        _ if prop.starts_with("audio:") => match &prop[6..] {
                            "secure_url" => {
                                if let Some(ref mut audio) = current_audio {
                                    audio.secure_url = Some(content);
                                }
                            }
                            "type" => {
                                if let Some(ref mut audio) = current_audio {
                                    audio.r#type = Some(content);
                                }
                            }
                            _ => {}
                        },
                        _ if prop.starts_with("locale:") => {
                            if &prop[7..] == "alternate" {
                                og.locale_alternate.push(content);
                            }
                        }
                        "video" => {
                            // Save previous video if exists
                            if let Some(video) = current_video.take() {
                                og.videos.push(video);
                            }

                            let resolved_url =
                                url_utils::resolve_url(base_url, &content).unwrap_or(content);

                            // Start new video
                            current_video =
                                Some(OgVideo { url: resolved_url, ..Default::default() });
                        }
                        "audio" => {
                            // Save previous audio if exists
                            if let Some(audio) = current_audio.take() {
                                og.audios.push(audio);
                            }

                            let resolved_url =
                                url_utils::resolve_url(base_url, &content).unwrap_or(content);

                            // Start new audio
                            current_audio =
                                Some(OgAudio { url: resolved_url, ..Default::default() });
                        }
                        _ => {}
                    }
                } else if let Some(prop) = property.strip_prefix("article:") {
                    has_article_data = true;
                    match prop {
                        "published_time" => article_data.published_time = Some(content),
                        "modified_time" => article_data.modified_time = Some(content),
                        "expiration_time" => article_data.expiration_time = Some(content),
                        "author" => article_data.author.push(content),
                        "section" => article_data.section = Some(content),
                        "tag" => article_data.tag.push(content),
                        _ => {}
                    }
                } else if let Some(prop) = property.strip_prefix("book:") {
                    has_book_data = true;
                    match prop {
                        "author" => book_data.author.push(content),
                        "isbn" => book_data.isbn = Some(content),
                        "release_date" => book_data.release_date = Some(content),
                        "tag" => book_data.tag.push(content),
                        _ => {}
                    }
                } else if let Some(prop) = property.strip_prefix("profile:") {
                    has_profile_data = true;
                    match prop {
                        "first_name" => profile_data.first_name = Some(content),
                        "last_name" => profile_data.last_name = Some(content),
                        "username" => profile_data.username = Some(content),
                        "gender" => profile_data.gender = Some(content),
                        _ => {}
                    }
                } else if let Some(prop) = property.strip_prefix("fb:") {
                    // Phase 6: Facebook platform integration
                    match prop {
                        "app_id" => og.fb_app_id = Some(content),
                        "admins" => og.fb_admins = Some(content),
                        _ => {}
                    }
                }
            }
        }
    }

    // Save final image/video/audio if exists
    if let Some(img) = current_image {
        og.images.push(img);
    }
    if let Some(video) = current_video {
        og.videos.push(video);
    }
    if let Some(audio) = current_audio {
        og.audios.push(audio);
    }

    // Set type-specific metadata if any was found
    if has_article_data {
        og.article = Some(article_data);
    }
    if has_book_data {
        og.book = Some(book_data);
    }
    if has_profile_data {
        og.profile = Some(profile_data);
    }

    Ok(og)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic() {
        let html = r#"<meta property="og:title" content="Test">"#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, Some("Test".to_string()));
    }

    #[test]
    fn test_extract_empty() {
        let html = "";
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, None);
    }

    // Phase 6: Facebook Platform Tests
    #[test]
    fn test_facebook_app_id() {
        let html = r#"<meta property="fb:app_id" content="123456789">"#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.fb_app_id, Some("123456789".to_string()));
    }

    #[test]
    fn test_facebook_admins() {
        let html = r#"<meta property="fb:admins" content="user1,user2,user3">"#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.fb_admins, Some("user1,user2,user3".to_string()));
    }

    #[test]
    fn test_facebook_platform_with_og() {
        let html = r#"
            <meta property="og:title" content="Test Page">
            <meta property="fb:app_id" content="987654321">
            <meta property="fb:admins" content="admin1,admin2">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, Some("Test Page".to_string()));
        assert_eq!(og.fb_app_id, Some("987654321".to_string()));
        assert_eq!(og.fb_admins, Some("admin1,admin2".to_string()));
    }
}
