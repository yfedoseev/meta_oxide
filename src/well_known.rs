//! Parsers for common `.well-known/*` documents and neighbouring site-level
//! text files (`llms.txt`, `humans.txt`).
//!
//! None of these are HTML. Consumers fetch the bytes themselves and hand
//! them to these parsers — `meta_oxide` never issues network calls.
//!
//! Included:
//!
//! - [`parse_security_txt`] — RFC 9116 security.txt.
//! - [`parse_humans_txt`] — informal humanstxt.org format (best-effort).
//! - [`parse_llms_txt`] — Markdown-ish LLM site guide (Howard 2024 proposal).
//! - [`parse_webfinger_jrd`] — RFC 7033 WebFinger JRD JSON.

use crate::errors::{MicroformatError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// security.txt (RFC 9116)
// ---------------------------------------------------------------------------

/// Parsed RFC 9116 `security.txt`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SecurityTxt {
    /// Contact URIs (`mailto:`, `https://`, `tel:`) — in declaration order.
    pub contact: Vec<String>,
    /// ISO 8601 expiration timestamp.
    pub expires: Option<String>,
    /// Encryption key URIs.
    pub encryption: Vec<String>,
    /// Acknowledgement / hall-of-fame URLs.
    pub acknowledgments: Vec<String>,
    /// Supported natural languages (RFC 5646 tags).
    pub preferred_languages: Vec<String>,
    /// Canonical URL of this security.txt document.
    pub canonical: Vec<String>,
    /// Policy URL.
    pub policy: Vec<String>,
    /// Hiring / career URLs.
    pub hiring: Vec<String>,
    /// CSAF provider metadata URL.
    pub csaf: Vec<String>,
    /// Catch-all: any other `Key: value` lines we don't model.
    pub other: Vec<(String, String)>,
}

/// Parse an RFC 9116 `security.txt` document.
///
/// Tolerant: comments (`#…`) and blank lines are ignored, key matching is
/// case-insensitive, and unrecognised keys go to [`SecurityTxt::other`].
pub fn parse_security_txt(text: &str) -> SecurityTxt {
    let mut out = SecurityTxt::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if value.is_empty() {
            continue;
        }
        match key.as_str() {
            "contact" => out.contact.push(value),
            "expires" => out.expires = Some(value),
            "encryption" => out.encryption.push(value),
            "acknowledgments" | "acknowledgements" => out.acknowledgments.push(value),
            "preferred-languages" => {
                for lang in value.split(',') {
                    let lang = lang.trim();
                    if !lang.is_empty() {
                        out.preferred_languages.push(lang.to_string());
                    }
                }
            }
            "canonical" => out.canonical.push(value),
            "policy" => out.policy.push(value),
            "hiring" => out.hiring.push(value),
            "csaf" => out.csaf.push(value),
            _ => out.other.push((key, value)),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// humans.txt (informal; humanstxt.org)
// ---------------------------------------------------------------------------

/// Parsed `humans.txt`. Sections are keyed by uppercase heading (`TEAM`,
/// `THANKS`, `TECHNOLOGY COLOPHON`); lines are kept in their original order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HumansTxt {
    /// Ordered sections: `(heading, lines)`.
    pub sections: Vec<(String, Vec<String>)>,
}

/// Parse a `humans.txt` document.
///
/// Headings are detected as lines ending in `:` or wrapped in `/* ... */`.
/// Hanging-indent continuations stay with their preceding line.
pub fn parse_humans_txt(text: &str) -> HumansTxt {
    let mut out = HumansTxt::default();
    let mut current: Option<(String, Vec<String>)> = None;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("/*") && line.ends_with("*/") {
            if let Some(section) = current.take() {
                out.sections.push(section);
            }
            let heading = line.trim_start_matches("/*").trim_end_matches("*/").trim().to_string();
            current = Some((heading.to_ascii_uppercase(), Vec::new()));
            continue;
        }
        if let Some(header) = line.strip_suffix(':') {
            if !header.contains(' ') || header.to_ascii_uppercase() == *header {
                if let Some(section) = current.take() {
                    out.sections.push(section);
                }
                current = Some((header.trim().to_ascii_uppercase(), Vec::new()));
                continue;
            }
        }
        match current.as_mut() {
            Some((_, lines)) => lines.push(line.to_string()),
            None => {
                current = Some(("PREAMBLE".to_string(), vec![line.to_string()]));
            }
        }
    }
    if let Some(section) = current.take() {
        out.sections.push(section);
    }
    out
}

// ---------------------------------------------------------------------------
// llms.txt (Howard 2024 proposal)
// ---------------------------------------------------------------------------

/// A link referenced from an `llms.txt` bullet point.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LlmsLink {
    /// Visible link text.
    pub text: String,
    /// Target URL.
    pub url: String,
    /// Trailing free-text description after the link (separated by `:` or `-`).
    pub description: Option<String>,
}

