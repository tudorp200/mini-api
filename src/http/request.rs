use Result;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers : HashMap<String, String>,
    pub body: String,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: &str, path: &str, body: &str) -> Self {
        Request {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: body.to_string(),
            path_params : HashMap::new(),
            query_params : HashMap::new()
        }
    }

    // GET <path> HTTP Version
    // Host : address
    //
    pub fn parse_request(raw_http_request: &str) -> Result<Request, &str> {
        if let Some((header_part, body_part)) = raw_http_request.split_once("\r\n\r\n") {
            let (headers, method, full_path) = Self::parse_header(header_part)?;
            
            let mut path = full_path.to_string();
            let mut query_params = HashMap::new();
            
            if let Some((base_path, query_str)) = full_path.split_once('?') {
                path = base_path.to_string();
                for pair in query_str.split('&') {
                    if let Some((key, value)) = pair.split_once('=') {
                        query_params.insert(key.to_string(), value.to_string());
                    }
                }
            }

            Ok(Request {
                method: method.to_string(),
                path,
                headers,
                body: body_part.to_string(),
                path_params : HashMap::new(), // will populated by the router
                query_params,
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
