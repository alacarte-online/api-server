use http::Request;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::recipe::database;
use crate::recipe::database::IngredientToInsert;

#[derive(Debug, Serialize, Deserialize)]
struct PostRecipeRequestData {
    pub recipe_name: String,
    pub brief_description: String,
    pub image_uri: String,
    pub method: String,
    pub user_id: i64,
    pub ingredients: Vec<PostIngredientRequestData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostIngredientRequestData {
    pub ingredient_name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRecipeResponseData {
    pub recipe_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostIngredientIdAndAmount {
    pub ingredient_id: i64,
    pub amount: String,
}

pub async fn handle_post_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> anyhow::Result<PostRecipeResponseData> {
    let put_recipe_request: PostRecipeRequestData = serde_json::from_slice(request.body())?;

    let ingredient_ids_and_amounts = get_ingredient_ids_and_amounts(put_recipe_request.ingredients.clone(), db_pool).await?;
    let recipe_id = insert_recipe(&put_recipe_request, db_pool).await?;

    for ingredient_id_amount in ingredient_ids_and_amounts {
        let ingredient_id = ingredient_id_amount.ingredient_id;
        let amount = ingredient_id_amount.amount;
        let recipe_ingredients = database::RecipeIngredientToInsert { recipe_id, ingredient_id, amount };
        recipe_ingredients.insert_into_db(db_pool).await?;
    }

    Ok(PostRecipeResponseData { recipe_id })
}

async fn get_ingredient_ids_and_amounts(put_ingredient_requests: Vec<PostIngredientRequestData>, db_pool: &PgPool) -> anyhow::Result<Vec<PostIngredientIdAndAmount>> {
    let mut ids_and_amounts = vec![];
    for put_ingredient_request in put_ingredient_requests.iter() {
        let id = get_or_insert_ingredient_from_name(&put_ingredient_request.ingredient_name, db_pool).await?;
        ids_and_amounts.push(PostIngredientIdAndAmount {ingredient_id: id, amount: put_ingredient_request.amount.clone()});
    }
    Ok(ids_and_amounts)
}

async fn get_or_insert_ingredient_from_name(ingredient_name: &str, db_pool: &PgPool) -> anyhow::Result<i64> {
    let matching_ingredient = database::Ingredient::try_fetch_from_ingredient_name(db_pool, ingredient_name).await?;
    match matching_ingredient {
        Some(ingredient_details) => {
            log::debug!("Id: {}", ingredient_details.id);
            Ok(ingredient_details.id)
        },
        None => {
            log::debug!("No ingredient found for {}", ingredient_name);
            let ingredient_to_insert = IngredientToInsert { name: ingredient_name.to_string() };
            let inserted_ingredient_id = ingredient_to_insert.insert_into_db(db_pool).await?;
            Ok(inserted_ingredient_id)
        }
    }
}

async fn insert_recipe(recipe: &PostRecipeRequestData, db_pool: &PgPool) -> anyhow::Result<i64> {
    let inserted_recipe = sqlx::query!("INSERT INTO recipes
            (name, brief_description, method, image_uri, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;",
        recipe.recipe_name,
        recipe.brief_description,
        recipe.method,
        recipe.image_uri,
        recipe.user_id)
        .fetch_one(db_pool).await?;
    Ok(inserted_recipe.id)
}