/// An `llms.txt` section (H2 heading + link list).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LlmsSection {
    /// Section title (from the H2 heading).
    pub title: String,
    /// Free-form prose paragraphs between the heading and the bullet list.
    pub summary: Option<String>,
    /// Bulleted links declared in this section.
    pub links: Vec<LlmsLink>,
}

/// Parsed `llms.txt`.
///
/// See <https://llmstxt.org/> for the proposed spec.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LlmsTxt {
    /// H1 title (site or project name).
    pub title: Option<String>,
    /// Blockquote immediately after the H1 (optional short description).
    pub summary: Option<String>,
    /// Free-form paragraphs between the blockquote and the first H2.
    pub intro: Option<String>,
    /// H2-level sections.
    pub sections: Vec<LlmsSection>,
}

/// Parse an `llms.txt` document.
///
/// The parser is intentionally lenient: anything outside the expected
/// Markdown shape is collected into the nearest section's `summary`.
pub fn parse_llms_txt(text: &str) -> LlmsTxt {
    let mut out = LlmsTxt::default();
    let mut intro_buf: Vec<String> = Vec::new();
    let mut section_summary: Vec<String> = Vec::new();
    let mut current: Option<LlmsSection> = None;
    let mut saw_summary_quote = false;

    for raw in text.lines() {
        let line = raw.trim_end();
        if let Some(h1) = line.strip_prefix("# ") {
            out.title = Some(h1.trim().to_string());
            continue;
        }
        if let Some(h2) = line.strip_prefix("## ") {
            finalise_section(&mut current, &mut section_summary, &mut out.sections);
            if !intro_buf.is_empty() && out.intro.is_none() {
                out.intro = Some(intro_buf.join("\n").trim().to_string());
                intro_buf.clear();
            }
            current = Some(LlmsSection { title: h2.trim().to_string(), ..LlmsSection::default() });
            continue;
        }
        if out.summary.is_none() && !saw_summary_quote {
            if let Some(quote) = line.strip_prefix("> ") {
                out.summary = Some(quote.trim().to_string());
                saw_summary_quote = true;
                continue;
            }
        }
        let bullet = line
            .strip_prefix("- ")
            .or_else(|| line.strip_prefix("* "))
            .or_else(|| line.strip_prefix("+ "));
        if let Some(bullet) = bullet {
            if let Some(link) = parse_markdown_link(bullet) {
                if let Some(section) = current.as_mut() {
                    section.links.push(link);
                } else {
                    // Stray link before any H2 — fold into intro.
                    intro_buf.push(format!("- [{}]({})", link.text, link.url));
                }
                continue;
            }
        }
        if line.is_empty() {
            continue;
        }
        match current.as_mut() {
            Some(_) => section_summary.push(line.to_string()),
            None => intro_buf.push(line.to_string()),
        }
    }
    finalise_section(&mut current, &mut section_summary, &mut out.sections);
    if out.intro.is_none() && !intro_buf.is_empty() {
        out.intro = Some(intro_buf.join("\n").trim().to_string());
    }
    out
}

fn finalise_section(
    current: &mut Option<LlmsSection>,
    summary_buf: &mut Vec<String>,
    sections: &mut Vec<LlmsSection>,
) {
    if let Some(mut section) = current.take() {
        if !summary_buf.is_empty() {
            section.summary = Some(summary_buf.join("\n").trim().to_string());
            summary_buf.clear();
        }
        sections.push(section);
    }
}

fn parse_markdown_link(text: &str) -> Option<LlmsLink> {
    // Shape: `[text](url)` optionally followed by `: description` or ` - description`.
    let text = text.trim();
    let lb = text.find('[')?;
    let rb_rel = text[lb + 1..].find("](")?;
    let rb = lb + 1 + rb_rel;
    let close_rel = text[rb + 2..].find(')')?;
    let url_end = rb + 2 + close_rel;
    let label = text[lb + 1..rb].to_string();
    let url = text[rb + 2..url_end].to_string();
    let tail = text[url_end + 1..].trim_start();
    let description = tail
        .strip_prefix(':')
        .or_else(|| tail.strip_prefix('-'))
        .or_else(|| tail.strip_prefix("—"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    Some(LlmsLink { text: label, url, description })
}

// ---------------------------------------------------------------------------
// WebFinger JRD (RFC 7033)
// ---------------------------------------------------------------------------

/// Parsed WebFinger JRD response (RFC 7033 §4).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebFinger {
    /// The subject URI (e.g. `acct:alice@example.com`).
    pub subject: Option<String>,
    /// Alternative identifiers for the same subject.
    pub aliases: Vec<String>,
    /// Properties map — arbitrary URI-keyed string values.
    pub properties: std::collections::BTreeMap<String, String>,
    /// Links declared in the JRD.
    pub links: Vec<WebFingerLink>,
}

/// One WebFinger link (RFC 7033 §4.4.4).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebFingerLink {
    /// Relation type (URI or registered short name).
    pub rel: Option<String>,
    /// MIME type of the target.
    pub kind: Option<String>,
    /// Target URI.
    pub href: Option<String>,
    /// Template string (when `href` is absent — common for `remote-follow` links).
    pub template: Option<String>,
    /// Titles keyed by language tag.
    pub titles: std::collections::BTreeMap<String, String>,
    /// Properties map.
    pub properties: std::collections::BTreeMap<String, String>,
}

