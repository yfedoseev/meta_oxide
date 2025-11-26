"""
Tests for JSON-LD SoftwareApplication type (Schema.org)

Tests the extraction and parsing of SoftwareApplication structured data following TDD approach.
SoftwareApplication is used for software, apps, tools, and applications.
"""

import meta_oxide
import pytest


class TestSoftwareApplicationBasic:
    """Test basic SoftwareApplication extraction"""

    def test_softwareapplication_minimal(self):
        """Test minimal SoftwareApplication with just name"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "My App"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "My App"

    def test_softwareapplication_with_description(self):
        """Test SoftwareApplication with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "PhotoEditor Pro",
                "description": "Professional photo editing software with advanced features",
                "url": "https://example.com/photoeditor"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "PhotoEditor Pro"
        assert (
            objects[0]["description"]
            == "Professional photo editing software with advanced features"
        )
        assert objects[0]["url"] == "https://example.com/photoeditor"

    def test_softwareapplication_with_image(self):
        """Test SoftwareApplication with image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Game Pro",
                "image": "https://example.com/images/game-icon.png"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Game Pro"
        assert objects[0]["image"] == "https://example.com/images/game-icon.png"


class TestSoftwareApplicationDetails:
    """Test application-specific details"""

    def test_softwareapplication_with_category(self):
        """Test SoftwareApplication with applicationCategory"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Chess Master",
                "applicationCategory": "GameApplication"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Chess Master"
        assert objects[0]["applicationCategory"] == "GameApplication"

    def test_softwareapplication_with_operating_system_string(self):
        """Test SoftwareApplication with operatingSystem as string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Windows App",
                "operatingSystem": "Windows 10"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["operatingSystem"] == "Windows 10"

    def test_softwareapplication_with_operating_system_array(self):
        """Test SoftwareApplication with operatingSystem as array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Cross-Platform App",
                "operatingSystem": ["Windows", "macOS", "Linux"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert "operatingSystem" in objects[0]
        assert isinstance(objects[0]["operatingSystem"], list)
        assert len(objects[0]["operatingSystem"]) == 3

    def test_softwareapplication_with_version(self):
        """Test SoftwareApplication with softwareVersion"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "MyApp",
                "softwareVersion": "2.5.1",
                "datePublished": "2024-01-15"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["softwareVersion"] == "2.5.1"
        assert objects[0]["datePublished"] == "2024-01-15"

    def test_softwareapplication_with_categories(self):
        """Test various applicationCategory values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Business Suite",
                "applicationCategory": "BusinessApplication"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["applicationCategory"] == "BusinessApplication"


class TestSoftwareApplicationDownloadInstall:
    """Test download and install related fields"""

    def test_softwareapplication_with_download_url(self):
        """Test SoftwareApplication with downloadUrl"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "PDF Reader",
                "downloadUrl": "https://example.com/downloads/pdf-reader.exe",
                "fileSize": "45MB"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["downloadUrl"] == "https://example.com/downloads/pdf-reader.exe"
        assert objects[0]["fileSize"] == "45MB"

    def test_softwareapplication_with_install_url(self):
        """Test SoftwareApplication with installUrl"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Mobile Game",
                "installUrl": "https://play.google.com/store/apps/details?id=com.example.game"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert (
            objects[0]["installUrl"]
            == "https://play.google.com/store/apps/details?id=com.example.game"
        )

    def test_softwareapplication_with_filesize_formats(self):
        """Test various fileSize formats"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Large App",
                "fileSize": "2.3 GB",
                "downloadUrl": "https://example.com/download"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["fileSize"] == "2.3 GB"


class TestSoftwareApplicationRatingsOffers:
    """Test ratings and offers"""

    def test_softwareapplication_with_aggregate_rating(self):
        """Test SoftwareApplication with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Popular App",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.6",
                    "ratingCount": "8864",
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
        assert objects[0]["@type"] == "SoftwareApplication"
        assert "aggregateRating" in objects[0]
        assert objects[0]["aggregateRating"]["@type"] == "AggregateRating"
        assert objects[0]["aggregateRating"]["ratingValue"] == "4.6"

    def test_softwareapplication_with_offers_free(self):
        """Test SoftwareApplication with free offer"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Free App",
                "offers": {
                    "@type": "Offer",
                    "price": "0",
                    "priceCurrency": "USD"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert "offers" in objects[0]
        assert objects[0]["offers"]["price"] == "0"

    def test_softwareapplication_with_offers_paid(self):
        """Test SoftwareApplication with paid offer"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Premium App",
                "offers": {
                    "@type": "Offer",
                    "price": "49.99",
                    "priceCurrency": "USD",
                    "availability": "https://schema.org/InStock"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "offers" in objects[0]
        assert objects[0]["offers"]["price"] == "49.99"
        assert objects[0]["offers"]["priceCurrency"] == "USD"


class TestSoftwareApplicationScreenshots:
    """Test screenshots and images"""

    def test_softwareapplication_with_single_screenshot(self):
        """Test SoftwareApplication with single screenshot"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Beautiful App",
                "screenshot": "https://example.com/screenshots/app-screen1.png"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["screenshot"] == "https://example.com/screenshots/app-screen1.png"

    def test_softwareapplication_with_multiple_screenshots(self):
        """Test SoftwareApplication with multiple screenshots"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Feature-Rich App",
                "screenshot": [
                    "https://example.com/screenshots/screen1.png",
                    "https://example.com/screenshots/screen2.png",
                    "https://example.com/screenshots/screen3.png"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert isinstance(objects[0]["screenshot"], list)
        assert len(objects[0]["screenshot"]) == 3


class TestSoftwareApplicationRealWorld:
    """Test real-world examples"""

    def test_desktop_application_complete(self):
        """Test complete desktop application"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Video Editor Pro",
                "description": "Professional video editing software for creators",
                "applicationCategory": "MultimediaApplication",
                "operatingSystem": ["Windows 10", "Windows 11", "macOS 12+"],
                "softwareVersion": "3.2.1",
                "datePublished": "2024-02-15",
                "url": "https://example.com/video-editor",
                "image": "https://example.com/images/video-editor-icon.png",
                "downloadUrl": "https://example.com/downloads/video-editor-3.2.1.exe",
                "fileSize": "1.2 GB",
                "offers": {
                    "@type": "Offer",
                    "price": "99.99",
                    "priceCurrency": "USD"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.7",
                    "ratingCount": "1523"
                },
                "author": {
                    "@type": "Organization",
                    "name": "Creative Software Inc"
                },
                "screenshot": [
                    "https://example.com/screenshots/editor-1.png",
                    "https://example.com/screenshots/editor-2.png"
                ],
                "softwareRequirements": "Requires 8GB RAM, 2GB disk space",
                "releaseNotes": "Version 3.2.1 includes performance improvements and bug fixes"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Video Editor Pro"
        assert objects[0]["applicationCategory"] == "MultimediaApplication"
        assert objects[0]["softwareVersion"] == "3.2.1"
        assert "offers" in objects[0]
        assert "aggregateRating" in objects[0]
        assert "screenshot" in objects[0]
        assert objects[0]["softwareRequirements"] == "Requires 8GB RAM, 2GB disk space"
        assert "releaseNotes" in objects[0]

    def test_mobile_application(self):
        """Test mobile application"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Fitness Tracker",
                "description": "Track your workouts and health goals",
                "applicationCategory": "HealthApplication",
                "operatingSystem": "Android",
                "softwareVersion": "5.1.0",
                "installUrl": "https://play.google.com/store/apps/details?id=com.fitness.tracker",
                "offers": {
                    "@type": "Offer",
                    "price": "0",
                    "priceCurrency": "USD"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.5",
                    "ratingCount": "50000"
                },
                "screenshot": [
                    "https://example.com/mobile-screen1.png",
                    "https://example.com/mobile-screen2.png",
                    "https://example.com/mobile-screen3.png"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Fitness Tracker"
        assert objects[0]["applicationCategory"] == "HealthApplication"
        assert objects[0]["operatingSystem"] == "Android"
        assert objects[0]["installUrl"].startswith("https://play.google.com")
        assert objects[0]["offers"]["price"] == "0"

    def test_web_application(self):
        """Test web application"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Cloud Docs",
                "description": "Collaborative document editing in the cloud",
                "applicationCategory": "BusinessApplication",
                "operatingSystem": "Web Browser",
                "url": "https://docs.example.com",
                "offers": {
                    "@type": "Offer",
                    "price": "9.99",
                    "priceCurrency": "USD"
                },
                "author": {
                    "@type": "Organization",
                    "name": "Cloud Software Corp"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Cloud Docs"
        assert objects[0]["operatingSystem"] == "Web Browser"
        assert "author" in objects[0]


class TestSoftwareApplicationAdvanced:
    """Test advanced features"""

    def test_softwareapplication_with_author_person(self):
        """Test SoftwareApplication with Person author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Indie Game",
                "author": {
                    "@type": "Person",
                    "name": "John Developer"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "author" in objects[0]
        assert objects[0]["author"]["@type"] == "Person"

    def test_softwareapplication_with_permissions(self):
        """Test SoftwareApplication with permissions"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Camera App",
                "permissions": ["Camera", "Storage", "Location"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "permissions" in objects[0]
        assert isinstance(objects[0]["permissions"], list)

    def test_softwareapplication_with_release_notes(self):
        """Test SoftwareApplication with releaseNotes"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "MyApp",
                "softwareVersion": "2.0.0",
                "releaseNotes": "Major update: New UI design, performance improvements, bug fixes"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["softwareVersion"] == "2.0.0"
        assert "releaseNotes" in objects[0]
        assert "Major update" in objects[0]["releaseNotes"]


class TestSoftwareApplicationIntegration:
    """Test integration with extract_all"""

    def test_extract_all_includes_softwareapplication(self):
        """Test that extract_all includes SoftwareApplication"""
        html = """
        <html>
        <head>
            <title>My App Page</title>
            <meta name="description" content="Download our amazing app">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Amazing App",
                "applicationCategory": "GameApplication",
                "operatingSystem": "iOS"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        result = meta_oxide.extract_all(html)

        assert result is not None
        assert "jsonld" in result
        jsonld_objects = result["jsonld"]
        assert len(jsonld_objects) >= 1

        # Find the SoftwareApplication
        app = None
        for obj in jsonld_objects:
            if obj.get("@type") == "SoftwareApplication":
                app = obj
                break

        assert app is not None
        assert app["name"] == "Amazing App"
        assert app["applicationCategory"] == "GameApplication"


class TestSoftwareApplicationEdgeCases:
    """Test edge cases"""

    def test_softwareapplication_empty(self):
        """Test empty SoftwareApplication"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"

    def test_softwareapplication_with_null_values(self):
        """Test SoftwareApplication with null values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "Test App",
                "description": null,
                "url": null
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "Test App"

    def test_multiple_softwareapplications(self):
        """Test multiple SoftwareApplication objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "App One"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "App Two"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["@type"] == "SoftwareApplication"
        assert objects[1]["@type"] == "SoftwareApplication"
        assert objects[0]["name"] == "App One"
        assert objects[1]["name"] == "App Two"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
