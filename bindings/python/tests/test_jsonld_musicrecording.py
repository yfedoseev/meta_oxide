"""
Tests for JSON-LD MusicRecording type support (Schema.org MusicRecording)

This module tests the extraction of MusicRecording structured data from JSON-LD,
following the Schema.org MusicRecording specification.
"""

import meta_oxide
import pytest


class TestMusicRecordingBasic:
    """Test basic MusicRecording extraction with minimal fields"""

    def test_musicrecording_minimal(self):
        """Test extracting MusicRecording with only required name field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Bohemian Rhapsody"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "MusicRecording"
        assert objects[0]["name"] == "Bohemian Rhapsody"

    def test_musicrecording_with_artist(self):
        """Test MusicRecording with byArtist as Person"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Imagine",
                "byArtist": {
                    "@type": "Person",
                    "name": "John Lennon"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "MusicRecording"
        assert objects[0]["name"] == "Imagine"
        assert "byArtist" in objects[0]

    def test_musicrecording_with_music_group(self):
        """Test MusicRecording with byArtist as MusicGroup"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Hey Jude",
                "byArtist": {
                    "@type": "MusicGroup",
                    "name": "The Beatles"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Hey Jude"
        assert "byArtist" in objects[0]

    def test_musicrecording_with_duration(self):
        """Test MusicRecording with ISO 8601 duration"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Stairway to Heaven",
                "duration": "PT8M02S"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Stairway to Heaven"
        assert objects[0]["duration"] == "PT8M02S"


class TestMusicRecordingAlbumPlaylist:
    """Test MusicRecording album and playlist relationships"""

    def test_musicrecording_in_album(self):
        """Test MusicRecording with inAlbum property"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Come Together",
                "inAlbum": {
                    "@type": "MusicAlbum",
                    "name": "Abbey Road",
                    "albumReleaseType": "StudioAlbum"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Come Together"
        assert "inAlbum" in objects[0]

    def test_musicrecording_in_playlist(self):
        """Test MusicRecording with inPlaylist property"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Thriller",
                "inPlaylist": {
                    "@type": "MusicPlaylist",
                    "name": "80s Hits"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Thriller"
        assert "inPlaylist" in objects[0]

    def test_musicrecording_album_with_artist(self):
        """Test complete MusicRecording with album and artist"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Billie Jean",
                "byArtist": {
                    "@type": "Person",
                    "name": "Michael Jackson"
                },
                "inAlbum": {
                    "@type": "MusicAlbum",
                    "name": "Thriller"
                },
                "duration": "PT4M54S"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]
        assert obj["name"] == "Billie Jean"
        assert obj["duration"] == "PT4M54S"
        assert "byArtist" in obj
        assert "inAlbum" in obj


class TestMusicRecordingIdentifiers:
    """Test MusicRecording identifier fields"""

    def test_musicrecording_with_isrc(self):
        """Test MusicRecording with ISRC code"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Shape of You",
                "isrcCode": "GBAHS1600214"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Shape of You"
        assert objects[0]["isrcCode"] == "GBAHS1600214"

    def test_musicrecording_recording_of(self):
        """Test MusicRecording with recordingOf (MusicComposition)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Symphony No. 5 in C minor",
                "recordingOf": {
                    "@type": "MusicComposition",
                    "name": "Symphony No. 5",
                    "composer": {
                        "@type": "Person",
                        "name": "Ludwig van Beethoven"
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
        assert objects[0]["name"] == "Symphony No. 5 in C minor"
        assert "recordingOf" in objects[0]

    def test_musicrecording_with_url(self):
        """Test MusicRecording with URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Blinding Lights",
                "url": "https://example.com/songs/blinding-lights"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["url"] == "https://example.com/songs/blinding-lights"


class TestMusicRecordingProduction:
    """Test MusicRecording production-related fields"""

    def test_musicrecording_with_producer(self):
        """Test MusicRecording with producer"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Lose Yourself",
                "producer": {
                    "@type": "Person",
                    "name": "Eminem"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Lose Yourself"
        assert "producer" in objects[0]

    def test_musicrecording_with_record_label(self):
        """Test MusicRecording with recordLabel"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Humble",
                "recordLabel": {
                    "@type": "Organization",
                    "name": "Top Dawg Entertainment"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Humble"
        assert "recordLabel" in objects[0]

    def test_musicrecording_with_dates(self):
        """Test MusicRecording with datePublished and dateCreated"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Rolling in the Deep",
                "datePublished": "2010-11-29",
                "dateCreated": "2010-06-15"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Rolling in the Deep"
        assert objects[0]["datePublished"] == "2010-11-29"
        assert objects[0]["dateCreated"] == "2010-06-15"


class TestMusicRecordingRatingsAudio:
    """Test MusicRecording ratings and audio object"""

    def test_musicrecording_with_aggregate_rating(self):
        """Test MusicRecording with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Hotel California",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "1234"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Hotel California"
        assert "aggregateRating" in objects[0]

    def test_musicrecording_with_review(self):
        """Test MusicRecording with review"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Smells Like Teen Spirit",
                "review": {
                    "@type": "Review",
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": "5"
                    },
                    "author": {
                        "@type": "Person",
                        "name": "Music Critic"
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
        assert objects[0]["name"] == "Smells Like Teen Spirit"
        assert "review" in objects[0]

    def test_musicrecording_with_audio_object(self):
        """Test MusicRecording with audio (AudioObject)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "What's Going On",
                "audio": {
                    "@type": "AudioObject",
                    "contentUrl": "https://example.com/audio/whats-going-on.mp3",
                    "encodingFormat": "audio/mpeg"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "What's Going On"
        assert "audio" in objects[0]


class TestMusicRecordingRealWorld:
    """Test real-world MusicRecording examples"""

    def test_musicrecording_pop_song_complete(self):
        """Test complete pop song with all common fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Levitating",
                "description": "Upbeat pop song with disco influences",
                "url": "https://example.com/songs/levitating",
                "image": "https://example.com/images/levitating.jpg",
                "byArtist": {
                    "@type": "Person",
                    "name": "Dua Lipa"
                },
                "inAlbum": {
                    "@type": "MusicAlbum",
                    "name": "Future Nostalgia",
                    "albumReleaseType": "StudioAlbum"
                },
                "duration": "PT3M23S",
                "datePublished": "2020-03-27",
                "genre": "Pop",
                "isrcCode": "GBUM72001234",
                "recordLabel": {
                    "@type": "Organization",
                    "name": "Warner Records"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.7",
                    "reviewCount": "5678"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        # Check basic fields
        assert obj["@type"] == "MusicRecording"
        assert obj["name"] == "Levitating"
        assert obj["description"] == "Upbeat pop song with disco influences"
        assert obj["url"] == "https://example.com/songs/levitating"
        assert obj["duration"] == "PT3M23S"
        assert obj["datePublished"] == "2020-03-27"
        assert obj["genre"] == "Pop"
        assert obj["isrcCode"] == "GBUM72001234"

        # Check nested objects
        assert "byArtist" in obj
        assert "inAlbum" in obj
        assert "recordLabel" in obj
        assert "aggregateRating" in obj

    def test_musicrecording_classical_recording(self):
        """Test classical music recording with composition"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Piano Concerto No. 21 in C major, K. 467",
                "byArtist": {
                    "@type": "MusicGroup",
                    "name": "Vienna Philharmonic Orchestra"
                },
                "recordingOf": {
                    "@type": "MusicComposition",
                    "name": "Piano Concerto No. 21",
                    "composer": {
                        "@type": "Person",
                        "name": "Wolfgang Amadeus Mozart"
                    }
                },
                "duration": "PT28M15S",
                "datePublished": "1985-09-12",
                "genre": "Classical",
                "recordLabel": {
                    "@type": "Organization",
                    "name": "Deutsche Grammophon"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        assert obj["@type"] == "MusicRecording"
        assert obj["name"] == "Piano Concerto No. 21 in C major, K. 467"
        assert obj["duration"] == "PT28M15S"
        assert obj["genre"] == "Classical"
        assert "byArtist" in obj
        assert "recordingOf" in obj
        assert "recordLabel" in obj

    def test_musicrecording_podcast_episode(self):
        """Test podcast episode as MusicRecording"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "The Future of AI - Episode 42",
                "description": "An in-depth discussion about artificial intelligence",
                "byArtist": {
                    "@type": "Person",
                    "name": "Tech Podcast Host"
                },
                "duration": "PT45M30S",
                "datePublished": "2024-01-15",
                "audio": {
                    "@type": "AudioObject",
                    "contentUrl": "https://example.com/podcasts/episode-42.mp3",
                    "encodingFormat": "audio/mpeg"
                },
                "genre": "Technology"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        assert obj["@type"] == "MusicRecording"
        assert obj["name"] == "The Future of AI - Episode 42"
        assert obj["duration"] == "PT45M30S"
        assert obj["genre"] == "Technology"
        assert "byArtist" in obj
        assert "audio" in obj


class TestMusicRecordingEdgeCases:
    """Test edge cases for MusicRecording extraction"""

    def test_musicrecording_empty_fields(self):
        """Test MusicRecording with some empty/null fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Test Song",
                "description": null
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Test Song"

    def test_multiple_musicrecordings(self):
        """Test extracting multiple MusicRecording objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Song One"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Song Two"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["name"] == "Song One"
        assert objects[1]["name"] == "Song Two"

    def test_musicrecording_in_graph(self):
        """Test MusicRecording within @graph array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "MusicRecording",
                        "name": "Graph Song"
                    },
                    {
                        "@type": "Person",
                        "name": "Artist Name"
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
        recording = next(obj for obj in objects if obj.get("@type") == "MusicRecording")
        assert recording["name"] == "Graph Song"


class TestMusicRecordingIntegration:
    """Test MusicRecording integration with extract_all()"""

    def test_extract_all_includes_musicrecording(self):
        """Test that extract_all() properly includes MusicRecording objects"""
        html = """
        <html>
        <head>
            <title>Music Page</title>
            <meta property="og:title" content="Best Song Ever">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "MusicRecording",
                "name": "Perfect Song",
                "description": "The most amazing song you'll ever hear"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "MusicRecording"
        assert data["jsonld"][0]["name"] == "Perfect Song"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
