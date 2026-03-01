pub mod concurrency; 
pub mod http;
pub mod traits;

pub mod Models;
pub mod Repositories;
pub mod Controllers;

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Arc;

use http::request::Request;
use http::response::Response;
use http::router::Router;


fn get_books(req: Request) -> Response {
    println!("Path Variable ID: {:?}", req.path_params.get("id")); 
    println!("Query Author: {:?}", req.query_params.get("author"));
    Response::new(200, "OK", "logs")
}

fn main() {

    let manager = PostgresConnectionManager::new(
        "postgres://admin:password123@localhost:5432/db".parse().unwrap(),
        NoTls,
    );
    
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create Postgres connection pool");

    let mut app = Router::new();
    
    app.get("/books/:id", get_books);

    app.listen("127.0.0.1:4221");
}

