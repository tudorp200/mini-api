use crate::traits::Model;
use postgres::{Row, types::ToSql};
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub basket_id: i32,
    pub total_paid: f32,
    pub status: String,
}

impl Model for Order {
    fn table_name() -> &'static str { "orders" }

    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            basket_id: row.get("basket_id"),
            total_paid: row.get("total_paid"),
            status: row.get("status"),
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO orders (basket_id, total_paid, status) VALUES ($1, $2, $3)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.basket_id, &self.total_paid, &self.status]
    }

    fn update_query() -> &'static str {
        "UPDATE orders SET basket_id = $1, total_paid = $2, status = $3 WHERE id = $4"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.basket_id, &self.total_paid, &self.status, &self.id]
    }
}