"""
Tests for JSON-LD Book type support (Schema.org Book)

This module tests the extraction of Book structured data from JSON-LD,
following the Schema.org Book specification.
"""

import meta_oxide
import pytest


class TestBookBasic:
    """Test basic Book extraction with minimal fields"""

    def test_book_minimal(self):
        """Test extracting Book with only name field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Great Gatsby"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Book"
        assert objects[0]["name"] == "The Great Gatsby"

    def test_book_with_author_and_isbn(self):
        """Test Book with name, author, and ISBN"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "1984",
                "author": {
                    "@type": "Person",
                    "name": "George Orwell"
                },
                "isbn": "978-0-452-28423-4"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Book"
        assert objects[0]["name"] == "1984"
        assert objects[0]["isbn"] == "978-0-452-28423-4"
        assert "author" in objects[0]

    def test_book_with_description_and_url(self):
        """Test Book with description and URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "To Kill a Mockingbird",
                "description": "A gripping tale of racial injustice and childhood innocence",
                "url": "https://example.com/books/mockingbird"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "To Kill a Mockingbird"
        assert (
            objects[0]["description"]
            == "A gripping tale of racial injustice and childhood innocence"
        )
        assert objects[0]["url"] == "https://example.com/books/mockingbird"

    def test_book_with_single_image(self):
        """Test Book with a single image URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Pride and Prejudice",
                "image": "https://example.com/covers/pride-prejudice.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Pride and Prejudice"
        assert objects[0]["image"] == "https://example.com/covers/pride-prejudice.jpg"


