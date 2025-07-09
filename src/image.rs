use std::fs;
use std::path::{Path, PathBuf};
use http::{Method, Request, Response};
use crate::Config;
use crate::http::responses::bad_request_response;

pub fn can_handle_request(request: &Request<Vec<u8>>) -> bool {
    request.uri().path().starts_with("/image/")
}

pub fn handle_request(request: Request<Vec<u8>>, config: &Config) -> anyhow::Result<Response<Vec<u8>>> {
    match *request.method() {
        Method::GET => handle_get_request(&request, config),
        // Method::PUT => handle_put_request(&request, config),
        _ => invalid_response(http::StatusCode::METHOD_NOT_ALLOWED)
    }
}

// TODO Rewrite this route
fn handle_get_request(request: &Request<Vec<u8>>, config: &Config) -> anyhow::Result<Response<Vec<u8>>> {
    let bad_get_strings = vec!["..", "$", "~"];
    for bad_string in bad_get_strings {
        if request.uri().path().contains(bad_string) {
            return Ok(bad_request_response())
        }
    }

    if request.uri() == "/image" || request.uri() == "/image/" {
        let response = http::Response::builder()
            .status(http::status::StatusCode::OK)
            .header("Content-Length", 0)
            // TODO Other format types
            .header("Content-Type", "image/jpg")
            .body(vec![])?;
        return Ok(response)
    }

    let image_path = resolve_image_path(request.uri(), &config.image_folder)?;

    if !image_path.exists() {
        return invalid_response(http::StatusCode::NOT_FOUND);
    }

    println!("Requested image path: {}", image_path.display());
    let data = fs::read(&image_path)?;

    let response = http::Response::builder()
        .status(http::status::StatusCode::OK)
        .header("Content-Length", data.len())
        // TODO Other format types
        .header("Content-Type", "image/jpg")
        .body(data)?;
    Ok(response)
}

fn handle_put_request(request: &Request<Vec<u8>>, config: &Config) -> anyhow::Result<Response<Vec<u8>>> {
    let image_path = resolve_image_path(request.uri(), &config.image_folder)?;
    let created = !image_path.exists();

    if let Some(parent) = image_path.parent() {
        if !parent.exists() { fs::create_dir_all(parent)? }
    }

    fs::write(image_path, request.body())?;

    let response_status = match created {
        true => http::StatusCode::CREATED,
        false => http::StatusCode::NO_CONTENT
    };

    let response = http::Response::builder()
        .status(response_status)
        .body(vec![])?;

    Ok(response)
}

fn invalid_response(status_code: http::status::StatusCode) -> anyhow::Result<Response<Vec<u8>>> {
    Ok(http::Response::builder().status(status_code).body(vec![])?)
}

fn resolve_image_path(uri: &http::Uri, image_folder: &Path) -> anyhow::Result<PathBuf> {
    let trim_length = "/image/".len();
    let mut root = PathBuf::from(image_folder);
    root.push(&uri.path()[trim_length..]);
    Ok(root)
}