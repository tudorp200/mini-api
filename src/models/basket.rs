use crate::traits::Model;
use postgres::{Row, types::ToSql};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Basket {
    pub id: i32,
    pub status: String,
}

impl Model for Basket {
    fn table_name() -> &'static str { "baskets" }

    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            status: row.get("status"),
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO baskets (status) VALUES ($1)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.status]
    }

    fn update_query() -> &'static str {
        "UPDATE baskets SET status = $1 WHERE id = $2"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.status, &self.id]
    }
}