"""
Tests for JSON-LD FAQPage extraction (TDD approach)

Following Schema.org FAQPage specification:
https://schema.org/FAQPage
"""

import meta_oxide
import pytest


class TestFAQPageBasic:
    """Test basic FAQPage extraction"""

    def test_faqpage_basic(self):
        """Test minimal FAQPage with one Q&A"""
        html = """
        <html>
        <head>
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
                            "text": "JSON-LD is a lightweight Linked Data format."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert "mainEntity" in objects[0]
        # mainEntity should be a JSON string containing the array
        assert "Question" in str(objects[0]["mainEntity"])


class TestFAQPageMultipleQuestions:
    """Test FAQPage with multiple Q&A pairs"""

    def test_faqpage_multiple_questions(self):
        """Test FAQPage with multiple Q&A pairs"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What is Schema.org?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Schema.org is a collaborative initiative to create structured data schemas."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "Why use structured data?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Structured data helps search engines understand your content better."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "How to implement FAQPage?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Use JSON-LD script tags with FAQPage schema."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert "mainEntity" in objects[0]
        # Should contain multiple questions
        main_entity_str = str(objects[0]["mainEntity"])
        assert "What is Schema.org?" in main_entity_str or "Schema.org" in main_entity_str
        assert "Why use structured data?" in main_entity_str or "structured data" in main_entity_str


class TestFAQPageWithMetadata:
    """Test FAQPage with additional metadata"""

    def test_faqpage_with_metadata(self):
        """Test FAQPage with author, datePublished"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "Common Questions About SEO",
                "description": "Frequently asked questions about search engine optimization",
                "datePublished": "2024-01-15",
                "author": {
                    "@type": "Person",
                    "name": "John SEO Expert"
                },
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What is SEO?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "SEO stands for Search Engine Optimization."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert objects[0]["name"] == "Common Questions About SEO"
        assert (
            objects[0]["description"]
            == "Frequently asked questions about search engine optimization"
        )
        assert objects[0]["datePublished"] == "2024-01-15"
        assert "author" in objects[0]


class TestFAQPageComplete:
    """Test complete FAQPage with all fields"""

    def test_faqpage_complete(self):
        """Test FAQPage with all possible fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "Complete FAQ Page",
                "description": "A comprehensive FAQ about our services",
                "url": "https://example.com/faq",
                "datePublished": "2024-01-15T10:00:00Z",
                "author": {
                    "@type": "Organization",
                    "name": "Example Corp",
                    "url": "https://example.com"
                },
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What services do you offer?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "We offer web development, SEO, and consulting services."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "How can I contact you?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "You can reach us via email at contact@example.com or call us at 555-1234."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        faq = objects[0]

        assert faq["@type"] == "FAQPage"
        assert faq["name"] == "Complete FAQ Page"
        assert faq["description"] == "A comprehensive FAQ about our services"
        assert faq["url"] == "https://example.com/faq"
        assert faq["datePublished"] == "2024-01-15T10:00:00Z"
        assert "author" in faq
        assert "mainEntity" in faq


class TestFAQPageEdgeCases:
    """Test edge cases for FAQPage"""

    def test_faqpage_empty_mainentity(self):
        """Test FAQPage with empty mainEntity array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "Empty FAQ",
                "mainEntity": []
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert objects[0]["name"] == "Empty FAQ"
        assert "mainEntity" in objects[0]

    def test_faqpage_minimal(self):
        """Test minimal FAQPage with just type"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"

    def test_faqpage_with_rich_answer(self):
        """Test FAQPage with rich answer content (HTML in text)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "How to format code?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "<p>Use the <code>code</code> tag for inline code.</p>"
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert "mainEntity" in objects[0]


class TestFAQPageIntegration:
    """Test FAQPage integration with extract_all()"""

    def test_extract_all_includes_faqpage(self):
        """Test that extract_all() properly extracts FAQPage"""
        html = """
        <html>
        <head>
            <title>FAQ - Example Site</title>
            <meta name="description" content="Frequently asked questions">
            <meta property="og:title" content="FAQ Page">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "Product FAQ",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "Is shipping free?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Yes, we offer free shipping on orders over $50."
                        }
                    }
                ]
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
        assert data["jsonld"][0]["@type"] == "FAQPage"
        assert data["jsonld"][0]["name"] == "Product FAQ"

    def test_multiple_jsonld_with_faqpage(self):
        """Test FAQPage alongside other JSON-LD types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Organization",
                "name": "Example Corp"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "Who are you?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "We are Example Corp."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        types = [obj["@type"] for obj in objects]
        assert "Organization" in types
        assert "FAQPage" in types


class TestFAQPageRealWorld:
    """Test real-world FAQPage examples"""

    def test_ecommerce_faq(self):
        """Test e-commerce FAQ page"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "Shipping & Returns FAQ",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What is your return policy?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "We accept returns within 30 days of purchase for a full refund."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "How long does shipping take?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Standard shipping takes 5-7 business days. Express shipping takes 2-3 business days."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "Do you ship internationally?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Yes, we ship to over 50 countries worldwide."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "FAQPage"
        assert objects[0]["name"] == "Shipping & Returns FAQ"

    def test_technical_documentation_faq(self):
        """Test technical documentation FAQ"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "FAQPage",
                "name": "API Documentation FAQ",
                "description": "Common questions about our API",
                "url": "https://api.example.com/faq",
                "datePublished": "2024-01-15",
                "author": {
                    "@type": "Organization",
                    "name": "Example API Team"
                },
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "How do I get an API key?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Sign up for an account and navigate to the API Keys section in your dashboard."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "What are the rate limits?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Free tier: 1000 requests/day. Pro tier: 100,000 requests/day."
                        }
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        faq = objects[0]
        assert faq["@type"] == "FAQPage"
        assert faq["name"] == "API Documentation FAQ"
        assert faq["description"] == "Common questions about our API"
        assert faq["url"] == "https://api.example.com/faq"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
