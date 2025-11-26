"""
Tests for JSON-LD VideoObject type

Tests the extraction of Schema.org VideoObject structured data
"""

import meta_oxide
import pytest


class TestVideoObjectBasic:
    """Test basic VideoObject extraction"""

    def test_video_basic(self):
        """Test minimal video with name only"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "How to Code in Python"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "How to Code in Python"

    def test_video_with_description(self):
        """Test video with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Python Tutorial",
                "description": "Learn Python programming from scratch"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Python Tutorial"
        assert objects[0]["description"] == "Learn Python programming from scratch"


class TestVideoObjectUrls:
    """Test VideoObject URL fields"""

    def test_video_with_urls(self):
        """Test video with contentUrl and embedUrl"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Sample Video",
                "contentUrl": "https://example.com/video.mp4",
                "embedUrl": "https://example.com/embed/video123",
                "url": "https://example.com/watch?v=123"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Sample Video"
        assert objects[0]["contentUrl"] == "https://example.com/video.mp4"
        assert objects[0]["embedUrl"] == "https://example.com/embed/video123"
        assert objects[0]["url"] == "https://example.com/watch?v=123"


class TestVideoObjectMetadata:
    """Test VideoObject metadata fields"""

    def test_video_with_metadata(self):
        """Test video with duration and uploadDate"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Cooking Tutorial",
                "description": "How to make pasta",
                "uploadDate": "2024-01-15T10:00:00Z",
                "duration": "PT5M30S"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Cooking Tutorial"
        assert objects[0]["uploadDate"] == "2024-01-15T10:00:00Z"
        assert objects[0]["duration"] == "PT5M30S"

    def test_video_with_dimensions(self):
        """Test video with width and height"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "HD Video",
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
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["width"] == 1920
        assert objects[0]["height"] == 1080


class TestVideoObjectThumbnails:
    """Test VideoObject thumbnail handling"""

    def test_video_with_thumbnail_string(self):
        """Test video with single thumbnail URL as string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Video with Thumbnail",
                "thumbnailUrl": "https://example.com/thumbnail.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["thumbnailUrl"] == "https://example.com/thumbnail.jpg"

    def test_video_with_thumbnail_array(self):
        """Test video with multiple thumbnail URLs"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Video with Multiple Thumbnails",
                "thumbnailUrl": [
                    "https://example.com/thumb1.jpg",
                    "https://example.com/thumb2.jpg",
                    "https://example.com/thumb3.jpg"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        # Array values are currently returned as JSON strings per codebase pattern
        assert "thumbnailUrl" in objects[0]


class TestVideoObjectStats:
    """Test VideoObject interaction statistics"""

    def test_video_with_stats(self):
        """Test video with interactionStatistic for view count"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Popular Video",
                "interactionStatistic": {
                    "@type": "InteractionCounter",
                    "interactionType": "https://schema.org/WatchAction",
                    "userInteractionCount": 12345
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Popular Video"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "interactionStatistic" in objects[0]


class TestVideoObjectAuthorship:
    """Test VideoObject author and publisher fields"""

    def test_video_with_author(self):
        """Test video with author (Person)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Tutorial Video",
                "author": {
                    "@type": "Person",
                    "name": "Jane Doe",
                    "url": "https://example.com/jane"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Tutorial Video"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "author" in objects[0]

    def test_video_with_publisher(self):
        """Test video with publisher (Organization)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Corporate Video",
                "publisher": {
                    "@type": "Organization",
                    "name": "Acme Corp",
                    "logo": {
                        "@type": "ImageObject",
                        "url": "https://acme.com/logo.png"
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
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Corporate Video"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "publisher" in objects[0]


class TestVideoObjectComplete:
    """Test complete VideoObject with all fields"""

    def test_video_complete(self):
        """Test video with all supported fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Complete Video Example",
                "description": "A comprehensive video with all metadata",
                "thumbnailUrl": "https://example.com/thumbnail.jpg",
                "uploadDate": "2024-01-15T10:00:00Z",
                "duration": "PT10M30S",
                "contentUrl": "https://example.com/video.mp4",
                "embedUrl": "https://example.com/embed/video",
                "url": "https://example.com/watch?v=abc123",
                "width": 1920,
                "height": 1080,
                "author": {
                    "@type": "Person",
                    "name": "John Creator"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Video Platform Inc",
                    "logo": {
                        "@type": "ImageObject",
                        "url": "https://platform.com/logo.png"
                    }
                },
                "interactionStatistic": {
                    "@type": "InteractionCounter",
                    "interactionType": "https://schema.org/WatchAction",
                    "userInteractionCount": 999999
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        video = objects[0]

        assert video["@type"] == "VideoObject"
        assert video["name"] == "Complete Video Example"
        assert video["description"] == "A comprehensive video with all metadata"
        assert video["thumbnailUrl"] == "https://example.com/thumbnail.jpg"
        assert video["uploadDate"] == "2024-01-15T10:00:00Z"
        assert video["duration"] == "PT10M30S"
        assert video["contentUrl"] == "https://example.com/video.mp4"
        assert video["embedUrl"] == "https://example.com/embed/video"
        assert video["url"] == "https://example.com/watch?v=abc123"
        assert video["width"] == 1920
        assert video["height"] == 1080
        # Complex objects are present
        assert "author" in video
        assert "publisher" in video
        assert "interactionStatistic" in video


class TestVideoObjectEdgeCases:
    """Test edge cases for VideoObject"""

    def test_video_empty_optional_fields(self):
        """Test that optional fields can be omitted"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Minimal Video"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Minimal Video"
        # Optional fields should not be present if not provided
        assert "description" not in objects[0]
        assert "duration" not in objects[0]
        assert "uploadDate" not in objects[0]

    def test_video_in_graph(self):
        """Test VideoObject within @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "VideoObject",
                        "name": "First Video"
                    },
                    {
                        "@type": "VideoObject",
                        "name": "Second Video"
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
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "First Video"
        assert objects[1]["@type"] == "VideoObject"
        assert objects[1]["name"] == "Second Video"


class TestVideoObjectIntegration:
    """Test VideoObject integration with extract_all()"""

    def test_extract_all_includes_video(self):
        """Test that extract_all() includes VideoObject"""
        html = """
        <html>
        <head>
            <title>Video Page</title>
            <meta property="og:title" content="OG Title">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Test Video",
                "description": "A test video"
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
        assert data["jsonld"][0]["@type"] == "VideoObject"
        assert data["jsonld"][0]["name"] == "Test Video"

    def test_multiple_types_with_video(self):
        """Test VideoObject alongside other Schema.org types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Video Content"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Article Content"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        types = [obj["@type"] for obj in objects]
        assert "VideoObject" in types
        assert "Article" in types


class TestVideoObjectRealWorld:
    """Test with real-world VideoObject examples"""

    def test_youtube_style_video(self):
        """Test YouTube-style VideoObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Cat Videos Compilation 2024",
                "description": "Funny cat videos compilation",
                "thumbnailUrl": [
                    "https://i.ytimg.com/vi/abc123/maxresdefault.jpg",
                    "https://i.ytimg.com/vi/abc123/hqdefault.jpg"
                ],
                "uploadDate": "2024-01-20T15:30:00Z",
                "duration": "PT15M45S",
                "contentUrl": "https://www.youtube.com/v/abc123",
                "embedUrl": "https://www.youtube.com/embed/abc123",
                "interactionStatistic": {
                    "@type": "InteractionCounter",
                    "interactionType": "https://schema.org/WatchAction",
                    "userInteractionCount": 1234567
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Cat Videos Compilation 2024"
        assert objects[0]["duration"] == "PT15M45S"

    def test_vimeo_style_video(self):
        """Test Vimeo-style VideoObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "VideoObject",
                "name": "Creative Short Film",
                "description": "An award-winning short film",
                "thumbnailUrl": "https://i.vimeocdn.com/video/123456.jpg",
                "uploadDate": "2024-02-01T12:00:00Z",
                "duration": "PT8M20S",
                "width": 1920,
                "height": 1080,
                "embedUrl": "https://player.vimeo.com/video/123456",
                "author": {
                    "@type": "Person",
                    "name": "Indie Filmmaker",
                    "url": "https://vimeo.com/indiefilmmaker"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "VideoObject"
        assert objects[0]["name"] == "Creative Short Film"
        assert objects[0]["width"] == 1920
        assert objects[0]["height"] == 1080


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
