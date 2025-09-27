use http::Request;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize)]
struct PostRecipeRequestData {
    pub recipe_name: String,
    pub brief_description: String,
    pub image_uri: Option<String>,
    pub method: Option<String>,
    pub user_id: i64,
    pub ingredients: Option<Vec<PostIngredientRequestData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PostIngredientRequestData {
    pub ingredient_id: i64,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRecipeResponseData {
    pub recipe_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecipeIngredientData {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub amount: String,
}

pub async fn handle_post_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> anyhow::Result<PostRecipeResponseData> {
    let put_recipe_request: PostRecipeRequestData = serde_json::from_slice(request.body())?;

    let mut tx = db_pool.begin().await?;

    // Add recipe to recipes table
    let inserted_recipe_id = insert_recipe(&put_recipe_request, &mut tx).await?;

    if let Some(ingredients) = put_recipe_request.ingredients {
        for ingredient in ingredients {
            let recipe_ingredient_data = RecipeIngredientData {
                recipe_id: inserted_recipe_id,
                ingredient_id: ingredient.ingredient_id,
                amount: ingredient.amount,
            };
            let _ = insert_ingredient_recipe(&recipe_ingredient_data, &mut tx).await?;
        }
    }

    tx.commit().await?;

    Ok(PostRecipeResponseData { recipe_id: inserted_recipe_id })
}

#[derive(sqlx::FromRow)]
struct InsertedRecipe {
    id: i64
}

async fn insert_recipe(recipe: &PostRecipeRequestData, transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<i64> {
    let inserted_recipe: InsertedRecipe = sqlx::query_as("INSERT INTO recipes
            (name, brief_description, method, image_uri, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;")
        .bind(&recipe.recipe_name)
        .bind(&recipe.brief_description)
        .bind(&recipe.method)
        .bind(&recipe.image_uri)
        .bind(&recipe.user_id)
        .fetch_one(&mut **transaction).await?;
    Ok(inserted_recipe.id)
}

#[derive(sqlx::FromRow)]
struct InsertedRecipeIngredient {
    id: i64
}

async fn insert_ingredient_recipe(recipe_ingredient_data: &RecipeIngredientData, transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<i64> {
    let inserted_row: InsertedRecipeIngredient = sqlx::query_as("INSERT INTO recipe_ingredients
            (recipe_id, ingredient_id, amount)
            VALUES ($1, $2)
            RETURNING id;")
        .bind(&recipe_ingredient_data.recipe_id)
        .bind(&recipe_ingredient_data.ingredient_id)
        .bind(&recipe_ingredient_data.amount)
        .fetch_one(&mut **transaction).await?;
    Ok(inserted_row.id)
}