use crate::http::request::Request;
use crate::http::response::Response;

use crate::models::order::Order;
use crate::models::product::Product;

use crate::traits::Repository;

use crate::repositories::base_repository::BaseRepository;
use crate::repositories::basket_item_repository::BasketItemRepository;

use crate::models::basket::Basket;

use std::sync::Arc;

pub struct OrderController {
    order_repo: Arc<BaseRepository<Order>>,
    basket_repo: Arc<BaseRepository<Basket>>,
    basket_item_repo: Arc<BasketItemRepository>,
    product_repo: Arc<BaseRepository<Product>>,
}

impl OrderController {
    pub fn new(
        order_repo: Arc<BaseRepository<Order>>,
        basket_repo: Arc<BaseRepository<Basket>>,
        basket_item_repo: Arc<BasketItemRepository>,
        product_repo: Arc<BaseRepository<Product>>,
    ) -> Self {
        Self {
            order_repo,
            basket_repo,
            basket_item_repo,
            product_repo,
        }
    }

    // POST /orders
    pub fn checkout(&self, req: Request) -> Response {
        println!("Received checkout request for order: {:?}", req.body);
        let body: serde_json::Value = serde_json::from_str(&req.body).unwrap_or_default();
        let basket_id = body["basket_id"].as_i64().unwrap_or(0) as i32;

        let mut basket = match self.basket_repo.find_by_id(basket_id) {
            Ok(b) => b,
            Err(_) => return Response::new(404, "Not Found", "Basket not found"),
        };

        if basket.status == "Checked Out" {
            return Response::new(
                400,
                "Bad Request",
                "This basket has already been checked out!",
            );
        }

        let items = match self.basket_item_repo.find_by_basket_id(basket_id) {
            Ok(items) if !items.is_empty() => items,
            Ok(_) => return Response::new(400, "Bad Request", "Basket is empty"),
            Err(e) => return Response::new(500, "Internal Server Error", &e),
        };

        let mut total_price = 0.0;
        let mut products_to_update = Vec::new();

        for item in items {
            let mut product = match self.product_repo.find_by_id(item.product_id) {
                Ok(p) => p,
                Err(_) => return Response::new(500, "Internal Error", "Missing product reference"),
            };

            if product.stock < item.quantity {
                return Response::new(
                    400,
                    "Bad Request",
                    &format!("Stock ran out for {}", product.name),
                );
            }

            total_price += product.price * (item.quantity as f32);
            product.stock -= item.quantity; // Subtract stock
            products_to_update.push(product);
        }

        basket.status = "Checked Out".to_string();
        if let Err(e) = self.basket_repo.update(&basket) {
            return Response::new(500, "Internal Server Error", &e);
        }

        let new_order = Order {
            id: 0,
            basket_id,
            total_paid: total_price,
            status: "COMPLETED".to_string(),
        };
        if let Err(e) = self.order_repo.save(&new_order) {
            return Response::new(500, "Internal Server Error", &e);
        }

        for p in products_to_update {
            let _ = self.product_repo.update(&p);
        }

        Response::new(
            201,
            "Created",
            &format!("Order placed successfully! Total: ${:.2}", total_price),
        )
    }
}
