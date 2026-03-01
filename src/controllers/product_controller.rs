use crate::http::request::Request;
use crate::http::response::Response;
use crate::models::product::Product;
use crate::repositories::base_repository::BaseRepository;
use std::sync::Arc;

pub struct ProductController {
    repo: Arc<BaseRepository<Product>>,
}

impl ProductController {
    pub fn new(repo: Arc<BaseRepository<Product>>) -> Self {
        Self { repo }
    }

    pub fn get_paginated(&self, req: Request) -> Response {
        let page = req.query_params.get("page")
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(1)
            .max(1);
            
        let limit = req.query_params.get("limit")
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(10)
            .clamp(1, 100);

        let offset = (page - 1) * limit;

        match self.repo.find_all_paginated(limit, offset) {
            Ok(products) => {
                let body = serde_json::to_string(&products).unwrap();
                Response::new(200, "OK", &body)
            }
            Err(e) => Response::new(500, "Internal Server Error", &e),
        }
    }
}
