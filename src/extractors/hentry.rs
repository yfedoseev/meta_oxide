use crate::errors::{MicroformatError, Result};
use crate::types::{HCard, HEntry};
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Extract h-entry microformats from HTML
pub fn extract(html: &str, _base_url: Option<&str>) -> Result<Vec<HEntry>> {
    let document = Html::parse_document(html);
    let mut entries = Vec::new();

    let selector = Selector::parse(".h-entry")
        .map_err(|e| MicroformatError::ParseError(e.to_string()))?;

    for element in document.select(&selector) {
        let mut entry = HEntry {
            name: None,
            summary: None,
            content: None,
            published: None,
            updated: None,
            author: None,
            url: None,
            category: Vec::new(),
            additional_properties: HashMap::new(),
        };

        // Extract name (p-name)
        if let Ok(name_sel) = Selector::parse(".p-name") {
            if let Some(name_elem) = element.select(&name_sel).next() {
                entry.name = Some(name_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract summary (p-summary)
        if let Ok(summary_sel) = Selector::parse(".p-summary") {
            if let Some(summary_elem) = element.select(&summary_sel).next() {
                entry.summary = Some(summary_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract content (e-content)
        if let Ok(content_sel) = Selector::parse(".e-content") {
            if let Some(content_elem) = element.select(&content_sel).next() {
                entry.content = Some(content_elem.inner_html().trim().to_string());
            }
        }

        // Extract published date (dt-published)
        if let Ok(pub_sel) = Selector::parse(".dt-published") {
            if let Some(pub_elem) = element.select(&pub_sel).next() {
                entry.published = pub_elem
                    .value()
                    .attr("datetime")
                    .map(|s| s.to_string())
                    .or_else(|| Some(pub_elem.text().collect::<String>().trim().to_string()));
            }
        }

        // Extract updated date (dt-updated)
        if let Ok(upd_sel) = Selector::parse(".dt-updated") {
            if let Some(upd_elem) = element.select(&upd_sel).next() {
                entry.updated = upd_elem
                    .value()
                    .attr("datetime")
                    .map(|s| s.to_string())
                    .or_else(|| Some(upd_elem.text().collect::<String>().trim().to_string()));
            }
        }

        // Extract URL (u-url)
        if let Ok(url_sel) = Selector::parse(".u-url") {
            if let Some(url_elem) = element.select(&url_sel).next() {
                entry.url = url_elem
                    .value()
                    .attr("href")
                    .map(|s| s.to_string());
            }
        }

        // Extract categories (p-category)
        if let Ok(cat_sel) = Selector::parse(".p-category") {
            for cat_elem in element.select(&cat_sel) {
                entry.category.push(cat_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract author (p-author h-card)
        if let Ok(author_sel) = Selector::parse(".p-author.h-card") {
            if let Some(author_elem) = element.select(&author_sel).next() {
                let author_html = author_elem.html();
                if let Ok(cards) = super::hcard::extract(&author_html, None) {
                    if let Some(card) = cards.first() {
                        entry.author = Some(Box::new(card.clone()));
                    }
                }
            }
        }

        entries.push(entry);
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hentry() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">My Blog Post</h1>
                <div class="e-content">This is the content of my blog post.</div>
                <time class="dt-published" datetime="2024-01-01">January 1, 2024</time>
                <a class="p-category" href="/tag/rust">Rust</a>
            </article>
        "#;

        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, Some("My Blog Post".to_string()));
        assert_eq!(entries[0].published, Some("2024-01-01".to_string()));
        assert_eq!(entries[0].category, vec!["Rust"]);
    }
}
