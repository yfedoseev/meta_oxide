use crate::microformat_extractor;
use crate::types::HRecipe;

microformat_extractor! {
    HRecipe, ".h-recipe" {
        name: text(".p-name"),
        summary: text(".p-summary"),
        ingredient: multi_text(".p-ingredient"),
        instructions: text(".e-instructions"),
        duration: text(".p-duration"),
        yield_: text(".p-yield"),
        nutrition: text(".p-nutrition"),
        photo: url(".u-photo"),
        author: text(".p-author"),
        published: date(".dt-published"),
        category: multi_text(".p-category"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hrecipe() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Chocolate Chip Cookies</span>
                <span class="p-summary">Delicious homemade cookies</span>
                <span class="p-ingredient">2 cups flour</span>
                <span class="p-ingredient">1 cup sugar</span>
                <span class="p-ingredient">1 cup chocolate chips</span>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].name, Some("Chocolate Chip Cookies".to_string()));
        assert_eq!(recipes[0].summary, Some("Delicious homemade cookies".to_string()));
        assert_eq!(recipes[0].ingredient.len(), 3);
        assert_eq!(recipes[0].ingredient[0], "2 cups flour");
    }

    #[test]
    fn test_hrecipe_with_instructions() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Simple Pasta</span>
                <div class="e-instructions">
                    Boil water, add pasta, cook for 10 minutes.
                </div>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert!(recipes[0].instructions.as_ref().unwrap().contains("Boil water"));
    }

    #[test]
    fn test_hrecipe_with_duration_and_yield() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Quick Salad</span>
                <span class="p-duration">15 minutes</span>
                <span class="p-yield">4 servings</span>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].duration, Some("15 minutes".to_string()));
        assert_eq!(recipes[0].yield_, Some("4 servings".to_string()));
    }

    #[test]
    fn test_hrecipe_with_photo() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Pizza</span>
                <img class="u-photo" src="https://example.com/pizza.jpg" alt="Pizza" />
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].photo, Some("https://example.com/pizza.jpg".to_string()));
    }

    #[test]
    fn test_hrecipe_with_author() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Chef's Special</span>
                <span class="p-author">Gordon Ramsay</span>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].author, Some("Gordon Ramsay".to_string()));
    }

    #[test]
    fn test_hrecipe_with_categories() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Tiramisu</span>
                <span class="p-category">Dessert</span>
                <span class="p-category">Italian</span>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].category.len(), 2);
        assert_eq!(recipes[0].category[0], "Dessert");
        assert_eq!(recipes[0].category[1], "Italian");
    }

    #[test]
    fn test_multiple_hrecipes() {
        let html = r#"
            <div class="h-recipe">
                <span class="p-name">Recipe 1</span>
            </div>
            <div class="h-recipe">
                <span class="p-name">Recipe 2</span>
            </div>
        "#;

        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 2);
        assert_eq!(recipes[0].name, Some("Recipe 1".to_string()));
        assert_eq!(recipes[1].name, Some("Recipe 2".to_string()));
    }

    #[test]
    fn test_hrecipe_empty() {
        let html = "<html><body><p>No recipes here</p></body></html>";
        let recipes = extract(html, None).unwrap();
        assert_eq!(recipes.len(), 0);
    }
}
