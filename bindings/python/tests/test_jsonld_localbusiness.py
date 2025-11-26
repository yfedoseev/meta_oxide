"""
Tests for JSON-LD LocalBusiness type extraction

Following TDD approach: These tests are written FIRST and will initially fail.
Once the LocalBusiness type is implemented, these tests should pass.
"""

import meta_oxide
import pytest


class TestLocalBusinessBasic:
    """Test basic LocalBusiness extraction"""

    def test_business_basic(self):
        """Test minimal business with just name"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Joe's Coffee Shop"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "LocalBusiness"
        assert objects[0]["name"] == "Joe's Coffee Shop"

    def test_business_with_description(self):
        """Test business with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Tech Repair Shop",
                "description": "Expert computer and phone repairs"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "LocalBusiness"
        assert objects[0]["name"] == "Tech Repair Shop"
        assert objects[0]["description"] == "Expert computer and phone repairs"


class TestLocalBusinessAddress:
    """Test LocalBusiness with address information"""

    def test_business_with_address(self):
        """Test business with PostalAddress object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Main Street Bakery",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "123 Main Street",
                    "addressLocality": "Springfield",
                    "addressRegion": "IL",
                    "postalCode": "62701",
                    "addressCountry": "US"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "LocalBusiness"
        assert objects[0]["name"] == "Main Street Bakery"
        assert "address" in objects[0]
        # Address is stored as JSON string per existing pattern
        assert "streetAddress" in str(objects[0]["address"])
        assert "Springfield" in str(objects[0]["address"])

    def test_business_with_simple_address(self):
        """Test business with simple address fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Corner Store",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "456 Oak Avenue"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "address" in objects[0]


class TestLocalBusinessContact:
    """Test LocalBusiness with contact information"""

    def test_business_with_telephone(self):
        """Test business with telephone number"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Pizza Palace",
                "telephone": "+1-555-123-4567"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["telephone"] == "+1-555-123-4567"

    def test_business_with_email(self):
        """Test business with email address"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Consulting LLC",
                "email": "info@consulting.com"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["email"] == "info@consulting.com"

    def test_business_with_url(self):
        """Test business with website URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Web Design Co",
                "url": "https://webdesign.example.com"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["url"] == "https://webdesign.example.com"

    def test_business_with_contact(self):
        """Test business with all contact info"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Full Contact Business",
                "telephone": "+1-555-999-0000",
                "email": "contact@business.com",
                "url": "https://business.com"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["telephone"] == "+1-555-999-0000"
        assert objects[0]["email"] == "contact@business.com"
        assert objects[0]["url"] == "https://business.com"


class TestLocalBusinessGeo:
    """Test LocalBusiness with geographic coordinates"""

    def test_business_with_geo(self):
        """Test business with GeoCoordinates"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Mountain View Cafe",
                "geo": {
                    "@type": "GeoCoordinates",
                    "latitude": "37.3861",
                    "longitude": "-122.0839"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Mountain View Cafe"
        assert "geo" in objects[0]
        # Geo is stored as JSON string per existing pattern
        assert "latitude" in str(objects[0]["geo"])
        assert "longitude" in str(objects[0]["geo"])


class TestLocalBusinessHours:
    """Test LocalBusiness with opening hours"""

    def test_business_with_hours(self):
        """Test business with openingHoursSpecification"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Daily Diner",
                "openingHoursSpecification": [
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": "Monday",
                        "opens": "08:00",
                        "closes": "18:00"
                    },
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": "Tuesday",
                        "opens": "08:00",
                        "closes": "18:00"
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
        assert objects[0]["name"] == "Daily Diner"
        assert "openingHoursSpecification" in objects[0]
        # Hours array is stored as JSON string per existing pattern
        hours_str = str(objects[0]["openingHoursSpecification"])
        assert "Monday" in hours_str
        assert "08:00" in hours_str


