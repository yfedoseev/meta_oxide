//! ActivityPub / Activity Streams 2.0 discovery and parsing.
//!
//! HTML-level discovery surfaces `rel="alternate" type="application/activity+json"`
//! links plus adjacent Mastodon/fediverse hints. Companion
//! [`parse_as2_actor`] / [`parse_as2_object`] functions parse the
//! linked JSON representation into a lightweight summary.

use crate::errors::{MicroformatError, Result};
use crate::extractors::common::{html_utils, url_utils};
use crate::types::activitypub::{ActivityPubDiscovery, ActorSummary, ObjectSummary};
use scraper::Html;
use serde_json::Value;

#[cfg(test)]
mod tests;

/// Discover ActivityPub links in an HTML string.
pub fn extract(html: &str, base_url: Option<&str>) -> Result<ActivityPubDiscovery> {
    extract_from_dom(&html_utils::parse_html(html), base_url)
}

/// Discover ActivityPub links in an already-parsed DOM.
pub fn extract_from_dom(document: &Html, base_url: Option<&str>) -> Result<ActivityPubDiscovery> {
    let mut out = ActivityPubDiscovery::default();

    // rel="alternate" with activity+json type
    if let Ok(sel) = html_utils::create_selector("link[rel][type][href]") {
        for el in document.select(&sel) {
            let ty = html_utils::get_attr(&el, "type").unwrap_or_default();
            let rel = html_utils::get_attr(&el, "rel").unwrap_or_default();
            let href = html_utils::get_attr(&el, "href").unwrap_or_default();
            if href.trim().is_empty() {
                continue;
            }
            let ty_l = ty.to_ascii_lowercase();
            let rel_tokens: Vec<&str> = rel.split_ascii_whitespace().collect();
            if ty_l.contains("application/activity+json")
                && rel_tokens.iter().any(|t| t.eq_ignore_ascii_case("alternate"))
            {
                out.alternate_url = Some(resolve(base_url, &href));
            }
        }
    }

    // rel="me" — identity verification (Mastodon links back to personal sites this way).
    if let Ok(sel) = html_utils::create_selector("link[rel~=\"me\"][href], a[rel~=\"me\"][href]") {
        for el in document.select(&sel) {
            if let Some(href) = html_utils::get_attr(&el, "href") {
                let href = href.trim();
                if !href.is_empty() {
                    out.rel_me.push(resolve(base_url, href));
                }
            }
        }
    }

    // <meta name="fediverse:creator" content="@user@instance">
    if let Ok(sel) = html_utils::create_selector("meta[name][content]") {
        for el in document.select(&sel) {
            let name = html_utils::get_attr(&el, "name").unwrap_or_default().to_ascii_lowercase();
            let content =
                html_utils::get_attr(&el, "content").unwrap_or_default().trim().to_string();
            if content.is_empty() {
                continue;
            }
            if name == "fediverse:creator" {
                out.fediverse_creator = Some(content);
            }
        }
    }

    Ok(out)
}

/// Parse an AS2 Actor JSON document into a lightweight [`ActorSummary`].
///
/// Accepts either a bare actor object or a document with an `actor` field.
/// Unknown fields are ignored.
pub fn parse_as2_actor(json: &str) -> Result<ActorSummary> {
    let root: Value = serde_json::from_str(json)
        .map_err(|e| MicroformatError::ParseError(format!("as2 actor: {}", e)))?;
    let v = root.get("actor").unwrap_or(&root);
    if !is_as2(v) && !is_actor_kind(v.get("type")) {
        // Still try — the context isn't mandatory in every federation.
    }
    Ok(ActorSummary {
        id: string_field(v, "id"),
        kind: type_field(v),
        preferred_username: string_field(v, "preferredUsername"),
        name: string_field(v, "name"),
        summary: string_field(v, "summary"),
        url: url_field(v, "url"),
        inbox: string_field(v, "inbox"),
        outbox: string_field(v, "outbox"),
        followers: string_field(v, "followers"),
        following: string_field(v, "following"),
        icon: url_field(v, "icon"),
    })
}

/// Parse an AS2 Object JSON document (Note, Article, Page, …).
pub fn parse_as2_object(json: &str) -> Result<ObjectSummary> {
    let root: Value = serde_json::from_str(json)
        .map_err(|e| MicroformatError::ParseError(format!("as2 object: {}", e)))?;
    let v = root.get("object").unwrap_or(&root);
    Ok(ObjectSummary {
        id: string_field(v, "id"),
        kind: type_field(v),
        attributed_to: string_field(v, "attributedTo").or_else(|| url_field(v, "attributedTo")),
        content: string_field(v, "content"),
        summary: string_field(v, "summary"),
        published: string_field(v, "published"),
        url: url_field(v, "url"),
        in_reply_to: string_field(v, "inReplyTo"),
    })
}

fn is_as2(v: &Value) -> bool {
    let Some(ctx) = v.get("@context") else { return false };
    match ctx {
        Value::String(s) => s.contains("activitystreams"),
        Value::Array(arr) => {
            arr.iter().any(|v| v.as_str().map(|s| s.contains("activitystreams")).unwrap_or(false))
        }
        _ => false,
    }
}

fn is_actor_kind(ty: Option<&Value>) -> bool {
    let Some(ty) = ty else { return false };
    matches!(
        ty.as_str(),
        Some("Person")
            | Some("Service")
            | Some("Application")
            | Some("Group")
            | Some("Organization")
    )
}

fn string_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)?.as_str().map(str::to_string)
}

fn type_field(v: &Value) -> Option<String> {
    match v.get("type")? {
        Value::String(s) => Some(s.clone()),
        Value::Array(a) => a.iter().find_map(|v| v.as_str().map(str::to_string)),
        _ => None,
    }
}

fn url_field(v: &Value, key: &str) -> Option<String> {
    let raw = v.get(key)?;
    match raw {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => map
            .get("href")
            .and_then(|h| h.as_str())
            .map(str::to_string)
            .or_else(|| map.get("url").and_then(|h| h.as_str()).map(str::to_string)),
        Value::Array(arr) => arr.iter().find_map(|v| url_field_single(v)),
        _ => None,
    }
}

fn url_field_single(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => map.get("href").and_then(|h| h.as_str()).map(str::to_string),
        _ => None,
    }
}

fn resolve(base_url: Option<&str>, url: &str) -> String {
    url_utils::resolve_url(base_url, url).unwrap_or_else(|_| url.to_string())
}
