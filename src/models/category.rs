use postgres::{Row, types::ToSql};
use serde::{Serialize, Deserialize};
use crate::traits::Model;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Model for Category {
    fn table_name() -> &'static str { "categories" }

    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO categories (name) VALUES ($1)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.name]
    }

    fn update_query() -> &'static str {
        "UPDATE categories SET name = $1 WHERE id = $2"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.name, &self.id]
    }
}