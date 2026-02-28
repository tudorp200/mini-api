pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub body: String,
}

impl Response {
    pub fn new(code: u16, text: &str, body: &str) -> Self {
        Response {
            status_code: code,
            status_text: text.to_string(),
            body: body.to_string(),
        }
    }

    pub fn to_http_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            self.body.len(),
            self.body
        )
    }
}
