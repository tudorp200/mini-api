use crate::http::request::Request;
use crate::http::response::Response;
use crate::repositories::base_repository::BaseRepository;
use crate::traits::{Model, Repository};
use std::sync::Arc;

pub struct BaseController<T: Model + 'static> {
    repo: Arc<BaseRepository<T>>,
}

impl<T: Model + 'static> BaseController<T> {
    pub fn new(repo: Arc<BaseRepository<T>>) -> Self {
        Self { repo }
    }

    // GET /resource
    pub fn get_all(&self, _req: Request) -> Response {
        match self.repo.find_all() {
            Ok(items) => {
                let body = serde_json::to_string(&items).unwrap();
                Response::new(200, "OK", &body)
            }
            Err(e) => Response::new(500, "Internal Server Error", &e),
        }
    }

    // GET /resource/:id
    pub fn get_by_id(&self, req: Request) -> Response {
        if let Some(id_str) = req.path_params.get("id") {
            if let Ok(id) = id_str.parse::<i32>() {
                match self.repo.find_by_id(id) {
                    Ok(item) => {
                        let body = serde_json::to_string(&item).unwrap();
                        Response::new(200, "OK", &body)
                    }
                    Err(_) => Response::new(404, "Not Found", "Item not found"),
                }
            } else {
                Response::new(400, "Bad Request", "Invalid ID format")
            }
        } else {
            Response::new(400, "Bad Request", "Missing ID parameter")
        }
    }

    // POST /resource
    pub fn create(&self, req: Request) -> Response {
        match serde_json::from_str::<T>(&req.body) {
            Ok(item) => {
                match self.repo.save(&item) {
                    Ok(_) => Response::new(201, "Created", "Item created successfully"),
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }
            }
            Err(_) => Response::new(400, "Bad Request", "Invalid JSON format"),
        }
    }

    // PUT /resource/:id
    pub fn update(&self, req: Request) -> Response {
        if req.path_params.get("id").is_none() {
            return Response::new(400, "Bad Request", "Missing ID parameter");
        }

        match serde_json::from_str::<T>(&req.body) {
            Ok(item) => {
                match self.repo.update(&item) {
                    Ok(_) => Response::new(200, "OK", "Item updated successfully"),
                    Err(e) if e == "Not Found" => Response::new(404, "Not Found", "Item ID does not exist"),
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }
            }
            Err(_) => Response::new(400, "Bad Request", "Invalid JSON format"),
        }
    }

    // DELETE /resource/:id
    pub fn delete(&self, req: Request) -> Response {
        if let Some(id_str) = req.path_params.get("id") {
            if let Ok(id) = id_str.parse::<i32>() {
                match self.repo.delete(id) {
                    Ok(_) => Response::new(204, "No Content", ""),
                    Err(e) => Response::new(500, "Internal Server Error", &e),
                }
            } else {
                Response::new(400, "Bad Request", "Invalid ID format")
            }
        } else {
            Response::new(400, "Bad Request", "Missing ID parameter")
        }
    }
}