class TestLocalBusinessRating:
    """Test LocalBusiness with ratings and reviews"""

    def test_business_with_rating(self):
        """Test business with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Top Rated Store",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.8",
                    "reviewCount": "127"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Top Rated Store"
        assert "aggregateRating" in objects[0]
        rating_str = str(objects[0]["aggregateRating"])
        assert "4.8" in rating_str
        assert "127" in rating_str

    def test_business_with_reviews(self):
        """Test business with review array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Reviewed Business",
                "review": [
                    {
                        "@type": "Review",
                        "author": {
                            "@type": "Person",
                            "name": "John Doe"
                        },
                        "reviewRating": {
                            "@type": "Rating",
                            "ratingValue": "5"
                        },
                        "reviewBody": "Great service!"
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
        assert objects[0]["name"] == "Reviewed Business"
        assert "review" in objects[0]
        review_str = str(objects[0]["review"])
        assert "John Doe" in review_str
        assert "Great service" in review_str


class TestLocalBusinessRestaurant:
    """Test Restaurant (subtype of LocalBusiness)"""

    def test_business_restaurant(self):
        """Test Restaurant with servesCuisine"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Restaurant",
                "name": "Italian Bistro",
                "servesCuisine": ["Italian", "Mediterranean"],
                "priceRange": "$$"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Restaurant"
        assert objects[0]["name"] == "Italian Bistro"
        assert "servesCuisine" in objects[0]
        assert objects[0]["priceRange"] == "$$"

    def test_business_with_price_range(self):
        """Test business with priceRange"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Budget Shop",
                "priceRange": "$"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["priceRange"] == "$"


class TestLocalBusinessImages:
    """Test LocalBusiness with images"""

    def test_business_with_single_image(self):
        """Test business with single image string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Photo Studio",
                "image": "https://example.com/studio.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["image"] == "https://example.com/studio.jpg"

    def test_business_with_multiple_images(self):
        """Test business with array of images"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Gallery Shop",
                "image": [
                    "https://example.com/img1.jpg",
                    "https://example.com/img2.jpg"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "image" in objects[0]
        # Image array is stored as JSON string per existing pattern
        assert "img1.jpg" in str(objects[0]["image"])


class TestLocalBusinessComplete:
    """Test complete LocalBusiness with all fields"""

    def test_business_complete(self):
        """Test business with all possible fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Restaurant",
                "name": "The Complete Restaurant",
                "description": "A restaurant with everything",
                "image": "https://example.com/restaurant.jpg",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "789 Complete Ave",
                    "addressLocality": "Full City",
                    "addressRegion": "CA",
                    "postalCode": "90210",
                    "addressCountry": "US"
                },
                "telephone": "+1-555-FULL-123",
                "email": "info@complete.com",
                "url": "https://complete-restaurant.com",
                "geo": {
                    "@type": "GeoCoordinates",
                    "latitude": "34.0522",
                    "longitude": "-118.2437"
                },
                "openingHoursSpecification": [
                    {
                        "@type": "OpeningHoursSpecification",
                        "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
                        "opens": "11:00",
                        "closes": "22:00"
                    }
                ],
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "500"
                },
                "review": [
                    {
                        "@type": "Review",
                        "author": {
                            "@type": "Person",
                            "name": "Happy Customer"
                        },
                        "reviewRating": {
                            "@type": "Rating",
                            "ratingValue": "5"
                        },
                        "reviewBody": "Best restaurant ever!"
                    }
                ],
                "priceRange": "$$$",
                "servesCuisine": ["French", "American"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        # Verify all basic fields
        assert obj["@type"] == "Restaurant"
        assert obj["name"] == "The Complete Restaurant"
        assert obj["description"] == "A restaurant with everything"
        assert obj["image"] == "https://example.com/restaurant.jpg"
        assert obj["telephone"] == "+1-555-FULL-123"
        assert obj["email"] == "info@complete.com"
        assert obj["url"] == "https://complete-restaurant.com"
        assert obj["priceRange"] == "$$$"

        # Verify complex fields exist (stored as JSON strings)
        assert "address" in obj
        assert "geo" in obj
        assert "openingHoursSpecification" in obj
        assert "aggregateRating" in obj
        assert "review" in obj
        assert "servesCuisine" in obj


class TestLocalBusinessSubtypes:
    """Test various LocalBusiness subtypes"""

    def test_store_subtype(self):
        """Test Store subtype"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Store",
                "name": "General Store"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Store"
        assert objects[0]["name"] == "General Store"

    def test_cafe_subtype(self):
        """Test Cafe/CafeOrCoffeeShop subtype"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "CafeOrCoffeeShop",
                "name": "The Coffee House"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "CafeOrCoffeeShop"


class TestLocalBusinessIntegration:
    """Test LocalBusiness integration with extract_all()"""

    def test_extract_all_includes_localbusiness(self):
        """Test that extract_all() includes LocalBusiness"""
        html = """
        <html>
        <head>
            <title>Business Page</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "Test Business",
                "telephone": "+1-555-0000"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "LocalBusiness"
        assert data["jsonld"][0]["name"] == "Test Business"
        assert data["jsonld"][0]["telephone"] == "+1-555-0000"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
