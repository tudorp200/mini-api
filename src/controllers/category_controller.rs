use crate::http::request::Request;
use crate::http::response::Response;
use crate::models::product::Product;
use crate::repositories::base_repository::BaseRepository;
use std::sync::Arc;

pub struct CategoryController {
    product_repo: Arc<BaseRepository<Product>>,
}

impl CategoryController {
    pub fn new(product_repo: Arc<BaseRepository<Product>>) -> Self {
        Self { product_repo }
    }

    pub fn get_products(&self, req: Request) -> Response {
        if let Some(id_str) = req.path_params.get("id") {
            if let Ok(category_id) = id_str.parse::<i32>() {
                
                let min_price = req.query_params.get("min_price").and_then(|v| v.parse::<f32>().ok());
                let max_price = req.query_params.get("max_price").and_then(|v| v.parse::<f32>().ok());
                let page = req.query_params.get("page").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
                let limit = req.query_params.get("limit").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).clamp(1, 100);
                
                let offset = (page - 1) * limit;

                match self.product_repo.find_by_category_with_filters(category_id, min_price, max_price, limit, offset) {
                    Ok(products) => {
                        let body = serde_json::to_string(&products).unwrap();
                        Response::new(200, "OK", &body)
                    }
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }

            } else {
                Response::new(400, "Bad Request", "Invalid Category ID format")
            }
        } else {
            Response::new(400, "Bad Request", "Missing Category ID")
        }
    }
}