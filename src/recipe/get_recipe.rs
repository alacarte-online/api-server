use crate::http::responses::{bad_request_response, internal_server_error_response, json_ok, not_found_response};
use crate::recipe::{database};
use http::Response;
use serde::Serialize;
use sqlx::PgPool;
use crate::recipe::database::RecipeIngredientsView;

pub async fn get_recipe_with_id(db_pool: &PgPool, id: &str) -> Response<Vec<u8>> {
    let id = match id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return bad_request_response()
    };

    let recipe = GetRecipeResponse::fetch_from_recipe_id(db_pool, id).await;
    let recipe = match recipe {
        Ok(recipe) => recipe,
        Err(err) => {
            println!("Error handling get recipe request: {}", err);
            return internal_server_error_response()
        }
    };

    let recipe = match recipe {
        Some(recipe) => recipe,
        None => return not_found_response()
    };

    let json = serde_json::to_string(&recipe);
    let json = match json {
        Ok(json) => json,
        Err(err) => {
            println!("Error handling get recipe request: {}", err);
            return internal_server_error_response()
        }
    };

    json_ok(json)
}

#[derive(Debug, Serialize)]
struct GetRecipeResponse {
    pub recipe_id: i64,
    pub recipe_name: String,
    pub brief_description: String,
    pub method: Option<String>,
    pub image_uri: Option<String>,
    pub user_id: i64,
    pub user_name: String,
    pub ingredients: Option<Vec<GetRecipeIngredientsResponse>>
}

impl GetRecipeResponse {
    pub fn from(recipe_details: database::RecipeDetails, ingredient_details: Option<Vec<GetRecipeIngredientsResponse>>) -> Self {
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
        let recipe_details = database::RecipeDetails::fetch_from_recipe_id(db_pool, recipe_id).await?;
        let recipe_details = match recipe_details {
            Some(recipe_details) => recipe_details,
            None => return Ok(None)
        };

        let recipe_ingredients = database::RecipeIngredientsView::fetch_from_recipe_id(db_pool, recipe_id).await?;
        let ingredient_details = recipe_ingredients.map(|details| details.into_iter().map(GetRecipeIngredientsResponse::from).collect());
        Ok(Some(Self::from(recipe_details, ingredient_details)))
    }
}

#[derive(Debug, Serialize)]
struct GetRecipeIngredientsResponse {
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub amount: String
}

impl From<RecipeIngredientsView> for GetRecipeIngredientsResponse {
    fn from(recipe_ingredients: RecipeIngredientsView) -> Self {
        let ingredient_id = recipe_ingredients.ingredient_id;
        let ingredient_name = recipe_ingredients.ingredient_name;
        let amount = recipe_ingredients.amount;
        Self { ingredient_id, ingredient_name, amount }
    }
}