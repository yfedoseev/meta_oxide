//! Types for social media tags (Phase 2)
//!
//! This module defines types for Open Graph Protocol and Twitter Cards,
//! which control how links appear when shared on social media platforms.
//!
//! - **Open Graph**: Used by Facebook, LinkedIn, WhatsApp, Slack, Discord (60%+ adoption)
//! - **Twitter Cards**: Used by Twitter/X for link previews (45% adoption)

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Open Graph Protocol data (Facebook, LinkedIn, WhatsApp, Slack, Discord)
///
/// 60%+ of websites use Open Graph to control link preview appearance.
/// Specification: https://ogp.me/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenGraph {
    // Basic metadata (required by spec)
    /// The title of the object as it should appear in the graph
    pub title: Option<String>,
    /// The type of object (article, website, video.movie, music.song, etc.)
    pub r#type: Option<String>,
    /// The canonical URL of the object
    pub url: Option<String>,
    /// Primary image URL that should represent the object
    pub image: Option<String>,

    // Optional metadata
    /// A one to two sentence description of the object
    pub description: Option<String>,
    /// The name of the overall site (e.g., "IMDb", "Wikipedia")
    pub site_name: Option<String>,
    /// The locale tag (e.g., "en_US", "es_ES")
    pub locale: Option<String>,
    /// Array of alternate locales this page is available in
    pub locale_alternate: Vec<String>,

    // Image metadata
    /// All images with full metadata (width, height, type, alt text)
    pub images: Vec<OgImage>,

    // Video metadata
    /// Video objects associated with this content
    pub videos: Vec<OgVideo>,

    // Audio metadata
    /// Audio objects associated with this content
    pub audios: Vec<OgAudio>,

    // Type-specific metadata
    /// Article-specific metadata (when type="article")
    pub article: Option<OgArticle>,
    /// Book-specific metadata (when type="book")
    pub book: Option<OgBook>,
    /// Profile-specific metadata (when type="profile")
    pub profile: Option<OgProfile>,

    // Platform integration (Phase 6)
    /// Facebook App ID for platform integration
    pub fb_app_id: Option<String>,
    /// Facebook Admin user IDs (comma-separated)
    pub fb_admins: Option<String>,
}

/// Open Graph Image with full metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgImage {
    /// URL of the image
    pub url: String,
    /// HTTPS version of the image URL
    pub secure_url: Option<String>,
    /// MIME type of the image (e.g., "image/jpeg", "image/png")
    pub r#type: Option<String>,
    /// Width in pixels
    pub width: Option<u32>,
    /// Height in pixels
    pub height: Option<u32>,
    /// Alt text for accessibility
    pub alt: Option<String>,
}

/// Open Graph Video metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgVideo {
    /// URL of the video
    pub url: String,
    /// HTTPS version of the video URL
    pub secure_url: Option<String>,
    /// MIME type of the video (e.g., "video/mp4", "application/x-shockwave-flash")
    pub r#type: Option<String>,
    /// Width in pixels
    pub width: Option<u32>,
    /// Height in pixels
    pub height: Option<u32>,
}

/// Open Graph Audio metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgAudio {
    /// URL of the audio file
    pub url: String,
    /// HTTPS version of the audio URL
    pub secure_url: Option<String>,
    /// MIME type of the audio (e.g., "audio/mpeg", "audio/vorbis")
    pub r#type: Option<String>,
}

/// Article-specific Open Graph metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgArticle {
    /// ISO 8601 datetime when the article was published
    pub published_time: Option<String>,
    /// ISO 8601 datetime when the article was last modified
    pub modified_time: Option<String>,
    /// ISO 8601 datetime when the article will expire
    pub expiration_time: Option<String>,
    /// URLs to author profile pages
    pub author: Vec<String>,
    /// High-level section name (e.g., "Technology", "Sports")
    pub section: Option<String>,
    /// Keywords/tags associated with the article
    pub tag: Vec<String>,
}

/// Book-specific Open Graph metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgBook {
    /// URLs to author profile pages
    pub author: Vec<String>,
    /// ISBN number
    pub isbn: Option<String>,
    /// Date the book was released
    pub release_date: Option<String>,
    /// Keywords/tags associated with the book
    pub tag: Vec<String>,
}

/// Profile-specific Open Graph metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OgProfile {
    /// First name
    pub first_name: Option<String>,
    /// Last name
    pub last_name: Option<String>,
    /// Username
    pub username: Option<String>,
    /// Gender
    pub gender: Option<String>,
}

