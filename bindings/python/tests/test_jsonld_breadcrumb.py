"""
Tests for JSON-LD BreadcrumbList type

Following TDD approach - these tests are written FIRST and will fail initially
"""

import meta_oxide
import pytest


class TestBreadcrumbListBasic:
    """Test basic BreadcrumbList extraction"""

    def test_breadcrumb_basic(self):
        """Test simple breadcrumb trail with basic structure"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Books",
                        "item": "https://example.com/books"
                    },
                    {
                        "@type": "ListItem",
                        "position": 3,
                        "name": "Science Fiction",
                        "item": "https://example.com/books/sci-fi"
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
        assert objects[0]["@type"] == "BreadcrumbList"
        assert "itemListElement" in objects[0]

        items = objects[0]["itemListElement"]
        assert len(items) == 3

        # Check first item
        assert items[0]["@type"] == "ListItem"
        assert items[0]["position"] == 1
        assert items[0]["name"] == "Home"
        assert items[0]["item"] == "https://example.com"

        # Check last item
        assert items[2]["position"] == 3
        assert items[2]["name"] == "Science Fiction"

    def test_breadcrumb_with_positions(self):
        """Test breadcrumb with explicit position values"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Products"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Electronics"
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
        assert objects[0]["@type"] == "BreadcrumbList"

        items = objects[0]["itemListElement"]
        assert len(items) == 2
        assert items[0]["position"] == 1
        assert items[1]["position"] == 2

    def test_breadcrumb_with_items(self):
        """Test breadcrumb with item URLs"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://example.com/"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Category",
                        "item": "https://example.com/category"
                    },
                    {
                        "@type": "ListItem",
                        "position": 3,
                        "name": "Product",
                        "item": "https://example.com/category/product"
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
        items = objects[0]["itemListElement"]

        # Verify all items have URLs
        assert items[0]["item"] == "https://example.com/"
        assert items[1]["item"] == "https://example.com/category"
        assert items[2]["item"] == "https://example.com/category/product"

    def test_breadcrumb_complete(self):
        """Test breadcrumb with all fields including name and numberOfItems"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "name": "Product Navigation",
                "numberOfItems": 4,
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://store.example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Clothing",
                        "item": "https://store.example.com/clothing"
                    },
                    {
                        "@type": "ListItem",
                        "position": 3,
                        "name": "Men's Clothing",
                        "item": "https://store.example.com/clothing/mens"
                    },
                    {
                        "@type": "ListItem",
                        "position": 4,
                        "name": "Shirts",
                        "item": "https://store.example.com/clothing/mens/shirts"
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
        breadcrumb = objects[0]

        assert breadcrumb["@type"] == "BreadcrumbList"
        assert breadcrumb["name"] == "Product Navigation"
        assert breadcrumb["numberOfItems"] == 4

        items = breadcrumb["itemListElement"]
        assert len(items) == 4

        # Verify structure
        for i, item in enumerate(items, 1):
            assert item["@type"] == "ListItem"
            assert item["position"] == i
            assert "name" in item
            assert "item" in item


class TestBreadcrumbListEdgeCases:
    """Test edge cases for BreadcrumbList"""

    def test_breadcrumb_minimal(self):
        """Test minimal breadcrumb with just required fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home"
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
        assert objects[0]["@type"] == "BreadcrumbList"
        assert len(objects[0]["itemListElement"]) == 1

    def test_breadcrumb_empty_list(self):
        """Test breadcrumb with empty itemListElement"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": []
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "BreadcrumbList"
        assert objects[0]["itemListElement"] == []

    def test_breadcrumb_with_thing_object(self):
        """Test breadcrumb where item is a Thing object instead of URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": {
                            "@type": "Thing",
                            "@id": "https://example.com",
                            "name": "Homepage"
                        }
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Category"
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
        assert objects[0]["@type"] == "BreadcrumbList"
        items = objects[0]["itemListElement"]

        # First item should have object
        assert "item" in items[0]
        # Could be either dict or string representation


class TestBreadcrumbListMultiple:
    """Test multiple breadcrumb scenarios"""

    def test_multiple_breadcrumbs(self):
        """Test page with multiple BreadcrumbList objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "First Breadcrumb"
                    }
                ]
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Second Breadcrumb"
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
        assert all(obj["@type"] == "BreadcrumbList" for obj in objects)

    def test_breadcrumb_in_graph(self):
        """Test BreadcrumbList inside @graph"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "BreadcrumbList",
                        "itemListElement": [
                            {
                                "@type": "ListItem",
                                "position": 1,
                                "name": "Home",
                                "item": "https://example.com"
                            },
                            {
                                "@type": "ListItem",
                                "position": 2,
                                "name": "Products",
                                "item": "https://example.com/products"
                            }
                        ]
                    },
                    {
                        "@type": "Organization",
                        "name": "Example Org"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        # @graph should be expanded
        assert len(objects) == 2

        # Find the breadcrumb
        breadcrumb = next(obj for obj in objects if obj.get("@type") == "BreadcrumbList")
        assert len(breadcrumb["itemListElement"]) == 2


class TestBreadcrumbListIntegration:
    """Test BreadcrumbList integration with extract_all()"""

    def test_extract_all_with_breadcrumb(self):
        """Test that extract_all() includes BreadcrumbList"""
        html = """
        <html>
        <head>
            <title>Product Page</title>
            <meta property="og:title" content="Product">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Products",
                        "item": "https://example.com/products"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1

        breadcrumb = data["jsonld"][0]
        assert breadcrumb["@type"] == "BreadcrumbList"
        assert len(breadcrumb["itemListElement"]) == 2

    def test_extract_all_breadcrumb_with_product(self):
        """Test breadcrumb alongside other JSON-LD types"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "BreadcrumbList",
                        "itemListElement": [
                            {
                                "@type": "ListItem",
                                "position": 1,
                                "name": "Home"
                            }
                        ]
                    },
                    {
                        "@type": "Product",
                        "name": "Widget",
                        "sku": "12345"
                    }
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 2

        types = [obj["@type"] for obj in data["jsonld"]]
        assert "BreadcrumbList" in types
        assert "Product" in types


class TestBreadcrumbListRealWorld:
    """Test with real-world BreadcrumbList examples"""

    def test_ecommerce_breadcrumb(self):
        """Test typical e-commerce breadcrumb navigation"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Home",
                        "item": "https://shop.example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "Electronics",
                        "item": "https://shop.example.com/electronics"
                    },
                    {
                        "@type": "ListItem",
                        "position": 3,
                        "name": "Computers",
                        "item": "https://shop.example.com/electronics/computers"
                    },
                    {
                        "@type": "ListItem",
                        "position": 4,
                        "name": "Laptops",
                        "item": "https://shop.example.com/electronics/computers/laptops"
                    },
                    {
                        "@type": "ListItem",
                        "position": 5,
                        "name": "Gaming Laptops"
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
        breadcrumb = objects[0]

        assert breadcrumb["@type"] == "BreadcrumbList"
        assert len(breadcrumb["itemListElement"]) == 5

        # Verify progression
        items = breadcrumb["itemListElement"]
        assert items[0]["name"] == "Home"
        assert items[4]["name"] == "Gaming Laptops"

        # Last item might not have URL (current page)
        assert "item" in items[0]

    def test_documentation_breadcrumb(self):
        """Test documentation site breadcrumb"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": [
                    {
                        "@type": "ListItem",
                        "position": 1,
                        "name": "Documentation",
                        "item": "https://docs.example.com"
                    },
                    {
                        "@type": "ListItem",
                        "position": 2,
                        "name": "API Reference",
                        "item": "https://docs.example.com/api"
                    },
                    {
                        "@type": "ListItem",
                        "position": 3,
                        "name": "Authentication",
                        "item": "https://docs.example.com/api/auth"
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
        assert objects[0]["@type"] == "BreadcrumbList"
        assert len(objects[0]["itemListElement"]) == 3


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
