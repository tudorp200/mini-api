use crate::http::request::Request;
use crate::http::response::Response;

use crate::models::product::Product;
use crate::traits::Repository;

use crate::repositories::base_repository::BaseRepository;
use crate::repositories::basket_item_repository::BasketItemRepository;

use std::sync::Arc;
pub struct BasketController {
    basket_item_repo: Arc<BasketItemRepository>,
    product_repo: Arc<BaseRepository<Product>>,
}

impl BasketController {
    pub fn new(
        basket_item_repo: Arc<BasketItemRepository>,
        product_repo: Arc<BaseRepository<Product>>,
    ) -> Self {
        Self {
            basket_item_repo,
            product_repo,
        }
    }

    // GET /baskets/:id/total
    pub fn get_total(&self, req: Request) -> Response {
        let basket_id = req
            .path_params
            .get("id")
            .and_then(|id| id.parse::<i32>().ok())
            .unwrap_or(0);

        let items = self
            .basket_item_repo
            .find_by_basket_id(basket_id)
            .unwrap_or_default();

        let mut total = 0.0;

        for item in items {
            if let Ok(p) = self.product_repo.find_by_id(item.product_id) {
                total += p.price * (item.quantity as f32);
            }
        }

        let body = format!(r#"{{"basket_id": {}, "total": {:.2}}}"#, basket_id, total);
        Response::new(200, "OK", &body)
    }
}
