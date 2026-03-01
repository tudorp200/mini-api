use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CartItemModel {
    pub id: i32,
    // foreign keys
    pub product_id : i32,
    pub cart_id : i32
}

impl Model for CartItemModel {}