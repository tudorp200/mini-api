pub mod concurrency; 
pub mod http;
pub mod traits;

pub mod Models;
pub mod Repositories;
pub mod Controllers;

use http::request::Request;
use http::response::Response;
use http::router::Router;


fn get_books(req: Request) -> Response {
    println!("Path Variable ID: {:?}", req.path_params.get("id")); 
    println!("Query Author: {:?}", req.query_params.get("author"));
    Response::new(200, "OK", "logs")
}

fn main() {
    let mut app = Router::new();
    
    app.get("/books/:id", get_books);

    app.listen("127.0.0.1:4221");
}

