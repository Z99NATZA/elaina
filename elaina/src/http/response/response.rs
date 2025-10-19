use std::net::TcpStream;

use crate::http::StatusCode;
use std::io::Write;

pub struct Response {
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Response {
            status,
            body,
        }
    }
}

impl Response {
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let status = self.status();
        let empty_body = String::new();
        let body = self.body().unwrap_or(&empty_body);
        write!(stream, "HTTP/1.1 {:?}\r\n\r\n{}", status, body)?;
        
        Ok(())
    }
}
