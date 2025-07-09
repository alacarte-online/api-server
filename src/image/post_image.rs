use std::fmt::{Debug, Display, Formatter};
use std::io::Cursor;
use std::path::PathBuf;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use crate::Config;
use http::{Request, Response};
use image::{DynamicImage, ImageReader};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::http::responses;

const IMAGE_HEADER: &str = "data:image/jpeg;base64,";

#[derive(Debug, Serialize, Deserialize)]
pub struct PostImageData {
    pub data: String
}

pub fn handle_post_request(request: &Request<Vec<u8>>, config: &Config) -> Response<Vec<u8>> {
    log::debug!("Handling POST request for {}", request.uri());
    let post_image_request: PostImageData = match serde_json::from_slice(request.body()) {
        Ok(data) => data,
        Err(error) => {
            log::error!("Failed to parse POST image request: {}", error);
            return responses::bad_request_response_with_message("Invalid request body");
        }
    };

    let image = match create_image_from_data(post_image_request.data) {
        Ok(image) => image,
        Err(error) => {
            log::info!("Failed to create image: {}", error);
            return match error {
                ImageCreationError::BadImageHeaderError => {
                    responses::bad_request_response_with_message(&format!("{}", error))
                },
                ImageCreationError::ImageDecodingError(_) => {
                    responses::internal_server_error_response()
                }
            }
        }
    };

    let image_file_name = create_random_file_name();
    let mut image_path = config.image_folder.clone();
    image_path.push(image_file_name.clone());
    image_path.set_extension("jpg");

    match write_image_to_file(image, &image_path) {
        Ok(_) => {
            log::trace!("Successfully wrote image to {}", image_path.to_string_lossy());
            let created_location = String::from("/image/") + &image_path.file_name().expect("image file has no name").to_string_lossy();
            responses::created(created_location)
        },
        Err(error) => {
            log::error!("Failed to write image to {}: {}", image_path.to_string_lossy(), error);
            responses::internal_server_error_response()
        }
    }
}

fn create_random_file_name() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const NAME_LEN: usize = 10;
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    std::iter::repeat_with(one_char).take(NAME_LEN).collect()
}

fn create_image_from_data(base64_image_data: String) -> Result<DynamicImage, ImageCreationError> {

    if !&base64_image_data.starts_with(IMAGE_HEADER) {
        return Err(ImageCreationError::BadImageHeaderError);
    }

    match decode_image_data(base64_image_data) {
        Ok(image) => Ok(image),
        Err(err) => Err(ImageCreationError::ImageDecodingError(err)),
    }
}

fn decode_image_data(base64_image_data: String) -> anyhow::Result<DynamicImage> {
    let base_64_image = &base64_image_data[IMAGE_HEADER.len()..];
    let image_data = BASE64_STANDARD.decode(base_64_image)?;
    let img = ImageReader::new(Cursor::new(image_data)).with_guessed_format()?.decode()?;
    Ok(img)
}

#[derive(Debug)]
enum ImageCreationError {
    BadImageHeaderError,
    ImageDecodingError(anyhow::Error),
}

impl Display for ImageCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageCreationError::BadImageHeaderError =>
                f.write_fmt(format_args!("Image data must start with '{}'", IMAGE_HEADER)),
            ImageCreationError::ImageDecodingError(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for ImageCreationError {}

fn write_image_to_file(image: DynamicImage, file: &PathBuf) -> anyhow::Result<()> {
    if let Some(parent) = file.parent() {
        if !parent.exists() { std::fs::create_dir_all(parent)? }
    }

    let mut image_file = std::fs::File::create(file)?;
    image.write_to(&mut image_file, image::ImageFormat::Jpeg)?;
    Ok(())
}