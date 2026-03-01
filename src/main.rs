pub mod concurrency; 
pub mod http;
pub mod traits;

pub mod models;
pub mod repositories;
pub mod controllers;

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Arc;

use models::product::Product;
use controllers::product_controller::ProductController;

use http::router::Router;
use repositories::base_repository::BaseRepository;

fn main() {

    let manager = PostgresConnectionManager::new(
        "postgres://admin:admin@localhost:5432/db".parse().unwrap(),
        NoTls,
    );
    
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create Postgres connection pool");

    let product_repo = Arc::new(BaseRepository::<Product>::new(pool));
    let product_controller = Arc::new(ProductController::new(product_repo));

    let mut app = Router::new();

    let pc_get_all = Arc::clone(&product_controller);
    app.get("/products", move |req| pc_get_all.get_all(req));

    let pc_get_id = Arc::clone(&product_controller);
    app.get("/products/:id", move |req| pc_get_id.get_by_id(req));

    let pc_create = Arc::clone(&product_controller);
    app.post("/products", move |req| pc_create.create(req));

    app.listen("127.0.0.1:4221");
}

