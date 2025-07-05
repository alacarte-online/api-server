mod recipe_overview;
mod recipe_details;
mod recipe_ingredients;
mod recipe_details_with_ingredients;
mod ingredient_details;

use crate::http::responses::{bad_request_response, internal_server_error_response, method_not_allowed_response, not_found_response};
use crate::recipe::recipe_overview::{RecipeOverview, RecipeOverviewViewItem};
use futures::executor::block_on;
use http::{Method, Request, Response};
use sqlx::PgPool;
use crate::recipe::recipe_details_with_ingredients::RecipeDetailsWithIngredients;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/recipe/") || request.uri().path() == "/recipe"
}

pub fn handle_request(request: Request<Vec<u8>>, db_pool: &PgPool) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, db_pool),
        _ => method_not_allowed_response()
    }
}

fn handle_get_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> Response<Vec<u8>> {
    let url_chunks = request.uri().path()[1..].split("/").filter(|chunk| !chunk.is_empty()).collect::<Vec<&str>>();

    match url_chunks.len() {
        1 => handle_get_all_recipes_request(db_pool),
        2 => handle_get_single_recipe_request(db_pool, url_chunks.last().unwrap()),
        _ => bad_request_response()
    }
}

fn handle_get_all_recipes_request(db_pool: &PgPool) -> Response<Vec<u8>> {
    let json = block_on(get_all_recipes_json(db_pool));
    let json = match json {
        Ok(json) => json,
        Err(err) => {
            println!("Error handling get all recipes request: {}", err);
            return internal_server_error_response()
        }
    };

    create_ok_response_from_json(json)
}

fn handle_get_single_recipe_request(db_pool: &PgPool, id: &str) -> Response<Vec<u8>> {
    let recipe = block_on(get_recipe_from_uri_part(db_pool, id));
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

    create_ok_response_from_json(json)
}

fn create_ok_response_from_json(json: String) -> Response<Vec<u8>> {
    let response = http::Response::builder()
        .status(http::status::StatusCode::OK)
        .header("Content-Length", json.len())
        .header("Content-Type", "application/json")
        .body(json.into_bytes());
    response.unwrap_or_else(|err| {
        println!("Error creating ok response from json: {}", err);
        internal_server_error_response()
    })
}

async fn get_all_recipes(db_pool: &PgPool) -> anyhow::Result<Vec<RecipeOverview>> {
    let recipes = sqlx::query_as!(RecipeOverviewViewItem, "SELECT * FROM recipe_overviews;").fetch_all(db_pool).await?;
    let mut recipe_vec = vec![];
    for recipe in recipes {
        match recipe.try_into() {
            Ok(recipe) => recipe_vec.push(recipe),
            Err(err) => println!("{}", err)
        }
    }
    Ok(recipe_vec)
}

async fn get_all_recipes_json(db_pool: &PgPool) -> anyhow::Result<String> {
    let recipes = get_all_recipes(db_pool).await?;
    let json = serde_json::to_string(&recipes)?;

    Ok(json)
}

async fn get_recipe_from_uri_part(db_pool: &PgPool, id: &str) -> anyhow::Result<Option<RecipeDetailsWithIngredients>> {
    let id = id.parse::<i64>()?;
    RecipeDetailsWithIngredients::fetch_from_recipe_id(db_pool, id).await
}