"""Tests for h-recipe microformat extraction"""

import meta_oxide


def test_extract_hrecipe_basic():
    """Test basic h-recipe extraction"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Chocolate Chip Cookies</span>
            <span class="p-summary">Delicious homemade cookies</span>
            <span class="p-ingredient">2 cups flour</span>
            <span class="p-ingredient">1 cup sugar</span>
            <span class="p-ingredient">1 cup chocolate chips</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["name"] == "Chocolate Chip Cookies"
    assert recipes[0]["summary"] == "Delicious homemade cookies"
    assert len(recipes[0]["ingredient"]) == 3
    assert recipes[0]["ingredient"][0] == "2 cups flour"
    assert recipes[0]["ingredient"][1] == "1 cup sugar"
    assert recipes[0]["ingredient"][2] == "1 cup chocolate chips"


def test_hrecipe_with_instructions():
    """Test h-recipe with instructions"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Simple Pasta</span>
            <div class="e-instructions">
                Boil water, add pasta, cook for 10 minutes.
            </div>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["name"] == "Simple Pasta"
    assert "Boil water" in recipes[0]["instructions"]
    assert "cook for 10 minutes" in recipes[0]["instructions"]


def test_hrecipe_with_duration_and_yield():
    """Test h-recipe with duration and yield"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Quick Salad</span>
            <span class="p-duration">15 minutes</span>
            <span class="p-yield">4 servings</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["duration"] == "15 minutes"
    assert recipes[0]["yield"] == "4 servings"


def test_hrecipe_with_photo():
    """Test h-recipe with photo"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Pizza</span>
            <img class="u-photo" src="https://example.com/pizza.jpg" alt="Pizza" />
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["photo"] == "https://example.com/pizza.jpg"


def test_hrecipe_with_author():
    """Test h-recipe with author"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Chef's Special</span>
            <span class="p-author">Gordon Ramsay</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["author"] == "Gordon Ramsay"


def test_hrecipe_with_categories():
    """Test h-recipe with multiple categories"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Tiramisu</span>
            <span class="p-category">Dessert</span>
            <span class="p-category">Italian</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert len(recipes[0]["category"]) == 2
    assert recipes[0]["category"][0] == "Dessert"
    assert recipes[0]["category"][1] == "Italian"


def test_hrecipe_with_published_date():
    """Test h-recipe with published date"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Holiday Cake</span>
            <time class="dt-published" datetime="2024-12-25">December 25, 2024</time>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert recipes[0]["published"] == "2024-12-25"


def test_hrecipe_with_nutrition():
    """Test h-recipe with nutrition information"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Healthy Smoothie</span>
            <span class="p-nutrition">Calories: 150, Protein: 5g</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1
    assert "Calories" in recipes[0]["nutrition"]


def test_multiple_hrecipes():
    """Test extraction of multiple recipes"""
    html = """
        <div class="h-recipe">
            <span class="p-name">Recipe 1</span>
        </div>
        <div class="h-recipe">
            <span class="p-name">Recipe 2</span>
        </div>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 2
    assert recipes[0]["name"] == "Recipe 1"
    assert recipes[1]["name"] == "Recipe 2"


def test_hrecipe_empty():
    """Test page with no recipes"""
    html = "<html><body><p>No recipes here</p></body></html>"
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 0


def test_hrecipe_complete():
    """Test realistic complete recipe"""
    html = """
        <article class="h-recipe">
            <h1 class="p-name">Grandma's Apple Pie</h1>
            <p class="p-summary">A classic apple pie recipe passed down through generations</p>

            <span class="p-author">Jane Smith</span>
            <time class="dt-published" datetime="2024-01-15">January 15, 2024</time>

            <div class="p-category">Dessert</div>
            <div class="p-category">American</div>

            <span class="p-duration">90 minutes</span>
            <span class="p-yield">8 servings</span>

            <h2>Ingredients:</h2>
            <ul>
                <li class="p-ingredient">6 apples, peeled and sliced</li>
                <li class="p-ingredient">1 cup sugar</li>
                <li class="p-ingredient">2 tablespoons flour</li>
                <li class="p-ingredient">1 teaspoon cinnamon</li>
                <li class="p-ingredient">2 pie crusts</li>
            </ul>

            <h2>Instructions:</h2>
            <div class="e-instructions">
                Preheat oven to 350°F. Mix apples with sugar, flour, and cinnamon.
                Place in pie crust, cover with top crust. Bake for 60 minutes.
            </div>

            <img class="u-photo" src="https://example.com/apple-pie.jpg" alt="Apple Pie" />

            <p class="p-nutrition">Per serving: 320 calories, 2g protein, 45g carbohydrates</p>
        </article>
    """
    recipes = meta_oxide.extract_hrecipe(html)
    assert len(recipes) == 1

    recipe = recipes[0]
    assert recipe["name"] == "Grandma's Apple Pie"
    assert "classic apple pie" in recipe["summary"]
    assert recipe["author"] == "Jane Smith"
    assert recipe["published"] == "2024-01-15"
    assert recipe["duration"] == "90 minutes"
    assert recipe["yield"] == "8 servings"

    # Check ingredients
    assert len(recipe["ingredient"]) == 5
    assert "6 apples" in recipe["ingredient"][0]
    assert "cinnamon" in recipe["ingredient"][3]

    # Check instructions
    assert "Preheat oven" in recipe["instructions"]
    assert "350" in recipe["instructions"]

    # Check categories
    assert len(recipe["category"]) == 2
    assert "Dessert" in recipe["category"]
    assert "American" in recipe["category"]

    # Check photo
    assert recipe["photo"] == "https://example.com/apple-pie.jpg"

    # Check nutrition
    assert "320 calories" in recipe["nutrition"]