/// Twitter Card data (45% adoption)
///
/// Twitter Cards control how links appear on Twitter/X.
/// Falls back to Open Graph when Twitter-specific tags are missing.
/// Specification: https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/abouts-cards
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TwitterCard {
    /// Card type: "summary", "summary_large_image", "app", or "player"
    pub card: Option<String>,

    // Content metadata
    /// Title of the content (max 70 characters)
    pub title: Option<String>,
    /// Description of the content (max 200 characters)
    pub description: Option<String>,
    /// URL of the image to display
    pub image: Option<String>,
    /// Alt text for the image
    pub image_alt: Option<String>,

    // Site and creator
    /// @username of the website (e.g., "@nytimes")
    pub site: Option<String>,
    /// Twitter user ID of the website
    pub site_id: Option<String>,
    /// @username of the content creator
    pub creator: Option<String>,
    /// Twitter user ID of the content creator
    pub creator_id: Option<String>,

    // Type-specific metadata
    /// App card specific metadata
    pub app: Option<TwitterApp>,
    /// Player card specific metadata (video/audio)
    pub player: Option<TwitterPlayer>,
}

/// Twitter App card metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TwitterApp {
    // iPhone app
    /// Name of the iPhone app
    pub name_iphone: Option<String>,
    /// App ID in the App Store
    pub id_iphone: Option<String>,
    /// Custom URL scheme for iPhone app
    pub url_iphone: Option<String>,

    // iPad app
    /// Name of the iPad app
    pub name_ipad: Option<String>,
    /// App ID in the App Store
    pub id_ipad: Option<String>,
    /// Custom URL scheme for iPad app
    pub url_ipad: Option<String>,

    // Google Play app
    /// Name of the Android app
    pub name_googleplay: Option<String>,
    /// App ID in Google Play
    pub id_googleplay: Option<String>,
    /// Custom URL scheme for Android app
    pub url_googleplay: Option<String>,

    /// Country code if app is only available in specific countries
    pub country: Option<String>,
}

/// Twitter Player card metadata (video/audio)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TwitterPlayer {
    /// HTTPS URL to iframe player
    pub url: String,
    /// Width of iframe in pixels
    pub width: Option<u32>,
    /// Height of iframe in pixels
    pub height: Option<u32>,
    /// URL to raw video/audio stream (MP4, etc.)
    pub stream: Option<String>,
}

// Python conversion implementations

