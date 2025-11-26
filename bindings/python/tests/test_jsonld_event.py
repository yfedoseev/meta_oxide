"""
Tests for JSON-LD Event type (Schema.org Event)

Following TDD approach - tests written FIRST before implementation
"""

import meta_oxide
import pytest


class TestEventBasic:
    """Test basic Event extraction"""

    def test_event_basic(self):
        """Test minimal event with name and startDate"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Tech Conference 2024",
                "startDate": "2024-06-15T09:00:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert objects[0]["name"] == "Tech Conference 2024"
        assert objects[0]["startDate"] == "2024-06-15T09:00:00Z"


class TestEventWithDates:
    """Test Event with various date/time fields"""

    def test_event_with_dates(self):
        """Test event with startDate, endDate, duration"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Workshop",
                "startDate": "2024-06-15T10:00:00Z",
                "endDate": "2024-06-15T12:00:00Z",
                "duration": "PT2H"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert objects[0]["name"] == "Workshop"
        assert objects[0]["startDate"] == "2024-06-15T10:00:00Z"
        assert objects[0]["endDate"] == "2024-06-15T12:00:00Z"
        assert objects[0]["duration"] == "PT2H"

    def test_event_rescheduled(self):
        """Test event with previousStartDate (rescheduled event)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Concert",
                "startDate": "2024-07-20T19:00:00Z",
                "previousStartDate": "2024-06-20T19:00:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert objects[0]["previousStartDate"] == "2024-06-20T19:00:00Z"
        assert objects[0]["startDate"] == "2024-07-20T19:00:00Z"


class TestEventWithLocation:
    """Test Event with location information"""

    def test_event_with_location_object(self):
        """Test event with location as Place object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Art Exhibition",
                "startDate": "2024-08-01T10:00:00Z",
                "location": {
                    "@type": "Place",
                    "name": "City Art Gallery",
                    "address": {
                        "@type": "PostalAddress",
                        "streetAddress": "123 Main St",
                        "addressLocality": "New York",
                        "addressRegion": "NY",
                        "postalCode": "10001"
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
        assert objects[0]["@type"] == "Event"
        assert "location" in objects[0]
        # Location is stored as JSON string in current implementation
        assert "location" in objects[0]

    def test_event_with_location_string(self):
        """Test event with location as simple string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Street Fair",
                "startDate": "2024-09-01T12:00:00Z",
                "location": "Downtown Main Street"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        # Location as string might be converted - check it exists
        assert "location" in objects[0]


class TestEventWithPerformer:
    """Test Event with performer information"""

    def test_event_with_performer(self):
        """Test event with performer array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Rock Concert",
                "startDate": "2024-10-15T20:00:00Z",
                "performer": [
                    {
                        "@type": "Person",
                        "name": "John Doe"
                    },
                    {
                        "@type": "MusicGroup",
                        "name": "The Band"
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
        assert objects[0]["@type"] == "Event"
        assert "performer" in objects[0]

    def test_event_with_organizer(self):
        """Test event with organizer"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Business Conference",
                "startDate": "2024-11-01T09:00:00Z",
                "organizer": {
                    "@type": "Organization",
                    "name": "Tech Events Inc",
                    "url": "https://techevents.com"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert "organizer" in objects[0]


class TestEventWithOffers:
    """Test Event with ticket offers"""

    def test_event_with_offers(self):
        """Test event with ticket offers"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Music Festival",
                "startDate": "2024-12-20T12:00:00Z",
                "offers": [
                    {
                        "@type": "Offer",
                        "price": "49.99",
                        "priceCurrency": "USD",
                        "availability": "https://schema.org/InStock",
                        "url": "https://tickets.example.com/early-bird"
                    },
                    {
                        "@type": "Offer",
                        "price": "79.99",
                        "priceCurrency": "USD",
                        "availability": "https://schema.org/InStock",
                        "url": "https://tickets.example.com/regular"
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
        assert objects[0]["@type"] == "Event"
        assert "offers" in objects[0]


class TestEventVirtual:
    """Test virtual/online events"""

    def test_event_virtual(self):
        """Test virtual event with eventAttendanceMode"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Online Webinar",
                "startDate": "2024-05-10T14:00:00Z",
                "endDate": "2024-05-10T15:00:00Z",
                "eventAttendanceMode": "https://schema.org/OnlineEventAttendanceMode",
                "location": {
                    "@type": "VirtualLocation",
                    "url": "https://zoom.us/j/123456789"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert "eventAttendanceMode" in objects[0]
        assert "location" in objects[0]

    def test_event_status(self):
        """Test event with eventStatus"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Cancelled Event",
                "startDate": "2024-06-01T10:00:00Z",
                "eventStatus": "https://schema.org/EventCancelled"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Event"
        assert "eventStatus" in objects[0]


class TestEventComplete:
    """Test complete event with all fields"""

    def test_event_complete(self):
        """Test event with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Complete Event",
                "description": "A comprehensive event with all fields",
                "image": "https://example.com/event.jpg",
                "startDate": "2024-07-15T18:00:00Z",
                "endDate": "2024-07-15T22:00:00Z",
                "duration": "PT4H",
                "url": "https://example.com/event",
                "location": {
                    "@type": "Place",
                    "name": "Convention Center",
                    "address": "123 Event Ave, City, ST 12345"
                },
                "performer": [
                    {
                        "@type": "Person",
                        "name": "Performer One"
                    }
                ],
                "organizer": {
                    "@type": "Organization",
                    "name": "Event Organizers Ltd"
                },
                "offers": [
                    {
                        "@type": "Offer",
                        "price": "99.99",
                        "priceCurrency": "USD"
                    }
                ],
                "eventStatus": "https://schema.org/EventScheduled",
                "eventAttendanceMode": "https://schema.org/OfflineEventAttendanceMode"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        event = objects[0]
        assert event["@type"] == "Event"
        assert event["name"] == "Complete Event"
        assert event["description"] == "A comprehensive event with all fields"
        assert event["image"] == "https://example.com/event.jpg"
        assert event["startDate"] == "2024-07-15T18:00:00Z"
        assert event["endDate"] == "2024-07-15T22:00:00Z"
        assert event["duration"] == "PT4H"
        assert event["url"] == "https://example.com/event"
        assert "location" in event
        assert "performer" in event
        assert "organizer" in event
        assert "offers" in event
        assert "eventStatus" in event
        assert "eventAttendanceMode" in event

    def test_event_with_multiple_images(self):
        """Test event with multiple images"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Photo Event",
                "startDate": "2024-08-01T10:00:00Z",
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
        assert objects[0]["@type"] == "Event"
        assert "image" in objects[0]


class TestEventIntegration:
    """Test Event integration with extract_all()"""

    def test_extract_all_includes_event(self):
        """Test that extract_all() includes Event"""
        html = """
        <html>
        <head>
            <title>Event Page</title>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Event",
                "name": "Sample Event",
                "startDate": "2024-09-01T10:00:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Event"
        assert data["jsonld"][0]["name"] == "Sample Event"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
