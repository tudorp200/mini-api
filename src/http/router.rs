
use crate::concurrency::thread_pool::ThreadPool; 
use crate::http::{request::Request, response::Response};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

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

    fn handle_client(&self, mut stream: TcpStream){
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(0) => break, 
                Ok(n) => n,
                Err(_) => break,
            };

            let request_text = String::from_utf8_lossy(&buffer[..bytes_read]);

            let req = match Request::parse_request(&request_text) {
                Ok(parsed_req) => parsed_req,
                Err(e) => {
                    let error_response = Response::new(400, "Bad Request", "Malformed HTTP request");
                    let _ = stream.write_all(error_response.to_http_string().as_bytes());
                    break;
                }
            };
            
            let route_key = format!("{} {}", req.method, req.path);

            let response = match self.routes.get(&route_key) {
                Some(handler) => handler(req),
                _ => Response::new(404, "Not found", "Page not found")
            };

            if stream.write_all(response.to_http_string().as_bytes()).is_err() {
                break;
            }
        }
        
    }
}
