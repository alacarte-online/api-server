use serde::Serialize;
use crate::recipe::recipe_ingredients::RecipeIngredients;

#[derive(Debug, Serialize)]
pub struct IngredientDetails {
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub amount: String
}

impl From<RecipeIngredients> for IngredientDetails {
    fn from(recipe_ingredients: RecipeIngredients) -> Self {
        let ingredient_id = recipe_ingredients.ingredient_id;
        let ingredient_name = recipe_ingredients.ingredient_name;
        let amount = recipe_ingredients.amount;
        Self { ingredient_id, ingredient_name, amount }
    }
}