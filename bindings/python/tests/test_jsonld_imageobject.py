"""
Tests for JSON-LD ImageObject type

Tests the extraction of Schema.org ImageObject structured data
"""

import meta_oxide
import pytest


class TestImageObjectBasic:
    """Test basic ImageObject extraction"""

    def test_imageobject_basic(self):
        """Test minimal image with contentUrl"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/photo.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["contentUrl"] == "https://example.com/photo.jpg"

    def test_imageobject_with_name(self):
        """Test image with name/title"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Beautiful Sunset",
                "contentUrl": "https://example.com/sunset.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "Beautiful Sunset"
        assert objects[0]["contentUrl"] == "https://example.com/sunset.jpg"

    def test_imageobject_with_description(self):
        """Test image with description/alt text"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Mountain Landscape",
                "description": "Panoramic view of snow-capped mountains at dawn",
                "contentUrl": "https://example.com/mountains.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "Mountain Landscape"
        assert objects[0]["description"] == "Panoramic view of snow-capped mountains at dawn"


class TestImageObjectDimensions:
    """Test ImageObject width and height"""

    def test_imageobject_with_dimensions(self):
        """Test image with width and height in pixels"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/photo.jpg",
                "width": 1920,
                "height": 1080
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["width"] == 1920
        assert objects[0]["height"] == 1080

    def test_imageobject_portrait(self):
        """Test portrait orientation image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Portrait Photo",
                "contentUrl": "https://example.com/portrait.jpg",
                "width": 1080,
                "height": 1920
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["width"] == 1080
        assert objects[0]["height"] == 1920


class TestImageObjectEncoding:
    """Test ImageObject encodingFormat (MIME type)"""

    def test_imageobject_with_encoding(self):
        """Test image with encodingFormat"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/photo.jpg",
                "encodingFormat": "image/jpeg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["encodingFormat"] == "image/jpeg"

    def test_imageobject_png(self):
        """Test PNG image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/logo.png",
                "encodingFormat": "image/png"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["encodingFormat"] == "image/png"

    def test_imageobject_webp(self):
        """Test WebP image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/modern.webp",
                "encodingFormat": "image/webp"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["encodingFormat"] == "image/webp"


class TestImageObjectThumbnail:
    """Test ImageObject thumbnail (nested ImageObject)"""

    def test_imageobject_with_thumbnail(self):
        """Test image with thumbnail ImageObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "High Resolution Photo",
                "contentUrl": "https://example.com/photo-4k.jpg",
                "width": 3840,
                "height": 2160,
                "thumbnail": {
                    "@type": "ImageObject",
                    "contentUrl": "https://example.com/photo-thumb.jpg",
                    "width": 150,
                    "height": 150
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "High Resolution Photo"
        assert objects[0]["width"] == 3840
        # Thumbnail should be present as nested object
        assert "thumbnail" in objects[0]


class TestImageObjectUrls:
    """Test ImageObject URL fields"""

    def test_imageobject_content_and_page_url(self):
        """Test image with both contentUrl (direct file) and url (page)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Gallery Photo",
                "contentUrl": "https://example.com/images/photo123.jpg",
                "url": "https://example.com/gallery/photo123"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["contentUrl"] == "https://example.com/images/photo123.jpg"
        assert objects[0]["url"] == "https://example.com/gallery/photo123"


