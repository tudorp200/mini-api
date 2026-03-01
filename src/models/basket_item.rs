use postgres::{Row, types::ToSql};
use serde::{Serialize, Deserialize};
use crate::traits::Model;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasketItem {
    pub basket_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

impl Model for BasketItem {
    fn table_name() -> &'static str { "basket_items" }

    fn from_row(row: &Row) -> Self {
        Self {
            basket_id: row.get("basket_id"),
            product_id: row.get("product_id"),
            quantity: row.get("quantity"),
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO basket_items (basket_id, product_id, quantity) VALUES ($1, $2, $3)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.basket_id, &self.product_id, &self.quantity]
    }

    fn update_query() -> &'static str {
        // We update the quantity WHERE both IDs match
        "UPDATE basket_items SET quantity = $1 WHERE basket_id = $2 AND product_id = $3"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        // Order must match $1, $2, $3 exactly
        vec![&self.quantity, &self.basket_id, &self.product_id]
    }
}