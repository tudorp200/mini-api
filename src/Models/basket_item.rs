use crate::traits::Model;
use postgres::{Row, types::ToSql};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BasketItemModel {
    // foreign keys
    pub product_id : i32,
    pub basket_id : i32,
    pub quantity : i32,
}

impl Model for BasketItemModel {
    
    fn table_name() -> &'static str {
        return "basket_items";
    }
    
    fn from_row(row : &Row) -> Self {
        Self {
            product_id : row.get("product_id"),
            basket_id : row.get("basket_id"),
            quantity : row.get("quantity")
        }
    }

}