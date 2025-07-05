// https://thepacketgeek.com/rust/tcpstream/lines-codec/

use crate::http::{Header, RequestLine, StatusLine};
use http::HeaderName;
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use anyhow::Result;

pub struct HttpCodec {
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl HttpCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with `try_clone()`
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_response(&mut self, response: http::Response<Vec<u8>>) -> io::Result<()> {
        let status_line = StatusLine::new(
            response.status(),
            response.status().canonical_reason().unwrap_or("").to_string()
        );
        let bytes: Vec<u8> = status_line.into();
        self.writer.write_all(bytes.as_slice())?;

        for kvp in response.headers() {
            // TODO - Not all header values are strings!
            let header = format!{"{}: {}\r\n", kvp.0, kvp.1.to_str().unwrap_or("")};
            self.writer.write_all(header.as_bytes())?;
        }

        self.writer.write_all("\r\n".as_bytes())?;
        self.writer.write_all(response.body())?;

        Ok(())
    }

    pub fn receive_request(&mut self) -> Result<http::Request<Vec<u8>>> {
        let mut buffer = String::new();

        // Request line
        _ = self.reader.read_line(&mut buffer)?;
        let request_line = RequestLine::try_from(buffer.trim_end_matches("\r\n").to_string())?;
        buffer.clear();

        // Headers
        let mut header_map = HashMap::new();
        _ = self.reader.read_line(&mut buffer)?;
        while buffer != "\r\n" {
            let header = Header::try_from(buffer.trim_end_matches("\r\n").to_lowercase().to_string())?;
            header_map.insert(header.key, header.value);

            buffer.clear();
            _ = self.reader.read_line(&mut buffer)?;
        }

        let mut body = vec![];
        let content_type_header = HeaderName::from_static("content-type");
        let content_length_header = HeaderName::from_static("content-length");
        if let Some(content_type) = header_map.get(&content_type_header) {
            if let Some(content_length) = header_map.get(&content_length_header) {
                let content_type = content_type.to_str()?;
                let content_length = content_length.to_str()?.parse()?;
                body = self.read_body(content_type, content_length)?;
            }
        }

        let mut builder = http::request::Builder::new()
            .method(request_line.method)
            .uri(request_line.request_target)
            .version(request_line.protocol);

        if let Some(headers) = builder.headers_mut() {
            for (key, value) in header_map {
                headers.insert(key, value);
            }
        }
        
        let request = builder.body(body)?;

        Ok(request)
    }

    fn read_body(&mut self, _content_type: &str, content_length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; content_length];
        self.reader.read_exact(buffer.as_mut_slice())?;
        Ok(buffer)
    }
}