class TestImageObjectCaption:
    """Test ImageObject caption"""

    def test_imageobject_with_caption(self):
        """Test image with caption"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Historic Moment",
                "contentUrl": "https://example.com/historic.jpg",
                "caption": "First landing on the moon, July 20, 1969"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["caption"] == "First landing on the moon, July 20, 1969"


class TestImageObjectCreator:
    """Test ImageObject creator/photographer"""

    def test_imageobject_with_creator(self):
        """Test image with creator (Person)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Wildlife Photography",
                "contentUrl": "https://example.com/wildlife.jpg",
                "creator": {
                    "@type": "Person",
                    "name": "Jane Photographer",
                    "url": "https://example.com/photographers/jane"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "Wildlife Photography"
        # Creator should be present as nested object
        assert "creator" in objects[0]

    def test_imageobject_with_organization_creator(self):
        """Test image with creator (Organization)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Corporate Photo",
                "contentUrl": "https://example.com/corporate.jpg",
                "creator": {
                    "@type": "Organization",
                    "name": "Professional Photo Studio",
                    "url": "https://photostudio.com"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "creator" in objects[0]


class TestImageObjectCopyright:
    """Test ImageObject copyright and licensing"""

    def test_imageobject_with_license(self):
        """Test image with copyright holder and license"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Licensed Photo",
                "contentUrl": "https://example.com/photo.jpg",
                "copyrightHolder": {
                    "@type": "Person",
                    "name": "Copyright Owner"
                },
                "license": "https://creativecommons.org/licenses/by/4.0/"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["license"] == "https://creativecommons.org/licenses/by/4.0/"
        assert "copyrightHolder" in objects[0]

    def test_imageobject_creative_commons(self):
        """Test image with Creative Commons license"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "CC Licensed Image",
                "contentUrl": "https://example.com/cc-photo.jpg",
                "license": "https://creativecommons.org/licenses/by-sa/4.0/",
                "copyrightHolder": {
                    "@type": "Organization",
                    "name": "Photo Archive Foundation"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["license"] == "https://creativecommons.org/licenses/by-sa/4.0/"


class TestImageObjectUploadDate:
    """Test ImageObject uploadDate"""

    def test_imageobject_with_upload_date(self):
        """Test image with uploadDate (ISO 8601)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Recent Upload",
                "contentUrl": "https://example.com/recent.jpg",
                "uploadDate": "2024-01-15T10:30:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["uploadDate"] == "2024-01-15T10:30:00Z"

    def test_imageobject_upload_date_only(self):
        """Test image with date-only uploadDate"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "contentUrl": "https://example.com/dated.jpg",
                "uploadDate": "2024-03-20"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["uploadDate"] == "2024-03-20"


class TestImageObjectComplete:
    """Test complete ImageObject with all fields"""

    def test_imageobject_complete(self):
        """Test image with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Professional Landscape Photography",
                "description": "Award-winning landscape photograph of Yosemite Valley",
                "contentUrl": "https://example.com/images/yosemite-4k.jpg",
                "url": "https://example.com/gallery/yosemite",
                "width": 3840,
                "height": 2160,
                "encodingFormat": "image/jpeg",
                "thumbnail": {
                    "@type": "ImageObject",
                    "contentUrl": "https://example.com/images/yosemite-thumb.jpg",
                    "width": 200,
                    "height": 112
                },
                "caption": "Yosemite Valley at sunset, captured from Tunnel View",
                "creator": {
                    "@type": "Person",
                    "name": "Ansel Adams Jr.",
                    "url": "https://example.com/photographers/ansel"
                },
                "copyrightHolder": {
                    "@type": "Person",
                    "name": "Ansel Adams Jr."
                },
                "license": "https://creativecommons.org/licenses/by-nc-nd/4.0/",
                "uploadDate": "2024-02-15T14:22:30Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        img = objects[0]

        assert img["@type"] == "ImageObject"
        assert img["name"] == "Professional Landscape Photography"
        assert img["description"] == "Award-winning landscape photograph of Yosemite Valley"
        assert img["contentUrl"] == "https://example.com/images/yosemite-4k.jpg"
        assert img["url"] == "https://example.com/gallery/yosemite"
        assert img["width"] == 3840
        assert img["height"] == 2160
        assert img["encodingFormat"] == "image/jpeg"
        assert img["caption"] == "Yosemite Valley at sunset, captured from Tunnel View"
        assert img["license"] == "https://creativecommons.org/licenses/by-nc-nd/4.0/"
        assert img["uploadDate"] == "2024-02-15T14:22:30Z"
        # Complex nested objects
        assert "thumbnail" in img
        assert "creator" in img
        assert "copyrightHolder" in img


class TestImageObjectEdgeCases:
    """Test edge cases for ImageObject"""

    def test_imageobject_minimal(self):
        """Test minimal ImageObject with only required context"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        # Optional fields should not be present
        assert "name" not in objects[0]
        assert "contentUrl" not in objects[0]

    def test_imageobject_in_graph(self):
        """Test ImageObject within @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "ImageObject",
                        "name": "First Image",
                        "contentUrl": "https://example.com/img1.jpg"
                    },
                    {
                        "@type": "ImageObject",
                        "name": "Second Image",
                        "contentUrl": "https://example.com/img2.jpg"
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
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "First Image"
        assert objects[1]["@type"] == "ImageObject"
        assert objects[1]["name"] == "Second Image"


class TestImageObjectIntegration:
    """Test ImageObject integration with extract_all()"""

    def test_extract_all_includes_imageobject(self):
        """Test that extract_all() includes ImageObject"""
        html = """
        <html>
        <head>
            <title>Photo Gallery</title>
            <meta property="og:title" content="Photo Gallery">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Gallery Featured Image",
                "contentUrl": "https://example.com/featured.jpg",
                "width": 1200,
                "height": 800
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
        assert data["jsonld"][0]["@type"] == "ImageObject"
        assert data["jsonld"][0]["name"] == "Gallery Featured Image"
        assert data["jsonld"][0]["width"] == 1200

    def test_multiple_types_with_imageobject(self):
        """Test ImageObject alongside other Schema.org types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Article Hero Image",
                "contentUrl": "https://example.com/hero.jpg"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Photography Article",
                "image": "https://example.com/hero.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        types = [obj["@type"] for obj in objects]
        assert "ImageObject" in types
        assert "Article" in types


class TestImageObjectRealWorld:
    """Test with real-world ImageObject examples"""

    def test_photo_gallery(self):
        """Test realistic photo gallery ImageObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Northern Lights over Iceland",
                "description": "Aurora Borealis dancing over the Icelandic landscape",
                "contentUrl": "https://photos.example.com/iceland/northern-lights-8k.jpg",
                "url": "https://gallery.example.com/nature/iceland/aurora-001",
                "width": 7680,
                "height": 4320,
                "encodingFormat": "image/jpeg",
                "thumbnail": {
                    "@type": "ImageObject",
                    "contentUrl": "https://photos.example.com/iceland/northern-lights-thumb.jpg",
                    "width": 300,
                    "height": 169
                },
                "caption": "Northern Lights over Kirkjufell mountain, Iceland",
                "creator": {
                    "@type": "Person",
                    "name": "Nordic Photographer",
                    "url": "https://gallery.example.com/photographers/nordic"
                },
                "copyrightHolder": {
                    "@type": "Person",
                    "name": "Nordic Photographer"
                },
                "license": "https://creativecommons.org/licenses/by-nc/4.0/",
                "uploadDate": "2024-03-10T08:15:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        img = objects[0]
        assert img["@type"] == "ImageObject"
        assert img["name"] == "Northern Lights over Iceland"
        assert img["width"] == 7680
        assert img["height"] == 4320
        assert img["encodingFormat"] == "image/jpeg"

    def test_stock_photo(self):
        """Test stock photography ImageObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Business Meeting",
                "description": "Diverse team of professionals in a modern office",
                "contentUrl": "https://stock.example.com/photos/business-meeting-001.jpg",
                "width": 5120,
                "height": 3840,
                "encodingFormat": "image/jpeg",
                "creator": {
                    "@type": "Organization",
                    "name": "Professional Stock Photos Inc"
                },
                "license": "https://stock.example.com/licenses/standard",
                "uploadDate": "2024-01-22T00:00:00Z"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["name"] == "Business Meeting"
        assert "creator" in objects[0]

    def test_product_image(self):
        """Test product photography ImageObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "ImageObject",
                "name": "Wireless Headphones - Front View",
                "contentUrl": "https://cdn.shop.example.com/products/headphones-wh1000-front.jpg",
                "width": 2000,
                "height": 2000,
                "encodingFormat": "image/jpeg",
                "thumbnail": {
                    "@type": "ImageObject",
                    "contentUrl": "https://cdn.shop.example.com/products/headphones-wh1000-thumb.jpg",
                    "width": 250,
                    "height": 250
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "ImageObject"
        assert objects[0]["width"] == 2000
        assert objects[0]["height"] == 2000


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
