"""
Tests for JSON-LD TVSeries type

Tests the extraction of Schema.org TVSeries structured data for TV shows and series
"""

import meta_oxide
import pytest


class TestTVSeriesBasic:
    """Test basic TVSeries extraction"""

    def test_tvseries_basic(self):
        """Test minimal TVSeries with name only"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Breaking Bad"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Breaking Bad"

    def test_tvseries_with_description(self):
        """Test TVSeries with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Stranger Things",
                "description": "A science fiction horror series set in the 1980s"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Stranger Things"
        assert objects[0]["description"] == "A science fiction horror series set in the 1980s"

    def test_tvseries_with_season_episode_counts(self):
        """Test TVSeries with numberOfSeasons and numberOfEpisodes"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Game of Thrones",
                "numberOfSeasons": 8,
                "numberOfEpisodes": 73
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Game of Thrones"
        assert objects[0]["numberOfSeasons"] == 8
        assert objects[0]["numberOfEpisodes"] == 73

    def test_tvseries_with_urls(self):
        """Test TVSeries with url and image"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Crown",
                "url": "https://example.com/shows/the-crown",
                "image": "https://example.com/images/crown-poster.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Crown"
        assert objects[0]["url"] == "https://example.com/shows/the-crown"
        assert objects[0]["image"] == "https://example.com/images/crown-poster.jpg"


