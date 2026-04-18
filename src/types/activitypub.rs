//! ActivityPub / Activity Streams 2.0 metadata surface.
//!
//! Fediverse software (Mastodon, Pleroma, Pixelfed, Threads, Lemmy, …)
//! exposes pages in two discoverable ways:
//!
//! - `<link rel="alternate" type="application/activity+json" href="…">`
//!   on HTML pages — the AS2 JSON representation of the same resource.
//! - JSON payloads with `"@context": "https://www.w3.org/ns/activitystreams"`.
//!
//! This module gives us two things: a [`ActivityPubDiscovery`] struct
//! populated from HTML, and a [`ActorSummary`] / [`ObjectSummary`] shape
//! populated by [`crate::extractors::activitypub::parse_as2`] from a
//! linked JSON document.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// HTML-level ActivityPub discovery (links to the JSON representation).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityPubDiscovery {
    /// URL of the `application/activity+json` alternate representation.
    pub alternate_url: Option<String>,
    /// WebFinger / resource URI hint (`<meta name="fediverse:creator">`
    /// or similar emitted by Mastodon-adjacent tooling).
    pub fediverse_creator: Option<String>,
    /// Collected `rel="me"` links — useful for identity verification.
    /// Populated only from direct `rel="me"` `<link>`/`<a>` tags.
    pub rel_me: Vec<String>,
}

impl ActivityPubDiscovery {
    /// True when no ActivityPub signals were found.
    pub fn is_empty(&self) -> bool {
        self.alternate_url.is_none() && self.fediverse_creator.is_none() && self.rel_me.is_empty()
    }
}

/// Summary of an AS2 Actor (Person / Service / Application / Group / Organization).
///
/// Populated by [`crate::extractors::activitypub::parse_as2_actor`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ActorSummary {
    /// `id` — canonical actor URL.
    pub id: Option<String>,
    /// `type` (`Person`, `Service`, …).
    pub kind: Option<String>,
    /// `preferredUsername` (local handle).
    pub preferred_username: Option<String>,
    /// `name` (display name).
    pub name: Option<String>,
    /// `summary` (bio, plain or HTML per server).
    pub summary: Option<String>,
    /// `url` (human-facing profile URL).
    pub url: Option<String>,
    /// `inbox` collection URL.
    pub inbox: Option<String>,
    /// `outbox` collection URL.
    pub outbox: Option<String>,
    /// `followers` collection URL.
    pub followers: Option<String>,
    /// `following` collection URL.
    pub following: Option<String>,
    /// `icon.url` — avatar.
    pub icon: Option<String>,
}

impl ActorSummary {
    /// True when nothing was extracted.
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.name.is_none() && self.preferred_username.is_none()
    }
}

/// Summary of an AS2 Object (Note / Article / Page / Video / …).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectSummary {
    /// `id` — canonical object URL.
    pub id: Option<String>,
    /// `type`.
    pub kind: Option<String>,
    /// `attributedTo` (actor URL or embedded actor).
    pub attributed_to: Option<String>,
    /// `content` (HTML or plain).
    pub content: Option<String>,
    /// `summary`.
    pub summary: Option<String>,
    /// `published` timestamp (ISO 8601).
    pub published: Option<String>,
    /// `url` (human-facing permalink).
    pub url: Option<String>,
    /// `inReplyTo` (if this object is a reply).
    pub in_reply_to: Option<String>,
}

impl ObjectSummary {
    /// True when nothing was extracted.
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.content.is_none() && self.summary.is_none()
    }
}

#[cfg(feature = "python")]
macro_rules! impl_py_dict {
    ($ty:ty, $($field:ident => $key:literal),+ $(,)?) => {
        #[cfg(feature = "python")]
        impl $ty {
            pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
                let dict = PyDict::new(py);
                $(
                    if let Some(v) = &self.$field {
                        dict.set_item($key, v).ok();
                    }
                )+
                dict.unbind()
            }
        }
    };
}

#[cfg(feature = "python")]
impl_py_dict!(
    ActorSummary,
    id => "id",
    kind => "type",
    preferred_username => "preferredUsername",
    name => "name",
    summary => "summary",
    url => "url",
    inbox => "inbox",
    outbox => "outbox",
    followers => "followers",
    following => "following",
    icon => "icon",
);

#[cfg(feature = "python")]
impl_py_dict!(
    ObjectSummary,
    id => "id",
    kind => "type",
    attributed_to => "attributedTo",
    content => "content",
    summary => "summary",
    published => "published",
    url => "url",
    in_reply_to => "inReplyTo",
);

#[cfg(feature = "python")]
impl ActivityPubDiscovery {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        if let Some(v) = &self.alternate_url {
            dict.set_item("alternate_url", v).ok();
        }
        if let Some(v) = &self.fediverse_creator {
            dict.set_item("fediverse_creator", v).ok();
        }
        if !self.rel_me.is_empty() {
            dict.set_item("rel_me", self.rel_me.clone()).ok();
        }
        dict.unbind()
    }
}
