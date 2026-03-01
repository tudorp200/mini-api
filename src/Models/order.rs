use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OrderModel {
    pub id: i32,
    // foreign key
    pub basket_id : i32,
    pub price : i32,
    pub destination : String, 
}

impl Model for OrderModel {}