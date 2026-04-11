//! Published-date heuristic — first `<time datetime="...">`.

use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use scraper::{Html, Selector};

pub(super) fn apply(dom: &Html, graph: &mut MetaGraph) {
    if graph.heuristic_fills.contains_key("published_time") {
        return;
    }
    let Ok(sel) = Selector::parse("time[datetime]") else { return };
    if let Some(el) = dom.select(&sel).next() {
        if let Some(dt) = el.value().attr("datetime") {
            let trimmed = dt.trim();
            if !trimmed.is_empty() {
                graph.heuristic_fills.insert(
                    "published_time".to_string(),
                    FieldValue::new(
                        trimmed.to_string(),
                        FieldSource::heuristic("first_time_datetime"),
                    ),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FieldSource, MetaParser};

    #[test]
    fn time_datetime_fills_published() {
        let html = r#"<time datetime="2026-04-10T08:30:00Z">April 10</time>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let dt = graph.heuristic_fills.get("published_time").expect("published");
        assert_eq!(dt.value, "2026-04-10T08:30:00Z");
        assert!(matches!(dt.source, FieldSource::Heuristic(ref s) if s == "first_time_datetime"));
    }

    #[test]
    fn empty_datetime_ignored() {
        let html = r#"<time datetime="">Text</time>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(!graph.heuristic_fills.contains_key("published_time"));
    }

    #[test]
    fn time_without_datetime_attribute_ignored() {
        let html = r#"<time>April 10</time>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(!graph.heuristic_fills.contains_key("published_time"));
    }

    #[test]
    fn first_time_wins() {
        let html = r#"
            <time datetime="2026-01-01">First</time>
            <time datetime="2026-12-31">Second</time>
        "#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert_eq!(graph.heuristic_fills.get("published_time").unwrap().value, "2026-01-01");
    }
}
