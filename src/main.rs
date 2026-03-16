pub mod concurrency;
pub mod http;
pub mod traits;

pub mod controllers;
pub mod models;
pub mod repositories;

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Arc;

use models::basket::Basket;
use models::category::Category;
use models::order::Order;
use models::product::Product;

use controllers::base_controller::BaseController;
use controllers::basket_item_controller::BasketItemController;
use controllers::category_controller::CategoryController;
use controllers::order_controller::OrderController;
use controllers::product_controller::ProductController;

use http::router::Router;
use repositories::base_repository::BaseRepository;
use repositories::basket_item_repository::BasketItemRepository;

use crate::controllers::basket_controller::BasketController;

fn define_products_api(
    custom_prod_ctrl: Arc<ProductController>,
    base_prod_ctrl: Arc<BaseController<Product>>,
    app: &mut Router,
) {
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
}

fn define_category_api(
    custom_category_ctrl: Arc<CategoryController>,
    base_category_ctrl: Arc<BaseController<Category>>,
    app: &mut Router,
) {
    let cc = Arc::clone(&custom_category_ctrl);
    app.get("/categories/:id/products", move |req| cc.get_products(req));

    let pc = Arc::clone(&base_category_ctrl);
    app.get("/categories", move |req| pc.get_all(req));

    let pc = Arc::clone(&base_category_ctrl);
    app.get("/categories/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&base_category_ctrl);
    app.post("/categories", move |req| pc.create(req));

    let pc = Arc::clone(&base_category_ctrl);
    app.put("/categories/:id", move |req| pc.update(req));

    let pc = Arc::clone(&base_category_ctrl);
    app.delete("/categories/:id", move |req| pc.delete(req));
}

fn define_basket_api(
    custom_basket_ctrl: Arc<BasketController>,
    base_basket_ctrl: Arc<BaseController<Basket>>,
    app: &mut Router,
) {
    let bc = Arc::clone(&custom_basket_ctrl);
    app.get("/baskets/:id/total", move |req| bc.get_total(req));

    let pc = Arc::clone(&base_basket_ctrl);
    app.get("/baskets", move |req| pc.get_all(req));

    let pc = Arc::clone(&base_basket_ctrl);
    app.get("/baskets/:id", move |req| pc.get_by_id(req));

    let pc = Arc::clone(&base_basket_ctrl);
    app.post("/baskets", move |req| pc.create(req));

    let pc = Arc::clone(&base_basket_ctrl);
    app.put("/baskets/:id", move |req| pc.update(req));

    let pc = Arc::clone(&base_basket_ctrl);
    app.delete("/baskets/:id", move |req| pc.delete(req));
}

fn define_basket_item_api(basket_item_ctrl: Arc<BasketItemController>, app: &mut Router) {
    let bic = Arc::clone(&basket_item_ctrl);
    app.get("/baskets/:id/items", move |req| bic.get_all(req));

    let bic = Arc::clone(&basket_item_ctrl);
    app.post("/baskets/:id/items", move |req| bic.create(req));

    let bic = Arc::clone(&basket_item_ctrl);
    app.put("/baskets/:id/items/:product_id", move |req| bic.update(req));

    let bic = Arc::clone(&basket_item_ctrl);
    app.delete("/baskets/:id/items/:product_id", move |req| bic.delete(req));
}

fn define_order_api(
    custom_order_ctrl: Arc<OrderController>,
    order_controller: Arc<BaseController<Order>>,
    app: &mut Router,
) {
    let coc = Arc::clone(&custom_order_ctrl);
    app.post("/orders/checkout", move |req| coc.checkout(req));

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
}

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

    // REPOSITORIES

    let product_repo = Arc::new(BaseRepository::<Product>::new(pool.clone()));
    let category_repo = Arc::new(BaseRepository::<Category>::new(pool.clone()));
    let basket_item_repo = Arc::new(BasketItemRepository::new(pool.clone()));
    let basket_repo = Arc::new(BaseRepository::<Basket>::new(pool.clone()));
    let order_repo = Arc::new(BaseRepository::<Order>::new(pool.clone()));

    // BASE CONTROLLERS

    let base_prod_ctrl = Arc::new(BaseController::<Product>::new(Arc::clone(&product_repo)));
    let basket_item_ctrl = Arc::new(BasketItemController::new(
        Arc::clone(&basket_item_repo),
        Arc::clone(&product_repo),
    ));
    let base_category_ctrl = Arc::new(BaseController::<Category>::new(category_repo));
    let basket_controller = Arc::new(BaseController::<Basket>::new(Arc::clone(&basket_repo)));
    let order_controller = Arc::new(BaseController::<Order>::new(Arc::clone(&order_repo)));

    // CUSTOM CONTROLLERS

    let custom_prod_ctrl = Arc::new(ProductController::new(Arc::clone(&product_repo)));
    let custom_category_ctrl = Arc::new(CategoryController::new(Arc::clone(&product_repo)));
    let custom_order_ctrl = Arc::new(OrderController::new(
        Arc::clone(&order_repo),
        Arc::clone(&basket_repo),
        Arc::clone(&basket_item_repo),
        Arc::clone(&product_repo),
    ));
    let custom_basket_ctrl = Arc::new(BasketController::new(
        Arc::clone(&basket_item_repo),
        Arc::clone(&product_repo),
    ));

    define_products_api(custom_prod_ctrl, base_prod_ctrl, &mut app);
    define_category_api(custom_category_ctrl, base_category_ctrl, &mut app);
    define_basket_item_api(basket_item_ctrl, &mut app);
    define_basket_api(custom_basket_ctrl, basket_controller, &mut app);
    define_order_api(custom_order_ctrl, order_controller, &mut app);

    app.listen("127.0.0.1:4221");
}
