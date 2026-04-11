//! Heuristic / CSS-fallback layer.
//!
//! When structured-data extractors come up empty, the heuristic layer applies
//! cheap CSS-based rules to fill in the obvious fields: `<h1>` → title, first
//! `<p>` → description, `<article>` → body, `<time>` → published date, etc.
//!
//! Each rule produces a [`crate::FieldValue`] tagged with
//! [`crate::FieldSource::Heuristic`] so callers can tell at a glance whether
//! a value came from a high-confidence structured source or a best-effort guess.
//!
//! [`apply`] is wired into [`crate::MetaParser::parse_dom`] behind the
//! [`crate::MetaParser::with_heuristics`] flag (default `true`). It only fills
//! `graph.heuristic_fills` for fields the structured extractors didn't already
//! cover, so structured data always wins.

use crate::parser_facade::MetaGraph;
use scraper::Html;

mod body;
mod date;
mod description;
mod image;
mod price;
mod title;

/// Run every heuristic rule against `dom`, populating `graph.heuristic_fills`
/// for any logical field the structured extractors didn't supply.
pub fn apply(dom: &Html, graph: &mut MetaGraph, _base_url: Option<&str>) {
    title::apply(dom, graph);
    description::apply(dom, graph);
    image::apply(dom, graph);
    body::apply(dom, graph);
    date::apply(dom, graph);
    price::apply(dom, graph);
}
