use crate::microformat_extractor;
use crate::types::HAdr;

microformat_extractor! {
    HAdr, ".h-adr" {
        street_address: text(".p-street-address"),
        extended_address: text(".p-extended-address"),
        post_office_box: text(".p-post-office-box"),
        locality: text(".p-locality"),
        region: text(".p-region"),
        postal_code: text(".p-postal-code"),
        country_name: text(".p-country-name"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hadr() {
        let html = r#"
            <div class="h-adr">
                <span class="p-street-address">123 Main St</span>
                <span class="p-locality">San Francisco</span>
                <span class="p-region">CA</span>
                <span class="p-postal-code">94102</span>
                <span class="p-country-name">USA</span>
            </div>
        "#;

        let addresses = extract(html, None).unwrap();
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].street_address, Some("123 Main St".to_string()));
        assert_eq!(addresses[0].locality, Some("San Francisco".to_string()));
        assert_eq!(addresses[0].region, Some("CA".to_string()));
        assert_eq!(addresses[0].postal_code, Some("94102".to_string()));
        assert_eq!(addresses[0].country_name, Some("USA".to_string()));
    }

    #[test]
    fn test_hadr_with_extended() {
        let html = r#"
            <div class="h-adr">
                <span class="p-street-address">123 Main Street</span>
                <span class="p-extended-address">Suite 400</span>
                <span class="p-locality">San Francisco</span>
            </div>
        "#;

        let addresses = extract(html, None).unwrap();
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].extended_address, Some("Suite 400".to_string()));
    }

    #[test]
    fn test_hadr_with_po_box() {
        let html = r#"
            <div class="h-adr">
                <span class="p-post-office-box">PO Box 123</span>
                <span class="p-locality">Springfield</span>
            </div>
        "#;

        let addresses = extract(html, None).unwrap();
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].post_office_box, Some("PO Box 123".to_string()));
    }

    #[test]
    fn test_multiple_hadr() {
        let html = r#"
            <div class="h-adr">
                <span class="p-locality">New York</span>
            </div>
            <div class="h-adr">
                <span class="p-locality">Los Angeles</span>
            </div>
        "#;

        let addresses = extract(html, None).unwrap();
        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].locality, Some("New York".to_string()));
        assert_eq!(addresses[1].locality, Some("Los Angeles".to_string()));
    }
}
