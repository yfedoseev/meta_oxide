"""
Tests for JSON-LD WebSite type (Schema.org WebSite)

Following TDD approach - tests written FIRST before implementation
"""

import meta_oxide
import pytest


class TestWebSiteBasic:
    """Test basic WebSite extraction"""

    def test_website_basic(self):
        """Test minimal website with name"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Example Tech Blog"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "WebSite"
        assert objects[0]["name"] == "Example Tech Blog"


class TestWebSiteWithSearchAction:
    """Test WebSite with potentialAction for site search"""

    def test_website_with_search_action(self):
        """Test website with SearchAction for site search"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Example Site",
                "url": "https://example.com",
                "potentialAction": {
                    "@type": "SearchAction",
                    "target": "https://example.com/search?q={search_term_string}",
                    "query-input": "required name=search_term_string"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "WebSite"
        assert objects[0]["name"] == "Example Site"
        assert objects[0]["url"] == "https://example.com"
        assert "potentialAction" in objects[0]


class TestWebSiteWithPublisher:
    """Test WebSite with publisher organization"""

    def test_website_with_publisher(self):
        """Test website with publisher organization"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Tech News",
                "url": "https://technews.example.com",
                "publisher": {
                    "@type": "Organization",
                    "name": "Tech Media Corp",
                    "logo": {
                        "@type": "ImageObject",
                        "url": "https://example.com/logo.png"
                    }
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "WebSite"
        assert objects[0]["name"] == "Tech News"
        assert "publisher" in objects[0]


class TestWebSiteWithLanguage:
    """Test WebSite with inLanguage"""

    def test_website_with_language(self):
        """Test website with inLanguage field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "French Blog",
                "url": "https://fr.example.com",
                "inLanguage": "fr-FR"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "WebSite"
        assert objects[0]["name"] == "French Blog"
        assert objects[0]["inLanguage"] == "fr-FR"


class TestWebSiteWithCopyright:
    """Test WebSite with copyright information"""

    def test_website_with_copyright(self):
        """Test website with copyright holder and year"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Corporate Site",
                "url": "https://corporate.example.com",
                "copyrightHolder": {
                    "@type": "Organization",
                    "name": "Example Corp"
                },
                "copyrightYear": "2024"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "WebSite"
        assert objects[0]["name"] == "Corporate Site"
        assert "copyrightHolder" in objects[0]
        assert objects[0]["copyrightYear"] == "2024"


class TestWebSiteComplete:
    """Test complete WebSite with all fields"""

    def test_website_complete(self):
        """Test website with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Complete Example Site",
                "description": "A comprehensive website with all metadata",
                "url": "https://complete.example.com",
                "image": "https://example.com/logo.png",
                "inLanguage": "en-US",
                "copyrightYear": "2024",
                "copyrightHolder": {
                    "@type": "Organization",
                    "name": "Complete Inc"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Complete Publishing"
                },
                "potentialAction": {
                    "@type": "SearchAction",
                    "target": "https://complete.example.com/search?q={search_term_string}",
                    "query-input": "required name=search_term_string"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        website = objects[0]
        assert website["@type"] == "WebSite"
        assert website["name"] == "Complete Example Site"
        assert website["description"] == "A comprehensive website with all metadata"
        assert website["url"] == "https://complete.example.com"
        assert website["image"] == "https://example.com/logo.png"
        assert website["inLanguage"] == "en-US"
        assert website["copyrightYear"] == "2024"
        assert "copyrightHolder" in website
        assert "publisher" in website
        assert "potentialAction" in website


class TestWebSiteRealistic:
    """Test realistic WebSite examples"""

    def test_blog_website(self):
        """Test realistic blog website"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Personal Tech Blog",
                "description": "Articles about software development, AI, and technology",
                "url": "https://myblog.dev",
                "inLanguage": "en-US",
                "potentialAction": {
                    "@type": "SearchAction",
                    "target": "https://myblog.dev/search?q={search_term_string}",
                    "query-input": "required name=search_term_string"
                },
                "publisher": {
                    "@type": "Person",
                    "name": "Jane Developer",
                    "url": "https://myblog.dev/about"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        website = objects[0]
        assert website["@type"] == "WebSite"
        assert website["name"] == "Personal Tech Blog"
        assert "description" in website
        assert "potentialAction" in website
        assert "publisher" in website

    def test_ecommerce_website(self):
        """Test e-commerce site with search"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Example Store",
                "description": "Online shopping for electronics and gadgets",
                "url": "https://store.example.com",
                "image": {
                    "@type": "ImageObject",
                    "url": "https://store.example.com/logo.png",
                    "width": 600,
                    "height": 60
                },
                "potentialAction": {
                    "@type": "SearchAction",
                    "target": {
                        "@type": "EntryPoint",
                        "urlTemplate": "https://store.example.com/search?q={search_term_string}"
                    },
                    "query-input": "required name=search_term_string"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Example Store Inc",
                    "logo": "https://store.example.com/publisher-logo.png"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        website = objects[0]
        assert website["@type"] == "WebSite"
        assert website["name"] == "Example Store"
        assert website["description"] == "Online shopping for electronics and gadgets"
        assert website["url"] == "https://store.example.com"
        assert "image" in website
        assert "potentialAction" in website
        assert "publisher" in website


class TestWebSiteIntegration:
    """Test WebSite integration with extract_all()"""

    def test_extract_all_includes_website(self):
        """Test that extract_all() includes WebSite"""
        html = """
        <html>
        <head>
            <title>My Website</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Sample Website",
                "url": "https://sample.example.com"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "WebSite"
        assert data["jsonld"][0]["name"] == "Sample Website"
        assert data["jsonld"][0]["url"] == "https://sample.example.com"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
