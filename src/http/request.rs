use Result;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers : HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(method: &str, path: &str, body: &str) -> Self {
        Request {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: body.to_string(),
        }
    }

    // GET <path> HTTP Version
    // Host : address
    //
    pub fn parse_request(raw_http_request: &str) -> Result<Request, &str> {
        if let Some((header_part, body_part)) = raw_http_request.split_once("\r\n\r\n") {
            let (headers, method, path) = Self::parse_header(header_part)?;

            Ok(Request {
                method: method.to_string(),
                path: path.to_string(),
                headers,
                body: body_part.to_string(),
            })
        } else{ 
            return Err("Missing double CRLF")
        }
    }
    
    // retursn a Result<(method , path), error> 
    fn parse_header<'a>(header_part : &'a str) -> Result<(HashMap<String, String>, &'a str, &'a str), &'static str> {
        let mut headers = HashMap::new();
        let mut method = "";
        let mut path = "";

        for (i, line) in header_part.lines().enumerate(){
            
            if line.is_empty() {
                break;
            }

            if i == 0 {
                let mut tokens = line.split_whitespace();
                method = tokens.next().ok_or("Missing method")?;
                path = tokens.next().ok_or("missing path")?;
            } else if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }

        }
        return Ok((headers, method, path))
    }

}
