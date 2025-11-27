use crate::microformat_extractor;
use crate::types::HFeed;

microformat_extractor! {
    HFeed, ".h-feed" {
        name: text(".p-name"),
        author: text(".p-author"),
        url: url(".u-url"),
        photo: url(".u-photo"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hfeed() {
        let html = r#"
            <div class="h-feed">
                <span class="p-name">My Blog</span>
                <span class="p-author">John Doe</span>
                <a class="u-url" href="https://example.com/feed">Feed</a>
            </div>
        "#;

        let feeds = extract(html, None).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].name, Some("My Blog".to_string()));
        assert_eq!(feeds[0].author, Some("John Doe".to_string()));
        assert_eq!(feeds[0].url, Some("https://example.com/feed".to_string()));
    }

    #[test]
    fn test_hfeed_with_photo() {
        let html = r#"
            <div class="h-feed">
                <span class="p-name">Photo Blog</span>
                <img class="u-photo" src="https://example.com/logo.jpg" alt="Logo" />
            </div>
        "#;

        let feeds = extract(html, None).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].photo, Some("https://example.com/logo.jpg".to_string()));
    }

    #[test]
    fn test_multiple_hfeeds() {
        let html = r#"
            <div class="h-feed">
                <span class="p-name">Feed 1</span>
            </div>
            <div class="h-feed">
                <span class="p-name">Feed 2</span>
            </div>
        "#;

        let feeds = extract(html, None).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].name, Some("Feed 1".to_string()));
        assert_eq!(feeds[1].name, Some("Feed 2".to_string()));
    }
}
