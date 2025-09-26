use http::Response;
use sqlx::PgPool;
use crate::http::responses::{internal_server_error_response, json_ok};
use crate::recipe::{database};

pub async fn get_all_recipes(db_pool: &PgPool) -> Response<Vec<u8>> {
    let recipes = database::RecipeOverview::get_all_recipe_overviews(db_pool).await;
    let json = match recipes {
        Ok(recipe_vec) => serde_json::to_string(&recipe_vec),
        Err(err) => {
            log::info!("Error handling get all recipes request: {}", err);
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

