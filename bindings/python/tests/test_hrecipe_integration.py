"""Integration tests for h-recipe microformat with extract_all()"""

import meta_oxide


def test_extract_all_includes_hrecipe():
    """Test that extract_all() properly includes h-recipe in microformats"""
    html = """
    <html>
    <head>
        <title>Chocolate Chip Cookies Recipe</title>
        <meta property="og:title" content="Best Chocolate Chip Cookies">
    </head>
    <body>
        <div class="h-recipe">
            <h1 class="p-name">Chocolate Chip Cookies</h1>
            <p class="p-summary">The best homemade chocolate chip cookies</p>
            <span class="p-author">Jane Smith</span>
            <time class="dt-published" datetime="2024-01-15">Jan 15, 2024</time>
            <span class="p-duration">PT30M</span>
            <span class="p-yield">24 cookies</span>

            <ul>
                <li class="p-ingredient">2 cups flour</li>
                <li class="p-ingredient">1 cup sugar</li>
                <li class="p-ingredient">1 cup chocolate chips</li>
            </ul>

            <div class="e-instructions">
                Mix all ingredients and bake at 350°F for 12 minutes.
            </div>

            <img class="u-photo" src="https://example.com/cookies.jpg" alt="Cookies" />
            <span class="p-category">Dessert</span>
            <span class="p-category">Baking</span>
        </div>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html, "https://example.com")

    # Verify basic structure
    assert "microformats" in data
    assert "h-recipe" in data["microformats"]

    # Verify h-recipe was extracted
    recipes = data["microformats"]["h-recipe"]
    assert len(recipes) == 1

    recipe = recipes[0]
    assert recipe["name"] == "Chocolate Chip Cookies"
    assert recipe["summary"] == "The best homemade chocolate chip cookies"
    assert recipe["author"] == "Jane Smith"
    assert recipe["published"] == "2024-01-15"
    assert recipe["duration"] == "PT30M"
    assert recipe["yield"] == "24 cookies"

    # Verify ingredients
    assert len(recipe["ingredient"]) == 3
    assert "2 cups flour" in recipe["ingredient"]
    assert "1 cup sugar" in recipe["ingredient"]
    assert "1 cup chocolate chips" in recipe["ingredient"]

    # Verify instructions
    assert "Mix all ingredients" in recipe["instructions"]
    assert "350°F" in recipe["instructions"]

    # Verify photo
    assert recipe["photo"] == "https://example.com/cookies.jpg"

    # Verify categories
    assert len(recipe["category"]) == 2
    assert "Dessert" in recipe["category"]
    assert "Baking" in recipe["category"]


def test_extract_all_multiple_hrecipes():
    """Test extract_all with multiple recipes"""
    html = """
    <html>
    <body>
        <div class="h-recipe">
            <h2 class="p-name">Pancakes</h2>
            <span class="p-ingredient">Flour</span>
            <span class="p-ingredient">Eggs</span>
        </div>
        <div class="h-recipe">
            <h2 class="p-name">Waffles</h2>
            <span class="p-ingredient">Flour</span>
            <span class="p-ingredient">Milk</span>
        </div>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html)

    assert "microformats" in data
    assert "h-recipe" in data["microformats"]

    recipes = data["microformats"]["h-recipe"]
    assert len(recipes) == 2

    names = [r["name"] for r in recipes]
    assert "Pancakes" in names
    assert "Waffles" in names


def test_extract_all_mixed_microformats_with_recipe():
    """Test extract_all with h-recipe alongside other microformats"""
    html = """
    <html>
    <body>
        <div class="h-card">
            <span class="p-name">Chef Gordon</span>
        </div>
        <div class="h-recipe">
            <h1 class="p-name">Beef Wellington</h1>
            <span class="p-author">Chef Gordon</span>
        </div>
        <div class="h-entry">
            <h2 class="p-name">My Cooking Blog</h2>
        </div>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html)

    assert "microformats" in data

    # All three microformats should be present
    assert "h-card" in data["microformats"]
    assert "h-recipe" in data["microformats"]
    assert "h-entry" in data["microformats"]

    # Verify counts
    assert len(data["microformats"]["h-card"]) == 1
    assert len(data["microformats"]["h-recipe"]) == 1
    assert len(data["microformats"]["h-entry"]) == 1

    # Verify recipe data
    recipe = data["microformats"]["h-recipe"][0]
    assert recipe["name"] == "Beef Wellington"
    assert recipe["author"] == "Chef Gordon"


def test_extract_all_no_recipes():
    """Test extract_all when there are no recipes"""
    html = """
    <html>
    <head>
        <title>Blog Post</title>
    </head>
    <body>
        <article class="h-entry">
            <h1 class="p-name">My Blog Post</h1>
        </article>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html)

    # microformats should exist but not h-recipe
    assert "microformats" in data
    assert "h-entry" in data["microformats"]
    assert "h-recipe" not in data["microformats"]


def test_extract_all_recipe_minimal():
    """Test extract_all with minimal recipe (only name)"""
    html = """
    <html>
    <body>
        <div class="h-recipe">
            <span class="p-name">Simple Recipe</span>
        </div>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html)

    assert "microformats" in data
    assert "h-recipe" in data["microformats"]

    recipes = data["microformats"]["h-recipe"]
    assert len(recipes) == 1
    assert recipes[0]["name"] == "Simple Recipe"
