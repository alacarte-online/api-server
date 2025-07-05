use http::Response;

pub fn internal_server_error_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::INTERNAL_SERVER_ERROR).body(Vec::new()).expect("error building internal server error response")
}

pub fn bad_request_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::BAD_REQUEST).body(Vec::new()).expect("error building bad request response")
}

pub fn not_found_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::NOT_FOUND).body(Vec::new()).expect("error building not found response")
}

pub fn method_not_allowed_response() -> Response<Vec<u8>> {
    http::Response::builder().status(http::status::StatusCode::METHOD_NOT_ALLOWED).body(Vec::new()).expect("error building method not allowed response")
}