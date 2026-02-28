use crate::http::{request::Request, response::Response};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

type Handler = fn(Request) -> Response;

// METHOD + PATH : Actual implemented method
pub struct Router {
    pub routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        print!("get");
        self.routes.insert(format!("GET {}", path), handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes.insert(format!("POST {}", path), handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.routes.insert(format!("PUT {}", path), handler);
    }
    
    pub fn patch(&mut self, path: &str, handler: Handler) {print!("client connected");
        self.routes.insert(format!("PATCH {}", path), handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.routes.insert(format!("DELETE {}", path), handler);
    }

    pub fn listen(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        for mut stream in listener.incoming().flatten() {
            self.handle_client(&mut stream);
        }
    }

    fn handle_client(&self, stream: &mut TcpStream){
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request_text = String::from_utf8_lossy(&buffer);

        let req = Request::parse_request(&request_text).unwrap();

        let route_key = format!("{} {}", req.method, req.path);

        let response = match self.routes.get(&route_key) {
            Some(handler) => handler(req),
            _ => Response::new(404, "Not found", "Page not found")
        };

        stream.write_all(response.to_http_string().as_bytes()).unwrap();
    }
}
