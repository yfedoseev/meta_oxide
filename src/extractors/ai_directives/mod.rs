//! AI crawl and provenance directive extractor.
//!
//! See [`crate::types::ai_directives`] for the output shape.

use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::types::ai_directives::AiDirectives;
use scraper::Html;

#[cfg(test)]
mod tests;

const KNOWN_AI_BOTS: &[&str] = &[
    "gptbot",
    "chatgpt-user",
    "ccbot",
    "anthropic-ai",
    "claudebot",
    "claude-web",
    "perplexitybot",
    "cohere-ai",
    "google-extended",
    "applebot-extended",
    "bytespider",
    "amazonbot",
    "meta-externalagent",
    "facebookbot",
    "img2dataset",
];

const AI_GENERATOR_MARKERS: &[&str] =
    &["gpt", "claude", "gemini", "llama", "mistral", "copilot", "openai", " ai "];

/// Extract AI directives from an HTML string.
pub fn extract(html: &str) -> Result<AiDirectives> {
    extract_from_dom(&html_utils::parse_html(html))
}

/// Extract AI directives from an already-parsed DOM.
pub fn extract_from_dom(document: &Html) -> Result<AiDirectives> {
    let mut out = AiDirectives::default();

    let Ok(selector) = html_utils::create_selector("meta[name][content]") else {
        return Ok(out);
    };

    for element in document.select(&selector) {
        let Some(name) = html_utils::get_attr(&element, "name") else { continue };
        let Some(content) = html_utils::get_attr(&element, "content") else { continue };
        let content_trim = content.trim().to_string();
        if content_trim.is_empty() {
            continue;
        }
        let lname = name.to_ascii_lowercase();

        if lname == "robots" {
            for token in content_trim.split(',') {
                let token = token.trim().to_ascii_lowercase();
                if token.is_empty() {
                    continue;
                }
                match token.as_str() {
                    "noai" => out.noai = true,
                    "noimageai" => out.noimageai = true,
                    "noml" => out.noml = true,
                    _ => {}
                }
                out.robots.push(token);
            }
            continue;
        }

        if KNOWN_AI_BOTS.contains(&lname.as_str()) {
            out.per_bot.insert(lname.clone(), content_trim.clone());
            for token in content_trim.split(',') {
                let token = token.trim().to_ascii_lowercase();
                match token.as_str() {
                    "noai" => out.noai = true,
                    "noimageai" => out.noimageai = true,
                    "noml" => out.noml = true,
                    _ => {}
                }
            }
            continue;
        }

        match lname.as_str() {
            "ai-generated" => out.ai_generated = Some(content_trim),
            "ai-training" => out.ai_training = Some(content_trim),
            "generator" => {
                let needle = content_trim.to_ascii_lowercase();
                let padded = format!(" {} ", needle);
                if AI_GENERATOR_MARKERS.iter().any(|m| padded.contains(m)) {
                    out.generator_ai = Some(content_trim);
                }
            }
            _ => {}
        }
    }

    Ok(out)
}
