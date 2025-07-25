mod get_recipe;
mod get_all_recipes;
mod database;
mod post_recipe;
mod put_recipe;

use futures::executor::block_on;
use crate::http::responses::{bad_request_response, internal_server_error_response, method_not_allowed_response, unauthorized_response};
use crate::recipe::get_recipe::get_recipe_with_id;
use http::{Method, Request, Response, Uri};
use sqlx::PgPool;
use crate::authorization::Authorization;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/recipe/") || request.uri().path() == "/recipe"
}

pub fn handle_request(request: Request<Vec<u8>>, db_pool: &PgPool, auth_handler: &Authorization) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, db_pool),
        Method::POST => handle_post_request(&request, db_pool, auth_handler),
        Method::PUT => handle_put_request(&request, db_pool, auth_handler),
        _ => method_not_allowed_response()
    }
}

pub fn chunk_url(uri: &Uri) -> Vec<&str> {
    uri.path()[1..].split("/").filter(|chunk| !chunk.is_empty()).collect::<Vec<&str>>()
}

fn handle_get_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> Response<Vec<u8>> {
    let url_chunks = chunk_url(request.uri());

    match url_chunks.len() {
        1 => block_on(get_all_recipes::get_all_recipes(db_pool)),
        2 => block_on(get_recipe_with_id(db_pool, url_chunks.last().unwrap())),
        _ => bad_request_response()
    }
}

fn handle_post_request(request: &Request<Vec<u8>>, db_pool: &PgPool, authorization: &Authorization) -> Response<Vec<u8>> {
    let url_chunks = chunk_url(request.uri());
    if url_chunks.len() != 1 {
        log::info!("Bad request - POST recipe request contained a sub-path");
        return bad_request_response()
    }

    match authorization.authenticate_request(request) {
        Ok(_) => (),
        Err(_) => {
            return unauthorized_response();
        }
    }

    let handle_request_result = block_on(post_recipe::handle_post_request(request, db_pool));
    let post_recipe_response_data = match handle_request_result {
        Ok(response_data) => response_data,
        Err(err) => {
            log::info!("Failed to handle post request: {}", err);
            return bad_request_response()
        }
    };

    let recipe_location = "/recipe/".to_string() + post_recipe_response_data.recipe_id.to_string().as_str();
    http::Response::builder()
        .status(http::status::StatusCode::CREATED)
        .header(http::header::LOCATION, recipe_location)
        .body(vec![])
        .expect("error building response")
}

fn handle_put_request(request: &Request<Vec<u8>>, db_pool: &PgPool, authorization: &Authorization) -> Response<Vec<u8>> {
    match authorization.authenticate_request(request) {
        Ok(_) => (),
        Err(_) => {
            return unauthorized_response();
        }
    }

    block_on(put_recipe::handle_put_request(request, db_pool))
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