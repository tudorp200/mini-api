pub mod concurrency; 
pub mod http;
pub mod traits;

pub mod models;
pub mod repositories;
pub mod controllers;

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Arc;

use models::product::Product;
use models::category::Category;
use models::basket::Basket;
use models::order::Order;

use controllers::base_controller::BaseController;
use controllers::product_controller::ProductController;
use controllers::category_controller::CategoryController;

use http::router::Router;
use repositories::base_repository::BaseRepository;

fn main() {
    let mut app = Router::new();

    let manager = PostgresConnectionManager::new(
        "postgres://admin:admin@localhost:5432/db".parse().unwrap(),
        NoTls,
    );
    
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create Postgres connection pool");

    // ==========================================
    // PRODUCTS API
    // ==========================================
    
    let product_repo = Arc::new(BaseRepository::<Product>::new(pool.clone()));

    let base_prod_ctrl = Arc::new(BaseController::<Product>::new(Arc::clone(&product_repo)));
    let custom_prod_ctrl = Arc::new(ProductController::new(Arc::clone(&product_repo)));

    let pc = Arc::clone(&custom_prod_ctrl);
    app.get("/products", move |req| pc.get_paginated(req));

    let pc = Arc::clone(&base_prod_ctrl);
    app.get("/products/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&base_prod_ctrl);
    app.post("/products", move |req| pc.create(req));

    let pc = Arc::clone(&base_prod_ctrl);
    app.put("/products/:id", move |req| pc.update(req));

    let pc = Arc::clone(&base_prod_ctrl);
    app.delete("/products/:id", move |req| pc.delete(req));

    let pc = Arc::clone(&base_prod_ctrl);
    app.get("/products/:id", move |req| pc.get_by_id(req));

    // ==========================================
    // CATEGORY API
    // ==========================================

    let category_repo = Arc::new(BaseRepository::<Category>::new(pool.clone()));

    let custom_category_ctrl = Arc::new(CategoryController::new(Arc::clone(&product_repo)));
    let cc = Arc::clone(&custom_category_ctrl);
    app.get("/categories/:id/products", move |req| cc.get_products(req));

    let category_controller = Arc::new(BaseController::<Category>::new(category_repo));

    let pc = Arc::clone(&category_controller);
    app.get("/categories", move |req| pc.get_all(req));

    let pc = Arc::clone(&category_controller);
    app.get("/categories/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&category_controller);
    app.post("/categories", move |req| pc.create(req));

    let pc = Arc::clone(&category_controller);
    app.put("/categories/:id", move |req| pc.update(req));

    let pc = Arc::clone(&category_controller);
    app.delete("/categories/:id", move |req| pc.delete(req));

    // ==========================================
    // BASKET API
    // ==========================================

    let basket_repo = Arc::new(BaseRepository::<Basket>::new(pool.clone()));
    let basket_controller = Arc::new(BaseController::<Basket>::new(basket_repo));

    let pc = Arc::clone(&basket_controller);
    app.get("/baskets", move |req| pc.get_all(req));

    let pc = Arc::clone(&basket_controller);
    app.get("/baskets/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&basket_controller);
    app.post("/baskets", move |req| pc.create(req));

    let pc = Arc::clone(&basket_controller);
    app.put("/baskets/:id", move |req| pc.update(req));

    let pc = Arc::clone(&basket_controller);
    app.delete("/baskets/:id", move |req| pc.delete(req));

    // ==========================================
    // ORDER API
    // ==========================================

    let order_repo = Arc::new(BaseRepository::<Order>::new(pool.clone()));
    let order_controller = Arc::new(BaseController::<Order>::new(order_repo));

    let pc = Arc::clone(&order_controller);
    app.get("/orders", move |req| pc.get_all(req));

    let pc = Arc::clone(&order_controller);
    app.get("/orders/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&order_controller);
    app.post("/orders", move |req| pc.create(req));

    let pc = Arc::clone(&order_controller);
    app.put("/orders/:id", move |req| pc.update(req));

    let pc = Arc::clone(&order_controller);
    app.delete("/orders/:id", move |req| pc.delete(req));

    app.listen("127.0.0.1:4221");
}


