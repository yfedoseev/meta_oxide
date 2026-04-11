//! Title heuristic — first `<h1>` text content.

use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use scraper::{Html, Selector};

pub(super) fn apply(dom: &Html, graph: &mut MetaGraph) {
    if graph.meta.title.is_some()
        || graph.open_graph.title.is_some()
        || graph.twitter.title.is_some()
        || graph.heuristic_fills.contains_key("title")
    {
        return;
    }
    let Ok(sel) = Selector::parse("h1") else { return };
    if let Some(el) = dom.select(&sel).next() {
        let text: String = el.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            graph.heuristic_fills.insert(
                "title".to_string(),
                FieldValue::new(text, FieldSource::heuristic("first_h1")),
            );
        }
    }
}
