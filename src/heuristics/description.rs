//! Description heuristic — first `<p>` longer than 20 characters.

use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use scraper::{Html, Selector};

const MIN_DESCRIPTION_LEN: usize = 20;

pub(super) fn apply(dom: &Html, graph: &mut MetaGraph) {
    if graph.meta.description.is_some()
        || graph.open_graph.description.is_some()
        || graph.twitter.description.is_some()
        || graph.heuristic_fills.contains_key("description")
    {
        return;
    }
    let Ok(sel) = Selector::parse("p") else { return };
    for el in dom.select(&sel) {
        let text: String =
            el.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
        if text.len() >= MIN_DESCRIPTION_LEN {
            graph.heuristic_fills.insert(
                "description".to_string(),
                FieldValue::new(text, FieldSource::heuristic("first_paragraph")),
            );
            break;
        }
    }
}
