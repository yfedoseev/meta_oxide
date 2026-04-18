//! Syndication feed parser (RSS 2.0, Atom 1.0, JSON Feed 1.1).
//!
//! The HTML-level discovery of feed `<link>` tags is already covered by
//! [`crate::extractors::rel_links`]. This module consumes feed payload
//! bytes that consumers have already fetched, and produces a unified
//! [`crate::types::feeds::Feed`] representation.
//!
//! Design notes:
//!
//! - RSS and Atom are parsed via `quick-xml` in streaming mode. We do
//!   *not* bring in `feed-rs` because we want the dependency footprint
//!   to stay lean and the result shape to match our other extractors.
//! - JSON Feed is parsed with `serde_json` (already a dependency).
//! - [`parse`] sniffs the payload and dispatches to the right parser.

use crate::errors::{MicroformatError, Result};
use crate::types::feeds::{Feed, FeedAuthor, FeedItem, FeedKind};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::Value;

#[cfg(test)]
mod tests;

/// Sniff the payload and dispatch to the appropriate parser.
///
/// Returns an error when the payload cannot be classified as RSS, Atom, or
/// JSON Feed. Whitespace is skipped before sniffing.
pub fn parse(bytes: &[u8]) -> Result<Feed> {
    let trimmed = skip_leading_whitespace(bytes);
    if trimmed.is_empty() {
        return Err(MicroformatError::ParseError("empty feed payload".into()));
    }
    match trimmed[0] {
        b'{' => parse_json_feed_bytes(trimmed),
        b'<' => parse_xml_feed(trimmed),
        _ => Err(MicroformatError::ParseError("unrecognised feed payload".into())),
    }
}

/// Parse a JSON Feed document from a `str`.
pub fn parse_json_feed(json: &str) -> Result<Feed> {
    parse_json_feed_bytes(json.as_bytes())
}

/// Parse an RSS 2.0 document from a `str`.
pub fn parse_rss(xml: &str) -> Result<Feed> {
    parse_xml_feed(xml.as_bytes())
}

/// Parse an Atom 1.0 document from a `str`.
pub fn parse_atom(xml: &str) -> Result<Feed> {
    parse_xml_feed(xml.as_bytes())
}

fn skip_leading_whitespace(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    // Skip UTF-8 BOM.
    if bytes[i..].starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[i + 3..]
    } else {
        &bytes[i..]
    }
}

fn parse_json_feed_bytes(bytes: &[u8]) -> Result<Feed> {
    let v: Value = serde_json::from_slice(bytes)
        .map_err(|e| MicroformatError::ParseError(format!("json feed: {}", e)))?;
    let mut feed = Feed::empty(FeedKind::JsonFeed);
    feed.title = string(&v, "title");
    feed.description = string(&v, "description");
    feed.home_page_url = string(&v, "home_page_url");
    feed.feed_url = string(&v, "feed_url");
    feed.language = string(&v, "language");
    feed.icon = string(&v, "icon").or_else(|| string(&v, "favicon"));

    if let Some(authors) = v.get("authors").and_then(Value::as_array) {
        for a in authors {
            feed.authors.push(jf_author(a));
        }
    } else if let Some(a) = v.get("author") {
        feed.authors.push(jf_author(a));
    }

    if let Some(items) = v.get("items").and_then(Value::as_array) {
        for item in items {
            feed.items.push(jf_item(item));
        }
    }
    Ok(feed)
}

fn jf_author(v: &Value) -> FeedAuthor {
    FeedAuthor { name: string(v, "name"), url: string(v, "url"), email: string(v, "email") }
}

