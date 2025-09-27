use http::Response;
use serde::Serialize;
use sqlx::PgPool;
use crate::http::responses::{internal_server_error_response, json_ok};

#[derive(Debug, Serialize)]
pub struct GetAllIngredients {
    pub ingredients: Vec<GetAllIngredientItem>
}

#[derive(Debug, Serialize)]
pub struct GetAllIngredientItem {
    pub id: i64,
    pub name: String
}

pub async fn get_all_ingredients(db_pool: &PgPool) -> Response<Vec<u8>> {
    let ingredients = get_all_ingredients_from_db(db_pool).await;
    let json = match ingredients {
        Ok(response_json) => serde_json::to_string(&response_json),
        Err(err) => {
            log::info!("Error handling get all ingredients request: {}", err);
            return internal_server_error_response()
        }
    };

    match json {
        Ok(json) => json_ok(json),
        Err(err) => {
            log::info!("Error handling get all recipes request: {}", err);
            internal_server_error_response()
        }
    }
}

pub async fn get_all_ingredients_from_db(db_pool: &PgPool) -> anyhow::Result<GetAllIngredients> {
    let ingredients = sqlx::query_as!(GetAllIngredientItem, "SELECT id, name FROM ingredients;").fetch_all(db_pool).await?;
    let response = GetAllIngredients { ingredients };
    Ok(response)
}