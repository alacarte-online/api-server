mod post_image;

use std::path::{Path, PathBuf};
use http::{Method, Request, Response};
use crate::authorization::Authorization;
use crate::Config;
use crate::http::responses;
use crate::http::responses::unauthorized_response;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    match *request.method() {
        Method::GET => request.uri().path().starts_with("/image/"),
        Method::POST => request.uri().path() == "/image",
        _ => false,
    }

}

pub fn handle_request(request: Request<Vec<u8>>, config: &Config, auth_handler: &Authorization) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, config),
        Method::POST => handle_post_request(&request, config, auth_handler),
        _ => responses::method_not_allowed_response()
    }
}

fn handle_get_request(request: &Request<Vec<u8>>, config: &Config) -> Response<Vec<u8>> {
    let bad_get_strings = vec!["..", "$", "~"];
    for bad_string in bad_get_strings {
        if request.uri().path().contains(bad_string) {
            log::info!("Bad request - uri contained forbidden string '{}'", bad_string);
            return responses::bad_request_response()
        }
    }

    let image_path = resolve_image_path(request.uri(), &config.image_folder);
    log::debug!("Requested image path '{}'", image_path.display());
    if !image_path.exists() || !image_path.is_file() {
        log::debug!("Returning not found response");
        return responses::not_found_response();
    }

    log::debug!("Loading image data");
    let image_data = std::fs::read(image_path);
    match image_data {
        Ok(image_data) => {
            log::debug!("Image data ok");
            http::Response::builder()
                .status(http::status::StatusCode::OK)
                .header("Content-Length", image_data.len())
                // TODO Other format types
                .header("Content-Type", "image/jpg")
                .body(image_data)
                .expect("error building response")
        },
        Err(err) => {
            log::debug!("Error loading image data - {}", err);
            responses::internal_server_error_response()
        }
    }
}

fn handle_post_request(request: &Request<Vec<u8>>, config: &Config, auth_handler: &Authorization) -> Response<Vec<u8>> {
    match auth_handler.authenticate_request(request) {
        Ok(_) => (),
        Err(_) => {
            return unauthorized_response();
        }
    }

    post_image::handle_post_request(request, config)
}

fn resolve_image_path(uri: &http::Uri, image_folder: &Path) -> PathBuf {
    let trim_length = "/image/".len();
    let mut root = PathBuf::from(image_folder);
    root.push(&uri.path()[trim_length..]);
    root
}