fn jf_item(v: &Value) -> FeedItem {
    let mut it = FeedItem {
        id: string(v, "id"),
        title: string(v, "title"),
        url: string(v, "url").or_else(|| string(v, "external_url")),
        summary: string(v, "summary"),
        content_html: string(v, "content_html"),
        content_text: string(v, "content_text"),
        published: string(v, "date_published"),
        updated: string(v, "date_modified"),
        ..FeedItem::default()
    };
    if let Some(authors) = v.get("authors").and_then(Value::as_array) {
        for a in authors {
            it.authors.push(jf_author(a));
        }
    } else if let Some(a) = v.get("author") {
        it.authors.push(jf_author(a));
    }
    if let Some(tags) = v.get("tags").and_then(Value::as_array) {
        for t in tags {
            if let Some(s) = t.as_str() {
                it.tags.push(s.to_string());
            }
        }
    }
    if let Some(att) = v.get("attachments").and_then(Value::as_array).and_then(|a| a.first()) {
        it.enclosure_url = string(att, "url");
        it.enclosure_type = string(att, "mime_type");
    }
    it
}

fn string(v: &Value, key: &str) -> Option<String> {
    v.get(key)?.as_str().map(str::to_string)
}

// ---------------------------------------------------------------------------
// XML (RSS + Atom)
// ---------------------------------------------------------------------------

fn parse_xml_feed(bytes: &[u8]) -> Result<Feed> {
    let mut reader = Reader::from_reader(bytes);
    let reader_config = reader.config_mut();
    reader_config.trim_text(true);

    // Walk until we find the root element and dispatch.
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                return match name.as_str() {
                    "rss" => parse_rss_inner(reader),
                    "feed" => parse_atom_inner(reader, &e),
                    // Some producers wrap with `rdf:RDF` (RSS 1.0); treat as RSS.
                    "RDF" => parse_rss_inner(reader),
                    other => Err(MicroformatError::ParseError(format!(
                        "unexpected feed root element: {}",
                        other
                    ))),
                };
            }
            Ok(Event::Eof) => return Err(MicroformatError::ParseError("empty xml feed".into())),
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
}

fn local_name(e: &BytesStart) -> String {
    let full = String::from_utf8_lossy(e.name().as_ref()).to_string();
    full.rsplit_once(':').map(|(_, local)| local.to_string()).unwrap_or(full)
}

fn attr(e: &BytesStart, key: &str) -> Option<String> {
    e.attributes().flatten().find_map(|a| {
        let a_name = String::from_utf8_lossy(a.key.as_ref()).to_string();
        let local = a_name.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(a_name);
        if local == key {
            Some(String::from_utf8_lossy(&a.value).to_string())
        } else {
            None
        }
    })
}

