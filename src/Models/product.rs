use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProductModel {
    pub id: i32,
    pub price: f32, 
    pub quantity: u32,
    // foreign key
    pub category_id: i32, 
    pub name: String,
}

impl Model for ProductModel {}