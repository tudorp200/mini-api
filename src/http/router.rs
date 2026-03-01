
use crate::concurrency::thread_pool::ThreadPool; 
use crate::http::{request::Request, response::Response};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

type Handler = fn(Request) -> Response;

// METHOD + PATH : Actual implemented method
pub struct Router {
    pub routes: Vec<(String, String, Handler)>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.routes.push(("GET".to_string(), path.to_string(), handler));
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes.push(("POST".to_string(), path.to_string(), handler));
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.routes.push(("PUT".to_string(), path.to_string(), handler));
    }

    pub fn patch(&mut self, path: &str, handler: Handler) {
        self.routes.push(("PATCH".to_string(), path.to_string(), handler));
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.routes.push(("DELETE".to_string(), path.to_string(), handler));
    }

    pub fn listen(self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(6);

        let shared_router = Arc::new(self);

        for stream in listener.incoming().flatten() {
            
            let router_clone = Arc::clone(&shared_router);
            pool.execute(move || {
                router_clone.handle_client(stream);
                Ok(()) 
                
            }).unwrap();
        }
    }

    fn match_route(template: &str, path: &str) -> Option<HashMap<String, String>> {
        let template_parts: Vec<&str> = template.split('/').filter(|s| !s.is_empty()).collect();
        let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if template_parts.len() != path_parts.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (t, p) in template_parts.iter().zip(path_parts.iter()) {
            if t.starts_with(':') {
                params.insert(t[1..].to_string(), p.to_string());
            } else if t != p {
                return None;
            }
        }
        Some(params)
    }

    fn handle_client(&self, mut stream: TcpStream){
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(0) => break, 
                Ok(n) => n,
                Err(_) => break,
            };

            let request_text = String::from_utf8_lossy(&buffer[..bytes_read]);

            let mut req = match Request::parse_request(&request_text) {
                Ok(parsed_req) => parsed_req,
                Err(e) => {
                    let error_response = Response::new(400, "Bad Request", "Malformed HTTP request");
                    let _ = stream.write_all(error_response.to_http_string().as_bytes());
                    break;
                }
            };
            
            let mut response = Response::new(404, "Not Found", "Page not found");

            for (route_method, route_path, handler) in &self.routes {
                if req.method == *route_method {
                    if let Some(params) = Self::match_route(route_path, &req.path) {
                        req.path_params = params;
                        response = handler(req);
                        break;
                    }
                }
            }
            if stream.write_all(response.to_http_string().as_bytes()).is_err() {
                break;
            }
        }
    }
}
