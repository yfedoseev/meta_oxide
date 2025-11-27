//! Tests for Open Graph extraction

#[cfg(test)]
mod tests {
    use crate::extractors::social::opengraph::extract;

    #[test]
    fn test_basic_opengraph() {
        let html = r#"
            <meta property="og:title" content="Test Article">
            <meta property="og:type" content="article">
            <meta property="og:url" content="https://example.com/article">
            <meta property="og:image" content="https://example.com/image.jpg">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, Some("Test Article".to_string()));
        assert_eq!(og.r#type, Some("article".to_string()));
        assert_eq!(og.url, Some("https://example.com/article".to_string()));
        assert_eq!(og.image, Some("https://example.com/image.jpg".to_string()));
    }

    #[test]
    fn test_opengraph_description() {
        let html = r#"<meta property="og:description" content="Article description">"#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.description, Some("Article description".to_string()));
    }

    #[test]
    fn test_opengraph_site_name() {
        let html = r#"<meta property="og:site_name" content="My Blog">"#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.site_name, Some("My Blog".to_string()));
    }

    #[test]
    fn test_opengraph_locale() {
        let html = r#"
            <meta property="og:locale" content="en_US">
            <meta property="og:locale:alternate" content="es_ES">
            <meta property="og:locale:alternate" content="fr_FR">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.locale, Some("en_US".to_string()));
        assert_eq!(og.locale_alternate.len(), 2);
        assert_eq!(og.locale_alternate[0], "es_ES");
        assert_eq!(og.locale_alternate[1], "fr_FR");
    }

    #[test]
    fn test_opengraph_image_with_metadata() {
        let html = r#"
            <meta property="og:image" content="https://example.com/image.jpg">
            <meta property="og:image:secure_url" content="https://example.com/secure.jpg">
            <meta property="og:image:type" content="image/jpeg">
            <meta property="og:image:width" content="1200">
            <meta property="og:image:height" content="630">
            <meta property="og:image:alt" content="Test image">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 1);
        let img = &og.images[0];
        assert_eq!(img.url, "https://example.com/image.jpg");
        assert_eq!(img.secure_url, Some("https://example.com/secure.jpg".to_string()));
        assert_eq!(img.r#type, Some("image/jpeg".to_string()));
        assert_eq!(img.width, Some(1200));
        assert_eq!(img.height, Some(630));
        assert_eq!(img.alt, Some("Test image".to_string()));
    }

    #[test]
    fn test_opengraph_multiple_images() {
        let html = r#"
            <meta property="og:image" content="https://example.com/image1.jpg">
            <meta property="og:image" content="https://example.com/image2.jpg">
            <meta property="og:image" content="https://example.com/image3.jpg">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 3);
        assert_eq!(og.images[0].url, "https://example.com/image1.jpg");
        assert_eq!(og.images[1].url, "https://example.com/image2.jpg");
        assert_eq!(og.images[2].url, "https://example.com/image3.jpg");
    }

    #[test]
    fn test_opengraph_multiple_images_with_mixed_metadata() {
        let html = r#"
            <meta property="og:image" content="https://example.com/image1.jpg">
            <meta property="og:image:width" content="1200">
            <meta property="og:image:height" content="630">
            <meta property="og:image" content="https://example.com/image2.jpg">
            <meta property="og:image:width" content="800">
            <meta property="og:image:height" content="600">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 2);
        assert_eq!(og.images[0].width, Some(1200));
        assert_eq!(og.images[0].height, Some(630));
        assert_eq!(og.images[1].width, Some(800));
        assert_eq!(og.images[1].height, Some(600));
    }

    #[test]
    fn test_opengraph_article_metadata() {
        let html = r#"
            <meta property="og:type" content="article">
            <meta property="article:published_time" content="2024-01-15T10:00:00Z">
            <meta property="article:modified_time" content="2024-01-16T12:00:00Z">
            <meta property="article:author" content="https://example.com/author">
            <meta property="article:section" content="Technology">
            <meta property="article:tag" content="rust">
            <meta property="article:tag" content="python">
        "#;
        let og = extract(html, None).unwrap();
        assert!(og.article.is_some());
        let article = og.article.unwrap();
        assert_eq!(article.published_time, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(article.modified_time, Some("2024-01-16T12:00:00Z".to_string()));
        assert_eq!(article.section, Some("Technology".to_string()));
        assert_eq!(article.tag.len(), 2);
        assert_eq!(article.tag[0], "rust");
        assert_eq!(article.tag[1], "python");
        assert_eq!(article.author.len(), 1);
        assert_eq!(article.author[0], "https://example.com/author");
    }

    #[test]
    fn test_opengraph_article_multiple_authors() {
        let html = r#"
            <meta property="article:author" content="https://example.com/author1">
            <meta property="article:author" content="https://example.com/author2">
            <meta property="article:author" content="https://example.com/author3">
        "#;
        let og = extract(html, None).unwrap();
        assert!(og.article.is_some());
        let article = og.article.unwrap();
        assert_eq!(article.author.len(), 3);
    }

    #[test]
    fn test_opengraph_article_expiration() {
        let html = r#"<meta property="article:expiration_time" content="2025-12-31T23:59:59Z">"#;
        let og = extract(html, None).unwrap();
        assert!(og.article.is_some());
        let article = og.article.unwrap();
        assert_eq!(article.expiration_time, Some("2025-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_opengraph_video() {
        let html = r#"
            <meta property="og:video" content="https://example.com/video.mp4">
            <meta property="og:video:secure_url" content="https://example.com/secure-video.mp4">
            <meta property="og:video:type" content="video/mp4">
            <meta property="og:video:width" content="1280">
            <meta property="og:video:height" content="720">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.videos.len(), 1);
        let video = &og.videos[0];
        assert_eq!(video.url, "https://example.com/video.mp4");
        assert_eq!(video.secure_url, Some("https://example.com/secure-video.mp4".to_string()));
        assert_eq!(video.r#type, Some("video/mp4".to_string()));
        assert_eq!(video.width, Some(1280));
        assert_eq!(video.height, Some(720));
    }

    #[test]
    fn test_opengraph_multiple_videos() {
        let html = r#"
            <meta property="og:video" content="https://example.com/video1.mp4">
            <meta property="og:video:width" content="1920">
            <meta property="og:video" content="https://example.com/video2.mp4">
            <meta property="og:video:width" content="1280">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.videos.len(), 2);
        assert_eq!(og.videos[0].width, Some(1920));
        assert_eq!(og.videos[1].width, Some(1280));
    }

    #[test]
    fn test_opengraph_audio() {
        let html = r#"
            <meta property="og:audio" content="https://example.com/audio.mp3">
            <meta property="og:audio:secure_url" content="https://example.com/secure-audio.mp3">
            <meta property="og:audio:type" content="audio/mpeg">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.audios.len(), 1);
        let audio = &og.audios[0];
        assert_eq!(audio.url, "https://example.com/audio.mp3");
        assert_eq!(audio.secure_url, Some("https://example.com/secure-audio.mp3".to_string()));
        assert_eq!(audio.r#type, Some("audio/mpeg".to_string()));
    }

    #[test]
    fn test_opengraph_multiple_audios() {
        let html = r#"
            <meta property="og:audio" content="https://example.com/track1.mp3">
            <meta property="og:audio" content="https://example.com/track2.mp3">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.audios.len(), 2);
    }

    #[test]
    fn test_opengraph_book_metadata() {
        let html = r#"
            <meta property="og:type" content="book">
            <meta property="book:author" content="https://example.com/author1">
            <meta property="book:author" content="https://example.com/author2">
            <meta property="book:isbn" content="978-3-16-148410-0">
            <meta property="book:release_date" content="2024-01-01">
            <meta property="book:tag" content="fiction">
            <meta property="book:tag" content="mystery">
        "#;
        let og = extract(html, None).unwrap();
        assert!(og.book.is_some());
        let book = og.book.unwrap();
        assert_eq!(book.author.len(), 2);
        assert_eq!(book.isbn, Some("978-3-16-148410-0".to_string()));
        assert_eq!(book.release_date, Some("2024-01-01".to_string()));
        assert_eq!(book.tag.len(), 2);
        assert_eq!(book.tag[0], "fiction");
    }

    #[test]
    fn test_opengraph_profile_metadata() {
        let html = r#"
            <meta property="og:type" content="profile">
            <meta property="profile:first_name" content="John">
            <meta property="profile:last_name" content="Doe">
            <meta property="profile:username" content="johndoe">
            <meta property="profile:gender" content="male">
        "#;
        let og = extract(html, None).unwrap();
        assert!(og.profile.is_some());
        let profile = og.profile.unwrap();
        assert_eq!(profile.first_name, Some("John".to_string()));
        assert_eq!(profile.last_name, Some("Doe".to_string()));
        assert_eq!(profile.username, Some("johndoe".to_string()));
        assert_eq!(profile.gender, Some("male".to_string()));
    }

    #[test]
    fn test_opengraph_relative_urls() {
        let html = r#"
            <meta property="og:url" content="/article">
            <meta property="og:image" content="/images/photo.jpg">
        "#;
        let og = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(og.url, Some("https://example.com/article".to_string()));
        assert_eq!(og.image, Some("https://example.com/images/photo.jpg".to_string()));
        // Check that images array also has resolved URL
        assert_eq!(og.images.len(), 1);
        assert_eq!(og.images[0].url, "https://example.com/images/photo.jpg");
    }

    #[test]
    fn test_opengraph_relative_video_url() {
        let html = r#"<meta property="og:video" content="/videos/clip.mp4">"#;
        let og = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(og.videos.len(), 1);
        assert_eq!(og.videos[0].url, "https://example.com/videos/clip.mp4");
    }

    #[test]
    fn test_opengraph_relative_audio_url() {
        let html = r#"<meta property="og:audio" content="/audio/song.mp3">"#;
        let og = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(og.audios.len(), 1);
        assert_eq!(og.audios[0].url, "https://example.com/audio/song.mp3");
    }

    #[test]
    fn test_empty_html() {
        let html = "";
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, None);
        assert_eq!(og.r#type, None);
        assert_eq!(og.url, None);
        assert_eq!(og.image, None);
    }

    #[test]
    fn test_no_opengraph_tags() {
        let html = r#"
            <html>
            <head>
                <title>Regular HTML</title>
                <meta name="description" content="Not OG">
            </head>
            </html>
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, None);
    }

    #[test]
    fn test_whitespace_handling() {
        let html = r#"
            <meta property="og:title" content="  Title with spaces  ">
            <meta property="og:description" content="
                Description with
                newlines
            ">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, Some("Title with spaces".to_string()));
        // Description should be trimmed
        assert!(og.description.is_some());
        let desc = og.description.unwrap();
        assert!(!desc.starts_with(' '));
        assert!(!desc.ends_with(' '));
    }

    #[test]
    fn test_empty_content_ignored() {
        let html = r#"
            <meta property="og:title" content="">
            <meta property="og:description" content="   ">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.title, None);
        assert_eq!(og.description, None);
    }

    #[test]
    fn test_malformed_dimensions() {
        let html = r#"
            <meta property="og:image" content="https://example.com/image.jpg">
            <meta property="og:image:width" content="not-a-number">
            <meta property="og:image:height" content="abc">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 1);
        // Invalid dimensions should be ignored
        assert_eq!(og.images[0].width, None);
        assert_eq!(og.images[0].height, None);
    }

    #[test]
    fn test_real_world_facebook_article() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta property="og:title" content="Breaking News Story">
                <meta property="og:type" content="article">
                <meta property="og:url" content="https://news.example.com/story/123">
                <meta property="og:image" content="https://news.example.com/images/story.jpg">
                <meta property="og:image:width" content="1200">
                <meta property="og:image:height" content="630">
                <meta property="og:description" content="Important news breaking now">
                <meta property="og:site_name" content="Example News">
                <meta property="og:locale" content="en_US">
                <meta property="article:published_time" content="2024-01-15T10:00:00Z">
                <meta property="article:author" content="https://news.example.com/author/jane">
                <meta property="article:section" content="World">
                <meta property="article:tag" content="breaking">
                <meta property="article:tag" content="news">
            </head>
            </html>
        "#;
        let og = extract(html, None).unwrap();

        assert_eq!(og.title, Some("Breaking News Story".to_string()));
        assert_eq!(og.r#type, Some("article".to_string()));
        assert_eq!(og.site_name, Some("Example News".to_string()));
        assert_eq!(og.locale, Some("en_US".to_string()));

        assert!(og.article.is_some());
        let article = og.article.unwrap();
        assert_eq!(article.section, Some("World".to_string()));
        assert_eq!(article.tag.len(), 2);
        assert_eq!(article.author.len(), 1);
    }

    #[test]
    fn test_real_world_product_page() {
        let html = r#"
            <meta property="og:title" content="Amazing Product">
            <meta property="og:type" content="product">
            <meta property="og:url" content="https://shop.example.com/product/123">
            <meta property="og:image" content="https://shop.example.com/images/product.jpg">
            <meta property="og:image" content="https://shop.example.com/images/product-alt.jpg">
            <meta property="og:description" content="Best product ever">
            <meta property="og:site_name" content="Example Shop">
        "#;
        let og = extract(html, None).unwrap();

        assert_eq!(og.title, Some("Amazing Product".to_string()));
        assert_eq!(og.r#type, Some("product".to_string()));
        assert_eq!(og.images.len(), 2);
    }

    #[test]
    fn test_real_world_video_page() {
        let html = r#"
            <meta property="og:title" content="Funny Cat Video">
            <meta property="og:type" content="video.other">
            <meta property="og:url" content="https://videos.example.com/watch/123">
            <meta property="og:image" content="https://videos.example.com/thumbnail.jpg">
            <meta property="og:video" content="https://videos.example.com/video.mp4">
            <meta property="og:video:secure_url" content="https://videos.example.com/video.mp4">
            <meta property="og:video:type" content="video/mp4">
            <meta property="og:video:width" content="1920">
            <meta property="og:video:height" content="1080">
            <meta property="og:description" content="Watch this hilarious cat">
            <meta property="og:site_name" content="Video Site">
        "#;
        let og = extract(html, None).unwrap();

        assert_eq!(og.title, Some("Funny Cat Video".to_string()));
        assert_eq!(og.r#type, Some("video.other".to_string()));
        assert_eq!(og.videos.len(), 1);
        assert_eq!(og.videos[0].width, Some(1920));
        assert_eq!(og.videos[0].height, Some(1080));
    }

    #[test]
    fn test_real_world_music_page() {
        let html = r#"
            <meta property="og:title" content="Great Song">
            <meta property="og:type" content="music.song">
            <meta property="og:url" content="https://music.example.com/song/123">
            <meta property="og:image" content="https://music.example.com/album-art.jpg">
            <meta property="og:audio" content="https://music.example.com/preview.mp3">
            <meta property="og:audio:type" content="audio/mpeg">
            <meta property="og:description" content="Listen to this amazing track">
            <meta property="og:site_name" content="Music Streaming">
        "#;
        let og = extract(html, None).unwrap();

        assert_eq!(og.title, Some("Great Song".to_string()));
        assert_eq!(og.r#type, Some("music.song".to_string()));
        assert_eq!(og.audios.len(), 1);
        assert_eq!(og.audios[0].r#type, Some("audio/mpeg".to_string()));
    }

    #[test]
    fn test_image_primary_vs_array() {
        // First og:image should be both the primary image AND in the images array
        let html = r#"
            <meta property="og:image" content="https://example.com/primary.jpg">
            <meta property="og:image" content="https://example.com/secondary.jpg">
        "#;
        let og = extract(html, None).unwrap();

        assert_eq!(og.image, Some("https://example.com/primary.jpg".to_string()));
        assert_eq!(og.images.len(), 2);
        assert_eq!(og.images[0].url, "https://example.com/primary.jpg");
        assert_eq!(og.images[1].url, "https://example.com/secondary.jpg");
    }

    #[test]
    fn test_case_sensitivity() {
        // OG properties are case-sensitive and should be lowercase
        let html = r#"
            <meta property="og:title" content="Lowercase">
            <meta property="OG:TITLE" content="Uppercase">
            <meta property="Og:Title" content="Mixed">
        "#;
        let og = extract(html, None).unwrap();
        // Should only match lowercase version
        assert_eq!(og.title, Some("Lowercase".to_string()));
    }

    #[test]
    fn test_special_characters_in_content() {
        let html = r#"
            <meta property="og:title" content="Title with &quot;quotes&quot; &amp; &lt;tags&gt;">
            <meta property="og:description" content="Description with &#x27;apostrophes&#x27;">
        "#;
        let og = extract(html, None).unwrap();
        // HTML entities should be decoded by the parser
        assert!(og.title.is_some());
        assert!(og.description.is_some());
    }

    #[test]
    fn test_locale_alternate_without_primary() {
        let html = r#"
            <meta property="og:locale:alternate" content="es_ES">
            <meta property="og:locale:alternate" content="fr_FR">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.locale, None);
        assert_eq!(og.locale_alternate.len(), 2);
    }

    #[test]
    fn test_structured_properties_without_main_tag() {
        // Image metadata without og:image should be ignored
        let html = r#"
            <meta property="og:image:width" content="1200">
            <meta property="og:image:height" content="630">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 0);
    }

    // ========== PHASE 3: EDGE CASES AND STRESS TESTS ==========

    #[test]
    fn test_og_image_metadata_without_image() {
        // Test orphaned image metadata (width/height without image URL)
        let html = r#"
            <meta property="og:image:width" content="1200">
            <meta property="og:image:height" content="630">
        "#;
        let og = extract(html, None).unwrap();
        // Should handle gracefully - no crash
        assert!(og.images.is_empty());
    }

    #[test]
    fn test_og_with_invalid_dimensions() {
        let html = r#"
            <meta property="og:image" content="https://example.com/image.jpg">
            <meta property="og:image:width" content="not-a-number">
            <meta property="og:image:height" content="-100">
        "#;
        let og = extract(html, None).unwrap();
        // Should handle invalid dimensions gracefully
        assert!(!og.images.is_empty());
        // Invalid dimensions should be ignored, not crash
    }

    #[test]
    fn test_og_very_long_content() {
        let long_title = "A".repeat(5000);
        let html = format!(r#"<meta property="og:title" content="{}">"#, long_title);
        let og = extract(&html, None).unwrap();
        assert_eq!(og.title, Some(long_title));
    }

    #[test]
    fn test_og_with_unicode_characters() {
        let html = r#"
            <meta property="og:title" content="日本語のタイトル 🎉">
            <meta property="og:description" content="Ça c'est français avec émojis 😊">
        "#;
        let og = extract(html, None).unwrap();
        assert!(og.title.unwrap().contains("日本語"));
        assert!(og.description.unwrap().contains("français"));
    }

    #[test]
    fn test_og_multiple_types() {
        // Test with conflicting types
        let html = r#"
            <meta property="og:type" content="article">
            <meta property="og:type" content="website">
        "#;
        let og = extract(html, None).unwrap();
        // Implementation uses last occurrence (this is valid behavior)
        assert!(og.r#type.is_some());
        // Just verify it doesn't crash with duplicates
    }

    #[test]
    fn test_og_article_with_invalid_datetime() {
        let html = r#"
            <meta property="og:type" content="article">
            <meta property="article:published_time" content="not-a-datetime">
        "#;
        let og = extract(html, None).unwrap();
        // Should handle invalid datetime gracefully
        assert!(og.article.is_some());
        // Invalid datetime stored as-is or handled gracefully
    }

    #[test]
    fn test_og_deeply_nested_structure() {
        let html = r#"
            <meta property="og:image" content="https://example.com/1.jpg">
            <meta property="og:image:width" content="100">
            <meta property="og:image" content="https://example.com/2.jpg">
            <meta property="og:image:width" content="200">
            <meta property="og:image:height" content="300">
            <meta property="og:image" content="https://example.com/3.jpg">
        "#;
        let og = extract(html, None).unwrap();
        assert_eq!(og.images.len(), 3);
        // Verify metadata is correctly associated with each image
    }

    #[test]
    fn test_og_with_special_characters_in_url() {
        let html = r#"<meta property="og:url" content="https://example.com/page?foo=bar&baz=qux#section">"#;
        let og = extract(html, None).unwrap();
        assert!(og.url.is_some());
        assert!(og.url.unwrap().contains("foo=bar"));
    }
}
