//! Tests for JSON-LD extraction

use crate::extractors::jsonld::{extract, extract_by_type};

#[cfg(test)]
mod jsonld_tests {
    use super::*;

    #[test]
    fn test_extract_basic_article() {
        let html = r#"
            <html>
            <head>
                <script type="application/ld+json">
                {
                    "@context": "https://schema.org",
                    "@type": "Article",
                    "headline": "Test Article",
                    "description": "A test article"
                }
                </script>
            </head>
            </html>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);

        let obj = &objects[0];
        assert!(obj.type_.is_some());
        assert_eq!(obj.properties.get("headline").unwrap().as_str(), Some("Test Article"));
    }

    #[test]
    fn test_extract_multiple_scripts() {
        let html = r#"
            <html>
            <head>
                <script type="application/ld+json">
                {"@type": "Article", "headline": "Article 1"}
                </script>
                <script type="application/ld+json">
                {"@type": "Person", "name": "John Doe"}
                </script>
            </head>
            </html>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn test_extract_with_graph() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {"@type": "Article", "headline": "Article 1"},
                    {"@type": "Person", "name": "Author"}
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn test_extract_by_type() {
        let html = r#"
            <script type="application/ld+json">
            {"@type": "Article", "headline": "Article"}
            </script>
            <script type="application/ld+json">
            {"@type": "Person", "name": "Person"}
            </script>
            <script type="application/ld+json">
            {"@type": "Article", "headline": "Another Article"}
            </script>
        "#;

        let articles = extract_by_type(html, "Article").unwrap();
        assert_eq!(articles.len(), 2);

        let people = extract_by_type(html, "Person").unwrap();
        assert_eq!(people.len(), 1);
    }

    #[test]
    fn test_extract_empty_html() {
        let html = "";
        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn test_extract_no_jsonld() {
        let html = "<html><body><p>No JSON-LD here</p></body></html>";
        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn test_extract_invalid_json() {
        let html = r#"
            <script type="application/ld+json">
            {invalid json here
            </script>
        "#;

        // Should not crash, just return empty
        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn test_extract_product() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Test Product",
                "description": "Product description",
                "sku": "ABC123",
                "offers": {
                    "@type": "Offer",
                    "price": "29.99",
                    "priceCurrency": "USD"
                }
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("Test Product"));
    }

    #[test]
    fn test_extract_with_nested_objects() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test",
                "author": {
                    "@type": "Person",
                    "name": "Jane Smith"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Publisher Inc"
                }
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("author"));
        assert!(objects[0].properties.contains_key("publisher"));
    }

    #[test]
    fn test_extract_with_array_values() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test",
                "keywords": ["rust", "programming", "web"]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("keywords"));
    }

    #[test]
    fn test_extract_multiple_types() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": ["Article", "BlogPosting"],
                "headline": "Blog Post"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);

        // Should match either type
        let articles = extract_by_type(html, "Article").unwrap();
        assert_eq!(articles.len(), 1);

        let blogs = extract_by_type(html, "BlogPosting").unwrap();
        assert_eq!(blogs.len(), 1);
    }

    #[test]
    fn test_extract_with_id() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "@id": "https://example.com/article/123",
                "headline": "Test"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].id, Some("https://example.com/article/123".to_string()));
    }

    #[test]
    fn test_extract_empty_script() {
        let html = r#"
            <script type="application/ld+json">
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn test_extract_whitespace_only() {
        let html = r#"
            <script type="application/ld+json">


            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn test_extract_organization() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Organization",
                "name": "Acme Corp",
                "url": "https://acme.com",
                "logo": "https://acme.com/logo.png",
                "sameAs": [
                    "https://twitter.com/acme",
                    "https://facebook.com/acme"
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("Acme Corp"));
    }

    #[test]
    fn test_extract_person() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Person",
                "name": "John Doe",
                "email": "john@example.com",
                "jobTitle": "Software Engineer"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("John Doe"));
    }

    #[test]
    fn test_extract_with_numeric_values() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Widget",
                "offers": {
                    "price": 29.99,
                    "quantity": 100
                }
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("offers"));
    }

    #[test]
    fn test_extract_with_boolean_values() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Widget",
                "inStock": true,
                "discontinued": false
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("inStock").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_extract_with_null_values() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Thing",
                "name": "Test",
                "description": null
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.get("description").unwrap().is_null());
    }

    #[test]
    fn test_extract_mixed_content() {
        let html = r#"
            <html>
            <head>
                <script type="text/javascript">
                var x = 123;
                </script>
                <script type="application/ld+json">
                {"@type": "Article", "headline": "Test"}
                </script>
                <script>
                console.log("hello");
                </script>
            </head>
            </html>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_extract_recipe() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Chocolate Cake",
                "author": "Jane Doe",
                "datePublished": "2024-01-15",
                "description": "A delicious chocolate cake",
                "prepTime": "PT30M",
                "cookTime": "PT1H",
                "recipeYield": "8 servings"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("Chocolate Cake"));
    }

    #[test]
    fn test_extract_event() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Tech Conference 2024",
                "startDate": "2024-06-15T09:00:00",
                "endDate": "2024-06-17T18:00:00",
                "location": {
                    "@type": "Place",
                    "name": "Convention Center"
                }
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(
            objects[0].properties.get("name").unwrap().as_str(),
            Some("Tech Conference 2024")
        );
    }

    #[test]
    fn test_extract_faq_page() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What is JSON-LD?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "JSON-LD is a structured data format"
                        }
                    }
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("mainEntity"));
    }

    #[test]
    fn test_extract_breadcrumb_list() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Category",
                        "item": "https://example.com/category"
                    }
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("itemListElement"));
    }

    #[test]
    fn test_extract_video_object() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Tutorial Video",
                "description": "Learn how to use JSON-LD",
                "thumbnailUrl": "https://example.com/thumb.jpg",
                "uploadDate": "2024-01-15",
                "duration": "PT15M"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("Tutorial Video"));
    }

    #[test]
    fn test_extract_local_business() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Joe's Pizza",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "123 Main St",
                    "addressLocality": "Springfield",
                    "addressRegion": "IL",
                    "postalCode": "62701"
                },
                "telephone": "+1-555-0100"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("name").unwrap().as_str(), Some("Joe's Pizza"));
    }

    #[test]
    fn test_extract_graph_with_multiple_types() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "WebSite",
                        "name": "Example Site",
                        "url": "https://example.com"
                    },
                    {
                        "@type": "Organization",
                        "name": "Example Org"
                    },
                    {
                        "@type": "Person",
                        "name": "John Doe"
                    }
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 3);
    }

    #[test]
    fn test_extract_by_type_no_matches() {
        let html = r#"
            <script type="application/ld+json">
            {"@type": "Article", "headline": "Test"}
            </script>
        "#;

        let products = extract_by_type(html, "Product").unwrap();
        assert_eq!(products.len(), 0);
    }

    #[test]
    fn test_extract_by_type_no_type_field() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "name": "No Type Field"
            }
            </script>
        "#;

        let articles = extract_by_type(html, "Article").unwrap();
        assert_eq!(articles.len(), 0);
    }

    #[test]
    fn test_extract_unicode_content() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "测试文章",
                "description": "Тестовая статья",
                "author": "José García"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("headline"));
    }

    #[test]
    fn test_extract_escaped_characters() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test \"quoted\" text",
                "description": "Line 1\nLine 2"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_extract_large_object() {
        // Create a large JSON-LD object to test performance
        let large_json = format!(
            r#"{{
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Large Article",
                "articleBody": "{}"
            }}"#,
            "Lorem ipsum ".repeat(1000)
        );

        let html = format!(r#"<script type="application/ld+json">{}</script>"#, large_json);

        let objects = extract(&html, None).unwrap();
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_extract_deeply_nested() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "author": {
                    "@type": "Person",
                    "name": "John",
                    "worksFor": {
                        "@type": "Organization",
                        "name": "Company",
                        "address": {
                            "@type": "PostalAddress",
                            "addressCountry": "US"
                        }
                    }
                }
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("author"));
    }

    #[test]
    fn test_extract_array_of_objects() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "review": [
                    {
                        "@type": "Review",
                        "reviewRating": {"ratingValue": "5"}
                    },
                    {
                        "@type": "Review",
                        "reviewRating": {"ratingValue": "4"}
                    }
                ]
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert!(objects[0].properties.contains_key("review"));
    }

    #[test]
    fn test_extract_with_base_url() {
        // Base URL is not used for JSON-LD extraction but ensure it doesn't cause errors
        let html = r#"
            <script type="application/ld+json">
            {"@type": "Article", "headline": "Test"}
            </script>
        "#;

        let objects = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_extract_malformed_json_multiple_scripts() {
        let html = r#"
            <script type="application/ld+json">
            {invalid}
            </script>
            <script type="application/ld+json">
            {"@type": "Article", "headline": "Valid"}
            </script>
            <script type="application/ld+json">
            {also invalid
            </script>
        "#;

        // Should extract the valid one and skip the invalid ones
        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].properties.get("headline").unwrap().as_str(), Some("Valid"));
    }

    #[test]
    fn test_extract_json_with_comments() {
        // JSON technically doesn't support comments, but some sites include them
        let html = r#"
            <script type="application/ld+json">
            {
                "@type": "Article",
                "headline": "Test"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_extract_special_characters_in_values() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test < > & \" '",
                "url": "https://example.com/page?foo=bar&baz=qux"
            }
            </script>
        "#;

        let objects = extract(html, None).unwrap();
        assert_eq!(objects.len(), 1);
    }
}
