use crate::errors::{MicroformatError, Result};
use crate::types::HEvent;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Extract h-event microformats from HTML
pub fn extract(html: &str, _base_url: Option<&str>) -> Result<Vec<HEvent>> {
    let document = Html::parse_document(html);
    let mut events = Vec::new();

    let selector = Selector::parse(".h-event")
        .map_err(|e| MicroformatError::ParseError(e.to_string()))?;

    for element in document.select(&selector) {
        let mut event = HEvent {
            name: None,
            summary: None,
            start: None,
            end: None,
            location: None,
            url: None,
            description: None,
            additional_properties: HashMap::new(),
        };

        // Extract name (p-name)
        if let Ok(name_sel) = Selector::parse(".p-name") {
            if let Some(name_elem) = element.select(&name_sel).next() {
                event.name = Some(name_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract summary (p-summary)
        if let Ok(summary_sel) = Selector::parse(".p-summary") {
            if let Some(summary_elem) = element.select(&summary_sel).next() {
                event.summary = Some(summary_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract start time (dt-start)
        if let Ok(start_sel) = Selector::parse(".dt-start") {
            if let Some(start_elem) = element.select(&start_sel).next() {
                event.start = start_elem
                    .value()
                    .attr("datetime")
                    .map(|s| s.to_string())
                    .or_else(|| Some(start_elem.text().collect::<String>().trim().to_string()));
            }
        }

        // Extract end time (dt-end)
        if let Ok(end_sel) = Selector::parse(".dt-end") {
            if let Some(end_elem) = element.select(&end_sel).next() {
                event.end = end_elem
                    .value()
                    .attr("datetime")
                    .map(|s| s.to_string())
                    .or_else(|| Some(end_elem.text().collect::<String>().trim().to_string()));
            }
        }

        // Extract location (p-location)
        if let Ok(loc_sel) = Selector::parse(".p-location") {
            if let Some(loc_elem) = element.select(&loc_sel).next() {
                event.location = Some(loc_elem.text().collect::<String>().trim().to_string());
            }
        }

        // Extract URL (u-url)
        if let Ok(url_sel) = Selector::parse(".u-url") {
            if let Some(url_elem) = element.select(&url_sel).next() {
                event.url = url_elem
                    .value()
                    .attr("href")
                    .map(|s| s.to_string());
            }
        }

        // Extract description (p-description or e-description)
        if let Ok(desc_sel) = Selector::parse(".p-description, .e-description") {
            if let Some(desc_elem) = element.select(&desc_sel).next() {
                event.description = Some(desc_elem.text().collect::<String>().trim().to_string());
            }
        }

        events.push(event);
    }

    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hevent() {
        let html = r#"
            <div class="h-event">
                <h1 class="p-name">Rust Conference 2024</h1>
                <time class="dt-start" datetime="2024-05-15T09:00">May 15, 2024 at 9:00 AM</time>
                <time class="dt-end" datetime="2024-05-15T17:00">May 15, 2024 at 5:00 PM</time>
                <span class="p-location">Convention Center</span>
            </div>
        "#;

        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, Some("Rust Conference 2024".to_string()));
        assert_eq!(events[0].start, Some("2024-05-15T09:00".to_string()));
        assert_eq!(events[0].location, Some("Convention Center".to_string()));
    }
}