#[cfg(feature = "python")]
impl OpenGraph {
    /// Convert OpenGraph to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        // Basic metadata
        if let Some(ref v) = self.title {
            let _ = dict.set_item("title", v);
        }
        if let Some(ref v) = self.r#type {
            let _ = dict.set_item("type", v);
        }
        if let Some(ref v) = self.url {
            let _ = dict.set_item("url", v);
        }
        if let Some(ref v) = self.image {
            let _ = dict.set_item("image", v);
        }
        if let Some(ref v) = self.description {
            let _ = dict.set_item("description", v);
        }
        if let Some(ref v) = self.site_name {
            let _ = dict.set_item("site_name", v);
        }
        if let Some(ref v) = self.locale {
            let _ = dict.set_item("locale", v);
        }

        // Lists and complex types
        if !self.locale_alternate.is_empty() {
            let _ = dict.set_item("locale_alternate", self.locale_alternate.clone());
        }
        if !self.images.is_empty() {
            let images: Vec<_> = self.images.iter().map(|img| img.to_py_dict(py)).collect();
            let _ = dict.set_item("images", images);
        }
        if !self.videos.is_empty() {
            let videos: Vec<_> = self.videos.iter().map(|v| v.to_py_dict(py)).collect();
            let _ = dict.set_item("videos", videos);
        }
        if !self.audios.is_empty() {
            let audios: Vec<_> = self.audios.iter().map(|a| a.to_py_dict(py)).collect();
            let _ = dict.set_item("audios", audios);
        }
        if let Some(ref article) = self.article {
            let _ = dict.set_item("article", article.to_py_dict(py));
        }
        if let Some(ref book) = self.book {
            let _ = dict.set_item("book", book.to_py_dict(py));
        }
        if let Some(ref profile) = self.profile {
            let _ = dict.set_item("profile", profile.to_py_dict(py));
        }

        // Platform integration (Phase 6)
        if let Some(ref v) = self.fb_app_id {
            let _ = dict.set_item("fb_app_id", v);
        }
        if let Some(ref v) = self.fb_admins {
            let _ = dict.set_item("fb_admins", v);
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgImage {
    /// Convert OgImage to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        let _ = dict.set_item("url", &self.url);
        if let Some(ref v) = self.secure_url {
            let _ = dict.set_item("secure_url", v);
        }
        if let Some(ref v) = self.r#type {
            let _ = dict.set_item("type", v);
        }
        if let Some(v) = self.width {
            let _ = dict.set_item("width", v);
        }
        if let Some(v) = self.height {
            let _ = dict.set_item("height", v);
        }
        if let Some(ref v) = self.alt {
            let _ = dict.set_item("alt", v);
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgVideo {
    /// Convert OgVideo to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        let _ = dict.set_item("url", &self.url);
        if let Some(ref v) = self.secure_url {
            let _ = dict.set_item("secure_url", v);
        }
        if let Some(ref v) = self.r#type {
            let _ = dict.set_item("type", v);
        }
        if let Some(v) = self.width {
            let _ = dict.set_item("width", v);
        }
        if let Some(v) = self.height {
            let _ = dict.set_item("height", v);
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgAudio {
    /// Convert OgAudio to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        let _ = dict.set_item("url", &self.url);
        if let Some(ref v) = self.secure_url {
            let _ = dict.set_item("secure_url", v);
        }
        if let Some(ref v) = self.r#type {
            let _ = dict.set_item("type", v);
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgArticle {
    /// Convert OgArticle to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(ref v) = self.published_time {
            let _ = dict.set_item("published_time", v);
        }
        if let Some(ref v) = self.modified_time {
            let _ = dict.set_item("modified_time", v);
        }
        if let Some(ref v) = self.expiration_time {
            let _ = dict.set_item("expiration_time", v);
        }
        if !self.author.is_empty() {
            let _ = dict.set_item("author", self.author.clone());
        }
        if let Some(ref v) = self.section {
            let _ = dict.set_item("section", v);
        }
        if !self.tag.is_empty() {
            let _ = dict.set_item("tag", self.tag.clone());
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgBook {
    /// Convert OgBook to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if !self.author.is_empty() {
            let _ = dict.set_item("author", self.author.clone());
        }
        if let Some(ref v) = self.isbn {
            let _ = dict.set_item("isbn", v);
        }
        if let Some(ref v) = self.release_date {
            let _ = dict.set_item("release_date", v);
        }
        if !self.tag.is_empty() {
            let _ = dict.set_item("tag", self.tag.clone());
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OgProfile {
    /// Convert OgProfile to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(ref v) = self.first_name {
            let _ = dict.set_item("first_name", v);
        }
        if let Some(ref v) = self.last_name {
            let _ = dict.set_item("last_name", v);
        }
        if let Some(ref v) = self.username {
            let _ = dict.set_item("username", v);
        }
        if let Some(ref v) = self.gender {
            let _ = dict.set_item("gender", v);
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl TwitterCard {
    /// Convert TwitterCard to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(ref v) = self.card {
            let _ = dict.set_item("card", v);
        }
        if let Some(ref v) = self.title {
            let _ = dict.set_item("title", v);
        }
        if let Some(ref v) = self.description {
            let _ = dict.set_item("description", v);
        }
        if let Some(ref v) = self.image {
            let _ = dict.set_item("image", v);
        }
        if let Some(ref v) = self.image_alt {
            let _ = dict.set_item("image_alt", v);
        }
        if let Some(ref v) = self.site {
            let _ = dict.set_item("site", v);
        }
        if let Some(ref v) = self.site_id {
            let _ = dict.set_item("site_id", v);
        }
        if let Some(ref v) = self.creator {
            let _ = dict.set_item("creator", v);
        }
        if let Some(ref v) = self.creator_id {
            let _ = dict.set_item("creator_id", v);
        }

        // Complex types
        if let Some(ref app) = self.app {
            let _ = dict.set_item("app", app.to_py_dict(py));
        }
        if let Some(ref player) = self.player {
            let _ = dict.set_item("player", player.to_py_dict(py));
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl TwitterApp {
    /// Convert TwitterApp to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(ref v) = self.name_iphone {
            let _ = dict.set_item("name_iphone", v);
        }
        if let Some(ref v) = self.id_iphone {
            let _ = dict.set_item("id_iphone", v);
        }
        if let Some(ref v) = self.url_iphone {
            let _ = dict.set_item("url_iphone", v);
        }
        if let Some(ref v) = self.name_ipad {
            let _ = dict.set_item("name_ipad", v);
        }
        if let Some(ref v) = self.id_ipad {
            let _ = dict.set_item("id_ipad", v);
        }
        if let Some(ref v) = self.url_ipad {
            let _ = dict.set_item("url_ipad", v);
        }
        if let Some(ref v) = self.name_googleplay {
            let _ = dict.set_item("name_googleplay", v);
        }
        if let Some(ref v) = self.id_googleplay {
            let _ = dict.set_item("id_googleplay", v);
        }
        if let Some(ref v) = self.url_googleplay {
            let _ = dict.set_item("url_googleplay", v);
        }
        if let Some(ref v) = self.country {
            let _ = dict.set_item("country", v);
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl TwitterPlayer {
    /// Convert TwitterPlayer to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        let _ = dict.set_item("url", &self.url);
        if let Some(v) = self.width {
            let _ = dict.set_item("width", v);
        }
        if let Some(v) = self.height {
            let _ = dict.set_item("height", v);
        }
        if let Some(ref v) = self.stream {
            let _ = dict.set_item("stream", v);
        }
        dict.unbind()
    }
}
