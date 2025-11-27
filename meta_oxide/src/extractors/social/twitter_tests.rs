//! Tests for Twitter Card extraction

#[cfg(test)]
mod tests {
    use crate::extractors::social::twitter::{extract, extract_with_fallback};

    #[test]
    fn test_basic_twitter_card() {
        let html = r#"
            <meta name="twitter:card" content="summary">
            <meta name="twitter:title" content="Test Tweet">
            <meta name="twitter:description" content="Tweet description">
            <meta name="twitter:image" content="https://example.com/image.jpg">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("summary".to_string()));
        assert_eq!(card.title, Some("Test Tweet".to_string()));
        assert_eq!(card.description, Some("Tweet description".to_string()));
        assert_eq!(card.image, Some("https://example.com/image.jpg".to_string()));
    }

    #[test]
    fn test_twitter_card_large_image() {
        let html = r#"<meta name="twitter:card" content="summary_large_image">"#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("summary_large_image".to_string()));
    }

    #[test]
    fn test_twitter_card_app() {
        let html = r#"<meta name="twitter:card" content="app">"#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("app".to_string()));
    }

    #[test]
    fn test_twitter_card_player() {
        let html = r#"<meta name="twitter:card" content="player">"#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("player".to_string()));
    }

    #[test]
    fn test_twitter_site_and_creator() {
        let html = r#"
            <meta name="twitter:site" content="@example">
            <meta name="twitter:creator" content="@author">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.site, Some("@example".to_string()));
        assert_eq!(card.creator, Some("@author".to_string()));
    }

    #[test]
    fn test_twitter_site_and_creator_ids() {
        let html = r#"
            <meta name="twitter:site:id" content="123456">
            <meta name="twitter:creator:id" content="789012">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.site_id, Some("123456".to_string()));
        assert_eq!(card.creator_id, Some("789012".to_string()));
    }

    #[test]
    fn test_twitter_image_alt() {
        let html = r#"<meta name="twitter:image:alt" content="Image description">"#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.image_alt, Some("Image description".to_string()));
    }

    #[test]
    fn test_twitter_player_card_full() {
        let html = r#"
            <meta name="twitter:card" content="player">
            <meta name="twitter:player" content="https://example.com/player">
            <meta name="twitter:player:width" content="1280">
            <meta name="twitter:player:height" content="720">
            <meta name="twitter:player:stream" content="https://example.com/stream.mp4">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, Some("player".to_string()));
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        assert_eq!(player.url, "https://example.com/player");
        assert_eq!(player.width, Some(1280));
        assert_eq!(player.height, Some(720));
        assert_eq!(player.stream, Some("https://example.com/stream.mp4".to_string()));
    }

    #[test]
    fn test_twitter_player_minimal() {
        let html = r#"<meta name="twitter:player" content="https://example.com/player">"#;
        let card = extract(html, None).unwrap();
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        assert_eq!(player.url, "https://example.com/player");
        assert_eq!(player.width, None);
        assert_eq!(player.height, None);
    }

    #[test]
    fn test_twitter_app_card_iphone() {
        let html = r#"
            <meta name="twitter:card" content="app">
            <meta name="twitter:app:name:iphone" content="MyApp">
            <meta name="twitter:app:id:iphone" content="123456">
            <meta name="twitter:app:url:iphone" content="myapp://open">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_iphone, Some("MyApp".to_string()));
        assert_eq!(app.id_iphone, Some("123456".to_string()));
        assert_eq!(app.url_iphone, Some("myapp://open".to_string()));
    }

    #[test]
    fn test_twitter_app_card_ipad() {
        let html = r#"
            <meta name="twitter:app:name:ipad" content="MyApp HD">
            <meta name="twitter:app:id:ipad" content="234567">
            <meta name="twitter:app:url:ipad" content="myapp://open-ipad">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_ipad, Some("MyApp HD".to_string()));
        assert_eq!(app.id_ipad, Some("234567".to_string()));
        assert_eq!(app.url_ipad, Some("myapp://open-ipad".to_string()));
    }

    #[test]
    fn test_twitter_app_card_googleplay() {
        let html = r#"
            <meta name="twitter:app:name:googleplay" content="My Android App">
            <meta name="twitter:app:id:googleplay" content="com.example.app">
            <meta name="twitter:app:url:googleplay" content="myapp://open-android">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_googleplay, Some("My Android App".to_string()));
        assert_eq!(app.id_googleplay, Some("com.example.app".to_string()));
        assert_eq!(app.url_googleplay, Some("myapp://open-android".to_string()));
    }

    #[test]
    fn test_twitter_app_card_all_platforms() {
        let html = r#"
            <meta name="twitter:card" content="app">
            <meta name="twitter:app:name:iphone" content="MyApp">
            <meta name="twitter:app:id:iphone" content="123456">
            <meta name="twitter:app:name:ipad" content="MyApp HD">
            <meta name="twitter:app:id:ipad" content="234567">
            <meta name="twitter:app:name:googleplay" content="My Android App">
            <meta name="twitter:app:id:googleplay" content="com.example.app">
            <meta name="twitter:app:country" content="US">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_iphone, Some("MyApp".to_string()));
        assert_eq!(app.name_ipad, Some("MyApp HD".to_string()));
        assert_eq!(app.name_googleplay, Some("My Android App".to_string()));
        assert_eq!(app.country, Some("US".to_string()));
    }

    #[test]
    fn test_twitter_relative_url() {
        let html = r#"<meta name="twitter:image" content="/images/photo.jpg">"#;
        let card = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(card.image, Some("https://example.com/images/photo.jpg".to_string()));
    }

    #[test]
    fn test_twitter_player_relative_url() {
        let html = r#"
            <meta name="twitter:player" content="/player/embed">
            <meta name="twitter:player:stream" content="/videos/stream.mp4">
        "#;
        let card = extract(html, Some("https://example.com")).unwrap();
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        assert_eq!(player.url, "https://example.com/player/embed");
        assert_eq!(player.stream, Some("https://example.com/videos/stream.mp4".to_string()));
    }

    #[test]
    fn test_empty_html() {
        let html = "";
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, None);
        assert_eq!(card.title, None);
    }

    #[test]
    fn test_no_twitter_tags() {
        let html = r#"
            <html>
            <head>
                <title>Regular HTML</title>
                <meta name="description" content="Not Twitter">
            </head>
            </html>
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.card, None);
    }

    #[test]
    fn test_whitespace_handling() {
        let html = r#"
            <meta name="twitter:title" content="  Title with spaces  ">
            <meta name="twitter:description" content="
                Description with
                newlines
            ">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.title, Some("Title with spaces".to_string()));
        // Description should be trimmed
        assert!(card.description.is_some());
        let desc = card.description.unwrap();
        assert!(!desc.starts_with(' '));
        assert!(!desc.ends_with(' '));
    }

    #[test]
    fn test_empty_content_ignored() {
        let html = r#"
            <meta name="twitter:title" content="">
            <meta name="twitter:description" content="   ">
        "#;
        let card = extract(html, None).unwrap();
        assert_eq!(card.title, None);
        assert_eq!(card.description, None);
    }

    #[test]
    fn test_malformed_player_dimensions() {
        let html = r#"
            <meta name="twitter:player" content="https://example.com/player">
            <meta name="twitter:player:width" content="not-a-number">
            <meta name="twitter:player:height" content="abc">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        // Invalid dimensions should be ignored
        assert_eq!(player.width, None);
        assert_eq!(player.height, None);
    }

    #[test]
    fn test_real_world_summary_card() {
        let html = r#"
            <meta name="twitter:card" content="summary">
            <meta name="twitter:site" content="@example">
            <meta name="twitter:title" content="Article Title">
            <meta name="twitter:description" content="Article description">
            <meta name="twitter:image" content="https://example.com/image.jpg">
        "#;
        let card = extract(html, None).unwrap();

        assert_eq!(card.card, Some("summary".to_string()));
        assert_eq!(card.site, Some("@example".to_string()));
        assert_eq!(card.title, Some("Article Title".to_string()));
        assert_eq!(card.description, Some("Article description".to_string()));
        assert_eq!(card.image, Some("https://example.com/image.jpg".to_string()));
    }

    #[test]
    fn test_real_world_large_image_card() {
        let html = r#"
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@nytimes">
            <meta name="twitter:creator" content="@journalist">
            <meta name="twitter:title" content="Breaking News Story">
            <meta name="twitter:description" content="Important development in ongoing story">
            <meta name="twitter:image" content="https://news.example.com/hero-image.jpg">
            <meta name="twitter:image:alt" content="Photo of the scene">
        "#;
        let card = extract(html, None).unwrap();

        assert_eq!(card.card, Some("summary_large_image".to_string()));
        assert_eq!(card.site, Some("@nytimes".to_string()));
        assert_eq!(card.creator, Some("@journalist".to_string()));
        assert_eq!(card.image_alt, Some("Photo of the scene".to_string()));
    }

    #[test]
    fn test_real_world_player_card() {
        let html = r#"
            <meta name="twitter:card" content="player">
            <meta name="twitter:site" content="@youtube">
            <meta name="twitter:title" content="Funny Cat Video">
            <meta name="twitter:description" content="You won't believe what this cat does">
            <meta name="twitter:image" content="https://youtube.com/thumbnail.jpg">
            <meta name="twitter:player" content="https://youtube.com/embed/abc123">
            <meta name="twitter:player:width" content="1280">
            <meta name="twitter:player:height" content="720">
            <meta name="twitter:player:stream" content="https://youtube.com/stream/abc123.mp4">
        "#;
        let card = extract(html, None).unwrap();

        assert_eq!(card.card, Some("player".to_string()));
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        assert_eq!(player.width, Some(1280));
        assert_eq!(player.height, Some(720));
    }

    #[test]
    fn test_real_world_app_card() {
        let html = r#"
            <meta name="twitter:card" content="app">
            <meta name="twitter:site" content="@appstore">
            <meta name="twitter:title" content="Download Our App">
            <meta name="twitter:description" content="The best app for productivity">
            <meta name="twitter:image" content="https://apps.example.com/icon.png">
            <meta name="twitter:app:name:iphone" content="Productivity Plus">
            <meta name="twitter:app:id:iphone" content="987654321">
            <meta name="twitter:app:url:iphone" content="productivityplus://open">
            <meta name="twitter:app:name:googleplay" content="Productivity Plus">
            <meta name="twitter:app:id:googleplay" content="com.example.productivityplus">
        "#;
        let card = extract(html, None).unwrap();

        assert_eq!(card.card, Some("app".to_string()));
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_iphone, Some("Productivity Plus".to_string()));
        assert_eq!(app.id_iphone, Some("987654321".to_string()));
    }

    #[test]
    fn test_case_sensitivity() {
        // Twitter meta tags use name attribute (case-insensitive in HTML)
        let html = r#"
            <meta name="twitter:title" content="Lowercase">
            <meta name="TWITTER:TITLE" content="Uppercase">
            <meta name="Twitter:Title" content="Mixed">
        "#;
        let card = extract(html, None).unwrap();
        // Should match lowercase version (CSS selector is case-insensitive for name)
        assert_eq!(card.title, Some("Lowercase".to_string()));
    }

    #[test]
    fn test_special_characters_in_content() {
        let html = r#"
            <meta name="twitter:title" content="Title with &quot;quotes&quot; &amp; &lt;tags&gt;">
            <meta name="twitter:description" content="Description with &#x27;apostrophes&#x27;">
        "#;
        let card = extract(html, None).unwrap();
        // HTML entities should be decoded
        assert!(card.title.is_some());
        assert!(card.description.is_some());
    }

    #[test]
    fn test_player_without_dimensions() {
        let html = r#"<meta name="twitter:player" content="https://example.com/player">"#;
        let card = extract(html, None).unwrap();
        assert!(card.player.is_some());
        let player = card.player.unwrap();
        assert_eq!(player.url, "https://example.com/player");
        assert_eq!(player.width, None);
        assert_eq!(player.height, None);
        assert_eq!(player.stream, None);
    }

    #[test]
    fn test_app_metadata_without_main_tag() {
        // App metadata without twitter:app:name should still be captured
        let html = r#"
            <meta name="twitter:app:id:iphone" content="123456">
            <meta name="twitter:app:url:iphone" content="myapp://open">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.id_iphone, Some("123456".to_string()));
    }

    #[test]
    fn test_player_metadata_without_player_tag() {
        // Player dimensions without twitter:player URL should be ignored
        let html = r#"
            <meta name="twitter:player:width" content="1280">
            <meta name="twitter:player:height" content="720">
        "#;
        let card = extract(html, None).unwrap();
        // Without player URL, no player object should be created
        assert_eq!(card.player, None);
    }

    #[test]
    fn test_complete_card() {
        let html = r#"
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@example">
            <meta name="twitter:site:id" content="111111">
            <meta name="twitter:creator" content="@author">
            <meta name="twitter:creator:id" content="222222">
            <meta name="twitter:title" content="Complete Example">
            <meta name="twitter:description" content="All fields filled">
            <meta name="twitter:image" content="https://example.com/image.jpg">
            <meta name="twitter:image:alt" content="Image description">
        "#;
        let card = extract(html, None).unwrap();

        assert_eq!(card.card, Some("summary_large_image".to_string()));
        assert_eq!(card.site, Some("@example".to_string()));
        assert_eq!(card.site_id, Some("111111".to_string()));
        assert_eq!(card.creator, Some("@author".to_string()));
        assert_eq!(card.creator_id, Some("222222".to_string()));
        assert_eq!(card.title, Some("Complete Example".to_string()));
        assert_eq!(card.description, Some("All fields filled".to_string()));
        assert_eq!(card.image, Some("https://example.com/image.jpg".to_string()));
        assert_eq!(card.image_alt, Some("Image description".to_string()));
    }

    // ========== PHASE 3: EDGE CASES AND STRESS TESTS ==========

    #[test]
    fn test_twitter_with_invalid_dimensions() {
        let html = r#"
            <meta name="twitter:card" content="player">
            <meta name="twitter:player" content="https://example.com/player">
            <meta name="twitter:player:width" content="invalid">
            <meta name="twitter:player:height" content="-100">
        "#;
        let card = extract(html, None).unwrap();
        // Should handle invalid dimensions gracefully
        assert!(card.player.is_some());
    }

    #[test]
    fn test_twitter_app_partial_data() {
        let html = r#"
            <meta name="twitter:card" content="app">
            <meta name="twitter:app:name:iphone" content="MyApp">
            <meta name="twitter:app:id:iphone" content="123">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.app.is_some());
        let app = card.app.unwrap();
        assert_eq!(app.name_iphone, Some("MyApp".to_string()));
        // Other platforms should be None
        assert!(app.name_ipad.is_none());
    }

    #[test]
    fn test_twitter_with_very_long_content() {
        let long_title = "T".repeat(5000);
        let html = format!(r#"<meta name="twitter:title" content="{}">"#, long_title);
        let card = extract(&html, None).unwrap();
        assert_eq!(card.title, Some(long_title));
    }

    #[test]
    fn test_twitter_with_emoji_and_unicode() {
        let html = r#"
            <meta name="twitter:title" content="🎉 Announcement! 日本語">
            <meta name="twitter:description" content="Check this out 😊 Ça marche!">
        "#;
        let card = extract(html, None).unwrap();
        assert!(card.title.unwrap().contains("🎉"));
        assert!(card.description.unwrap().contains("😊"));
    }

    #[test]
    fn test_twitter_fallback_with_og_unicode() {
        let html = r#"
            <meta property="og:title" content="日本語タイトル">
            <meta property="og:description" content="Français description">
        "#;
        let card = extract_with_fallback(html, None).unwrap();
        assert!(card.title.unwrap().contains("日本語"));
    }

    #[test]
    fn test_twitter_with_malformed_player_url() {
        let html = r#"
            <meta name="twitter:card" content="player">
            <meta name="twitter:player" content="not a url">
        "#;
        let card = extract(html, None).unwrap();
        // Should handle malformed URL gracefully
        assert!(card.player.is_some());
    }
}
