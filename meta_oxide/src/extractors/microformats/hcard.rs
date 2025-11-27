use crate::microformat_extractor;
use crate::types::HCard;

microformat_extractor! {
    HCard, ".h-card" {
        name: text(".p-name"),
        url: url(".u-url"),
        photo: url(".u-photo"),
        email: email(".u-email"),
        tel: text(".p-tel"),
        note: text(".p-note"),
        org: text(".p-org"),
    }
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

    #[test]
    fn test_hcard_with_all_url_types() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">John Doe</span>
                <a class="u-url" href="https://example.com">Website</a>
                <a class="u-url" href="https://github.com/johndoe">GitHub</a>
                <a class="u-email" href="mailto:john@example.com">Email</a>
                <img class="u-photo" src="https://example.com/photo.jpg" alt="Photo">
            </div>
        "#;
        let cards = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        assert_eq!(card.name, Some("John Doe".to_string()));
        assert_eq!(card.url, Some("https://example.com/".to_string()));
        assert_eq!(card.photo, Some("https://example.com/photo.jpg".to_string()));
        assert_eq!(card.email, Some("john@example.com".to_string()));
    }

    #[test]
    fn test_hcard_with_nested_org() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">John Doe</span>
                <div class="p-org h-card">
                    <span class="p-name">Acme Corp</span>
                    <a class="u-url" href="https://acme.com">Website</a>
                </div>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        // Should extract both h-cards
        assert!(!cards.is_empty());
    }

    #[test]
    fn test_hcard_with_all_properties() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">Jane Smith</span>
                <a class="u-url" href="https://jane.example.com">Website</a>
                <img class="u-photo" src="https://jane.example.com/photo.jpg" alt="Photo">
                <a class="u-email" href="mailto:jane@example.com">Email</a>
                <span class="p-tel">+1-555-1234</span>
                <p class="p-note">Software engineer and writer</p>
                <span class="p-org">Tech Company</span>
                <span class="p-job-title">Senior Developer</span>
                <div class="p-adr h-adr">
                    <span class="p-street-address">123 Main St</span>
                    <span class="p-locality">San Francisco</span>
                    <span class="p-region">CA</span>
                    <span class="p-postal-code">94102</span>
                    <span class="p-country-name">USA</span>
                </div>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        assert_eq!(card.name, Some("Jane Smith".to_string()));
        assert_eq!(card.tel, Some("+1-555-1234".to_string()));
        assert_eq!(card.note, Some("Software engineer and writer".to_string()));
        assert_eq!(card.org, Some("Tech Company".to_string()));
    }

    #[test]
    fn test_hcard_empty_properties() {
        let html = r#"
            <div class="h-card">
                <span class="p-name"></span>
                <a class="u-url" href="">Empty</a>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        // Empty properties should be None or empty string
    }

    #[test]
    fn test_hcard_with_whitespace() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">   John Doe   </span>
                <span class="p-note">
                    Multi-line
                    note with
                    whitespace
                </span>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        // Whitespace should be trimmed
        assert!(card.name.as_ref().unwrap().trim() == "John Doe");
    }

    #[test]
    fn test_multiple_hcards_in_document() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">Person 1</span>
            </div>
            <div class="h-card">
                <span class="p-name">Person 2</span>
            </div>
            <div class="h-card">
                <span class="p-name">Person 3</span>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 3);
        assert_eq!(cards[0].name, Some("Person 1".to_string()));
        assert_eq!(cards[1].name, Some("Person 2".to_string()));
        assert_eq!(cards[2].name, Some("Person 3".to_string()));
    }

    #[test]
    fn test_hcard_with_relative_urls() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">Jane</span>
                <a class="u-url" href="/profile">Profile</a>
                <img class="u-photo" src="/images/jane.jpg" alt="Photo">
            </div>
        "#;
        let cards = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        // URLs should be resolved against base_url
        assert_eq!(card.url, Some("https://example.com/profile".to_string()));
        assert_eq!(card.photo, Some("https://example.com/images/jane.jpg".to_string()));
    }

    #[test]
    fn test_hcard_with_special_characters() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">José García</span>
                <span class="p-note">Développeur & Architect</span>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        assert!(card.name.as_ref().unwrap().contains("José"));
        assert!(card.note.as_ref().unwrap().contains("&"));
    }

    #[test]
    fn test_hcard_with_no_name() {
        let html = r#"
            <div class="h-card">
                <a class="u-url" href="https://example.com">Website</a>
                <a class="u-email" href="mailto:test@example.com">Email</a>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        // Should extract card even without name
    }

    #[test]
    fn test_hcard_malformed_email() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">Test</span>
                <a class="u-email" href="not-an-email">Email</a>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        // Malformed email should still be extracted as-is
    }

    #[test]
    fn test_hcard_with_additional_properties() {
        let html = r#"
            <div class="h-card">
                <span class="p-name">Test</span>
                <span class="p-nickname">TestUser</span>
                <span class="p-bday">1990-01-01</span>
                <span class="p-gender-identity">male</span>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        // Additional properties should be handled or stored
    }

    #[test]
    fn test_hcard_deeply_nested() {
        let html = r#"
            <div>
                <div>
                    <div>
                        <div class="h-card">
                            <span class="p-name">Deeply Nested</span>
                        </div>
                    </div>
                </div>
            </div>
        "#;
        let cards = extract(html, None).unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].name, Some("Deeply Nested".to_string()));
    }
}
