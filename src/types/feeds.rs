//! Syndication feed types (RSS 2.0, Atom 1.0, JSON Feed 1.1).
//!
//! Producers emit wildly different shapes, so we normalise into one
//! [`Feed`] with a [`FeedKind`] tag so consumers can tell them apart.
//! Per-kind raw payloads are deliberately *not* retained — callers who
//! need fidelity should re-parse the source bytes.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Which syndication variant produced this feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedKind {
    /// RSS 2.0 (also covers RSS 0.9x / 1.0 when their shapes are compatible).
    Rss,
    /// Atom 1.0 (RFC 4287).
    Atom,
    /// JSON Feed 1.1 (<https://www.jsonfeed.org/version/1.1/>).
    JsonFeed,
}

/// Unified feed representation across RSS / Atom / JSON Feed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    /// Which syndication variant this feed came from.
    pub kind: FeedKind,
    /// Feed title.
    pub title: Option<String>,
    /// Feed subtitle / description.
    pub description: Option<String>,
    /// Website (human-facing) URL.
    pub home_page_url: Option<String>,
    /// Self / feed URL (where the feed document lives).
    pub feed_url: Option<String>,
    /// Language code declared at the feed level.
    pub language: Option<String>,
    /// Icon / logo URL.
    pub icon: Option<String>,
    /// Last-updated timestamp (ISO 8601 or RFC 822 — normalised to raw string).
    pub updated: Option<String>,
    /// Authors declared at the feed level.
    pub authors: Vec<FeedAuthor>,
    /// Items / entries.
    pub items: Vec<FeedItem>,
}

impl Feed {
    /// Create an empty feed with the given kind.
    pub fn empty(kind: FeedKind) -> Self {
        Self {
            kind,
            title: None,
            description: None,
            home_page_url: None,
            feed_url: None,
            language: None,
            icon: None,
            updated: None,
            authors: Vec::new(),
            items: Vec::new(),
        }
    }
}

/// One feed author. Name is most commonly populated; url/email may be empty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedAuthor {
    /// Author display name.
    pub name: Option<String>,
    /// Author homepage URL.
    pub url: Option<String>,
    /// Author email address.
    pub email: Option<String>,
}

/// One feed item / entry / post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedItem {
    /// Stable identifier (`guid` / `id` / JSON Feed `id`).
    pub id: Option<String>,
    /// Item title.
    pub title: Option<String>,
    /// Permalink (RSS `link`, Atom `link[rel=alternate]`, JSON Feed `url`).
    pub url: Option<String>,
    /// Summary / description (short HTML).
    pub summary: Option<String>,
    /// Full content (HTML). Falls back to summary on RSS where `content:encoded`
    /// is absent.
    pub content_html: Option<String>,
    /// Plain-text content if exposed separately (JSON Feed `content_text`).
    pub content_text: Option<String>,
    /// Publish timestamp (raw string, format varies by feed kind).
    pub published: Option<String>,
    /// Updated timestamp.
    pub updated: Option<String>,
    /// Authors for this specific item (falls back to feed-level authors).
    pub authors: Vec<FeedAuthor>,
    /// Category / tag strings.
    pub tags: Vec<String>,
    /// Enclosure URL (podcast episode / attachment).
    pub enclosure_url: Option<String>,
    /// Enclosure MIME type.
    pub enclosure_type: Option<String>,
}

#[cfg(feature = "python")]
impl Feed {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(
            "kind",
            match self.kind {
                FeedKind::Rss => "rss",
                FeedKind::Atom => "atom",
                FeedKind::JsonFeed => "json_feed",
            },
        )
        .ok();
        if let Some(v) = &self.title {
            dict.set_item("title", v).ok();
        }
        if let Some(v) = &self.description {
            dict.set_item("description", v).ok();
        }
        if let Some(v) = &self.home_page_url {
            dict.set_item("home_page_url", v).ok();
        }
        if let Some(v) = &self.feed_url {
            dict.set_item("feed_url", v).ok();
        }
        if let Some(v) = &self.language {
            dict.set_item("language", v).ok();
        }
        if let Some(v) = &self.icon {
            dict.set_item("icon", v).ok();
        }
        if let Some(v) = &self.updated {
            dict.set_item("updated", v).ok();
        }
        if !self.authors.is_empty() {
            let authors: Vec<Py<PyDict>> = self.authors.iter().map(|a| a.to_py_dict(py)).collect();
            dict.set_item("authors", authors).ok();
        }
        if !self.items.is_empty() {
            let items: Vec<Py<PyDict>> = self.items.iter().map(|i| i.to_py_dict(py)).collect();
            dict.set_item("items", items).ok();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl FeedAuthor {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        if let Some(v) = &self.name {
            dict.set_item("name", v).ok();
        }
        if let Some(v) = &self.url {
            dict.set_item("url", v).ok();
        }
        if let Some(v) = &self.email {
            dict.set_item("email", v).ok();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl FeedItem {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        if let Some(v) = &self.id {
            dict.set_item("id", v).ok();
        }
        if let Some(v) = &self.title {
            dict.set_item("title", v).ok();
        }
        if let Some(v) = &self.url {
            dict.set_item("url", v).ok();
        }
        if let Some(v) = &self.summary {
            dict.set_item("summary", v).ok();
        }
        if let Some(v) = &self.content_html {
            dict.set_item("content_html", v).ok();
        }
        if let Some(v) = &self.content_text {
            dict.set_item("content_text", v).ok();
        }
        if let Some(v) = &self.published {
            dict.set_item("published", v).ok();
        }
        if let Some(v) = &self.updated {
            dict.set_item("updated", v).ok();
        }
        if !self.authors.is_empty() {
            let authors: Vec<Py<PyDict>> = self.authors.iter().map(|a| a.to_py_dict(py)).collect();
            dict.set_item("authors", authors).ok();
        }
        if !self.tags.is_empty() {
            dict.set_item("tags", self.tags.clone()).ok();
        }
        if let Some(v) = &self.enclosure_url {
            dict.set_item("enclosure_url", v).ok();
        }
        if let Some(v) = &self.enclosure_type {
            dict.set_item("enclosure_type", v).ok();
        }
        dict.unbind()
    }
}
