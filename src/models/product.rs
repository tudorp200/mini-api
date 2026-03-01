use postgres::{Row, types::ToSql};
use serde::{Serialize, Deserialize};
use crate::traits::Model;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub price: f32,
    pub stock: i32,
}

impl Model for Product {
    fn table_name() -> &'static str { "products" }

    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            category_id: row.get("category_id"),
            name: row.get("name"),
            price: row.get("price"),
            stock: row.get("stock"),
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO products (category_id, name, price, stock) VALUES ($1, $2, $3, $4)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.category_id, &self.name, &self.price, &self.stock]
    }

    fn update_query() -> &'static str {
        "UPDATE products SET category_id = $1, name = $2, price = $3, stock = $4 WHERE id = $5"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.category_id, &self.name, &self.price, &self.stock, &self.id]
    }
}