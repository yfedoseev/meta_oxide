use crate::microformat_extractor;
use crate::types::HReview;

microformat_extractor! {
    HReview, ".h-review" {
        name: text(".p-name"),
        summary: text(".p-summary"),
        content: html(".e-content"),
        description: text(".p-description, .e-description"),
        published: date(".dt-published"),
        dtreviewed: date(".dt-reviewed"),
        rating: number(".p-rating"),
        best: number(".p-best"),
        worst: number(".p-worst"),
        url: url(".u-url")
        ;
        (reviewer, reviewer_card): nested_hcard_or_text(".p-reviewer.h-card", ".p-reviewer"),
        (item, item_product): nested_hproduct_or_text(".p-item.h-product", ".p-item")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hreview() {
        let html = r#"
            <div class="h-review">
                <span class="p-summary">Great product!</span>
                <span class="p-rating">4.5</span>
                <span class="p-reviewer">John Doe</span>
                <span class="p-item">Laptop</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].summary, Some("Great product!".to_string()));
        assert_eq!(reviews[0].rating, Some(4.5));
        assert_eq!(reviews[0].reviewer, Some("John Doe".to_string()));
        assert_eq!(reviews[0].item, Some("Laptop".to_string()));
    }

    #[test]
    fn test_hreview_with_rating_scale() {
        let html = r#"
            <div class="h-review">
                <span class="p-summary">Excellent service</span>
                <span class="p-rating">9</span>
                <span class="p-best">10</span>
                <span class="p-worst">1</span>
                <span class="p-reviewer">Jane Smith</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].rating, Some(9.0));
        assert_eq!(reviews[0].best, Some(10.0));
        assert_eq!(reviews[0].worst, Some(1.0));
    }

    #[test]
    fn test_hreview_with_date() {
        let html = r#"
            <div class="h-review">
                <span class="p-summary">Good experience</span>
                <time class="dt-reviewed" datetime="2024-01-15">January 15, 2024</time>
                <span class="p-reviewer">Bob Johnson</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].dtreviewed, Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_hreview_with_description() {
        let html = r#"
            <div class="h-review">
                <span class="p-summary">Fantastic!</span>
                <span class="p-rating">5</span>
                <div class="p-description">
                    This is the best product I've ever used. Highly recommend!
                </div>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert!(reviews[0].description.as_ref().unwrap().contains("best product"));
    }

    #[test]
    fn test_hreview_with_url() {
        let html = r#"
            <div class="h-review">
                <a class="u-url" href="https://example.com/review/123">
                    <span class="p-summary">Amazing</span>
                </a>
                <span class="p-rating">4.8</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].url, Some("https://example.com/review/123".to_string()));
    }

    #[test]
    fn test_multiple_hreviews() {
        let html = r#"
            <div class="h-review">
                <span class="p-summary">Review 1</span>
                <span class="p-rating">4</span>
            </div>
            <div class="h-review">
                <span class="p-summary">Review 2</span>
                <span class="p-rating">5</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].summary, Some("Review 1".to_string()));
        assert_eq!(reviews[1].summary, Some("Review 2".to_string()));
    }

    #[test]
    fn test_hreview_empty() {
        let html = "<html><body><p>No reviews here</p></body></html>";
        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 0);
    }
}
