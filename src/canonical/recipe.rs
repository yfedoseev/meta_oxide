//! Canonical schema.org `Recipe` merged across formats.

use crate::canonical::helpers::{find_jsonld_of_type, jsonld_string, value_to_string};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use crate::types::jsonld::JsonLdObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A schema.org Recipe merged across formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Recipe {
    /// Dish name.
    pub name: Option<FieldValue<String>>,
    /// Description / summary.
    pub description: Option<FieldValue<String>>,
    /// Author / chef name.
    pub author: Option<FieldValue<String>>,
    /// Total preparation time (ISO 8601 duration).
    pub prep_time: Option<FieldValue<String>>,
    /// Total cooking time.
    pub cook_time: Option<FieldValue<String>>,
    /// Total elapsed time.
    pub total_time: Option<FieldValue<String>>,
    /// Yield string (e.g. `"4 servings"`).
    pub recipe_yield: Option<FieldValue<String>>,
    /// Ordered list of ingredients.
    pub ingredients: Vec<String>,
    /// Ordered list of instruction steps (text only).
    pub instructions: Vec<String>,
    /// Hero image URL.
    pub image: Option<FieldValue<String>>,
}

impl CanonicalMerge for Recipe {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["Recipe"])?;

        let mut recipe = Recipe::default();
        recipe.name = jsonld_string(jsonld, "name")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.name".into())));
        recipe.description = jsonld_string(jsonld, "description")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.description".into())));
        recipe.author = jsonld
            .properties
            .get("author")
            .and_then(value_to_string)
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.author".into())));
        recipe.prep_time = jsonld_string(jsonld, "prepTime")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.prepTime".into())));
        recipe.cook_time = jsonld_string(jsonld, "cookTime")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.cookTime".into())));
        recipe.total_time = jsonld_string(jsonld, "totalTime")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.totalTime".into())));
        recipe.recipe_yield = jsonld_string(jsonld, "recipeYield")
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.recipeYield".into())));
        recipe.ingredients = collect_strings(jsonld, "recipeIngredient");
        recipe.instructions = collect_instructions(jsonld);
        recipe.image = jsonld
            .properties
            .get("image")
            .and_then(value_to_string)
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Recipe.image".into())));

        recipe.name.as_ref()?;
        Some(recipe)
    }
}

fn collect_strings(item: &JsonLdObject, key: &str) -> Vec<String> {
    match item.properties.get(key) {
        Some(Value::Array(arr)) => arr.iter().filter_map(value_to_string).collect(),
        Some(other) => value_to_string(other).into_iter().collect(),
        None => Vec::new(),
    }
}

fn collect_instructions(item: &JsonLdObject) -> Vec<String> {
    match item.properties.get("recipeInstructions") {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| match v {
                Value::String(s) => Some(s.clone()),
                Value::Object(map) => map.get("text").and_then(value_to_string),
                _ => None,
            })
            .collect(),
        Some(Value::String(s)) => vec![s.clone()],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaParser;

    #[test]
    fn recipe_with_scalar_ingredient() {
        // Some sites emit ingredients as a single string, not array.
        let html = r#"
<script type="application/ld+json">
{
  "@type": "Recipe",
  "name": "Tea",
  "recipeIngredient": "hot water",
  "recipeInstructions": "Steep."
}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let recipe = graph.canonical_recipe().expect("recipe");
        assert_eq!(recipe.ingredients, vec!["hot water"]);
        assert_eq!(recipe.instructions, vec!["Steep."]);
    }

    #[test]
    fn unnamed_recipe_rejected() {
        let html = r#"<script type="application/ld+json">{"@type": "Recipe"}</script>"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_recipe().is_none());
    }

    #[test]
    fn merges_jsonld_recipe() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "Recipe",
  "name": "Pancakes",
  "recipeIngredient": ["flour", "milk", "egg"],
  "recipeInstructions": [
    {"@type": "HowToStep", "text": "Mix"},
    {"@type": "HowToStep", "text": "Cook"}
  ],
  "prepTime": "PT5M",
  "cookTime": "PT10M"
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let recipe = graph.canonical_recipe().expect("recipe");
        assert_eq!(recipe.name.unwrap().value, "Pancakes");
        assert_eq!(recipe.ingredients, vec!["flour", "milk", "egg"]);
        assert_eq!(recipe.instructions, vec!["Mix", "Cook"]);
    }
}
