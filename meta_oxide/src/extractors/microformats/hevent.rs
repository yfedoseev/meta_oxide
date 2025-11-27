use crate::microformat_extractor;
use crate::types::HEvent;

microformat_extractor! {
    HEvent, ".h-event" {
        name: text(".p-name"),
        summary: text(".p-summary"),
        start: date(".dt-start"),
        end: date(".dt-end"),
        location: text(".p-location"),
        url: url(".u-url"),
        description: text(".p-description, .e-description"),
    }
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

    #[test]
    fn test_hevent_with_all_properties() {
        let html = r#"
            <div class="h-event">
                <h1 class="p-name">Tech Conference 2024</h1>
                <time class="dt-start" datetime="2024-09-10T09:00:00-07:00">Sept 10, 9 AM</time>
                <time class="dt-end" datetime="2024-09-12T17:00:00-07:00">Sept 12, 5 PM</time>
                <p class="p-location">San Francisco Convention Center</p>
                <p class="p-summary">Annual technology conference</p>
                <div class="e-description">
                    <p>Join us for three days of talks, workshops, and networking.</p>
                </div>
                <a class="u-url" href="https://conference.example.com">Event website</a>
                <span class="p-category">technology</span>
                <span class="p-category">conference</span>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.name, Some("Tech Conference 2024".to_string()));
        assert_eq!(event.start, Some("2024-09-10T09:00:00-07:00".to_string()));
        assert_eq!(event.end, Some("2024-09-12T17:00:00-07:00".to_string()));
        assert!(event.location.is_some());
    }

    #[test]
    fn test_hevent_minimal() {
        let html = r#"
            <div class="h-event">
                <span class="p-name">Simple Event</span>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.name, Some("Simple Event".to_string()));
    }

    #[test]
    fn test_hevent_with_nested_location() {
        let html = r#"
            <div class="h-event">
                <span class="p-name">Event</span>
                <div class="p-location h-card">
                    <span class="p-name">Venue Name</span>
                    <span class="p-street-address">123 Main St</span>
                    <span class="p-locality">San Francisco</span>
                </div>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        // Location should be extracted
    }

    #[test]
    fn test_multiple_hevents() {
        let html = r#"
            <div class="h-event">
                <span class="p-name">Event 1</span>
            </div>
            <div class="h-event">
                <span class="p-name">Event 2</span>
            </div>
            <div class="h-event">
                <span class="p-name">Event 3</span>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_hevent_with_relative_urls() {
        let html = r#"
            <div class="h-event">
                <span class="p-name">Event</span>
                <a class="u-url" href="/events/123">Event page</a>
            </div>
        "#;
        let events = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        // URLs should be resolved against base_url
        assert_eq!(event.url, Some("https://example.com/events/123".to_string()));
    }

    #[test]
    fn test_hevent_with_duration() {
        let html = r#"
            <div class="h-event">
                <span class="p-name">Workshop</span>
                <time class="dt-start" datetime="2024-03-15T14:00:00">2 PM</time>
                <time class="dt-duration" datetime="PT2H">2 hours</time>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        // Duration should be handled if implemented
    }

    #[test]
    fn test_hevent_with_special_characters() {
        let html = r#"
            <div class="h-event">
                <h1 class="p-name">Événement spécial 🎉</h1>
                <p class="p-location">Café "Le français"</p>
            </div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert!(event.name.as_ref().unwrap().contains("Événement"));
    }

    #[test]
    fn test_hevent_deeply_nested() {
        let html = r#"
            <div><div><div>
                <div class="h-event">
                    <span class="p-name">Nested Event</span>
                </div>
            </div></div></div>
        "#;
        let events = extract(html, None).unwrap();
        assert_eq!(events.len(), 1);
    }
}
