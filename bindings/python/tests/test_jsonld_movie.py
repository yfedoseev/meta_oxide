"""
Tests for JSON-LD Movie type

Tests the extraction of Schema.org Movie structured data
"""

import meta_oxide
import pytest


class TestMovieBasic:
    """Test basic Movie extraction"""

    def test_movie_minimal(self):
        """Test minimal movie with name only"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "The Example Film"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "The Example Film"

    def test_movie_with_description(self):
        """Test movie with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Epic Adventure",
                "description": "An epic tale of adventure and discovery"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Epic Adventure"
        assert objects[0]["description"] == "An epic tale of adventure and discovery"

    def test_movie_with_duration(self):
        """Test movie with ISO 8601 duration"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Feature Length Film",
                "duration": "PT2H15M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Feature Length Film"
        assert objects[0]["duration"] == "PT2H15M"

    def test_movie_with_url_and_image(self):
        """Test movie with url and image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Visual Masterpiece",
                "url": "https://example.com/movie/visual-masterpiece",
                "image": "https://example.com/posters/masterpiece.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["url"] == "https://example.com/movie/visual-masterpiece"
        assert objects[0]["image"] == "https://example.com/posters/masterpiece.jpg"


class TestMoviePeople:
    """Test Movie people fields (director, actor, producer)"""

    def test_movie_with_single_director(self):
        """Test movie with single director as Person object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Directorial Vision",
                "director": {
                    "@type": "Person",
                    "name": "Christopher Nolan"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Directorial Vision"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "director" in objects[0]

    def test_movie_with_multiple_directors(self):
        """Test movie with multiple directors"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Co-Directed Film",
                "director": [
                    {
                        "@type": "Person",
                        "name": "Lana Wachowski"
                    },
                    {
                        "@type": "Person",
                        "name": "Lilly Wachowski"
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
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Co-Directed Film"
        assert "director" in objects[0]

    def test_movie_with_single_actor(self):
        """Test movie with single actor"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "One Person Show",
                "actor": {
                    "@type": "Person",
                    "name": "Tom Hanks"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert "actor" in objects[0]

    def test_movie_with_multiple_actors(self):
        """Test movie with multiple actors"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Ensemble Cast",
                "actor": [
                    {
                        "@type": "Person",
                        "name": "Leonardo DiCaprio"
                    },
                    {
                        "@type": "Person",
                        "name": "Tom Hardy"
                    },
                    {
                        "@type": "Person",
                        "name": "Elliot Page"
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
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Ensemble Cast"
        assert "actor" in objects[0]

    def test_movie_with_producer_and_author(self):
        """Test movie with producer and author (screenwriter)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Production Credits",
                "producer": {
                    "@type": "Person",
                    "name": "Emma Thomas"
                },
                "author": {
                    "@type": "Person",
                    "name": "Aaron Sorkin"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert "producer" in objects[0]
        assert "author" in objects[0]


class TestMovieMetadata:
    """Test Movie metadata fields"""

    def test_movie_with_genre(self):
        """Test movie with genre"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Sci-Fi Thriller",
                "genre": ["Science Fiction", "Thriller", "Drama"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Sci-Fi Thriller"
        assert "genre" in objects[0]

    def test_movie_with_content_rating(self):
        """Test movie with content rating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Teen Adventure",
                "contentRating": "PG-13"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["contentRating"] == "PG-13"

    def test_movie_with_dates(self):
        """Test movie with datePublished and dateCreated"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Historical Release",
                "datePublished": "2010-07-16",
                "dateCreated": "2009-12-01"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["datePublished"] == "2010-07-16"
        assert objects[0]["dateCreated"] == "2009-12-01"

    def test_movie_with_language_and_country(self):
        """Test movie with inLanguage and countryOfOrigin"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "International Film",
                "inLanguage": "en",
                "countryOfOrigin": "USA"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["inLanguage"] == "en"
        assert objects[0]["countryOfOrigin"] == "USA"


class TestMovieRatingsReviews:
    """Test Movie ratings and reviews"""

    def test_movie_with_aggregate_rating(self):
        """Test movie with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Highly Rated Film",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "8.8",
                    "ratingCount": "2500000",
                    "bestRating": "10",
                    "worstRating": "1"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Highly Rated Film"
        assert "aggregateRating" in objects[0]

    def test_movie_with_review(self):
        """Test movie with review"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Critically Acclaimed",
                "review": {
                    "@type": "Review",
                    "author": {
                        "@type": "Person",
                        "name": "Roger Ebert"
                    },
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": "4"
                    },
                    "reviewBody": "A masterpiece of modern cinema."
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Critically Acclaimed"
        assert "review" in objects[0]

    def test_movie_with_multiple_reviews(self):
        """Test movie with multiple reviews"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Well Reviewed",
                "review": [
                    {
                        "@type": "Review",
                        "author": {"@type": "Person", "name": "Critic One"},
                        "reviewRating": {"@type": "Rating", "ratingValue": "5"}
                    },
                    {
                        "@type": "Review",
                        "author": {"@type": "Person", "name": "Critic Two"},
                        "reviewRating": {"@type": "Rating", "ratingValue": "4"}
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
        assert objects[0]["@type"] == "Movie"
        assert "review" in objects[0]


class TestMovieProduction:
    """Test Movie production fields"""

    def test_movie_with_production_company(self):
        """Test movie with productionCompany"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Studio Production",
                "productionCompany": {
                    "@type": "Organization",
                    "name": "Warner Bros. Pictures"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Studio Production"
        assert "productionCompany" in objects[0]

    def test_movie_with_music_and_awards(self):
        """Test movie with musicBy and awards"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Award Winner",
                "musicBy": {
                    "@type": "Person",
                    "name": "Hans Zimmer"
                },
                "awards": "Academy Award for Best Picture"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Award Winner"
        assert "musicBy" in objects[0]
        assert objects[0]["awards"] == "Academy Award for Best Picture"

    def test_movie_with_trailer(self):
        """Test movie with trailer (VideoObject)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Upcoming Release",
                "trailer": {
                    "@type": "VideoObject",
                    "name": "Official Trailer",
                    "url": "https://example.com/trailer",
                    "thumbnailUrl": "https://example.com/trailer-thumb.jpg"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Upcoming Release"
        assert "trailer" in objects[0]


class TestMovieRealWorld:
    """Test with real-world Movie examples"""

    def test_movie_blockbuster(self):
        """Test blockbuster movie with comprehensive metadata"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Inception",
                "description": "A thief who steals corporate secrets through the use of dream-sharing technology",
                "url": "https://example.com/movies/inception",
                "image": "https://example.com/posters/inception.jpg",
                "director": {
                    "@type": "Person",
                    "name": "Christopher Nolan"
                },
                "actor": [
                    {"@type": "Person", "name": "Leonardo DiCaprio"},
                    {"@type": "Person", "name": "Joseph Gordon-Levitt"},
                    {"@type": "Person", "name": "Elliot Page"}
                ],
                "genre": ["Science Fiction", "Action", "Thriller"],
                "datePublished": "2010-07-16",
                "duration": "PT2H28M",
                "contentRating": "PG-13",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "8.8",
                    "ratingCount": "2300000"
                },
                "productionCompany": {
                    "@type": "Organization",
                    "name": "Warner Bros. Pictures"
                },
                "musicBy": {
                    "@type": "Person",
                    "name": "Hans Zimmer"
                },
                "countryOfOrigin": "USA",
                "inLanguage": "en"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        movie = objects[0]

        assert movie["@type"] == "Movie"
        assert movie["name"] == "Inception"
        assert (
            movie["description"]
            == "A thief who steals corporate secrets through the use of dream-sharing technology"
        )
        assert movie["url"] == "https://example.com/movies/inception"
        assert movie["image"] == "https://example.com/posters/inception.jpg"
        assert movie["datePublished"] == "2010-07-16"
        assert movie["duration"] == "PT2H28M"
        assert movie["contentRating"] == "PG-13"
        assert movie["countryOfOrigin"] == "USA"
        assert movie["inLanguage"] == "en"
        assert "director" in movie
        assert "actor" in movie
        assert "genre" in movie
        assert "aggregateRating" in movie
        assert "productionCompany" in movie
        assert "musicBy" in movie

    def test_movie_indie_film(self):
        """Test indie film with minimal but essential data"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Moonlight",
                "description": "A coming-of-age story of a young African-American man",
                "director": {
                    "@type": "Person",
                    "name": "Barry Jenkins"
                },
                "datePublished": "2016-10-21",
                "duration": "PT1H51M",
                "genre": "Drama",
                "awards": "Academy Award for Best Picture",
                "contentRating": "R"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Moonlight"
        assert objects[0]["duration"] == "PT1H51M"
        assert objects[0]["contentRating"] == "R"
        assert objects[0]["awards"] == "Academy Award for Best Picture"
        assert "director" in objects[0]

    def test_movie_documentary(self):
        """Test documentary film"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Planet Earth II",
                "description": "David Attenborough returns to narrate this landmark series",
                "genre": "Documentary",
                "datePublished": "2016-11-06",
                "duration": "PT8H",
                "contentRating": "TV-G",
                "director": {
                    "@type": "Person",
                    "name": "Various"
                },
                "productionCompany": {
                    "@type": "Organization",
                    "name": "BBC Natural History Unit"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Planet Earth II"
        assert objects[0]["duration"] == "PT8H"
        assert objects[0]["contentRating"] == "TV-G"
        assert "genre" in objects[0]


class TestMovieEdgeCases:
    """Test edge cases for Movie"""

    def test_movie_empty_optional_fields(self):
        """Test that optional fields can be omitted"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Minimal Movie"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "Minimal Movie"
        # Optional fields should not be present if not provided
        assert "description" not in objects[0]
        assert "duration" not in objects[0]
        assert "director" not in objects[0]

    def test_movie_in_graph(self):
        """Test Movie within @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Movie",
                        "name": "First Movie"
                    },
                    {
                        "@type": "Movie",
                        "name": "Second Movie"
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
        assert objects[0]["@type"] == "Movie"
        assert objects[0]["name"] == "First Movie"
        assert objects[1]["@type"] == "Movie"
        assert objects[1]["name"] == "Second Movie"


class TestMovieIntegration:
    """Test Movie integration with extract_all()"""

    def test_extract_all_includes_movie(self):
        """Test that extract_all() includes Movie"""
        html = """
        <html>
        <head>
            <title>Movie Page</title>
            <meta property="og:title" content="OG Title">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "Test Movie",
                "description": "A test movie for integration"
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
        assert data["jsonld"][0]["@type"] == "Movie"
        assert data["jsonld"][0]["name"] == "Test Movie"

    def test_multiple_types_with_movie(self):
        """Test Movie alongside other Schema.org types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Movie",
                "name": "The Film"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Article",
                "headline": "Review of The Film"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        types = [obj["@type"] for obj in objects]
        assert "Movie" in types
        assert "Article" in types


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
