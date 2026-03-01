use crate::http::request::Request;
use crate::http::response::Response;

use crate::models::basket_item::BasketItem;
use crate::models::product::Product;

use crate::repositories::base_repository::BaseRepository;
use crate::repositories::basket_item_repository::BasketItemRepository;

use std::sync::Arc;

use crate::traits::Repository;

pub struct BasketItemController {
    repo: Arc<BasketItemRepository>,
    product_repo: Arc<BaseRepository<Product>>,
}

impl BasketItemController {
    pub fn new(repo: Arc<BasketItemRepository>, product_repo : Arc<BaseRepository<Product>>) -> Self {
        Self { repo, product_repo }
    }

    // GET /baskets/:id/items 
    pub fn get_all(&self, req: Request) -> Response {
        if let Some(basket_id) = req.path_params.get("id").and_then(|id| id.parse::<i32>().ok()) {
            match self.repo.find_by_basket_id(basket_id) {
                Ok(items) => {
                    let body = serde_json::to_string(&items).unwrap();
                    Response::new(200, "OK", &body)
                }
                Err(e) => Response::new(500, "Internal Server Error", &e),
            }
        } else {
            Response::new(400, "Bad Request", "Missing or invalid Basket ID")
        }
    }

    // POST /baskets/:id/items 
    pub fn create(&self, req: Request) -> Response {
        let basket_id = match req.path_params.get("id").and_then(|id| id.parse::<i32>().ok()) {
            Some(id) => id,
            None => return Response::new(400, "Bad Request", "Invalid Basket ID"),
        };

        match serde_json::from_str::<serde_json::Value>(&req.body) {
            Ok(json) => {
                let product_id = json["product_id"].as_i64().unwrap_or(0) as i32;
                let quantity = json["quantity"].as_i64().unwrap_or(1) as i32;

                match self.product_repo.find_by_id(product_id) {
                    Ok(product) => {
                        if product.stock < quantity {
                            return Response::new(400, "Bad Request", &format!("Not enough stock. Only {} left.", product.stock));
                        }
                    },
                    Err(_) => return Response::new(404, "Not Found", "Product does not exist"),
                }

                let item = BasketItem { basket_id, product_id, quantity };
                match self.repo.save(&item) {
                    Ok(_) => Response::new(201, "Created", "Item added to basket"),
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }
            }
            Err(_) => Response::new(400, "Bad Request", "Invalid JSON format"),
        }
    }

    // PUT /baskets/:id/items/:product_id 
    pub fn update(&self, req: Request) -> Response {
        let b_id = req.path_params.get("id").and_then(|id| id.parse::<i32>().ok());
        let p_id = req.path_params.get("product_id").and_then(|id| id.parse::<i32>().ok());

        if let (Some(basket_id), Some(product_id)) = (b_id, p_id) {
            match serde_json::from_str::<BasketItem>(&req.body) {
                Ok(mut item) => {
                    item.basket_id = basket_id;
                    item.product_id = product_id;
                    match self.repo.update(&item) {
                        Ok(_) => Response::new(200, "OK", "Item updated successfully"),
                        Err(e) => Response::new(500, "Internal Server Error", &e),
                    }
                }
                Err(_) => Response::new(400, "Bad Request", "Invalid JSON format"),
            }
        } else {
            Response::new(400, "Bad Request", "Missing Basket or Product ID")
        }
    }

    // DELETE /baskets/:id/items/:product_id 
    pub fn delete(&self, req: Request) -> Response {
        let b_id = req.path_params.get("id").and_then(|id| id.parse::<i32>().ok());
        let p_id = req.path_params.get("product_id").and_then(|id| id.parse::<i32>().ok());

        if let (Some(basket_id), Some(product_id)) = (b_id, p_id) {
            match self.repo.delete_item(basket_id, product_id) {
                Ok(_) => Response::new(204, "No Content", ""),
                Err(e) => Response::new(500, "Internal Server Error", &e),
            }
        } else {
            Response::new(400, "Bad Request", "Missing Basket or Product ID")
        }
    }
}
