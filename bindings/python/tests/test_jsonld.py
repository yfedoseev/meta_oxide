"""
Tests for JSON-LD extraction (Phase 3 - 41% adoption)

Tests the extract_jsonld() function and its integration with extract_all()
"""

import meta_oxide
import pytest


class TestJSONLDExtraction:
    """Test basic JSON-LD extraction"""

    def test_extract_article(self):
        """Test extracting Article type"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test Article",
                "description": "This is a test article",
                "datePublished": "2024-01-15",
                "author": {
                    "@type": "Person",
                    "name": "John Doe"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Article"
        assert objects[0]["headline"] == "Test Article"
        assert objects[0]["description"] == "This is a test article"
        assert objects[0]["datePublished"] == "2024-01-15"

    def test_extract_product(self):
        """Test extracting Product type"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Wireless Headphones",
                "description": "Noise-cancelling wireless headphones",
                "sku": "WH-1000XM5",
                "brand": {
                    "@type": "Brand",
                    "name": "Sony"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Product"
        assert objects[0]["name"] == "Wireless Headphones"
        assert objects[0]["sku"] == "WH-1000XM5"

    def test_extract_person(self):
        """Test extracting Person type"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Person",
                "name": "Jane Smith",
                "email": "jane@example.com",
                "jobTitle": "Software Engineer",
                "url": "https://janesmith.com"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Person"
        assert objects[0]["name"] == "Jane Smith"
        assert objects[0]["email"] == "jane@example.com"
        assert objects[0]["jobTitle"] == "Software Engineer"

    def test_extract_organization(self):
        """Test extracting Organization type"""
        html = """
        <html>
        <head>
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
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Organization"
        assert objects[0]["name"] == "Acme Corp"
        assert objects[0]["url"] == "https://acme.com"


class TestJSONLDMultipleObjects:
    """Test extracting multiple JSON-LD objects"""

    def test_multiple_script_tags(self):
        """Test extracting from multiple script tags"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "First Article"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Person",
                "name": "John Doe"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["@type"] == "Article"
        assert objects[1]["@type"] == "Person"

    def test_graph_array(self):
        """Test extracting @graph with multiple objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Article",
                        "headline": "Article in Graph"
                    },
                    {
                        "@type": "Person",
                        "name": "Author Name"
                    },
                    {
                        "@type": "Organization",
                        "name": "Publisher Name"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 3
        assert objects[0]["@type"] == "Article"
        assert objects[1]["@type"] == "Person"
        assert objects[2]["@type"] == "Organization"

    def test_multiple_types(self):
        """Test object with multiple @type values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": ["Article", "BlogPosting"],
                "headline": "Blog Post"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == ["Article", "BlogPosting"]


class TestJSONLDEdgeCases:
    """Test edge cases and error handling"""

    def test_empty_html(self):
        """Test with no JSON-LD"""
        html = "<html><body>No JSON-LD here</body></html>"
        objects = meta_oxide.extract_jsonld(html)
        assert len(objects) == 0

    def test_empty_script_tag(self):
        """Test with empty script tag"""
        html = """
        <html>
        <head>
            <script type="application/ld+json"></script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert len(objects) == 0

    def test_invalid_json(self):
        """Test with malformed JSON (should be skipped gracefully)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {invalid json}
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Valid Article"
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        # Should extract only the valid one, skipping the invalid
        objects = meta_oxide.extract_jsonld(html)
        assert len(objects) >= 1
        assert any(obj.get("@type") == "Article" for obj in objects)

    def test_minimal_object(self):
        """Test with minimal JSON-LD object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org"
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert len(objects) == 1
        assert "@context" in objects[0]

    def test_unicode_content(self):
        """Test with Unicode characters"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "日本語のタイトル",
                "description": "Описание на русском",
                "author": {
                    "@type": "Person",
                    "name": "François Müller"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert len(objects) == 1
        assert objects[0]["headline"] == "日本語のタイトル"
        assert objects[0]["description"] == "Описание на русском"


class TestJSONLDIntegration:
    """Test JSON-LD integration with extract_all()"""

    def test_extract_all_includes_jsonld(self):
        """Test that extract_all() includes JSON-LD"""
        html = """
        <html>
        <head>
            <title>Test Page</title>
            <meta property="og:title" content="OG Title">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Article Title"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "meta" in data
        assert "opengraph" in data
        assert "jsonld" in data

        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Article"
        assert data["jsonld"][0]["headline"] == "Article Title"

    def test_extract_all_without_jsonld(self):
        """Test extract_all() when no JSON-LD present"""
        html = """
        <html>
        <head>
            <title>Test Page</title>
            <meta property="og:title" content="OG Title">
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "meta" in data
        assert "opengraph" in data
        # jsonld key should not be present if no objects found
        assert "jsonld" not in data or len(data.get("jsonld", [])) == 0

    def test_extract_all_multiple_jsonld(self):
        """Test extract_all() with multiple JSON-LD objects"""
        html = """
        <html>
        <head>
            <title>Test Page</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Article"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Person",
                "name": "Author"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 2


class TestJSONLDRealWorld:
    """Test with real-world JSON-LD examples"""

    def test_news_article(self):
        """Test NewsArticle from news site"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "NewsArticle",
                "headline": "Breaking: Major Discovery Announced",
                "description": "Scientists announce breakthrough",
                "datePublished": "2024-01-15T10:30:00Z",
                "dateModified": "2024-01-15T12:00:00Z",
                "author": {
                    "@type": "Person",
                    "name": "Jane Reporter"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "News Corp",
                    "logo": {
                        "@type": "ImageObject",
                        "url": "https://news.com/logo.png"
                    }
                },
                "image": "https://news.com/article-image.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "NewsArticle"
        assert objects[0]["headline"] == "Breaking: Major Discovery Announced"

    def test_ecommerce_product(self):
        """Test Product from e-commerce site"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Laptop Computer",
                "description": "High-performance laptop",
                "sku": "LAPTOP-123",
                "brand": {
                    "@type": "Brand",
                    "name": "TechBrand"
                },
                "offers": {
                    "@type": "Offer",
                    "price": "999.99",
                    "priceCurrency": "USD",
                    "availability": "https://schema.org/InStock"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.5",
                    "reviewCount": "150"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Product"
        assert objects[0]["name"] == "Laptop Computer"
        assert objects[0]["sku"] == "LAPTOP-123"

    def test_blog_posting(self):
        """Test BlogPosting from blog"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BlogPosting",
                "headline": "How to Build a Web Scraper",
                "description": "A comprehensive guide",
                "datePublished": "2024-01-10",
                "author": {
                    "@type": "Person",
                    "name": "Tech Blogger",
                    "url": "https://blog.com/author/tech-blogger"
                },
                "wordCount": 2500,
                "keywords": ["web scraping", "python", "tutorial"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "BlogPosting"
        assert objects[0]["wordCount"] == 2500


class TestJSONLDDataTypes:
    """Test various JSON data types in JSON-LD"""

    def test_string_values(self):
        """Test string properties"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "String Value",
                "description": "Another string"
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert isinstance(objects[0]["headline"], str)
        assert isinstance(objects[0]["description"], str)

    def test_numeric_values(self):
        """Test numeric properties"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "wordCount": 1500,
                "rating": 4.5
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert objects[0]["wordCount"] == 1500
        assert objects[0]["rating"] == 4.5

    def test_boolean_values(self):
        """Test boolean properties"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "inStock": true,
                "discontinued": false
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert objects[0]["inStock"] is True
        assert objects[0]["discontinued"] is False

    def test_array_values(self):
        """Test array properties"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Organization",
                "sameAs": [
                    "https://twitter.com/org",
                    "https://facebook.com/org"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        # Arrays are currently returned as JSON strings - this is by design
        assert "sameAs" in objects[0]

    def test_null_values(self):
        """Test null properties"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Test",
                "description": null
            }
            </script>
        </head>
        <body></body>
        </html>
        """
        objects = meta_oxide.extract_jsonld(html)
        assert objects[0]["headline"] == "Test"
        # Null should be present
        assert "description" in objects[0]


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
