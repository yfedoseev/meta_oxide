"""
Tests for JSON-LD AggregateRating type (Schema.org)

Tests the extraction and parsing of AggregateRating structured data following TDD approach.
AggregateRating represents the average rating from multiple reviews/ratings.
"""

import meta_oxide
import pytest


class TestAggregateRatingBasic:
    """Test basic AggregateRating extraction"""

    def test_aggregaterating_basic(self):
        """Test minimal rating with ratingValue"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.5
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.5

    def test_aggregaterating_with_bounds(self):
        """Test rating with bestRating and worstRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.2,
                "bestRating": 5,
                "worstRating": 1
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.2
        assert objects[0]["bestRating"] == 5.0
        assert objects[0]["worstRating"] == 1.0

    def test_aggregaterating_with_counts(self):
        """Test rating with ratingCount and reviewCount"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.8,
                "ratingCount": 156,
                "reviewCount": 89
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.8
        assert objects[0]["ratingCount"] == 156
        assert objects[0]["reviewCount"] == 89

    def test_aggregaterating_with_item(self):
        """Test rating with itemReviewed product"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.7,
                "itemReviewed": {
                    "@type": "Product",
                    "name": "Wireless Headphones",
                    "brand": "AudioTech"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.7
        assert "itemReviewed" in objects[0]
        assert objects[0]["itemReviewed"]["@type"] == "Product"
        assert objects[0]["itemReviewed"]["name"] == "Wireless Headphones"

    def test_aggregaterating_complete(self):
        """Test rating with all fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.6,
                "bestRating": 5,
                "worstRating": 1,
                "ratingCount": 245,
                "reviewCount": 187,
                "itemReviewed": {
                    "@type": "Product",
                    "name": "Premium Laptop",
                    "brand": "TechBrand",
                    "sku": "LAPTOP-2024"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.6
        assert objects[0]["bestRating"] == 5.0
        assert objects[0]["worstRating"] == 1.0
        assert objects[0]["ratingCount"] == 245
        assert objects[0]["reviewCount"] == 187
        assert "itemReviewed" in objects[0]
        assert objects[0]["itemReviewed"]["@type"] == "Product"


class TestAggregateRatingRealWorld:
    """Test realistic AggregateRating scenarios"""

    def test_product_rating(self):
        """Test realistic product rating example"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Smart TV 55 inch",
                "description": "4K Ultra HD Smart Television",
                "brand": "Samsung",
                "sku": "TV-55-4K-2024",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 4.5,
                    "bestRating": 5,
                    "worstRating": 1,
                    "ratingCount": 892,
                    "reviewCount": 456
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
        assert objects[0]["name"] == "Smart TV 55 inch"
        assert "aggregateRating" in objects[0]
        rating = objects[0]["aggregateRating"]
        assert rating["@type"] == "AggregateRating"
        assert rating["ratingValue"] == 4.5
        assert rating["ratingCount"] == 892
        assert rating["reviewCount"] == 456

    def test_restaurant_rating(self):
        """Test restaurant rating example"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Restaurant",
                "name": "The Golden Spoon",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "123 Main St",
                    "addressLocality": "New York",
                    "postalCode": 10001
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 4.8,
                    "bestRating": 5,
                    "ratingCount": 327,
                    "reviewCount": 215
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Restaurant"
        assert objects[0]["name"] == "The Golden Spoon"
        assert "aggregateRating" in objects[0]
        rating = objects[0]["aggregateRating"]
        assert rating["@type"] == "AggregateRating"
        assert rating["ratingValue"] == 4.8
        assert rating["ratingCount"] == 327


class TestAggregateRatingEdgeCases:
    """Test edge cases for AggregateRating"""

    def test_aggregaterating_empty(self):
        """Test empty AggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"

    def test_aggregaterating_integer_rating(self):
        """Test rating with integer values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4,
                "bestRating": 5,
                "ratingCount": 100
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.0
        assert objects[0]["bestRating"] == 5.0
        assert objects[0]["ratingCount"] == 100

    def test_aggregaterating_null_values(self):
        """Test rating with null values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.5,
                "bestRating": null,
                "worstRating": null
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.5

    def test_aggregaterating_decimal_precision(self):
        """Test rating with high decimal precision"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.687,
                "bestRating": 5.0
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        # Allow for floating point comparison
        assert abs(objects[0]["ratingValue"] - 4.687) < 0.001


class TestAggregateRatingIntegration:
    """Test AggregateRating integration with extract_all"""

    def test_extract_all_includes_aggregaterating(self):
        """Test that extract_all() includes AggregateRating"""
        html = """
        <html>
        <head>
            <title>Product Review Page</title>
            <meta property="og:title" content="Best Product Reviews">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Product",
                "name": "Test Product",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 4.5,
                    "ratingCount": 100
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        # Should have meta tags
        assert "meta" in data
        assert data["meta"]["title"] == "Product Review Page"

        # Should have JSON-LD with Product and AggregateRating
        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Product"
        assert "aggregateRating" in data["jsonld"][0]
        assert data["jsonld"][0]["aggregateRating"]["@type"] == "AggregateRating"
        assert data["jsonld"][0]["aggregateRating"]["ratingValue"] == 4.5

    def test_multiple_ratings_in_graph(self):
        """Test multiple items with ratings in @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Product",
                        "name": "Product A",
                        "aggregateRating": {
                            "@type": "AggregateRating",
                            "ratingValue": 4.5
                        }
                    },
                    {
                        "@type": "LocalBusiness",
                        "name": "Business B",
                        "aggregateRating": {
                            "@type": "AggregateRating",
                            "ratingValue": 4.8
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
        assert "aggregateRating" in objects[0]
        assert objects[0]["aggregateRating"]["ratingValue"] == 4.5

        assert objects[1]["@type"] == "LocalBusiness"
        assert "aggregateRating" in objects[1]
        assert objects[1]["aggregateRating"]["ratingValue"] == 4.8

    def test_aggregaterating_with_organization(self):
        """Test AggregateRating with Organization itemReviewed"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AggregateRating",
                "ratingValue": 4.3,
                "ratingCount": 521,
                "itemReviewed": {
                    "@type": "Organization",
                    "name": "Acme Corporation",
                    "url": "https://acme.example.com"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AggregateRating"
        assert objects[0]["ratingValue"] == 4.3
        assert objects[0]["itemReviewed"]["@type"] == "Organization"
        assert objects[0]["itemReviewed"]["name"] == "Acme Corporation"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
