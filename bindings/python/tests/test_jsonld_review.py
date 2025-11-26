"""
Tests for JSON-LD Review type (Schema.org)

Tests the extraction and parsing of Review structured data following TDD approach.
"""

import meta_oxide
import pytest


class TestReviewBasic:
    """Test basic Review extraction"""

    def test_review_basic(self):
        """Test minimal review with reviewBody"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "This is a great product! Highly recommend it to everyone."
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert (
            objects[0]["reviewBody"] == "This is a great product! Highly recommend it to everyone."
        )

    def test_review_with_rating(self):
        """Test review with reviewRating object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Excellent quality and fast shipping.",
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["reviewBody"] == "Excellent quality and fast shipping."
        assert "reviewRating" in objects[0]

    def test_review_with_item(self):
        """Test review with itemReviewed product/service"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Amazing headphones with great sound quality.",
                "itemReviewed": {
                    "@type": "Product",
                    "name": "Wireless Headphones XM5",
                    "brand": "Sony"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "4.5"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert "itemReviewed" in objects[0]
        assert "reviewRating" in objects[0]

    def test_review_with_author(self):
        """Test review with author Person object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Great service and friendly staff.",
                "author": {
                    "@type": "Person",
                    "name": "Jane Doe"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert "author" in objects[0]

    def test_review_complete(self):
        """Test review with all fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "name": "Excellent Product Review",
                "reviewBody": "I've been using this product for three months now and it exceeded all my expectations. The build quality is outstanding, performance is top-notch, and customer service was excellent.",
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5",
                    "worstRating": "1"
                },
                "itemReviewed": {
                    "@type": "Product",
                    "name": "Premium Laptop Pro",
                    "brand": {
                        "@type": "Brand",
                        "name": "TechBrand"
                    },
                    "sku": "LAPTOP-PRO-2024"
                },
                "author": {
                    "@type": "Person",
                    "name": "John Smith",
                    "url": "https://example.com/users/johnsmith"
                },
                "datePublished": "2024-01-15",
                "publisher": {
                    "@type": "Organization",
                    "name": "Review Site Inc",
                    "url": "https://reviews.example.com"
                },
                "url": "https://reviews.example.com/laptop-pro-review"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["name"] == "Excellent Product Review"
        assert "reviewBody" in objects[0]
        assert "reviewRating" in objects[0]
        assert "itemReviewed" in objects[0]
        assert "author" in objects[0]
        assert objects[0]["datePublished"] == "2024-01-15"
        assert "publisher" in objects[0]
        assert objects[0]["url"] == "https://reviews.example.com/laptop-pro-review"


class TestReviewVariations:
    """Test various Review variations"""

    def test_review_of_service(self):
        """Test review of a service (not product)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Excellent plumbing service, very professional.",
                "itemReviewed": {
                    "@type": "Service",
                    "name": "Emergency Plumbing",
                    "provider": {
                        "@type": "Organization",
                        "name": "ABC Plumbers"
                    }
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert "itemReviewed" in objects[0]

    def test_review_with_organization_author(self):
        """Test review with Organization as author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "After thorough testing by our lab, this product meets all standards.",
                "author": {
                    "@type": "Organization",
                    "name": "Tech Review Labs",
                    "url": "https://techlabs.example.com"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "4.5"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert "author" in objects[0]

    def test_review_with_date_formats(self):
        """Test review with different date formats"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Good product overall.",
                "datePublished": "2024-01-15T10:30:00Z",
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "4"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["datePublished"] == "2024-01-15T10:30:00Z"

    def test_review_minimal_with_name_only(self):
        """Test review with just name field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "name": "Quick Review"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["name"] == "Quick Review"


class TestReviewIntegration:
    """Test Review integration with other types"""

    def test_review_in_product(self):
        """Test Review as part of Product"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Smart Watch Pro",
                "review": {
                    "@type": "Review",
                    "reviewBody": "Best smartwatch I've ever used!",
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": "5"
                    },
                    "author": {
                        "@type": "Person",
                        "name": "Alex Johnson"
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
        assert objects[0]["@type"] == "Product"
        assert "review" in objects[0]

    def test_multiple_reviews(self):
        """Test multiple separate Review objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "First review text",
                "author": {
                    "@type": "Person",
                    "name": "User One"
                }
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Second review text",
                "author": {
                    "@type": "Person",
                    "name": "User Two"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["@type"] == "Review"
        assert objects[1]["@type"] == "Review"

    def test_review_in_graph(self):
        """Test Review in @graph array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Product",
                        "name": "Test Product"
                    },
                    {
                        "@type": "Review",
                        "reviewBody": "Great product!",
                        "reviewRating": {
                            "@type": "Rating",
                            "ratingValue": "5"
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
        assert objects[0]["@type"] == "Product"
        assert objects[1]["@type"] == "Review"


class TestReviewEdgeCases:
    """Test edge cases for Review"""

    def test_review_empty_fields(self):
        """Test review with empty optional fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"

    def test_review_with_null_values(self):
        """Test review with explicit null values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "reviewBody": "Good product",
                "name": null,
                "url": null
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["reviewBody"] == "Good product"

    def test_review_unicode_content(self):
        """Test review with Unicode characters"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Review",
                "name": "素晴らしい製品レビュー",
                "reviewBody": "この製品は素晴らしいです！",
                "author": {
                    "@type": "Person",
                    "name": "田中太郎"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Review"
        assert objects[0]["name"] == "素晴らしい製品レビュー"
        assert objects[0]["reviewBody"] == "この製品は素晴らしいです！"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
