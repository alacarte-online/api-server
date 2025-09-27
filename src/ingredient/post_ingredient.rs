use http::Request;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct PostIngredientRequestData {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostIngredientResponseData {
    pub id: i64,
}

pub async fn handle_post_ingredient_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> anyhow::Result<PostIngredientResponseData> {
    let post_recipe_request: PostIngredientRequestData = serde_json::from_slice(request.body())?;
    let created_id = insert_ingredient(post_recipe_request, db_pool).await?;
    let response_data = PostIngredientResponseData { id: created_id };
    Ok(response_data)
}

async fn insert_ingredient(ingredient: PostIngredientRequestData, db_pool: &PgPool) -> anyhow::Result<i64> {
    let inserted_recipe = sqlx::query!("INSERT INTO ingredients
            (name)
            VALUES ($1)
            RETURNING id;",
        ingredient.name)
        .fetch_one(db_pool).await?;
    Ok(inserted_recipe.id)
}