class TestTVSeriesPeople:
    """Test TVSeries people fields (actor, director, creator)"""

    def test_tvseries_with_single_actor(self):
        """Test TVSeries with single actor (Person object)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Fleabag",
                "actor": {
                    "@type": "Person",
                    "name": "Phoebe Waller-Bridge"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Fleabag"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "actor" in objects[0]

    def test_tvseries_with_multiple_actors(self):
        """Test TVSeries with multiple actors (array)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Friends",
                "actor": [
                    {
                        "@type": "Person",
                        "name": "Jennifer Aniston"
                    },
                    {
                        "@type": "Person",
                        "name": "Courteney Cox"
                    },
                    {
                        "@type": "Person",
                        "name": "Lisa Kudrow"
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Friends"
        # Complex objects are returned as JSON strings per codebase pattern
        assert "actor" in objects[0]

    def test_tvseries_with_director(self):
        """Test TVSeries with director"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "True Detective",
                "director": {
                    "@type": "Person",
                    "name": "Cary Joji Fukunaga"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "True Detective"
        assert "director" in objects[0]

    def test_tvseries_with_creator(self):
        """Test TVSeries with creator (Person or Organization)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Office",
                "creator": [
                    {
                        "@type": "Person",
                        "name": "Greg Daniels"
                    },
                    {
                        "@type": "Person",
                        "name": "Ricky Gervais"
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Office"
        assert "creator" in objects[0]


class TestTVSeriesMetadata:
    """Test TVSeries metadata fields"""

    def test_tvseries_with_genre(self):
        """Test TVSeries with genre"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Wire",
                "genre": ["Crime", "Drama", "Thriller"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Wire"
        assert "genre" in objects[0]

    def test_tvseries_with_content_rating(self):
        """Test TVSeries with contentRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Boys",
                "contentRating": "TV-MA"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Boys"
        assert objects[0]["contentRating"] == "TV-MA"

    def test_tvseries_with_dates(self):
        """Test TVSeries with datePublished, startDate, endDate"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Mad Men",
                "datePublished": "2007-07-19",
                "startDate": "2007-07-19",
                "endDate": "2015-05-17"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Mad Men"
        assert objects[0]["datePublished"] == "2007-07-19"
        assert objects[0]["startDate"] == "2007-07-19"
        assert objects[0]["endDate"] == "2015-05-17"

    def test_tvseries_with_language_and_country(self):
        """Test TVSeries with inLanguage and countryOfOrigin"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Dark",
                "inLanguage": "de",
                "countryOfOrigin": {
                    "@type": "Country",
                    "name": "Germany"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Dark"
        assert objects[0]["inLanguage"] == "de"
        assert "countryOfOrigin" in objects[0]


class TestTVSeriesStructure:
    """Test TVSeries structure fields (containsSeason, episode)"""

    def test_tvseries_with_seasons(self):
        """Test TVSeries with containsSeason"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Westworld",
                "containsSeason": [
                    {
                        "@type": "TVSeason",
                        "seasonNumber": 1,
                        "numberOfEpisodes": 10
                    },
                    {
                        "@type": "TVSeason",
                        "seasonNumber": 2,
                        "numberOfEpisodes": 10
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Westworld"
        assert "containsSeason" in objects[0]

    def test_tvseries_with_episodes(self):
        """Test TVSeries with episode array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Sherlock",
                "episode": [
                    {
                        "@type": "TVEpisode",
                        "episodeNumber": 1,
                        "name": "A Study in Pink"
                    },
                    {
                        "@type": "TVEpisode",
                        "episodeNumber": 2,
                        "name": "The Blind Banker"
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Sherlock"
        assert "episode" in objects[0]


class TestTVSeriesRatingsAndProduction:
    """Test TVSeries ratings and production fields"""

    def test_tvseries_with_aggregate_rating(self):
        """Test TVSeries with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Chernobyl",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 9.3,
                    "bestRating": 10,
                    "ratingCount": 500000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Chernobyl"
        assert "aggregateRating" in objects[0]

    def test_tvseries_with_review(self):
        """Test TVSeries with review"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Mandalorian",
                "review": {
                    "@type": "Review",
                    "author": {
                        "@type": "Person",
                        "name": "John Critic"
                    },
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": 5
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Mandalorian"
        assert "review" in objects[0]

    def test_tvseries_with_production_company(self):
        """Test TVSeries with productionCompany"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Witcher",
                "productionCompany": {
                    "@type": "Organization",
                    "name": "Netflix Studios"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "The Witcher"
        assert "productionCompany" in objects[0]

    def test_tvseries_with_trailer(self):
        """Test TVSeries with trailer (VideoObject)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "House of the Dragon",
                "trailer": {
                    "@type": "VideoObject",
                    "name": "Official Trailer",
                    "url": "https://example.com/trailer.mp4"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "House of the Dragon"
        assert "trailer" in objects[0]


class TestTVSeriesRealWorld:
    """Test with real-world TVSeries examples"""

    def test_tvseries_drama_complete(self):
        """Test complete drama series example"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "The Sopranos",
                "description": "A troubled mob boss seeks therapy while dealing with family and criminal life",
                "url": "https://example.com/shows/sopranos",
                "image": "https://example.com/images/sopranos-poster.jpg",
                "numberOfSeasons": 6,
                "numberOfEpisodes": 86,
                "genre": ["Crime", "Drama"],
                "contentRating": "TV-MA",
                "datePublished": "1999-01-10",
                "startDate": "1999-01-10",
                "endDate": "2007-06-10",
                "creator": {
                    "@type": "Person",
                    "name": "David Chase"
                },
                "actor": [
                    {
                        "@type": "Person",
                        "name": "James Gandolfini"
                    },
                    {
                        "@type": "Person",
                        "name": "Edie Falco"
                    }
                ],
                "productionCompany": {
                    "@type": "Organization",
                    "name": "HBO"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 9.2,
                    "bestRating": 10,
                    "ratingCount": 350000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        series = objects[0]

        assert series["@type"] == "TVSeries"
        assert series["name"] == "The Sopranos"
        assert (
            series["description"]
            == "A troubled mob boss seeks therapy while dealing with family and criminal life"
        )
        assert series["url"] == "https://example.com/shows/sopranos"
        assert series["numberOfSeasons"] == 6
        assert series["numberOfEpisodes"] == 86
        assert series["contentRating"] == "TV-MA"
        assert series["datePublished"] == "1999-01-10"
        assert series["startDate"] == "1999-01-10"
        assert series["endDate"] == "2007-06-10"
        assert "genre" in series
        assert "creator" in series
        assert "actor" in series
        assert "productionCompany" in series
        assert "aggregateRating" in series

    def test_tvseries_comedy(self):
        """Test comedy series example"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Brooklyn Nine-Nine",
                "description": "Comedy series about a talented but carefree detective",
                "url": "https://example.com/shows/b99",
                "image": "https://example.com/images/b99.jpg",
                "numberOfSeasons": 8,
                "numberOfEpisodes": 153,
                "genre": ["Comedy", "Police procedural"],
                "contentRating": "TV-14",
                "startDate": "2013-09-17",
                "endDate": "2021-09-16",
                "creator": [
                    {
                        "@type": "Person",
                        "name": "Dan Goor"
                    },
                    {
                        "@type": "Person",
                        "name": "Michael Schur"
                    }
                ],
                "actor": [
                    {
                        "@type": "Person",
                        "name": "Andy Samberg"
                    },
                    {
                        "@type": "Person",
                        "name": "Terry Crews"
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
        series = objects[0]

        assert series["@type"] == "TVSeries"
        assert series["name"] == "Brooklyn Nine-Nine"
        assert series["numberOfSeasons"] == 8
        assert series["numberOfEpisodes"] == 153
        assert series["contentRating"] == "TV-14"
        assert "genre" in series
        assert "creator" in series

    def test_tvseries_documentary(self):
        """Test documentary series example"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Planet Earth II",
                "description": "Groundbreaking nature documentary series",
                "url": "https://example.com/shows/planet-earth-2",
                "image": "https://example.com/images/planet-earth.jpg",
                "numberOfSeasons": 1,
                "numberOfEpisodes": 6,
                "genre": ["Documentary", "Nature"],
                "contentRating": "TV-G",
                "datePublished": "2016-11-06",
                "creator": {
                    "@type": "Organization",
                    "name": "BBC Natural History Unit"
                },
                "productionCompany": {
                    "@type": "Organization",
                    "name": "BBC"
                },
                "inLanguage": "en",
                "countryOfOrigin": {
                    "@type": "Country",
                    "name": "United Kingdom"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": 9.5,
                    "bestRating": 10,
                    "ratingCount": 125000
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        series = objects[0]

        assert series["@type"] == "TVSeries"
        assert series["name"] == "Planet Earth II"
        assert series["numberOfSeasons"] == 1
        assert series["numberOfEpisodes"] == 6
        assert series["contentRating"] == "TV-G"
        assert series["datePublished"] == "2016-11-06"
        assert series["inLanguage"] == "en"
        assert "genre" in series
        assert "countryOfOrigin" in series


class TestTVSeriesEdgeCases:
    """Test edge cases for TVSeries"""

    def test_tvseries_minimal_fields(self):
        """Test that optional fields can be omitted"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Minimal Series"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "Minimal Series"
        # Optional fields should not be present if not provided
        assert "description" not in objects[0]
        assert "numberOfSeasons" not in objects[0]
        assert "actor" not in objects[0]

    def test_tvseries_in_graph(self):
        """Test TVSeries within @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "TVSeries",
                        "name": "First Series"
                    },
                    {
                        "@type": "TVSeries",
                        "name": "Second Series"
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
        assert objects[0]["@type"] == "TVSeries"
        assert objects[0]["name"] == "First Series"
        assert objects[1]["@type"] == "TVSeries"
        assert objects[1]["name"] == "Second Series"


class TestTVSeriesIntegration:
    """Test TVSeries integration with extract_all()"""

    def test_extract_all_includes_tvseries(self):
        """Test that extract_all() includes TVSeries"""
        html = """
        <html>
        <head>
            <title>Series Page</title>
            <meta property="og:title" content="OG Title">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Test Series",
                "description": "A test TV series",
                "numberOfSeasons": 3
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
        assert data["jsonld"][0]["@type"] == "TVSeries"
        assert data["jsonld"][0]["name"] == "Test Series"
        assert data["jsonld"][0]["numberOfSeasons"] == 3

    def test_multiple_types_with_tvseries(self):
        """Test TVSeries alongside other Schema.org types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "TVSeries",
                "name": "Series Content"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Organization",
                "name": "Production Company"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        types = [obj["@type"] for obj in objects]
        assert "TVSeries" in types
        assert "Organization" in types


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
