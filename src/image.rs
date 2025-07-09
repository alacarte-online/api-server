use std::fs;
use std::path::{Path, PathBuf};
use http::{Method, Request, Response};
use crate::Config;
use crate::http::responses;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/image/")
}

pub fn handle_request(request: Request<Vec<u8>>, config: &Config) -> Response<Vec<u8>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, config),
        // Method::PUT => handle_put_request(&request, config),
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
    let image_data = fs::read(image_path);
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

// fn handle_put_request(request: &Request<Vec<u8>>, config: &Config) -> anyhow::Result<Response<Vec<u8>>> {
//     let image_path = resolve_image_path(request.uri(), &config.image_folder)?;
//     let created = !image_path.exists();
//
//     if let Some(parent) = image_path.parent() {
//         if !parent.exists() { fs::create_dir_all(parent)? }
//     }
//
//     fs::write(image_path, request.body())?;
//
//     let response_status = match created {
//         true => http::StatusCode::CREATED,
//         false => http::StatusCode::NO_CONTENT
//     };
//
//     let response = http::Response::builder()
//         .status(response_status)
//         .body(vec![])?;
//
//     Ok(response)
// }

fn resolve_image_path(uri: &http::Uri, image_folder: &Path) -> PathBuf {
    let trim_length = "/image/".len();
    let mut root = PathBuf::from(image_folder);
    root.push(&uri.path()[trim_length..]);
    root
}