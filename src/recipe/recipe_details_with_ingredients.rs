use serde::Serialize;
use sqlx::PgPool;
use crate::recipe::ingredient_details::IngredientDetails;
use crate::recipe::recipe_details::RecipeDetails;
use crate::recipe::recipe_ingredients::RecipeIngredients;

#[derive(Debug, Serialize)]
pub struct RecipeDetailsWithIngredients {
    pub recipe_id: i64,
    pub recipe_name: String,
    pub brief_description: String,
    pub method: String,
    pub image_uri: String,
    pub user_id: i64,
    pub user_name: String,
    pub ingredients: Vec<IngredientDetails>
}

impl RecipeDetailsWithIngredients {
    pub fn from(recipe_details: RecipeDetails, ingredient_details: Vec<IngredientDetails>) -> Self {
        let recipe_id = recipe_details.recipe_id;
        let recipe_name = recipe_details.recipe_name;
        let brief_description = recipe_details.brief_description;
        let method = recipe_details.method;
        let image_uri = recipe_details.image_uri;
        let user_id = recipe_details.user_id;
        let user_name = recipe_details.user_name;
        let ingredients = ingredient_details;

        Self {
            recipe_id,
            recipe_name,
            brief_description,
            method,
            image_uri,
            user_id,
            user_name,
            ingredients
        }
    }

    pub async fn fetch_from_recipe_id(db_pool: &PgPool, recipe_id: i64) -> anyhow::Result<Option<Self>> {
        let recipe_details = RecipeDetails::fetch_from_recipe_id(db_pool, recipe_id).await?;
        let recipe_details = match recipe_details {
            Some(recipe_details) => recipe_details,
            None => return Ok(None)
        };

        let recipe_ingredients = RecipeIngredients::fetch_from_recipe_id(db_pool, recipe_id).await?;

        let ingredient_details = recipe_ingredients.into_iter().map(IngredientDetails::from).collect::<Vec<IngredientDetails>>();
        Ok(Some(Self::from(recipe_details, ingredient_details)))
    }
}