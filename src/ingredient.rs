mod get_all_ingredients;
mod post_ingredient;

use futures::executor::block_on;
use http::{Method, Request, Response};
use sqlx::PgPool;
use crate::authorization::Authorization;
use crate::http::responses::{bad_request_response, method_not_allowed_response, unauthorized_response};
use crate::ingredient::get_all_ingredients::get_all_ingredients;
use crate::recipe::chunk_url;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/ingredient/") || request.uri().path() == "/ingredient"
}

pub fn handle_request(request: Request<Vec<u8>>, db_pool: &PgPool, auth_handler: &Authorization) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, db_pool),
        Method::POST => handle_post_request(&request, db_pool, auth_handler),
        _ => method_not_allowed_response()
    }
}

fn handle_get_request(request: &Request<Vec<u8>>, db_pool: &PgPool) -> Response<Vec<u8>> {
    let url_chunks = chunk_url(request.uri());

    match url_chunks.len() {
        1 => block_on(get_all_ingredients(db_pool)),
        _ => bad_request_response()
    }
}

fn handle_post_request(request: &Request<Vec<u8>>, db_pool: &PgPool, authorization: &Authorization) -> Response<Vec<u8>> {
    let url_chunks = chunk_url(request.uri());
    if url_chunks.len() != 1 {
        log::info!("Bad request - POST ingredient request contained a sub-path");
        return bad_request_response()
    }

    match authorization.authenticate_request(request) {
        Ok(_) => (),
        Err(_) => {
            return unauthorized_response();
        }
    }

    let handle_request_result = block_on(post_ingredient::handle_post_ingredient_request(request, db_pool));
    let post_ingredient_response_data = match handle_request_result {
        Ok(response_data) => response_data,
        Err(err) => {
            log::info!("Failed to handle post request: {}", err);
            return bad_request_response()
        }
    };

    let ingredient_location = "/ingredient/".to_string() + post_ingredient_response_data.id.to_string().as_str();
    http::Response::builder()
        .status(http::status::StatusCode::CREATED)
        .header(http::header::LOCATION, ingredient_location)
        .body(vec![])
        .expect("error building response")
}