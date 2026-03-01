use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProductModel {
    pub id: u32,
    pub price: f32, 
    pub quantity: u32,
    pub category: String, 
    pub name: String,
}

impl Model for ProductModel {}