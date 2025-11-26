"""
Tests for JSON-LD Recipe type support (Schema.org Recipe)

This module tests the extraction of Recipe structured data from JSON-LD,
following the Schema.org Recipe specification.
"""

import meta_oxide
import pytest


class TestRecipeBasic:
    """Test basic Recipe extraction with minimal fields"""

    def test_recipe_minimal(self):
        """Test extracting Recipe with only required name field"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Chocolate Chip Cookies"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Recipe"
        assert objects[0]["name"] == "Chocolate Chip Cookies"

    def test_recipe_with_description(self):
        """Test Recipe with name and description"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Classic Lasagna",
                "description": "A traditional Italian lasagna with layers of pasta, meat sauce, and cheese"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Recipe"
        assert objects[0]["name"] == "Classic Lasagna"
        assert (
            objects[0]["description"]
            == "A traditional Italian lasagna with layers of pasta, meat sauce, and cheese"
        )

    def test_recipe_with_single_image(self):
        """Test Recipe with a single image URL"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Banana Bread",
                "image": "https://example.com/banana-bread.jpg"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Banana Bread"
        assert objects[0]["image"] == "https://example.com/banana-bread.jpg"

    def test_recipe_with_multiple_images(self):
        """Test Recipe with multiple images as array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Pasta Carbonara",
                "image": [
                    "https://example.com/carbonara-1.jpg",
                    "https://example.com/carbonara-2.jpg",
                    "https://example.com/carbonara-3.jpg"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Pasta Carbonara"
        # Arrays are returned as JSON strings in current implementation
        assert "image" in objects[0]


class TestRecipeIngredients:
    """Test Recipe ingredient extraction"""

    def test_recipe_with_ingredients_array(self):
        """Test Recipe with multiple ingredients"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Simple Omelette",
                "recipeIngredient": [
                    "3 eggs",
                    "2 tablespoons butter",
                    "Salt and pepper to taste",
                    "1/4 cup shredded cheese"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["@type"] == "Recipe"
        assert objects[0]["name"] == "Simple Omelette"
        # Array property should be present
        assert "recipeIngredient" in objects[0]

    def test_recipe_with_single_ingredient(self):
        """Test Recipe with a single ingredient"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Popcorn",
                "recipeIngredient": ["Popcorn kernels"]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "recipeIngredient" in objects[0]


class TestRecipeInstructions:
    """Test Recipe instruction extraction"""

    def test_recipe_with_text_instructions(self):
        """Test Recipe with instructions as single text block"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Toast",
                "recipeInstructions": "Place bread in toaster. Toast until golden brown. Remove and serve immediately."
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert (
            objects[0]["recipeInstructions"]
            == "Place bread in toaster. Toast until golden brown. Remove and serve immediately."
        )

    def test_recipe_with_step_instructions(self):
        """Test Recipe with instructions as array of steps"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Pancakes",
                "recipeInstructions": [
                    "Mix flour, sugar, baking powder in a bowl",
                    "Add milk, egg, and melted butter",
                    "Pour batter onto hot griddle",
                    "Flip when bubbles form",
                    "Cook until golden brown"
                ]
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "recipeInstructions" in objects[0]


class TestRecipeTimes:
    """Test Recipe time fields (ISO 8601 duration format)"""

    def test_recipe_with_prep_time(self):
        """Test Recipe with preparation time"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Caesar Salad",
                "prepTime": "PT15M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["prepTime"] == "PT15M"

    def test_recipe_with_cook_time(self):
        """Test Recipe with cooking time"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Roast Chicken",
                "cookTime": "PT1H30M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["cookTime"] == "PT1H30M"

    def test_recipe_with_total_time(self):
        """Test Recipe with total time"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Slow Cooker Stew",
                "totalTime": "PT8H"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["totalTime"] == "PT8H"

    def test_recipe_with_all_times(self):
        """Test Recipe with prep, cook, and total time"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Beef Stew",
                "prepTime": "PT20M",
                "cookTime": "PT2H",
                "totalTime": "PT2H20M"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["prepTime"] == "PT20M"
        assert objects[0]["cookTime"] == "PT2H"
        assert objects[0]["totalTime"] == "PT2H20M"


class TestRecipeMetadata:
    """Test Recipe metadata fields"""

    def test_recipe_with_yield(self):
        """Test Recipe with recipeYield"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Apple Pie",
                "recipeYield": "8 servings"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["recipeYield"] == "8 servings"

    def test_recipe_with_category(self):
        """Test Recipe with recipeCategory"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Chocolate Cake",
                "recipeCategory": "Dessert"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["recipeCategory"] == "Dessert"

    def test_recipe_with_cuisine(self):
        """Test Recipe with recipeCuisine"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Spaghetti Bolognese",
                "recipeCuisine": "Italian"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["recipeCuisine"] == "Italian"

    def test_recipe_with_date_published(self):
        """Test Recipe with datePublished"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Summer Salad",
                "datePublished": "2024-06-15"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["datePublished"] == "2024-06-15"


class TestRecipeNutrition:
    """Test Recipe nutrition information"""

    def test_recipe_with_nutrition(self):
        """Test Recipe with nutrition object"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Grilled Salmon",
                "nutrition": {
                    "@type": "NutritionInformation",
                    "calories": "320 calories",
                    "proteinContent": "35g",
                    "fatContent": "18g",
                    "carbohydrateContent": "5g"
                }
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert "nutrition" in objects[0]


class TestRecipeAuthor:
    """Test Recipe author field"""

    def test_recipe_with_person_author(self):
        """Test Recipe with Person as author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "French Onion Soup",
                "author": {
                    "@type": "Person",
                    "name": "Julia Child"
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

    def test_recipe_with_string_author(self):
        """Test Recipe with string author"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Beef Wellington",
                "author": "Gordon Ramsay"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["author"] == "Gordon Ramsay"


class TestRecipeRating:
    """Test Recipe aggregateRating field"""

    def test_recipe_with_aggregate_rating(self):
        """Test Recipe with aggregateRating"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Best Brownies",
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.8",
                    "reviewCount": "245"
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


class TestRecipeComplete:
    """Test complete Recipe with all fields"""

    def test_recipe_complete(self):
        """Test Recipe with comprehensive field set"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Grandma's Apple Pie",
                "description": "A classic homemade apple pie recipe passed down through generations",
                "image": [
                    "https://example.com/apple-pie-1.jpg",
                    "https://example.com/apple-pie-2.jpg"
                ],
                "author": {
                    "@type": "Person",
                    "name": "Jane Smith"
                },
                "datePublished": "2024-01-15",
                "prepTime": "PT30M",
                "cookTime": "PT1H",
                "totalTime": "PT1H30M",
                "recipeYield": "8 servings",
                "recipeCategory": "Dessert",
                "recipeCuisine": "American",
                "recipeIngredient": [
                    "6 cups thinly sliced apples",
                    "3/4 cup white sugar",
                    "2 tablespoons all-purpose flour",
                    "3/4 teaspoon ground cinnamon",
                    "1/4 teaspoon ground nutmeg",
                    "1 recipe pastry for a 9 inch double crust pie"
                ],
                "recipeInstructions": [
                    "Preheat oven to 425 degrees F (220 degrees C)",
                    "Combine sugar, flour, cinnamon, and nutmeg in a bowl",
                    "Mix in apples until evenly coated",
                    "Place bottom crust in pie pan and fill with apple mixture",
                    "Cover with top crust and seal edges",
                    "Bake for 40-50 minutes until crust is golden brown"
                ],
                "nutrition": {
                    "@type": "NutritionInformation",
                    "calories": "410 calories",
                    "carbohydrateContent": "58g",
                    "fatContent": "19g",
                    "proteinContent": "4g"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "523"
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
        assert obj["@type"] == "Recipe"
        assert obj["name"] == "Grandma's Apple Pie"
        assert (
            obj["description"]
            == "A classic homemade apple pie recipe passed down through generations"
        )

        # Check time fields
        assert obj["prepTime"] == "PT30M"
        assert obj["cookTime"] == "PT1H"
        assert obj["totalTime"] == "PT1H30M"

        # Check metadata
        assert obj["recipeYield"] == "8 servings"
        assert obj["recipeCategory"] == "Dessert"
        assert obj["recipeCuisine"] == "American"
        assert obj["datePublished"] == "2024-01-15"

        # Check nested objects exist
        assert "author" in obj
        assert "nutrition" in obj
        assert "aggregateRating" in obj
        assert "recipeIngredient" in obj
        assert "recipeInstructions" in obj
        assert "image" in obj


class TestRecipeEdgeCases:
    """Test edge cases for Recipe extraction"""

    def test_recipe_empty_fields(self):
        """Test Recipe with some empty/null fields"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Simple Recipe",
                "description": null,
                "recipeIngredient": []
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 1
        assert objects[0]["name"] == "Simple Recipe"

    def test_multiple_recipes(self):
        """Test extracting multiple Recipe objects"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Recipe One"
            }
            </script>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Recipe Two"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        objects = meta_oxide.extract_jsonld(html)

        assert len(objects) == 2
        assert objects[0]["name"] == "Recipe One"
        assert objects[1]["name"] == "Recipe Two"

    def test_recipe_in_graph(self):
        """Test Recipe within @graph array"""
        html = """
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@graph": [
                    {
                        "@type": "Recipe",
                        "name": "Pasta Recipe"
                    },
                    {
                        "@type": "Person",
                        "name": "Chef Name"
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
        recipe = next(obj for obj in objects if obj.get("@type") == "Recipe")
        assert recipe["name"] == "Pasta Recipe"


class TestRecipeIntegration:
    """Test Recipe integration with extract_all()"""

    def test_extract_all_includes_recipe(self):
        """Test that extract_all() properly includes Recipe objects"""
        html = """
        <html>
        <head>
            <title>Recipe Page</title>
            <meta property="og:title" content="Best Recipe Ever">
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Recipe",
                "name": "Perfect Pancakes",
                "description": "The fluffiest pancakes you'll ever make"
            }
            </script>
        </head>
        <body></body>
        </html>
        """

        data = meta_oxide.extract_all(html)

        assert "jsonld" in data
        assert len(data["jsonld"]) == 1
        assert data["jsonld"][0]["@type"] == "Recipe"
        assert data["jsonld"][0]["name"] == "Perfect Pancakes"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
