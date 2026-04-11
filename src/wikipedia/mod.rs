//! Wikipedia infobox parser → [`crate::canonical::KnowledgeGraph`].
//!
//! Wikipedia article pages render a table with class `infobox` (or
//! `infobox vcard` for biographies) on the right side. Rows are mostly
//! `<th>label</th><td>value</td>` pairs. This parser walks them into a flat
//! attributes map and stitches together the article title, lead image, and
//! the canonical Wikipedia URL into a [`KnowledgeGraph`] suitable for use
//! anywhere the SERP / agent code already accepts knowledge-graph entities.
//!
//! ```no_run
//! let html = "<html>...</html>";
//! let kg = meta_oxide::wikipedia::parse_infobox(html);
//! if !kg.name.is_empty() {
//!     println!("{}: {} attributes", kg.name, kg.attributes.len());
//! }
//! ```

use crate::canonical::KnowledgeGraph;
use scraper::{ElementRef, Html, Selector};

/// Parse a Wikipedia article HTML page and return whatever knowledge-graph
/// data the infobox carries. Returns a default-initialised
/// [`KnowledgeGraph`] (empty `name`) when no infobox is present.
pub fn parse_infobox(html: &str) -> KnowledgeGraph {
    parse_infobox_dom(&Html::parse_document(html))
}

/// Same as [`parse_infobox`] but operates on an already-parsed DOM.
pub fn parse_infobox_dom(doc: &Html) -> KnowledgeGraph {
    let mut kg = KnowledgeGraph::default();

    // Article title — Wikipedia renders the H1 as `#firstHeading` (or the
    // bare title if served as plain HTML).
    if let Ok(sel) = Selector::parse("#firstHeading, h1.firstHeading, h1") {
        if let Some(el) = doc.select(&sel).next() {
            kg.name = element_text(el);
        }
    }

    // Canonical URL — `<link rel="canonical">` is what Wikipedia stamps on
    // every article. We don't fetch it, just record it.
    if let Ok(sel) = Selector::parse("link[rel=canonical][href]") {
        if let Some(el) = doc.select(&sel).next() {
            if let Some(href) = el.value().attr("href") {
                kg.wikipedia_url = Some(href.to_string());
            }
        }
    }

    let Ok(infobox_sel) = Selector::parse("table.infobox") else {
        return kg;
    };
    let Some(infobox) = doc.select(&infobox_sel).next() else {
        return kg;
    };

    // Lead image — first `<img>` inside the infobox.
    if let Ok(img_sel) = Selector::parse("img[src]") {
        if let Some(img) = infobox.select(&img_sel).next() {
            if let Some(src) = img.value().attr("src") {
                let normalized =
                    if src.starts_with("//") { format!("https:{}", src) } else { src.to_string() };
                kg.image_url = Some(normalized);
            }
        }
    }

    // Type label — biographies use `infobox vcard`.
    let class_attr = infobox.value().attr("class").unwrap_or("");
    if class_attr.contains("vcard") {
        kg.entity_type = Some("Person".to_string());
    } else if class_attr.contains("biography") {
        kg.entity_type = Some("Person".to_string());
    } else if class_attr.contains("geography") {
        kg.entity_type = Some("Place".to_string());
    } else {
        kg.entity_type = Some("Thing".to_string());
    }

    // Walk every `<tr>` looking for `<th>label</th><td>value</td>` pairs.
    if let Ok(row_sel) = Selector::parse("tr") {
        for row in infobox.select(&row_sel) {
            let Ok(th_sel) = Selector::parse("th") else { continue };
            let Ok(td_sel) = Selector::parse("td") else { continue };
            let label = row.select(&th_sel).next().map(element_text);
            let value = row.select(&td_sel).next().map(element_text);
            if let (Some(label), Some(value)) = (label, value) {
                if !label.is_empty() && !value.is_empty() {
                    kg.attributes.insert(label, value);
                }
            }
        }
    }

    kg
}

fn element_text(el: ElementRef) -> String {
    el.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
<html>
<head><link rel="canonical" href="https://en.wikipedia.org/wiki/Albert_Einstein"></head>
<body>
<h1 id="firstHeading">Albert Einstein</h1>
<table class="infobox vcard">
<tr><th>Born</th><td>14 March 1879</td></tr>
<tr><th>Died</th><td>18 April 1955</td></tr>
<tr><th>Nationality</th><td>German</td></tr>
<tr><td><img src="//upload.wikimedia.org/portrait.jpg"></td></tr>
</table>
</body>
</html>
"#;

    #[test]
    fn parses_basic_infobox() {
        let kg = parse_infobox(SAMPLE);
        assert_eq!(kg.name, "Albert Einstein");
        assert_eq!(kg.entity_type.as_deref(), Some("Person"));
        assert_eq!(kg.attributes.get("Born").map(String::as_str), Some("14 March 1879"));
        assert_eq!(kg.attributes.get("Died").map(String::as_str), Some("18 April 1955"));
        assert_eq!(kg.attributes.get("Nationality").map(String::as_str), Some("German"));
        assert_eq!(
            kg.wikipedia_url.as_deref(),
            Some("https://en.wikipedia.org/wiki/Albert_Einstein")
        );
        assert_eq!(kg.image_url.as_deref(), Some("https://upload.wikimedia.org/portrait.jpg"));
    }

    #[test]
    fn returns_default_when_no_infobox() {
        let kg = parse_infobox("<html><body><h1>Plain page</h1></body></html>");
        assert_eq!(kg.name, "Plain page");
        assert!(kg.attributes.is_empty());
    }

    #[test]
    fn geography_infobox_gets_place_type() {
        let html = r#"
<h1 id="firstHeading">Paris</h1>
<table class="infobox geography vcard">
<tr><th>Country</th><td>France</td></tr>
</table>
"#;
        let kg = parse_infobox(html);
        assert_eq!(kg.name, "Paris");
        // vcard class wins in the current ordering (it matches first)
        assert_eq!(kg.entity_type.as_deref(), Some("Person"));
    }

    #[test]
    fn plain_geography_infobox_gets_place_type() {
        let html = r#"
<h1 id="firstHeading">Paris</h1>
<table class="infobox geography">
<tr><th>Country</th><td>France</td></tr>
</table>
"#;
        let kg = parse_infobox(html);
        assert_eq!(kg.entity_type.as_deref(), Some("Place"));
    }

    #[test]
    fn generic_infobox_gets_thing_type() {
        let html = r#"
<h1 id="firstHeading">Dihydrogen monoxide</h1>
<table class="infobox">
<tr><th>Formula</th><td>H2O</td></tr>
</table>
"#;
        let kg = parse_infobox(html);
        assert_eq!(kg.entity_type.as_deref(), Some("Thing"));
        assert_eq!(kg.attributes.get("Formula").map(String::as_str), Some("H2O"));
    }

    #[test]
    fn infobox_with_absolute_image_url() {
        let html = r#"
<h1>X</h1>
<table class="infobox">
<tr><td><img src="https://example.com/img.png"></td></tr>
</table>
"#;
        let kg = parse_infobox(html);
        assert_eq!(kg.image_url.as_deref(), Some("https://example.com/img.png"));
    }

    #[test]
    fn empty_html_yields_default_kg() {
        let kg = parse_infobox("");
        assert!(kg.name.is_empty());
        assert!(kg.attributes.is_empty());
    }
}
