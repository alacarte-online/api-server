use http::{Request, Response};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::http::responses;
use crate::recipe::chunk_url;
use crate::recipe::database::recipe_overview;

#[derive(Debug, Serialize, Deserialize)]
struct PutRecipeRequestData {
    pub recipe_name: Option<String>,
    pub brief_description: Option<String>,
    pub image_uri: Option<String>,
    pub method: Option<String>,
    pub user_id: Option<i64>,
}

pub async fn handle_put_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> Response<Vec<u8>> {
    log::debug!("Handling PUT request for {}", request.uri());

    let uri_chunks = chunk_url(request.uri());
    if uri_chunks.len() != 2 {
        let message = "Request uri should contain 2 chunks";
        log::info!("{}", message);
        return responses::bad_request_response_with_message(message);
    }

    let recipe_id = uri_chunks[1];
    let recipe_id = match recipe_id.parse::<i64>() {
        Ok(id) => id,
        Err(err) => {
            log::info!("Failed parsing recipe id {} - {}", recipe_id, err);
            return responses::bad_request_response_with_message("Recipe id must be an integer");
        }
    };

    let recipe_exists = match recipe_with_id_exists(recipe_id, db_pool).await {
        Ok(exists) => exists,
        Err(err) => {
            log::error!("Error checking whether recipe {} exists - {}", recipe_id, err);
            return responses::internal_server_error_response();
        }
    };

    if !recipe_exists {
        log::debug!("Recipe did not exist for request {}", request.uri());
        return responses::not_found_response();
    }

    let put_recipe_request: PutRecipeRequestData = match serde_json::from_slice(request.body()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Error deserializing request response body - {}", err);
            return responses::internal_server_error_response();
        }
    };

    if let Some(recipe_name) = put_recipe_request.recipe_name {
        match update_recipe_name(recipe_id, &recipe_name, db_pool).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Error updating recipe name for recipe {} - {}", recipe_id, err);
                return responses::internal_server_error_response();
            }
        }
    }

    if let Some(brief_description) = put_recipe_request.brief_description {
        match update_brief_description(recipe_id, &brief_description, db_pool).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Error updating brief description for recipe {} - {}", recipe_id, err);
                return responses::internal_server_error_response();
            }
        }
    }

    if let Some(image_uri) = put_recipe_request.image_uri {
        match update_image_uri(recipe_id, &image_uri, db_pool).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Error updating image uri for recipe {} - {}", recipe_id, err);
                return responses::internal_server_error_response();
            }
        }
    }

    if let Some(recipe_method) = put_recipe_request.method {
        match update_recipe_method(recipe_id, &recipe_method, db_pool).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Error updating method for recipe {} - {}", recipe_id, err);
                return responses::internal_server_error_response();
            }
        }
    }

    if let Some(user_id) = put_recipe_request.user_id {
        match update_user_id(recipe_id, user_id, db_pool).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Error updating user id for recipe {} - {}", recipe_id, err);
                return responses::internal_server_error_response();
            }
        }
    }

    responses::empty_ok()
}

async fn recipe_with_id_exists(recipe_id: i64, db_pool: &PgPool) -> anyhow::Result<bool> {
    let recipe = recipe_overview::RecipeOverview::get_recipe_overview(recipe_id, db_pool).await?;
    Ok(recipe.is_some())
}

async fn update_recipe_name(recipe_id: i64, recipe_name: &str, db_pool: &PgPool) -> anyhow::Result<()> {

    let result = sqlx::query!("UPDATE recipes
            SET name = $1
            WHERE id = $2;",
        recipe_name, recipe_id)
        .execute(db_pool).await?;

    if result.rows_affected() == 0 {
        log::error!("Update image request for recipe {} updated 0 rows", recipe_id);
    }

    Ok(())
}

async fn update_brief_description(recipe_id: i64, brief_description: &str, db_pool: &PgPool) -> anyhow::Result<()> {

    let result = sqlx::query!("UPDATE recipes
            SET brief_description = $1
            WHERE id = $2;",
        brief_description, recipe_id)
        .execute(db_pool).await?;

    if result.rows_affected() == 0 {
        log::error!("Update image request for recipe {} updated 0 rows", recipe_id);
    }

    Ok(())
}

async fn update_image_uri(recipe_id: i64, image_uri: &str, db_pool: &PgPool) -> anyhow::Result<()> {

    let result = sqlx::query!("UPDATE recipes
            SET image_uri = $1
            WHERE id = $2;",
        image_uri, recipe_id)
        .execute(db_pool).await?;

    if result.rows_affected() == 0 {
        log::error!("Update image request for recipe {} updated 0 rows", recipe_id);
    }

    Ok(())
}

async fn update_recipe_method(recipe_id: i64, recipe_method: &str, db_pool: &PgPool) -> anyhow::Result<()> {

    let result = sqlx::query!("UPDATE recipes
            SET method = $1
            WHERE id = $2;",
        recipe_method, recipe_id)
        .execute(db_pool).await?;

    if result.rows_affected() == 0 {
        log::error!("Update image request for recipe {} updated 0 rows", recipe_id);
    }

    Ok(())
}

async fn update_user_id(recipe_id: i64, user_id: i64, db_pool: &PgPool) -> anyhow::Result<()> {

    let result = sqlx::query!("UPDATE recipes
            SET user_id = $1
            WHERE id = $2;",
        user_id, recipe_id)
        .execute(db_pool).await?;

    if result.rows_affected() == 0 {
        log::error!("Update image request for recipe {} updated 0 rows", recipe_id);
    }

    Ok(())
}