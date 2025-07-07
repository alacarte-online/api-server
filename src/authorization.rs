use std::fs;
use std::path::PathBuf;
use http::Request;

pub struct Authorization {
    auth_file: PathBuf,
}

impl Authorization {
    pub fn new(auth_file: PathBuf) -> Self {
        Self { auth_file }
    }

    pub fn authenticate_request(&self, request: &Request<Vec<u8>>) -> anyhow::Result<()> {
        log::info!("Authorizing request");
        let expected_token = fs::read_to_string(&self.auth_file)?.trim().to_string();
        log::debug!("Loaded auth token");
        let token_header = request.headers().get("Authorization");
        let token_header = match token_header {
            Some(header) => header,
            None => {
                log::info!("Missing authorization header");
                anyhow::bail!("Missing Authorization header")
            },
        };

        if expected_token != token_header.to_str()? {
            log::info!("Bad authorization header");
            anyhow::bail!("Invalid token");
        }

        log::info!("Request authorized");
        Ok(())
    }
}