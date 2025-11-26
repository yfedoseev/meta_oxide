"""
Tests for JSON-LD HowTo type support (Schema.org HowTo)

This module tests the extraction of HowTo structured data from JSON-LD,
following the Schema.org HowTo specification.
"""

import meta_oxide
import pytest


class TestHowToBasic:
    """Test basic HowTo extraction with minimal fields"""

    def test_howto_basic(self):
        """Test extracting HowTo with only required name field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Change a Tire"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "HowTo"
        assert objects[0]["name"] == "How to Change a Tire"

    def test_howto_with_description(self):
        """Test HowTo with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Make Coffee",
                "description": "A step-by-step guide to brewing the perfect cup of coffee"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "HowTo"
        assert objects[0]["name"] == "How to Make Coffee"
        assert (
            objects[0]["description"] == "A step-by-step guide to brewing the perfect cup of coffee"
        )

    def test_howto_with_single_image(self):
        """Test HowTo with a single image URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Plant a Tree",
                "image": "https://example.com/plant-tree.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "How to Plant a Tree"
        assert objects[0]["image"] == "https://example.com/plant-tree.jpg"

    def test_howto_with_multiple_images(self):
        """Test HowTo with multiple images as array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Build a Shelf",
                "image": [
                    "https://example.com/shelf-1.jpg",
                    "https://example.com/shelf-2.jpg",
                    "https://example.com/shelf-3.jpg"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "How to Build a Shelf"
        assert "image" in objects[0]

    def test_howto_with_url(self):
        """Test HowTo with URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Bake Bread",
                "url": "https://example.com/how-to-bake-bread"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["url"] == "https://example.com/how-to-bake-bread"


class TestHowToWithSteps:
    """Test HowTo with step-by-step instructions"""

    def test_howto_with_steps(self):
        """Test HowTo with multiple HowToStep objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Tie a Tie",
                "step": [
                    {
                        "@type": "HowToStep",
                        "name": "Start with the wide end",
                        "text": "Drape the tie around your neck with the wide end on your right",
                        "position": 1
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Cross over",
                        "text": "Cross the wide end over the narrow end",
                        "position": 2
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Pull through",
                        "text": "Bring the wide end up through the loop",
                        "position": 3
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
        assert objects[0]["@type"] == "HowTo"
        assert objects[0]["name"] == "How to Tie a Tie"
        assert "step" in objects[0]

    def test_howto_with_single_step(self):
        """Test HowTo with a single step"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Reset Password",
                "step": [
                    {
                        "@type": "HowToStep",
                        "text": "Click the 'Forgot Password' link and follow instructions"
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
        assert "step" in objects[0]

    def test_howto_with_detailed_steps(self):
        """Test HowTo with steps containing images and URLs"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Install a Light Fixture",
                "step": [
                    {
                        "@type": "HowToStep",
                        "name": "Turn off power",
                        "text": "Turn off the circuit breaker for the room",
                        "image": "https://example.com/step1.jpg",
                        "url": "https://example.com/step1"
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Remove old fixture",
                        "text": "Unscrew and carefully remove the old light fixture",
                        "image": "https://example.com/step2.jpg",
                        "url": "https://example.com/step2"
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
        assert "step" in objects[0]


class TestHowToWithTime:
    """Test HowTo with totalTime field (ISO 8601 duration format)"""

    def test_howto_with_time(self):
        """Test HowTo with totalTime in ISO 8601 format"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Oil Change",
                "totalTime": "PT30M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["totalTime"] == "PT30M"

    def test_howto_with_hours(self):
        """Test HowTo with time in hours"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Paint a Room",
                "totalTime": "PT4H"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["totalTime"] == "PT4H"

    def test_howto_with_complex_time(self):
        """Test HowTo with complex time duration"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Build a Deck",
                "totalTime": "P2DT6H"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["totalTime"] == "P2DT6H"


class TestHowToWithTools:
    """Test HowTo with tools array"""

    def test_howto_with_tools(self):
        """Test HowTo with required tools"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Hang a Picture",
                "tool": [
                    {
                        "@type": "HowToTool",
                        "name": "Hammer"
                    },
                    {
                        "@type": "HowToTool",
                        "name": "Level"
                    },
                    {
                        "@type": "HowToTool",
                        "name": "Pencil"
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
        assert "tool" in objects[0]

    def test_howto_with_single_tool(self):
        """Test HowTo with single tool"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Open a Bottle",
                "tool": {
                    "@type": "HowToTool",
                    "name": "Bottle Opener"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "tool" in objects[0]

    def test_howto_with_tool_strings(self):
        """Test HowTo with tools as simple strings"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Assemble Furniture",
                "tool": ["Screwdriver", "Allen wrench", "Rubber mallet"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "tool" in objects[0]


class TestHowToWithSupplies:
    """Test HowTo with supplies array"""

    def test_howto_with_supplies(self):
        """Test HowTo with required supplies"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Fix a Leaky Faucet",
                "supply": [
                    {
                        "@type": "HowToSupply",
                        "name": "Replacement washer"
                    },
                    {
                        "@type": "HowToSupply",
                        "name": "Plumber's tape"
                    },
                    {
                        "@type": "HowToSupply",
                        "name": "Lubricant"
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
        assert "supply" in objects[0]

    def test_howto_with_single_supply(self):
        """Test HowTo with single supply"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Clean Windows",
                "supply": {
                    "@type": "HowToSupply",
                    "name": "Glass cleaner"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "supply" in objects[0]


class TestHowToWithCost:
    """Test HowTo with estimatedCost field"""

    def test_howto_with_cost(self):
        """Test HowTo with estimated cost object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Build a Birdhouse",
                "estimatedCost": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": "25.00"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "estimatedCost" in objects[0]

    def test_howto_with_cost_string(self):
        """Test HowTo with cost as string"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Create a Garden",
                "estimatedCost": "$50-100"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["estimatedCost"] == "$50-100"


class TestHowToWithAuthor:
    """Test HowTo with author field"""

    def test_howto_with_person_author(self):
        """Test HowTo with Person as author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Code in Python",
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

    def test_howto_with_organization_author(self):
        """Test HowTo with Organization as author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Use Our Product",
                "author": {
                    "@type": "Organization",
                    "name": "Acme Corp"
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


class TestHowToComplete:
    """Test complete HowTo with all fields"""

    def test_howto_complete(self):
        """Test HowTo with comprehensive field set"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Build a Wooden Planter Box",
                "description": "A comprehensive guide to building a beautiful wooden planter box for your garden",
                "image": [
                    "https://example.com/planter-1.jpg",
                    "https://example.com/planter-2.jpg"
                ],
                "totalTime": "PT2H",
                "estimatedCost": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": "40.00"
                },
                "tool": [
                    {
                        "@type": "HowToTool",
                        "name": "Saw"
                    },
                    {
                        "@type": "HowToTool",
                        "name": "Drill"
                    },
                    {
                        "@type": "HowToTool",
                        "name": "Screwdriver"
                    }
                ],
                "supply": [
                    {
                        "@type": "HowToSupply",
                        "name": "Cedar boards (4x 2x6x6)"
                    },
                    {
                        "@type": "HowToSupply",
                        "name": "Wood screws (box of 50)"
                    },
                    {
                        "@type": "HowToSupply",
                        "name": "Wood stain"
                    }
                ],
                "step": [
                    {
                        "@type": "HowToStep",
                        "name": "Cut the boards",
                        "text": "Cut the cedar boards to size: 4 pieces at 24 inches for sides, 4 pieces at 12 inches for ends",
                        "position": 1
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Assemble the frame",
                        "text": "Use wood screws to attach the side pieces to the end pieces, creating a rectangular frame",
                        "position": 2
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Apply finish",
                        "text": "Sand the assembled box and apply wood stain for weather protection",
                        "position": 3
                    }
                ],
                "url": "https://example.com/how-to-build-planter-box",
                "author": {
                    "@type": "Person",
                    "name": "Bob Builder"
                },
                "datePublished": "2024-03-15"
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
        assert obj["@type"] == "HowTo"
        assert obj["name"] == "How to Build a Wooden Planter Box"
        assert (
            obj["description"]
            == "A comprehensive guide to building a beautiful wooden planter box for your garden"
        )

        # Check time and cost
        assert obj["totalTime"] == "PT2H"
        assert "estimatedCost" in obj

        # Check URL and date
        assert obj["url"] == "https://example.com/how-to-build-planter-box"
        assert obj["datePublished"] == "2024-03-15"

        # Check nested objects exist
        assert "author" in obj
        assert "tool" in obj
        assert "supply" in obj
        assert "step" in obj
        assert "image" in obj


class TestHowToDIYProject:
    """Test realistic DIY project HowTo example"""

    def test_howto_diy_project(self):
        """Test a realistic DIY project HowTo"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Replace a Doorknob",
                "description": "Learn how to replace a doorknob in 6 easy steps",
                "image": "https://example.com/doorknob.jpg",
                "totalTime": "PT45M",
                "estimatedCost": {
                    "@type": "MonetaryAmount",
                    "currency": "USD",
                    "value": "35.00"
                },
                "tool": [
                    "Screwdriver",
                    "Drill (optional)"
                ],
                "supply": [
                    "New doorknob set",
                    "2 screws"
                ],
                "step": [
                    {
                        "@type": "HowToStep",
                        "name": "Remove old knob",
                        "text": "Unscrew and remove the old doorknob",
                        "position": 1
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Install new knob",
                        "text": "Insert the new doorknob mechanism and secure with screws",
                        "position": 2
                    },
                    {
                        "@type": "HowToStep",
                        "name": "Test",
                        "text": "Test the doorknob to ensure it latches properly",
                        "position": 3
                    }
                ],
                "author": {
                    "@type": "Person",
                    "name": "Home Repair Expert"
                },
                "datePublished": "2024-02-20",
                "url": "https://example.com/replace-doorknob"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        obj = objects[0]

        assert obj["@type"] == "HowTo"
        assert obj["name"] == "How to Replace a Doorknob"
        assert obj["totalTime"] == "PT45M"
        assert "tool" in obj
        assert "supply" in obj
        assert "step" in obj
        assert "estimatedCost" in obj


class TestHowToEdgeCases:
    """Test edge cases for HowTo extraction"""

    def test_howto_empty_fields(self):
        """Test HowTo with some empty/null fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "Simple HowTo",
                "description": null,
                "step": []
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Simple HowTo"

    def test_multiple_howtos(self):
        """Test extracting multiple HowTo objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "HowTo One"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "HowTo Two"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["name"] == "HowTo One"
        assert objects[1]["name"] == "HowTo Two"

    def test_howto_in_graph(self):
        """Test HowTo within @graph array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "HowTo",
                        "name": "How to Install Software"
                    },
                    {
                        "@type": "Person",
                        "name": "Tech Expert"
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
        howto = next(obj for obj in objects if obj.get("@type") == "HowTo")
        assert howto["name"] == "How to Install Software"


class TestHowToIntegration:
    """Test HowTo integration with extract_all()"""

    def test_extract_all_includes_howto(self):
        """Test that extract_all() properly includes HowTo objects"""
        html = """
        <html>
        <head>
            <title>HowTo Page</title>
            <meta property="og:title" content="Best Tutorial Ever">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "HowTo",
                "name": "How to Learn Python",
                "description": "A comprehensive guide to learning Python programming"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "HowTo"
        assert data["jsonld"][0]["name"] == "How to Learn Python"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
