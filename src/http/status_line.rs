pub struct StatusLine {
    pub protocol: http::Version,
    pub status_code: http::StatusCode,
    pub status_text: String
}

impl StatusLine {
    pub fn new(status_code: http::StatusCode, status_text: String) -> StatusLine {
        StatusLine { protocol: http::Version::HTTP_11, status_code, status_text }
    }
}

impl From<StatusLine> for String {
    fn from(value: StatusLine) -> Self {
        format!("{} {} {}\r\n", "HTTP/1.1", value.status_code.as_str(), value.status_text)
    }
}

impl From<StatusLine> for Vec<u8> {
    fn from(value: StatusLine) -> Self {
        String::from(value).into_bytes()
    }
}