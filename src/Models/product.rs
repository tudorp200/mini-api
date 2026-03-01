use crate::traits::Model;
use postgres::{Row, types::ToSql};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProductModel {
    pub id: i32,
    pub price: f32, 
    pub stock: i32,
    // foreign key
    pub category_id: i32, 
    pub name: String,
}

impl Model for ProductModel {
    fn table_name() -> &'static str {
        "products"
    }
    
    fn from_row(row : &Row) -> Self {
        Self {
            id : row.get("id"),
            price : row.get("price"),
            name : row.get("name"),
            stock : row.get("stock"),
            category_id : row.get("category_id")
        }
    }

    fn insert_query() -> &'static str {
        "INSERT INTO products (category_id, name, price, stock) VALUES ($1, $2, $3, $4)"
    }

    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        // Just return a vector referencing your struct's fields in the exact order of the $1, $2 variables!
        vec![&self.category_id, &self.name, &self.price, &self.stock]
    }

    fn update_query() -> &'static str {
        "UPDATE products SET category_id = $1, name = $2, price = $3, stock = $4 WHERE id = $5"
    }

    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.category_id, &self.name, &self.price, &self.stock, &self.id]
    }
}