fn parse_rss_inner<R: std::io::BufRead>(mut reader: Reader<R>) -> Result<Feed> {
    let mut feed = Feed::empty(FeedKind::Rss);
    let mut buf = Vec::new();
    let mut in_channel = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                match name.as_str() {
                    "channel" => in_channel = true,
                    "item" if in_channel => {
                        let item = parse_rss_item(&mut reader)?;
                        feed.items.push(item);
                    }
                    "title" if in_channel => feed.title = Some(read_text(&mut reader, "title")?),
                    "description" if in_channel => {
                        feed.description = Some(read_text(&mut reader, "description")?)
                    }
                    "link" if in_channel => {
                        // RSS <link> is usually text content; Atom-namespaced link
                        // inside RSS uses attribute `href`.
                        if let Some(href) = attr(&e, "href") {
                            if feed.feed_url.is_none() && attr(&e, "rel").as_deref() == Some("self")
                            {
                                feed.feed_url = Some(href);
                            } else if feed.home_page_url.is_none() {
                                feed.home_page_url = Some(href);
                            }
                        } else {
                            let text = read_text(&mut reader, "link")?;
                            if !text.is_empty() && feed.home_page_url.is_none() {
                                feed.home_page_url = Some(text);
                            }
                        }
                    }
                    "language" if in_channel => {
                        feed.language = Some(read_text(&mut reader, "language")?)
                    }
                    "lastBuildDate" | "pubDate" if in_channel => {
                        let nm = name.clone();
                        feed.updated = Some(read_text(&mut reader, &nm)?);
                    }
                    "image" if in_channel => {
                        // Skip or drill into for <url>
                        if let Ok(url) = read_child_text(&mut reader, "image", "url") {
                            feed.icon = Some(url);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                if local_name(&BytesStart::from_content(
                    String::from_utf8_lossy(e.name().as_ref()).to_string(),
                    0,
                )) == "channel"
                {
                    in_channel = false;
                }
            }
            Ok(Event::Empty(e)) => {
                let name = local_name(&e);
                if name == "link" && in_channel {
                    if let Some(href) = attr(&e, "href") {
                        if attr(&e, "rel").as_deref() == Some("self") {
                            feed.feed_url = Some(href);
                        } else if feed.home_page_url.is_none() {
                            feed.home_page_url = Some(href);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(feed)
}

fn parse_rss_item<R: std::io::BufRead>(reader: &mut Reader<R>) -> Result<FeedItem> {
    let mut item = FeedItem::default();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                match name.as_str() {
                    "title" => item.title = Some(read_text(reader, "title")?),
                    "link" => item.url = Some(read_text(reader, "link")?),
                    "description" => item.summary = Some(read_text(reader, "description")?),
                    "encoded" => {
                        // content:encoded
                        item.content_html = Some(read_text(reader, "encoded")?);
                    }
                    "guid" => item.id = Some(read_text(reader, "guid")?),
                    "pubDate" => item.published = Some(read_text(reader, "pubDate")?),
                    "category" => {
                        let t = read_text(reader, "category")?;
                        if !t.is_empty() {
                            item.tags.push(t);
                        }
                    }
                    "author" | "creator" => {
                        let t = read_text(reader, &name)?;
                        if !t.is_empty() {
                            item.authors
                                .push(FeedAuthor { name: Some(t), ..FeedAuthor::default() });
                        }
                    }
                    _ => skip_element(reader, &name)?,
                }
            }
            Ok(Event::Empty(e)) => {
                let name = local_name(&e);
                if name == "enclosure" {
                    item.enclosure_url = attr(&e, "url");
                    item.enclosure_type = attr(&e, "type");
                }
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == "item" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(item)
}

fn parse_atom_inner<R: std::io::BufRead>(
    mut reader: Reader<R>,
    _root: &BytesStart,
) -> Result<Feed> {
    let mut feed = Feed::empty(FeedKind::Atom);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                match name.as_str() {
                    "title" => feed.title = Some(read_text(&mut reader, "title")?),
                    "subtitle" => feed.description = Some(read_text(&mut reader, "subtitle")?),
                    "updated" => feed.updated = Some(read_text(&mut reader, "updated")?),
                    "link" => atom_link(&e, &mut feed),
                    "author" => feed.authors.push(parse_atom_author(&mut reader)?),
                    "icon" | "logo" => {
                        let nm = name.clone();
                        let t = read_text(&mut reader, &nm)?;
                        if feed.icon.is_none() {
                            feed.icon = Some(t);
                        }
                    }
                    "entry" => feed.items.push(parse_atom_entry(&mut reader)?),
                    _ => skip_element(&mut reader, &name)?,
                }
            }
            Ok(Event::Empty(e)) => {
                if local_name(&e) == "link" {
                    atom_link(&e, &mut feed);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(feed)
}

fn atom_link(e: &BytesStart, feed: &mut Feed) {
    let href = match attr(e, "href") {
        Some(h) => h,
        None => return,
    };
    match attr(e, "rel").as_deref() {
        Some("self") => feed.feed_url = Some(href),
        Some("alternate") | None => {
            if feed.home_page_url.is_none() {
                feed.home_page_url = Some(href);
            }
        }
        _ => {}
    }
}

fn parse_atom_entry<R: std::io::BufRead>(reader: &mut Reader<R>) -> Result<FeedItem> {
    let mut item = FeedItem::default();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                match name.as_str() {
                    "id" => item.id = Some(read_text(reader, "id")?),
                    "title" => item.title = Some(read_text(reader, "title")?),
                    "summary" => item.summary = Some(read_text(reader, "summary")?),
                    "content" => item.content_html = Some(read_text(reader, "content")?),
                    "published" => item.published = Some(read_text(reader, "published")?),
                    "updated" => item.updated = Some(read_text(reader, "updated")?),
                    "author" => item.authors.push(parse_atom_author(reader)?),
                    "category" => {
                        if let Some(term) = attr(&e, "term") {
                            item.tags.push(term);
                        }
                        skip_element(reader, "category")?;
                    }
                    "link" => {
                        if let Some(href) = attr(&e, "href") {
                            if matches!(attr(&e, "rel").as_deref(), Some("alternate") | None)
                                && item.url.is_none()
                            {
                                item.url = Some(href);
                            } else if attr(&e, "rel").as_deref() == Some("enclosure") {
                                item.enclosure_url = Some(href);
                                item.enclosure_type = attr(&e, "type");
                            }
                        }
                        skip_element(reader, "link")?;
                    }
                    _ => skip_element(reader, &name)?,
                }
            }
            Ok(Event::Empty(e)) => {
                let name = local_name(&e);
                if name == "link" {
                    if let Some(href) = attr(&e, "href") {
                        if matches!(attr(&e, "rel").as_deref(), Some("alternate") | None)
                            && item.url.is_none()
                        {
                            item.url = Some(href);
                        } else if attr(&e, "rel").as_deref() == Some("enclosure") {
                            item.enclosure_url = Some(href);
                            item.enclosure_type = attr(&e, "type");
                        }
                    }
                } else if name == "category" {
                    if let Some(term) = attr(&e, "term") {
                        item.tags.push(term);
                    }
                }
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == "entry" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(item)
}

fn parse_atom_author<R: std::io::BufRead>(reader: &mut Reader<R>) -> Result<FeedAuthor> {
    let mut a = FeedAuthor::default();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                match name.as_str() {
                    "name" => a.name = Some(read_text(reader, "name")?),
                    "uri" | "url" => {
                        let nm = name.clone();
                        a.url = Some(read_text(reader, &nm)?);
                    }
                    "email" => a.email = Some(read_text(reader, "email")?),
                    _ => skip_element(reader, &name)?,
                }
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == "author" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(a)
}

fn read_text<R: std::io::BufRead>(reader: &mut Reader<R>, tag: &str) -> Result<String> {
    let mut buf = Vec::new();
    let mut out = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(t)) => {
                let decoded = t
                    .unescape()
                    .map(|c| c.into_owned())
                    .unwrap_or_else(|_| String::from_utf8_lossy(t.as_ref()).to_string());
                out.push_str(&decoded);
            }
            Ok(Event::CData(t)) => {
                out.push_str(&String::from_utf8_lossy(&t.into_inner()));
            }
            Ok(Event::Start(_)) => {
                // Nested elements (HTML inside <description>); best-effort flatten.
                // We don't recurse further here — quick-xml will give us end
                // events for each; skip until we see the matching close.
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == tag {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(out.trim().to_string())
}

fn read_child_text<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    parent: &str,
    child: &str,
) -> Result<String> {
    let mut buf = Vec::new();
    let mut out = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = local_name(&e);
                if name == child {
                    out = read_text(reader, child)?;
                } else {
                    skip_element(reader, &name)?;
                }
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == parent {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(out)
}

fn skip_element<R: std::io::BufRead>(reader: &mut Reader<R>, tag: &str) -> Result<()> {
    let mut depth = 1;
    let mut buf = Vec::new();
    while depth > 0 {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if local_name(&e) == tag {
                    depth += 1;
                }
            }
            Ok(Event::End(e)) => {
                let nm = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let local = nm.rsplit_once(':').map(|(_, l)| l.to_string()).unwrap_or(nm);
                if local == tag {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(MicroformatError::ParseError(format!("xml: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}
