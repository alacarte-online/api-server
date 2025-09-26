use http::Response;
use serde::Serialize;

pub fn empty_ok() -> Response<Vec<u8>> {
    http::Response::builder()
        .status(http::status::StatusCode::OK)
        .body(vec![])
        .expect("error building response")
}

pub fn json_ok(json: String) -> Response<Vec<u8>> {
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

pub fn created(location: String) -> Response<Vec<u8>> {
    http::Response::builder()
        .status(http::status::StatusCode::CREATED)
        .header(http::header::LOCATION, location)
        .body(vec![])
        .expect("error building response")
}

pub fn internal_server_error_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::INTERNAL_SERVER_ERROR).body(Vec::new()).expect("error building internal server error response")
}

pub fn bad_request_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::BAD_REQUEST).body(Vec::new()).expect("error building bad request response")
}

pub fn bad_request_response_with_message(message: &str) -> Response<Vec<u8>> {
    let message = message.to_string();
    let message = ResponseMessage { message };
    let message = serde_json::to_vec(&message).unwrap_or({
        log::error!("Failed to serialize ResponseMessage '{}'", message.message);
        Vec::new()
    });
    http::Response::builder().status(http::status::StatusCode::BAD_REQUEST).body(message).expect("error building bad request response")
}

#[derive(Debug, Serialize)]
struct ResponseMessage {
    message: String,
}

pub fn not_found_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::NOT_FOUND).body(Vec::new()).expect("error building not found response")
}

pub fn method_not_allowed_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::METHOD_NOT_ALLOWED).body(Vec::new()).expect("error building method not allowed response")
}

pub fn unauthorized_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::UNAUTHORIZED).body(Vec::new()).expect("error building unauthorized response")
}