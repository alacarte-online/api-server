use anyhow::anyhow;

pub struct Header {
    pub key: http::HeaderName,
    pub value: http::HeaderValue,
}

impl TryFrom<String> for Header {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut iterator = value.splitn(2, ':');

        let key = iterator.next().ok_or(anyhow!("failed to get header name"))?;
        let key = http::HeaderName::from_bytes(key.as_bytes()).map_err(|e| anyhow!(e.to_string()))?;

        let value = iterator.next().ok_or(anyhow!("failed to get header value"))?;
        let value = http::HeaderValue::from_bytes(value.trim().as_bytes()).map_err(|e| anyhow!(e.to_string()))?;

        Ok(Self { key, value })
    }
}