class TestBookDetails:
    """Test Book details (format, pages, edition, publisher)"""

    def test_book_with_book_format(self):
        """Test Book with bookFormat"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Hobbit",
                "bookFormat": "Hardcover"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["bookFormat"] == "Hardcover"

    def test_book_with_number_of_pages(self):
        """Test Book with numberOfPages"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Harry Potter and the Philosopher's Stone",
                "numberOfPages": 223
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["numberOfPages"] == 223

    def test_book_with_book_edition(self):
        """Test Book with bookEdition"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Lord of the Rings",
                "bookEdition": "50th Anniversary Edition"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["bookEdition"] == "50th Anniversary Edition"

    def test_book_with_publisher(self):
        """Test Book with publisher as Organization"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Dune",
                "publisher": {
                    "@type": "Organization",
                    "name": "Chilton Books"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Dune"
        assert "publisher" in objects[0]

    def test_book_with_date_published(self):
        """Test Book with datePublished"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Brave New World",
                "datePublished": "1932-01-01"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["datePublished"] == "1932-01-01"


class TestBookLanguageAndGenre:
    """Test Book language and genre fields"""

    def test_book_with_in_language(self):
        """Test Book with inLanguage"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Les Misérables",
                "inLanguage": "fr"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["inLanguage"] == "fr"

    def test_book_with_single_genre(self):
        """Test Book with single genre"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Foundation",
                "genre": "Science Fiction"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["genre"] == "Science Fiction"

    def test_book_with_multiple_genres(self):
        """Test Book with multiple genres as array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Martian",
                "genre": ["Science Fiction", "Thriller", "Adventure"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "genre" in objects[0]


class TestBookContributors:
    """Test Book contributors (author, illustrator, publisher)"""

    def test_book_with_string_author(self):
        """Test Book with author as string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Catcher in the Rye",
                "author": "J.D. Salinger"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["author"] == "J.D. Salinger"

    def test_book_with_person_author(self):
        """Test Book with Person as author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "A Brief History of Time",
                "author": {
                    "@type": "Person",
                    "name": "Stephen Hawking"
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

    def test_book_with_illustrator(self):
        """Test Book with illustrator"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Very Hungry Caterpillar",
                "author": {
                    "@type": "Person",
                    "name": "Eric Carle"
                },
                "illustrator": {
                    "@type": "Person",
                    "name": "Eric Carle"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "illustrator" in objects[0]


class TestBookRatingsAndReviews:
    """Test Book ratings, reviews, and offers"""

    def test_book_with_aggregate_rating(self):
        """Test Book with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Handmaid's Tale",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.6",
                    "reviewCount": "15234"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "aggregateRating" in objects[0]

    def test_book_with_review(self):
        """Test Book with review"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Sapiens",
                "review": {
                    "@type": "Review",
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": "5"
                    },
                    "author": {
                        "@type": "Person",
                        "name": "Book Reviewer"
                    },
                    "reviewBody": "An outstanding exploration of human history."
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "review" in objects[0]

    def test_book_with_offers(self):
        """Test Book with offers (price information)"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Educated",
                "offers": {
                    "@type": "Offer",
                    "price": "14.99",
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

    def test_book_with_abridged(self):
        """Test Book with abridged boolean"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "War and Peace",
                "abridged": false
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["abridged"] == False


class TestBookRealWorldExamples:
    """Test real-world Book examples"""

    def test_fiction_book_complete(self):
        """Test complete fiction book with all common fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Name of the Wind",
                "description": "The riveting first-person narrative of a young man who grows to be the most notorious magician his world has ever seen.",
                "author": {
                    "@type": "Person",
                    "name": "Patrick Rothfuss"
                },
                "isbn": "978-0-7564-0474-1",
                "bookFormat": "Hardcover",
                "numberOfPages": 662,
                "publisher": {
                    "@type": "Organization",
                    "name": "DAW Books"
                },
                "datePublished": "2007-03-27",
                "inLanguage": "en",
                "genre": ["Fantasy", "Adventure"],
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.5",
                    "reviewCount": "89234"
                },
                "offers": {
                    "@type": "Offer",
                    "price": "27.95",
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
        obj = objects[0]

        assert obj["@type"] == "Book"
        assert obj["name"] == "The Name of the Wind"
        assert obj["isbn"] == "978-0-7564-0474-1"
        assert obj["bookFormat"] == "Hardcover"
        assert obj["numberOfPages"] == 662
        assert obj["datePublished"] == "2007-03-27"
        assert obj["inLanguage"] == "en"
        assert "author" in obj
        assert "publisher" in obj
        assert "genre" in obj
        assert "aggregateRating" in obj
        assert "offers" in obj

    def test_textbook_complete(self):
        """Test complete textbook with academic fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Introduction to Algorithms",
                "description": "A comprehensive textbook on computer algorithms",
                "author": [
                    {
                        "@type": "Person",
                        "name": "Thomas H. Cormen"
                    },
                    {
                        "@type": "Person",
                        "name": "Charles E. Leiserson"
                    }
                ],
                "isbn": "978-0-262-03384-8",
                "bookFormat": "Hardcover",
                "numberOfPages": 1312,
                "bookEdition": "Third Edition",
                "publisher": {
                    "@type": "Organization",
                    "name": "MIT Press"
                },
                "datePublished": "2009-07-31",
                "inLanguage": "en",
                "genre": "Textbook"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        assert obj["@type"] == "Book"
        assert obj["name"] == "Introduction to Algorithms"
        assert obj["isbn"] == "978-0-262-03384-8"
        assert obj["bookEdition"] == "Third Edition"
        assert obj["numberOfPages"] == 1312
        assert "author" in obj

    def test_audiobook(self):
        """Test audiobook format"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Becoming",
                "author": {
                    "@type": "Person",
                    "name": "Michelle Obama"
                },
                "bookFormat": "AudioBook",
                "isbn": "978-1-5247-6313-8",
                "publisher": {
                    "@type": "Organization",
                    "name": "Random House Audio"
                },
                "datePublished": "2018-11-13",
                "inLanguage": "en",
                "abridged": false
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        assert obj["@type"] == "Book"
        assert obj["name"] == "Becoming"
        assert obj["bookFormat"] == "AudioBook"
        assert obj["abridged"] == False


class TestBookEdgeCases:
    """Test edge cases for Book extraction"""

    def test_book_empty_fields(self):
        """Test Book with some empty/null fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Untitled Book",
                "description": null,
                "genre": []
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Untitled Book"

    def test_multiple_books(self):
        """Test extracting multiple Book objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Book One"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "Book Two"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["name"] == "Book One"
        assert objects[1]["name"] == "Book Two"

    def test_book_in_graph(self):
        """Test Book within @graph array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Book",
                        "name": "The Chronicles of Narnia"
                    },
                    {
                        "@type": "Person",
                        "name": "C.S. Lewis"
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
        book = next(obj for obj in objects if obj.get("@type") == "Book")
        assert book["name"] == "The Chronicles of Narnia"


class TestBookIntegration:
    """Test Book integration with extract_all()"""

    def test_extract_all_includes_book(self):
        """Test that extract_all() properly includes Book objects"""
        html = """
        <html>
        <head>
            <title>Book Page</title>
            <meta property="og:title" content="Amazing Book">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Book",
                "name": "The Great Novel",
                "author": {
                    "@type": "Person",
                    "name": "Famous Author"
                },
                "isbn": "978-1-234-56789-0"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Book"
        assert data["jsonld"][0]["name"] == "The Great Novel"
        assert data["jsonld"][0]["isbn"] == "978-1-234-56789-0"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
