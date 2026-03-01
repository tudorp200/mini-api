use crate::http::request::Request;
use crate::http::response::Response;
use crate::models::product::Product;
use crate::repositories::base_repository::BaseRepository;
use crate::traits::Repository; 
use std::sync::Arc;


pub struct ProductController {
    repo: Arc<BaseRepository<Product>>,
}

impl ProductController {
    pub fn new(repo: Arc<BaseRepository<Product>>) -> Self {
        Self { repo }
    }

    // GET /products
    pub fn get_all(&self, _req: Request) -> Response {
        match self.repo.find_all() {
            Ok(products) => {
                let body = serde_json::to_string(&products).unwrap();
                Response::new(200, "OK", &body)
            },
            Err(e) => Response::new(500, "Internal Server Error", &e),
        }
    }

    // GET /products/:id
    pub fn get_by_id(&self, req: Request) -> Response {
        if let Some(id_str) = req.path_params.get("id") {
            if let Ok(id) = id_str.parse::<i32>() {
                match self.repo.find_by_id(id) {
                    Ok(product) => {
                        let body = serde_json::to_string(&product).unwrap();
                        Response::new(200, "OK", &body)
                    },
                    Err(_) => Response::new(404, "Not Found", "Product not found"),
                }
            } else {
                Response::new(400, "Bad Request", "Invalid ID format")
            }
        } else {
            Response::new(400, "Bad Request", "Missing ID")
        }
    }

    // POST /products
    pub fn create(&self, req: Request) -> Response {
        match serde_json::from_str::<Product>(&req.body) {
            Ok(product) => {
                match self.repo.save(&product) {
                    Ok(_) => Response::new(201, "Created", "Product created successfully"),
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }
            }
            Err(_) => Response::new(400, "Bad Request", "Invalid JSON format"),
        }
    }
}