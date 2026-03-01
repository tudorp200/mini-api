use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CartModel {
    pub id: i32,
}

impl Model for CartModel {}