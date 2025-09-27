mod get_all_ingredients;

use futures::executor::block_on;
use http::{Method, Request, Response};
use sqlx::PgPool;
use crate::authorization::Authorization;
use crate::http::responses::{bad_request_response, method_not_allowed_response};
use crate::ingredient::get_all_ingredients::get_all_ingredients;
use crate::recipe::chunk_url;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/ingredient/") || request.uri().path() == "/ingredient"
}

pub fn handle_request(request: Request<Vec<u8>>, db_pool: &PgPool, _auth_handler: &Authorization) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, db_pool),
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