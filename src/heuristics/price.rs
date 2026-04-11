//! Price heuristic — `[itemprop=price]`, `.price`, `[data-price]`.

use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use scraper::{Html, Selector};

const PRICE_RULES: &[(&str, &str)] = &[
    ("[itemprop=price]", "itemprop_price"),
    (".price", "class_price"),
    ("[data-price]", "data_price_attr"),
];

pub(super) fn apply(dom: &Html, graph: &mut MetaGraph) {
    if graph.heuristic_fills.contains_key("price") {
        return;
    }
    for (selector, rule_name) in PRICE_RULES {
        let Ok(sel) = Selector::parse(selector) else { continue };
        if let Some(el) = dom.select(&sel).next() {
            // Try data-price attribute first, then content attribute, then text.
            let raw = el
                .value()
                .attr("data-price")
                .map(str::to_string)
                .or_else(|| el.value().attr("content").map(str::to_string))
                .or_else(|| {
                    let text: String = el.text().collect::<String>().trim().to_string();
                    if text.is_empty() {
                        None
                    } else {
                        Some(text)
                    }
                });
            if let Some(value) = raw {
                graph.heuristic_fills.insert(
                    "price".to_string(),
                    FieldValue::new(value, FieldSource::heuristic(rule_name)),
                );
                return;
            }
        }
    }
}
