use crate::errors::{MicroformatError, Result};
use crate::types::HCard;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Extract h-card microformats from HTML
pub fn extract(html: &str, _base_url: Option<&str>) -> Result<Vec<HCard>> {
    let document = Html::parse_document(html);
    let mut cards = Vec::new();

    let selector = Selector::parse(".h-card")
        .map_err(|e| MicroformatError::ParseError(e.to_string()))?;

    for element in document.select(&selector) {
        let mut card = HCard {
            name: None,
            url: None,
            photo: None,
            email: None,
            tel: None,
            note: None,
            org: None,
            additional_properties: HashMap::new(),
        };

        // Extract name (p-name)
        if let Ok(name_sel) = Selector::parse(".p-name") {
            if let Some(name_elem) = element.select(&name_sel).next() {
                card.name = Some(name_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract URL (u-url)
        if let Ok(url_sel) = Selector::parse(".u-url") {
            if let Some(url_elem) = element.select(&url_sel).next() {
                card.url = url_elem
                    .value()
                    .attr("href")
                    .map(|s| s.to_string());
            }
        }

        // Extract photo (u-photo)
        if let Ok(photo_sel) = Selector::parse(".u-photo") {
            if let Some(photo_elem) = element.select(&photo_sel).next() {
                card.photo = photo_elem
                    .value()
                    .attr("src")
                    .or_else(|| photo_elem.value().attr("href"))
                    .map(|s| s.to_string());
            }
        }

        // Extract email (u-email)
        if let Ok(email_sel) = Selector::parse(".u-email") {
            if let Some(email_elem) = element.select(&email_sel).next() {
                let email = email_elem
                    .value()
                    .attr("href")
                    .map(|s| s.trim_start_matches("mailto:").to_string())
                    .unwrap_or_else(|| email_elem.text().collect::<String>().trim().to_string());
                card.email = Some(email);
            }
        }

        // Extract telephone (p-tel)
        if let Ok(tel_sel) = Selector::parse(".p-tel") {
            if let Some(tel_elem) = element.select(&tel_sel).next() {
                card.tel = Some(tel_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract note (p-note)
        if let Ok(note_sel) = Selector::parse(".p-note") {
            if let Some(note_elem) = element.select(&note_sel).next() {
                card.note = Some(note_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract organization (p-org)
        if let Ok(org_sel) = Selector::parse(".p-org") {
            if let Some(org_elem) = element.select(&org_sel).next() {
                card.org = Some(org_elem.text().collect::<String>().trim().to_string());
            }
        }

        cards.push(card);
    }

    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hcard() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">John Doe</span>
                <a class="u-url" href="https://example.com">Website</a>
                <a class="u-email" href="mailto:john@example.com">Email</a>
            </div>
        "#;

        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].name, Some("John Doe".to_string()));
        assert_eq!(cards[0].url, Some("https://example.com".to_string()));
        assert_eq!(cards[0].email, Some("john@example.com".to_string()));
    }
}