/// Parse a WebFinger JRD JSON document.
pub fn parse_webfinger_jrd(json: &str) -> Result<WebFinger> {
    let v: Value = serde_json::from_str(json)
        .map_err(|e| MicroformatError::ParseError(format!("webfinger: {}", e)))?;
    let mut wf = WebFinger {
        subject: v.get("subject").and_then(Value::as_str).map(str::to_string),
        ..WebFinger::default()
    };
    if let Some(aliases) = v.get("aliases").and_then(Value::as_array) {
        for a in aliases {
            if let Some(s) = a.as_str() {
                wf.aliases.push(s.to_string());
            }
        }
    }
    if let Some(props) = v.get("properties").and_then(Value::as_object) {
        for (k, v) in props {
            if let Some(s) = v.as_str() {
                wf.properties.insert(k.clone(), s.to_string());
            }
        }
    }
    if let Some(links) = v.get("links").and_then(Value::as_array) {
        for l in links {
            let mut link = WebFingerLink {
                rel: l.get("rel").and_then(Value::as_str).map(str::to_string),
                kind: l.get("type").and_then(Value::as_str).map(str::to_string),
                href: l.get("href").and_then(Value::as_str).map(str::to_string),
                template: l.get("template").and_then(Value::as_str).map(str::to_string),
                ..WebFingerLink::default()
            };
            if let Some(titles) = l.get("titles").and_then(Value::as_object) {
                for (k, v) in titles {
                    if let Some(s) = v.as_str() {
                        link.titles.insert(k.clone(), s.to_string());
                    }
                }
            }
            if let Some(props) = l.get("properties").and_then(Value::as_object) {
                for (k, v) in props {
                    if let Some(s) = v.as_str() {
                        link.properties.insert(k.clone(), s.to_string());
                    }
                }
            }
            wf.links.push(link);
        }
    }
    Ok(wf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_security_txt_basic() {
        let text = "# our security disclosures\nContact: mailto:security@example.com\nContact: https://example.com/report\nExpires: 2026-12-31T23:59:59Z\nPreferred-Languages: en, de\nAcknowledgments: https://example.com/hall-of-fame\n";
        let s = parse_security_txt(text);
        assert_eq!(s.contact.len(), 2);
        assert_eq!(s.expires.as_deref(), Some("2026-12-31T23:59:59Z"));
        assert_eq!(s.preferred_languages, vec!["en".to_string(), "de".to_string()]);
        assert_eq!(s.acknowledgments.len(), 1);
    }

    #[test]
    fn parses_humans_txt_sections() {
        let text = "/* TEAM */\nAlice - lead\nBob - eng\n\n/* THANKS */\nOpen source\n";
        let h = parse_humans_txt(text);
        assert_eq!(h.sections.len(), 2);
        assert_eq!(h.sections[0].0, "TEAM");
        assert_eq!(h.sections[0].1.len(), 2);
        assert_eq!(h.sections[1].0, "THANKS");
    }

    #[test]
    fn parses_llms_txt_basic() {
        let text = "# Example\n\n> short blurb.\n\nIntroductory prose.\n\n## Docs\n\nLinks to core docs.\n\n- [Quickstart](/docs/quickstart.md): 5-minute guide\n- [API Reference](/docs/api.md)\n\n## Optional\n\n- [Changelog](/changelog.md)\n";
        let l = parse_llms_txt(text);
        assert_eq!(l.title.as_deref(), Some("Example"));
        assert_eq!(l.summary.as_deref(), Some("short blurb."));
        assert!(l.intro.as_deref().unwrap().contains("Introductory prose"));
        assert_eq!(l.sections.len(), 2);
        assert_eq!(l.sections[0].title, "Docs");
        assert_eq!(l.sections[0].links.len(), 2);
        assert_eq!(l.sections[0].links[0].text, "Quickstart");
        assert_eq!(l.sections[0].links[0].description.as_deref(), Some("5-minute guide"));
    }

    #[test]
    fn parses_webfinger_jrd() {
        let json = r#"{
            "subject": "acct:alice@example.com",
            "aliases": ["https://example.com/@alice"],
            "links": [
                {"rel": "self", "type": "application/activity+json", "href": "https://example.com/users/alice"},
                {"rel": "http://ostatus.org/schema/1.0/subscribe", "template": "https://example.com/authorize_interaction?uri={uri}"}
            ]
        }"#;
        let wf = parse_webfinger_jrd(json).unwrap();
        assert_eq!(wf.subject.as_deref(), Some("acct:alice@example.com"));
        assert_eq!(wf.links.len(), 2);
        assert_eq!(wf.links[0].rel.as_deref(), Some("self"));
        assert!(wf.links[1].template.as_deref().unwrap().contains("authorize"));
    }

    #[test]
    fn invalid_webfinger_is_error() {
        assert!(parse_webfinger_jrd("not json").is_err());
    }
}
