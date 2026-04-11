//! Search-engine result page (SERP) parsers.
//!
//! Top-level module rather than `extractors::serp::*` because SERP has its
//! own parallel type hierarchy ([`SerpGraph`], [`KnowledgeGraph`],
//! [`FeaturedSnippet`], [`PeopleAlsoAsk`]) that doesn't fit the per-format
//! `extractors/*` layout.
//!
//! ```no_run
//! let html = "<html>...</html>";
//! let serp = meta_oxide::serp::parse_google(html);
//! if let Some(fs) = serp.featured_snippet {
//!     println!("{}: {}", fs.title, fs.text);
//! }
//! ```
//!
//! Backends:
//! - [`google::parse_google`] — Phase 4 / P0, lifted from `scraper_oxide`
//! - [`bing::parse_bing`] — stub, returns empty graph until implemented
//! - [`duckduckgo::parse_duckduckgo`] — stub, returns empty graph
//! - [`yandex::parse_yandex`] — stub, returns empty graph

pub mod bing;
pub mod duckduckgo;
pub mod google;
pub mod types;
pub mod yandex;

pub use bing::parse_bing;
pub use duckduckgo::parse_duckduckgo;
pub use google::parse_google;
pub use types::{
    FeaturedSnippet, FeaturedSnippetType, OrganicResult, PeopleAlsoAsk, SerpGraph, SiteLink,
};
pub use yandex::parse_yandex;

// Re-export the canonical KnowledgeGraph so callers can refer to it via the
// SERP module without having to know it lives in `crate::canonical`.
pub use crate::canonical::KnowledgeGraph;
