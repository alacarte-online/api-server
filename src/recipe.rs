mod get_recipe;
mod get_all_recipes;
mod database;

use futures::executor::block_on;
use crate::http::responses::{bad_request_response, internal_server_error_response, method_not_allowed_response};
use crate::recipe::get_recipe::get_recipe_with_id;
use http::{Method, Request, Response};
use sqlx::PgPool;

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
        1 => block_on(get_all_recipes::get_all_recipes(db_pool)),
        2 => block_on(get_recipe_with_id(db_pool, url_chunks.last().unwrap())),
        _ => bad_request_response()
    }
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