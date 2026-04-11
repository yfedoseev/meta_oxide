//! Body heuristic — text content of `<article>` or the largest `<main>` child.

use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use scraper::{Html, Selector};

pub(super) fn apply(dom: &Html, graph: &mut MetaGraph) {
    if graph.heuristic_fills.contains_key("body") {
        return;
    }
    if let Ok(sel) = Selector::parse("article") {
        if let Some(el) = dom.select(&sel).next() {
            let text: String =
                el.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
            if !text.is_empty() {
                graph.heuristic_fills.insert(
                    "body".to_string(),
                    FieldValue::new(text, FieldSource::heuristic("article_tag")),
                );
                return;
            }
        }
    }
    if let Ok(sel) = Selector::parse("main") {
        if let Some(el) = dom.select(&sel).next() {
            let text: String =
                el.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
            if !text.is_empty() {
                graph.heuristic_fills.insert(
                    "body".to_string(),
                    FieldValue::new(text, FieldSource::heuristic("main_tag")),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FieldSource, MetaParser};

    #[test]
    fn article_tag_fills_body() {
        let html = r#"<html><body><article>This is the article body text.</article></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let body = graph.heuristic_fills.get("body").expect("body from article");
        assert_eq!(body.value, "This is the article body text.");
        assert!(matches!(body.source, FieldSource::Heuristic(ref s) if s == "article_tag"));
    }

    #[test]
    fn main_tag_fills_body_when_no_article() {
        let html = r#"<html><body><main>Content inside main.</main></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let body = graph.heuristic_fills.get("body").expect("body from main");
        assert_eq!(body.value, "Content inside main.");
        assert!(matches!(body.source, FieldSource::Heuristic(ref s) if s == "main_tag"));
    }

    #[test]
    fn article_wins_over_main() {
        let html = r#"<article>Article text.</article><main>Main text.</main>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert_eq!(graph.heuristic_fills.get("body").unwrap().value, "Article text.");
    }

    #[test]
    fn no_body_when_neither_tag_present() {
        let html = r#"<html><body><p>Just a paragraph.</p></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(!graph.heuristic_fills.contains_key("body"));
    }

    #[test]
    fn empty_article_skipped() {
        let html = r#"<article></article><main>Main only.</main>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        // Empty article should fall through to main.
        assert_eq!(graph.heuristic_fills.get("body").unwrap().value, "Main only.");
    }
}
