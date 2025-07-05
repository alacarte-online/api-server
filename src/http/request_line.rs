use std::str::FromStr;
use anyhow::anyhow;

pub struct RequestLine {
    pub method :http::Method,
    pub request_target: http::Uri,
    pub protocol: http::Version,
}

impl TryFrom<String> for RequestLine {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut iterator = value.splitn(3, ' ');

        let method = iterator.next().ok_or(anyhow!("failed to get method name"))?;
        let method = http::Method::from_str(method).map_err(|_| anyhow!(format!("failed to get method name: {}", method)))?;

        let uri = iterator.next().ok_or(anyhow!("failed to get uri"))?;
        let uri = http::Uri::from_str(uri).map_err(|_| anyhow!(format!("failed to get uri: {}", uri)))?;

        let version = iterator.next().ok_or(anyhow!("failed to get version"))?;
        if version != "HTTP/1.1" {
            return Err(anyhow!(format!("unsupported request version: {}. only http1.1 is supported", version)));
        }

        Ok(Self {
            method,
            request_target: uri,
            protocol: http::Version::HTTP_11,
        })
    }
}