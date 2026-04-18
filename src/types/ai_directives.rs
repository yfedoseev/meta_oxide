//! AI-crawl and AI-provenance directives declared in HTML.
//!
//! Two things travel under this header:
//!
//! 1. **Crawl permissions** — whether the site wants its content used for
//!    AI training or image/ML indexing. Expressed as robots directives
//!    (`noai`, `noimageai`, `noml`), per-bot meta tags (`googlebot-news`,
//!    `gptbot`, `ccbot`, `perplexitybot`, `anthropic-ai`, `claudebot`),
//!    and increasingly as dedicated meta tags.
//! 2. **Content provenance** — whether the page itself is declaring that
//!    it was AI-generated (`ai-generated`, `generator=AI`). Distinct from
//!    C2PA, which is cryptographic; these are good-faith declarations.
//!
//! This extractor collects the signals — enforcement is a consumer concern.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Parsed AI-facing directives on a page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AiDirectives {
    /// Raw `robots` directives split on commas (`noai`, `noimageai`, `noml`,
    /// `none`, `noindex`, …). Lower-cased, whitespace-trimmed.
    pub robots: Vec<String>,
    /// True when `robots` contains `noai`.
    pub noai: bool,
    /// True when `robots` contains `noimageai`.
    pub noimageai: bool,
    /// True when `robots` contains `noml` (no ML / DMCA-style opt-out).
    pub noml: bool,
    /// Per-bot directive strings, keyed by bot name (lower-cased).
    /// Example keys: `googlebot`, `gptbot`, `ccbot`, `perplexitybot`,
    /// `anthropic-ai`, `claudebot`, `bytespider`.
    pub per_bot: BTreeMap<String, String>,
    /// `<meta name="ai-generated" content="...">` — free-form declaration.
    pub ai_generated: Option<String>,
    /// `<meta name="ai-training" content="...">` — opt-in/opt-out declaration.
    pub ai_training: Option<String>,
    /// `<meta name="generator" content="...">` — surfaced when it looks
    /// AI-adjacent (`GPT`, `Claude`, `Gemini`, `AI`, `LLM`).
    pub generator_ai: Option<String>,
}

impl AiDirectives {
    /// True when no AI-relevant signals were found.
    pub fn is_empty(&self) -> bool {
        self.robots.is_empty()
            && self.per_bot.is_empty()
            && self.ai_generated.is_none()
            && self.ai_training.is_none()
            && self.generator_ai.is_none()
    }
}

#[cfg(feature = "python")]
impl AiDirectives {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        if !self.robots.is_empty() {
            dict.set_item("robots", self.robots.clone()).ok();
        }
        dict.set_item("noai", self.noai).ok();
        dict.set_item("noimageai", self.noimageai).ok();
        dict.set_item("noml", self.noml).ok();
        if !self.per_bot.is_empty() {
            let bots = PyDict::new(py);
            for (k, v) in &self.per_bot {
                bots.set_item(k, v).ok();
            }
            dict.set_item("per_bot", bots).ok();
        }
        if let Some(v) = &self.ai_generated {
            dict.set_item("ai_generated", v).ok();
        }
        if let Some(v) = &self.ai_training {
            dict.set_item("ai_training", v).ok();
        }
        if let Some(v) = &self.generator_ai {
            dict.set_item("generator_ai", v).ok();
        }
        dict.unbind()
    }
}
