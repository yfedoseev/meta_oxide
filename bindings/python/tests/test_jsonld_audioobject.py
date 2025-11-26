"""
Tests for JSON-LD AudioObject type

Tests the extraction of Schema.org AudioObject structured data.
AudioObject is used for audio files, podcasts, music tracks, and audiobooks.
"""

import meta_oxide
import pytest


class TestAudioObjectBasic:
    """Test basic AudioObject extraction"""

    def test_audio_basic_name_only(self):
        """Test minimal audio with name only"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Podcast Episode 1"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "Podcast Episode 1"

    def test_audio_with_description(self):
        """Test audio with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Morning Meditation",
                "description": "A 10-minute guided meditation for starting your day"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "Morning Meditation"
        assert objects[0]["description"] == "A 10-minute guided meditation for starting your day"

    def test_audio_with_content_url(self):
        """Test audio with contentUrl"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Example Audio",
                "contentUrl": "https://example.com/audio.mp3"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["contentUrl"] == "https://example.com/audio.mp3"

    def test_audio_with_duration(self):
        """Test audio with ISO 8601 duration"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Short Track",
                "duration": "PT45M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["duration"] == "PT45M"


class TestAudioObjectEncoding:
    """Test AudioObject encoding and format fields"""

    def test_audio_with_encoding_format(self):
        """Test audio with encodingFormat (MIME type)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "MP3 Audio",
                "encodingFormat": "audio/mpeg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["encodingFormat"] == "audio/mpeg"

    def test_audio_with_bitrate(self):
        """Test audio with bitrate"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "High Quality Audio",
                "bitrate": "320 kbps",
                "encodingFormat": "audio/mp3"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["bitrate"] == "320 kbps"
        assert objects[0]["encodingFormat"] == "audio/mp3"

    def test_audio_various_formats(self):
        """Test audio with various encoding formats"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "WAV Recording",
                "encodingFormat": "audio/wav"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["encodingFormat"] == "audio/wav"


class TestAudioObjectUrls:
    """Test AudioObject URL fields"""

    def test_audio_with_urls(self):
        """Test audio with contentUrl, embedUrl, and url"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Sample Audio",
                "contentUrl": "https://example.com/audio.mp3",
                "embedUrl": "https://example.com/player/embed/123",
                "url": "https://example.com/listen/audio-123"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["contentUrl"] == "https://example.com/audio.mp3"
        assert objects[0]["embedUrl"] == "https://example.com/player/embed/123"
        assert objects[0]["url"] == "https://example.com/listen/audio-123"

    def test_audio_with_image_thumbnail(self):
        """Test audio with image/thumbnail URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Music Track",
                "image": "https://example.com/cover-art.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["image"] == "https://example.com/cover-art.jpg"

    def test_audio_with_thumbnail_object(self):
        """Test audio with thumbnail ImageObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Audio with Thumbnail",
                "thumbnail": {
                    "@type": "ImageObject",
                    "url": "https://example.com/thumb.jpg"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "thumbnail" in objects[0]


class TestAudioObjectMetadata:
    """Test AudioObject metadata fields"""

    def test_audio_with_dates(self):
        """Test audio with uploadDate and datePublished"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "New Podcast Episode",
                "uploadDate": "2024-11-07T10:00:00Z",
                "datePublished": "2024-11-07"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["uploadDate"] == "2024-11-07T10:00:00Z"
        assert objects[0]["datePublished"] == "2024-11-07"

    def test_audio_with_author(self):
        """Test audio with author (Person)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Audiobook Chapter",
                "author": {
                    "@type": "Person",
                    "name": "Jane Author",
                    "url": "https://example.com/authors/jane"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "Audiobook Chapter"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "author" in objects[0]

    def test_audio_with_publisher(self):
        """Test audio with publisher (Organization)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Corporate Podcast",
                "publisher": {
                    "@type": "Organization",
                    "name": "Podcast Network Inc",
                    "logo": {
                        "@type": "ImageObject",
                        "url": "https://network.com/logo.png"
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
        assert objects[0]["@type"] == "AudioObject"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "publisher" in objects[0]

    def test_audio_with_language(self):
        """Test audio with inLanguage"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Spanish Lesson",
                "inLanguage": "es-ES"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["inLanguage"] == "es-ES"


class TestAudioObjectTranscriptCaption:
    """Test AudioObject transcript and caption fields"""

    def test_audio_with_transcript_url(self):
        """Test audio with transcript URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Interview Audio",
                "transcript": "https://example.com/transcripts/interview-123.txt"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["transcript"] == "https://example.com/transcripts/interview-123.txt"

    def test_audio_with_transcript_text(self):
        """Test audio with transcript text"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Short Audio Clip",
                "transcript": "This is the full transcript of the audio content."
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["transcript"] == "This is the full transcript of the audio content."

    def test_audio_with_caption(self):
        """Test audio with caption"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Music Track",
                "caption": "Recorded live at Madison Square Garden, 2024"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["caption"] == "Recorded live at Madison Square Garden, 2024"


class TestAudioObjectRating:
    """Test AudioObject rating and interaction fields"""

    def test_audio_with_aggregate_rating(self):
        """Test audio with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Popular Podcast Episode",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 4.8,
                    "reviewCount": 250
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "aggregateRating" in objects[0]

    def test_audio_with_interaction_statistic(self):
        """Test audio with interactionStatistic"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Viral Audio Clip",
                "interactionStatistic": {
                    "@type": "InteractionCounter",
                    "interactionType": "https://schema.org/ListenAction",
                    "userInteractionCount": 1500000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "Viral Audio Clip"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "interactionStatistic" in objects[0]


class TestAudioObjectRealWorld:
    """Test with real-world AudioObject examples"""

    def test_podcast_episode(self):
        """Test complete podcast episode"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "The Future of AI - Episode 42",
                "description": "In this episode, we discuss the latest developments in artificial intelligence",
                "contentUrl": "https://cdn.example.com/podcasts/ep42.mp3",
                "embedUrl": "https://example.com/player/ep42",
                "duration": "PT1H15M30S",
                "encodingFormat": "audio/mpeg",
                "uploadDate": "2024-11-01T08:00:00Z",
                "datePublished": "2024-11-01",
                "author": {
                    "@type": "Person",
                    "name": "Dr. Tech Expert"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Tech Talk Podcast Network"
                },
                "inLanguage": "en-US",
                "image": "https://example.com/podcast-cover.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        audio = objects[0]
        assert audio["@type"] == "AudioObject"
        assert audio["name"] == "The Future of AI - Episode 42"
        assert audio["contentUrl"] == "https://cdn.example.com/podcasts/ep42.mp3"
        assert audio["duration"] == "PT1H15M30S"
        assert audio["encodingFormat"] == "audio/mpeg"
        assert audio["inLanguage"] == "en-US"

    def test_music_track(self):
        """Test music track AudioObject"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Bohemian Rhapsody",
                "description": "Classic rock song by Queen",
                "contentUrl": "https://music.example.com/tracks/bohemian-rhapsody.mp3",
                "duration": "PT5M55S",
                "encodingFormat": "audio/mpeg",
                "bitrate": "320 kbps",
                "author": {
                    "@type": "Person",
                    "name": "Freddie Mercury"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "EMI Records"
                },
                "datePublished": "1975-10-31",
                "image": "https://music.example.com/album-art/night-at-opera.jpg",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 5.0,
                    "reviewCount": 50000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        audio = objects[0]
        assert audio["@type"] == "AudioObject"
        assert audio["name"] == "Bohemian Rhapsody"
        assert audio["duration"] == "PT5M55S"
        assert audio["bitrate"] == "320 kbps"
        assert audio["datePublished"] == "1975-10-31"

    def test_audiobook_chapter(self):
        """Test audiobook chapter"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Chapter 1: The Beginning",
                "description": "First chapter of the audiobook",
                "contentUrl": "https://audiobooks.example.com/book123/ch01.mp3",
                "duration": "PT45M",
                "encodingFormat": "audio/mpeg",
                "author": {
                    "@type": "Person",
                    "name": "John Author"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Audiobook Publisher Inc"
                },
                "inLanguage": "en-US",
                "transcript": "https://audiobooks.example.com/book123/ch01-transcript.txt"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        audio = objects[0]
        assert audio["@type"] == "AudioObject"
        assert audio["name"] == "Chapter 1: The Beginning"
        assert audio["duration"] == "PT45M"
        assert audio["transcript"] == "https://audiobooks.example.com/book123/ch01-transcript.txt"


class TestAudioObjectComplete:
    """Test complete AudioObject with all fields"""

    def test_audio_complete(self):
        """Test audio with comprehensive field coverage"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Complete Audio Example",
                "description": "An audio file with all metadata",
                "contentUrl": "https://example.com/audio.mp3",
                "embedUrl": "https://example.com/player/embed",
                "url": "https://example.com/listen/audio",
                "duration": "PT30M",
                "encodingFormat": "audio/mpeg",
                "bitrate": "192 kbps",
                "uploadDate": "2024-11-07T12:00:00Z",
                "datePublished": "2024-11-07",
                "author": {
                    "@type": "Person",
                    "name": "Content Creator"
                },
                "publisher": {
                    "@type": "Organization",
                    "name": "Media Company"
                },
                "inLanguage": "en-US",
                "transcript": "https://example.com/transcript.txt",
                "caption": "Recorded in high quality stereo",
                "image": "https://example.com/cover.jpg",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 4.5,
                    "reviewCount": 100
                },
                "interactionStatistic": {
                    "@type": "InteractionCounter",
                    "interactionType": "https://schema.org/ListenAction",
                    "userInteractionCount": 10000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        audio = objects[0]

        assert audio["@type"] == "AudioObject"
        assert audio["name"] == "Complete Audio Example"
        assert audio["description"] == "An audio file with all metadata"
        assert audio["contentUrl"] == "https://example.com/audio.mp3"
        assert audio["embedUrl"] == "https://example.com/player/embed"
        assert audio["url"] == "https://example.com/listen/audio"
        assert audio["duration"] == "PT30M"
        assert audio["encodingFormat"] == "audio/mpeg"
        assert audio["bitrate"] == "192 kbps"
        assert audio["uploadDate"] == "2024-11-07T12:00:00Z"
        assert audio["datePublished"] == "2024-11-07"
        assert audio["inLanguage"] == "en-US"
        assert audio["transcript"] == "https://example.com/transcript.txt"
        assert audio["caption"] == "Recorded in high quality stereo"
        assert audio["image"] == "https://example.com/cover.jpg"
        # Complex objects are present
        assert "author" in audio
        assert "publisher" in audio
        assert "aggregateRating" in audio
        assert "interactionStatistic" in audio


class TestAudioObjectEdgeCases:
    """Test edge cases for AudioObject"""

    def test_audio_empty_optional_fields(self):
        """Test that optional fields can be omitted"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Minimal Audio"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "Minimal Audio"
        # Optional fields should not be present if not provided
        assert "description" not in objects[0]
        assert "duration" not in objects[0]
        assert "uploadDate" not in objects[0]

    def test_audio_in_graph(self):
        """Test AudioObject within @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "AudioObject",
                        "name": "First Audio"
                    },
                    {
                        "@type": "AudioObject",
                        "name": "Second Audio"
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
        assert objects[0]["@type"] == "AudioObject"
        assert objects[0]["name"] == "First Audio"
        assert objects[1]["@type"] == "AudioObject"
        assert objects[1]["name"] == "Second Audio"


class TestAudioObjectIntegration:
    """Test AudioObject integration with extract_all()"""

    def test_extract_all_includes_audio(self):
        """Test that extract_all() includes AudioObject"""
        html = """
        <html>
        <head>
            <title>Audio Page</title>
            <meta property="og:title" content="OG Title">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Test Audio",
                "description": "A test audio file"
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
        assert data["jsonld"][0]["@type"] == "AudioObject"
        assert data["jsonld"][0]["name"] == "Test Audio"

    def test_multiple_types_with_audio(self):
        """Test AudioObject alongside other Schema.org types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "AudioObject",
                "name": "Audio Content"
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
        assert "AudioObject" in types
        assert "Article" in types


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
