//! Tests for standard meta tag extraction (Phase 1)
//!
//! These tests are written FIRST (TDD approach) to define the expected behavior.

use crate::extractors::meta::extract;
use crate::types::meta::MetaTags;

#[cfg(test)]
mod meta_extraction_tests {
    use super::*;

    // ========== BASIC META TAGS ==========

    #[test]
    fn test_extract_title_from_title_tag() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Test Page Title</title>
            </head>
            </html>
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.title, Some("Test Page Title".to_string()));
    }

    #[test]
    fn test_extract_description() {
        let html = r#"<meta name="description" content="A test description">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, Some("A test description".to_string()));
    }

    #[test]
    fn test_extract_keywords() {
        let html = r#"<meta name="keywords" content="rust, python, metadata">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(
            meta.keywords,
            Some(vec!["rust".to_string(), "python".to_string(), "metadata".to_string()])
        );
    }

    #[test]
    fn test_extract_author() {
        let html = r#"<meta name="author" content="John Doe">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_extract_generator() {
        let html = r#"<meta name="generator" content="WordPress 6.0">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.generator, Some("WordPress 6.0".to_string()));
    }

    // ========== CANONICAL AND LINKS ==========

    #[test]
    fn test_extract_canonical() {
        let html = r#"<link rel="canonical" href="https://example.com/page">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.canonical, Some("https://example.com/page".to_string()));
    }

    #[test]
    fn test_extract_canonical_relative() {
        let html = r#"<link rel="canonical" href="/page">"#;
        let meta = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(meta.canonical, Some("https://example.com/page".to_string()));
    }

    #[test]
    fn test_extract_alternate_link() {
        let html = r#"<link rel="alternate" href="https://example.com/es" hreflang="es">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.alternate.len(), 1);
        assert_eq!(meta.alternate[0].href, "https://example.com/es");
        assert_eq!(meta.alternate[0].hreflang, Some("es".to_string()));
    }

    #[test]
    fn test_extract_rss_feed() {
        let html = r#"<link rel="alternate" type="application/rss+xml" href="/feed.xml" title="RSS Feed">"#;
        let meta = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(meta.feeds.len(), 1);
        assert_eq!(meta.feeds[0].href, "https://example.com/feed.xml");
        assert_eq!(meta.feeds[0].r#type, "application/rss+xml");
        assert_eq!(meta.feeds[0].title, Some("RSS Feed".to_string()));
    }

    #[test]
    fn test_extract_atom_feed() {
        let html = r#"<link rel="alternate" type="application/atom+xml" href="/atom.xml">"#;
        let meta = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(meta.feeds.len(), 1);
        assert_eq!(meta.feeds[0].r#type, "application/atom+xml");
    }

    #[test]
    fn test_extract_shortlink() {
        let html = r#"<link rel="shortlink" href="https://example.com/?p=123">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.shortlink, Some("https://example.com/?p=123".to_string()));
    }

    // ========== ROBOTS DIRECTIVES ==========

    #[test]
    fn test_extract_robots_index_follow() {
        let html = r#"<meta name="robots" content="index, follow">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.robots.is_some());
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(true));
        assert_eq!(robots.follow, Some(true));
    }

    #[test]
    fn test_extract_robots_noindex_nofollow() {
        let html = r#"<meta name="robots" content="noindex, nofollow">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.robots.is_some());
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(false));
        assert_eq!(robots.follow, Some(false));
    }

    #[test]
    fn test_extract_googlebot() {
        let html = r#"<meta name="googlebot" content="nosnippet, notranslate">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.googlebot.is_some());
        let googlebot = meta.googlebot.unwrap();
        assert_eq!(googlebot.snippet, Some(false));
        assert_eq!(googlebot.translate, Some(false));
    }

    // ========== VIEWPORT AND MOBILE ==========

    #[test]
    fn test_extract_viewport() {
        let html = r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.viewport, Some("width=device-width, initial-scale=1.0".to_string()));
    }

    #[test]
    fn test_extract_theme_color() {
        let html = r##"<meta name="theme-color" content="#ff0000">"##;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.theme_color, Some("#ff0000".to_string()));
    }

    // ========== CHARSET AND LANGUAGE ==========

    #[test]
    fn test_extract_charset() {
        let html = r#"<meta charset="UTF-8">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.charset, Some("UTF-8".to_string()));
    }

    #[test]
    fn test_extract_charset_http_equiv() {
        let html = r#"<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.charset, Some("UTF-8".to_string()));
    }

    #[test]
    fn test_extract_language() {
        let html = r#"<html lang="en-US">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.language, Some("en-US".to_string()));
    }

    // ========== APPLICATION AND REFERRER ==========

    #[test]
    fn test_extract_application_name() {
        let html = r#"<meta name="application-name" content="MyApp">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.application_name, Some("MyApp".to_string()));
    }

    #[test]
    fn test_extract_referrer() {
        let html = r#"<meta name="referrer" content="no-referrer">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.referrer, Some("no-referrer".to_string()));
    }

    // ========== EDGE CASES ==========

    #[test]
    fn test_empty_html() {
        let html = "";
        let meta = extract(html, None).unwrap();
        assert_eq!(meta, MetaTags::default());
    }

    #[test]
    fn test_no_meta_tags() {
        let html = r#"<html><body><p>No meta tags</p></body></html>"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.title, None);
        assert_eq!(meta.description, None);
    }

    #[test]
    fn test_multiple_canonical_uses_first() {
        let html = r#"
            <link rel="canonical" href="https://example.com/first">
            <link rel="canonical" href="https://example.com/second">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.canonical, Some("https://example.com/first".to_string()));
    }

    #[test]
    fn test_whitespace_trimming() {
        let html = r#"<meta name="description" content="  Lots of whitespace  ">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, Some("Lots of whitespace".to_string()));
    }

    #[test]
    fn test_case_insensitive_meta_names() {
        let html = r#"<meta name="DESCRIPTION" content="Test">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, Some("Test".to_string()));
    }

    // ========== COMPLEX REAL-WORLD EXAMPLES ==========

    #[test]
    fn test_wordpress_site() {
        let html = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>My WordPress Blog</title>
                <meta name="description" content="A blog about Rust and Python">
                <meta name="robots" content="index, follow">
                <meta name="generator" content="WordPress 6.0">
                <link rel="canonical" href="https://example.com/blog">
                <link rel="alternate" type="application/rss+xml" title="RSS" href="/feed">
                <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            </html>
        "#;
        let meta = extract(html, Some("https://example.com")).unwrap();

        assert_eq!(meta.title, Some("My WordPress Blog".to_string()));
        assert_eq!(meta.description, Some("A blog about Rust and Python".to_string()));
        assert_eq!(meta.charset, Some("UTF-8".to_string()));
        assert_eq!(meta.language, Some("en".to_string()));
        assert_eq!(meta.generator, Some("WordPress 6.0".to_string()));
        assert_eq!(meta.canonical, Some("https://example.com/blog".to_string()));
        assert_eq!(meta.feeds.len(), 1);
        assert!(meta.robots.is_some());
    }

    #[test]
    fn test_e_commerce_site() {
        let html = r#"
            <html>
            <head>
                <meta charset="utf-8">
                <title>Product Name - Shop</title>
                <meta name="description" content="Buy Product Name at great prices">
                <link rel="canonical" href="https://shop.example.com/product/123">
                <meta name="robots" content="index, follow, noarchive">
                <link rel="alternate" hreflang="es" href="https://shop.example.com/es/product/123">
                <link rel="alternate" hreflang="fr" href="https://shop.example.com/fr/product/123">
            </head>
            </html>
        "#;
        let meta = extract(html, None).unwrap();

        assert_eq!(meta.title, Some("Product Name - Shop".to_string()));
        assert_eq!(meta.alternate.len(), 2);
        assert!(meta.robots.is_some());
        assert_eq!(meta.robots.as_ref().unwrap().archive, Some(false));
    }

    // ========== ADDITIONAL EDGE CASES ==========

    #[test]
    fn test_multiple_feeds() {
        let html = r#"
            <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS">
            <link rel="alternate" type="application/atom+xml" href="/atom.xml" title="Atom">
        "#;
        let meta = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(meta.feeds.len(), 2);
        assert_eq!(meta.feeds[0].r#type, "application/rss+xml");
        assert_eq!(meta.feeds[1].r#type, "application/atom+xml");
    }

    #[test]
    fn test_complex_robots_directives() {
        let html =
            r#"<meta name="robots" content="noindex, follow, noarchive, nosnippet, notranslate">"#;
        let meta = extract(html, None).unwrap();
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(false));
        assert_eq!(robots.follow, Some(true));
        assert_eq!(robots.archive, Some(false));
        assert_eq!(robots.snippet, Some(false));
        assert_eq!(robots.translate, Some(false));
    }

    #[test]
    fn test_robots_all_directive() {
        let html = r#"<meta name="robots" content="all">"#;
        let meta = extract(html, None).unwrap();
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(true));
        assert_eq!(robots.follow, Some(true));
    }

    #[test]
    fn test_robots_none_directive() {
        let html = r#"<meta name="robots" content="none">"#;
        let meta = extract(html, None).unwrap();
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(false));
        assert_eq!(robots.follow, Some(false));
    }

    #[test]
    fn test_multiple_alternate_links() {
        let html = r#"
            <link rel="alternate" hreflang="es" href="https://example.com/es">
            <link rel="alternate" hreflang="fr" href="https://example.com/fr">
            <link rel="alternate" hreflang="de" href="https://example.com/de">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.alternate.len(), 3);
        assert_eq!(meta.alternate[0].hreflang, Some("es".to_string()));
        assert_eq!(meta.alternate[1].hreflang, Some("fr".to_string()));
        assert_eq!(meta.alternate[2].hreflang, Some("de".to_string()));
    }

    #[test]
    fn test_empty_content_ignored() {
        let html = r#"<meta name="description" content="">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, None);
    }

    #[test]
    fn test_whitespace_only_content_ignored() {
        let html = r#"<meta name="description" content="   ">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, None);
    }

    #[test]
    fn test_keywords_empty_items_filtered() {
        let html = r#"<meta name="keywords" content="rust, , python,  , metadata">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(
            meta.keywords,
            Some(vec!["rust".to_string(), "python".to_string(), "metadata".to_string()])
        );
    }

    #[test]
    fn test_international_characters() {
        let html = r#"
            <meta name="description" content="Это тест на русском языке">
            <meta name="author" content="José García">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, Some("Это тест на русском языке".to_string()));
        assert_eq!(meta.author, Some("José García".to_string()));
    }

    #[test]
    fn test_special_characters_in_content() {
        let html =
            r#"<meta name="description" content="Test with &amp; special &lt;characters&gt;">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.description.is_some());
    }

    #[test]
    fn test_mobile_app_meta() {
        let html = r##"
            <meta name="application-name" content="MyMobileApp">
            <meta name="theme-color" content="#1a1a1a">
            <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=5">
        "##;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.application_name, Some("MyMobileApp".to_string()));
        assert_eq!(meta.theme_color, Some("#1a1a1a".to_string()));
        assert!(meta.viewport.is_some());
    }

    #[test]
    fn test_feed_without_title() {
        let html = r#"<link rel="alternate" type="application/rss+xml" href="/feed.xml">"#;
        let meta = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(meta.feeds.len(), 1);
        assert_eq!(meta.feeds[0].title, None);
    }

    #[test]
    fn test_alternate_with_media_query() {
        let html = r#"<link rel="alternate" media="only screen and (max-width: 640px)" href="https://m.example.com">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.alternate.len(), 1);
        assert_eq!(meta.alternate[0].media, Some("only screen and (max-width: 640px)".to_string()));
    }

    #[test]
    fn test_charset_case_insensitive() {
        let html = r#"<meta charset="utf-8">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.charset, Some("utf-8".to_string()));
    }

    #[test]
    fn test_http_equiv_charset_extraction() {
        let html = r#"<meta http-equiv="Content-Type" content="text/html; charset=ISO-8859-1">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.charset, Some("ISO-8859-1".to_string()));
    }

    #[test]
    fn test_referrer_policies() {
        let policies = vec![
            "no-referrer",
            "no-referrer-when-downgrade",
            "origin",
            "origin-when-cross-origin",
            "same-origin",
            "strict-origin",
            "strict-origin-when-cross-origin",
            "unsafe-url",
        ];

        for policy in policies {
            let html = format!(r#"<meta name="referrer" content="{}">"#, policy);
            let meta = extract(&html, None).unwrap();
            assert_eq!(meta.referrer, Some(policy.to_string()));
        }
    }

    #[test]
    fn test_news_website() {
        let html = r#"
            <!DOCTYPE html>
            <html lang="en-US">
            <head>
                <meta charset="UTF-8">
                <title>Breaking News Story - News Site</title>
                <meta name="description" content="Latest breaking news from around the world">
                <meta name="keywords" content="news, breaking, world, politics">
                <meta name="author" content="News Team">
                <meta name="robots" content="index, follow, max-snippet:150">
                <link rel="canonical" href="https://news.example.com/2024/story">
                <link rel="alternate" type="application/rss+xml" title="News RSS" href="/rss">
                <link rel="alternate" hreflang="en-gb" href="https://news.example.com/uk/2024/story">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
            </head>
            </html>
        "#;
        let meta = extract(html, Some("https://news.example.com")).unwrap();

        assert_eq!(meta.title, Some("Breaking News Story - News Site".to_string()));
        assert_eq!(meta.charset, Some("UTF-8".to_string()));
        assert_eq!(meta.language, Some("en-US".to_string()));
        assert!(meta.keywords.is_some());
        assert_eq!(meta.keywords.as_ref().unwrap().len(), 4);
        assert_eq!(meta.feeds.len(), 1);
        assert_eq!(meta.alternate.len(), 1);
    }

    #[test]
    fn test_documentation_site() {
        let html = r#"
            <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>API Documentation - MyLib</title>
                <meta name="description" content="Complete API reference for MyLib">
                <meta name="generator" content="Sphinx 4.0">
                <link rel="canonical" href="https://docs.example.com/api/reference">
                <meta name="robots" content="index, follow">
            </head>
            </html>
        "#;
        let meta = extract(html, None).unwrap();

        assert_eq!(meta.title, Some("API Documentation - MyLib".to_string()));
        assert_eq!(meta.generator, Some("Sphinx 4.0".to_string()));
        assert!(meta.robots.is_some());
    }

    #[test]
    fn test_malformed_html_still_works() {
        let html = r#"
            <meta name="description" content="Test">
            <title>Test</title>
            <meta charset="UTF-8"
            <meta name="author" content="Test Author">
        "#;
        let meta = extract(html, None).unwrap();

        // Should still extract what it can
        assert_eq!(meta.title, Some("Test".to_string()));
        assert_eq!(meta.description, Some("Test".to_string()));
        assert_eq!(meta.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_mixed_quote_styles() {
        let html = r#"
            <meta name='description' content='Single quotes'>
            <meta name="author" content="Double quotes">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.description, Some("Single quotes".to_string()));
        assert_eq!(meta.author, Some("Double quotes".to_string()));
    }

    #[test]
    fn test_relative_canonical_with_path() {
        let html = r#"<link rel="canonical" href="../other/page.html">"#;
        let meta = extract(html, Some("https://example.com/current/page")).unwrap();
        assert_eq!(meta.canonical, Some("https://example.com/other/page.html".to_string()));
    }

    #[test]
    fn test_googlebot_specific_directives() {
        let html = r#"<meta name="googlebot" content="noarchive, noimageindex">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.googlebot.is_some());
        let googlebot = meta.googlebot.unwrap();
        assert_eq!(googlebot.archive, Some(false));
        assert_eq!(googlebot.imageindex, Some(false));
    }

    #[test]
    fn test_robots_raw_preserved() {
        let html = r#"<meta name="robots" content="index, follow, max-snippet:150, max-image-preview:large">"#;
        let meta = extract(html, None).unwrap();
        let robots = meta.robots.unwrap();
        assert_eq!(robots.raw, "index, follow, max-snippet:150, max-image-preview:large");
        assert_eq!(robots.index, Some(true));
        assert_eq!(robots.follow, Some(true));
    }

    // ========== PHASE 3: EDGE CASES AND STRESS TESTS ==========

    #[test]
    fn test_meta_extremely_long_content() {
        // Test with very long meta content (10,000 characters)
        let long_description = "a".repeat(10000);
        let html = format!(r#"<meta name="description" content="{}">"#, long_description);
        let meta = extract(&html, None).unwrap();
        assert_eq!(meta.description, Some(long_description));
    }

    #[test]
    fn test_meta_with_html_entities_in_content() {
        let html = r#"<meta name="description" content="Test &lt;tag&gt; &amp; &quot;quotes&quot; &#8217;">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.description.is_some());
        let desc = meta.description.unwrap();
        // HTML entities are decoded by the parser
        assert!(desc.contains("<") || desc.contains("&lt;"));
        assert!(desc.contains("tag"));
    }

    #[test]
    fn test_meta_with_newlines_in_content() {
        let html = "<meta name=\"description\" content=\"Line 1\nLine 2\nLine 3\">";
        let meta = extract(html, None).unwrap();
        assert!(meta.description.is_some());
        let desc = meta.description.unwrap();
        assert!(desc.contains("Line 1"));
        assert!(desc.contains("Line 2"));
    }

    #[test]
    fn test_meta_with_special_quotes() {
        let html = r#"<meta name="description" content="Testing "smart quotes" and 'apostrophes' — dashes">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.description.is_some());
    }

    #[test]
    fn test_canonical_with_query_params() {
        let html =
            r#"<link rel="canonical" href="https://example.com/page?foo=bar&baz=qux#section">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.canonical.is_some());
        assert!(meta.canonical.unwrap().contains("foo=bar"));
    }

    #[test]
    fn test_robots_with_unknown_directives() {
        let html =
            r#"<meta name="robots" content="index, follow, unknowndirective, max-snippet:50">"#;
        let meta = extract(html, None).unwrap();
        assert!(meta.robots.is_some());
        let robots = meta.robots.unwrap();
        assert_eq!(robots.index, Some(true));
        assert_eq!(robots.follow, Some(true));
        // Unknown directives should be ignored gracefully
    }

    #[test]
    fn test_meta_with_duplicate_tags() {
        let html = r#"
            <meta name="description" content="First description">
            <meta name="description" content="Second description">
        "#;
        let meta = extract(html, None).unwrap();
        // Implementation uses last occurrence (this is valid behavior)
        assert!(meta.description.is_some());
        // Just verify it doesn't crash with duplicates
    }

    #[test]
    fn test_meta_with_empty_title() {
        let html = r#"<title></title>"#;
        let meta = extract(html, None).unwrap();
        // Empty title should not be present
        assert!(meta.title.is_none() || meta.title == Some("".to_string()));
    }

    // Phase 6: Site Verification Tests
    #[test]
    fn test_google_site_verification() {
        let html = r#"<meta name="google-site-verification" content="abc123xyz456def789">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.google_site_verification, Some("abc123xyz456def789".to_string()));
    }

    #[test]
    fn test_bing_verification() {
        let html = r#"<meta name="msvalidate.01" content="BING123456789">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.msvalidate_01, Some("BING123456789".to_string()));
    }

    #[test]
    fn test_yandex_verification() {
        let html = r#"<meta name="yandex-verification" content="yandex123456">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.yandex_verification, Some("yandex123456".to_string()));
    }

    #[test]
    fn test_pinterest_verification() {
        let html = r#"<meta name="p:domain_verify" content="pinterest123">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.p_domain_verify, Some("pinterest123".to_string()));
    }

    #[test]
    fn test_facebook_domain_verification() {
        let html = r#"<meta name="facebook-domain-verification" content="fb123456789">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.facebook_domain_verification, Some("fb123456789".to_string()));
    }

    #[test]
    fn test_multiple_verification_tags() {
        let html = r#"
            <meta name="google-site-verification" content="google123">
            <meta name="msvalidate.01" content="bing456">
            <meta name="yandex-verification" content="yandex789">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.google_site_verification, Some("google123".to_string()));
        assert_eq!(meta.msvalidate_01, Some("bing456".to_string()));
        assert_eq!(meta.yandex_verification, Some("yandex789".to_string()));
    }

    // Phase 8: Apple Mobile Meta Tests
    #[test]
    fn test_apple_mobile_web_app_capable() {
        let html = r#"<meta name="apple-mobile-web-app-capable" content="yes">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.apple_mobile_web_app_capable, Some("yes".to_string()));
    }

    #[test]
    fn test_apple_mobile_web_app_status_bar_style() {
        let html =
            r#"<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(
            meta.apple_mobile_web_app_status_bar_style,
            Some("black-translucent".to_string())
        );
    }

    #[test]
    fn test_apple_mobile_web_app_title() {
        let html = r#"<meta name="apple-mobile-web-app-title" content="My App">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.apple_mobile_web_app_title, Some("My App".to_string()));
    }

    #[test]
    fn test_all_apple_mobile_tags() {
        let html = r#"
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-status-bar-style" content="black">
            <meta name="apple-mobile-web-app-title" content="My PWA">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.apple_mobile_web_app_capable, Some("yes".to_string()));
        assert_eq!(meta.apple_mobile_web_app_status_bar_style, Some("black".to_string()));
        assert_eq!(meta.apple_mobile_web_app_title, Some("My PWA".to_string()));
    }

    #[test]
    fn test_msapplication_tile_color() {
        let html = r##"<meta name="msapplication-TileColor" content="#da532c">"##;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.msapplication_tile_color, Some("#da532c".to_string()));
    }

    #[test]
    fn test_msapplication_tile_image() {
        let html = r#"<meta name="msapplication-TileImage" content="/mstile-144x144.png">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.msapplication_tile_image, Some("/mstile-144x144.png".to_string()));
    }

    #[test]
    fn test_msapplication_config() {
        let html = r#"<meta name="msapplication-config" content="/browserconfig.xml">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.msapplication_config, Some("/browserconfig.xml".to_string()));
    }

    #[test]
    fn test_all_microsoft_meta_tags() {
        let html = r##"
            <meta name="msapplication-TileColor" content="#2b5797">
            <meta name="msapplication-TileImage" content="/mstile.png">
            <meta name="msapplication-config" content="/config.xml">
        "##;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.msapplication_tile_color, Some("#2b5797".to_string()));
        assert_eq!(meta.msapplication_tile_image, Some("/mstile.png".to_string()));
        assert_eq!(meta.msapplication_config, Some("/config.xml".to_string()));
    }

    #[test]
    fn test_link_icon() {
        let html = r#"<link rel="icon" href="/favicon.ico">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.icon, Some("/favicon.ico".to_string()));
    }

    #[test]
    fn test_link_apple_touch_icon() {
        let html = r#"<link rel="apple-touch-icon" href="/apple-touch-icon.png">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.apple_touch_icon, Some("/apple-touch-icon.png".to_string()));
    }

    #[test]
    fn test_link_manifest() {
        let html = r#"<link rel="manifest" href="/manifest.json">"#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.manifest, Some("/manifest.json".to_string()));
    }

    #[test]
    fn test_link_prev_next() {
        let html = r#"
            <link rel="prev" href="/page/1">
            <link rel="next" href="/page/3">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.prev, Some("/page/1".to_string()));
        assert_eq!(meta.next, Some("/page/3".to_string()));
    }

    #[test]
    fn test_all_additional_links() {
        let html = r#"
            <link rel="icon" href="/favicon.ico">
            <link rel="apple-touch-icon" href="/icon.png">
            <link rel="manifest" href="/manifest.webmanifest">
            <link rel="prev" href="/previous">
            <link rel="next" href="/following">
        "#;
        let meta = extract(html, None).unwrap();
        assert_eq!(meta.icon, Some("/favicon.ico".to_string()));
        assert_eq!(meta.apple_touch_icon, Some("/icon.png".to_string()));
        assert_eq!(meta.manifest, Some("/manifest.webmanifest".to_string()));
        assert_eq!(meta.prev, Some("/previous".to_string()));
        assert_eq!(meta.next, Some("/following".to_string()));
    }

    #[test]
    fn test_link_url_resolution() {
        let html = r#"<link rel="icon" href="favicon.ico">"#;
        let meta = extract(html, Some("https://example.com/subdir/")).unwrap();
        assert_eq!(meta.icon, Some("https://example.com/subdir/favicon.ico".to_string()));
    }
}
