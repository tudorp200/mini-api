use crate::traits::Model;
use postgres::{Row, types::ToSql};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BasketModel {
    pub id: i32,
    pub status : String,
}

impl Model for BasketModel {
    
    fn table_name() -> &'static str {
        return "baskets";
    }
    
    fn from_row(row : &Row) -> Self {
        Self {
            id : row.get("id"),
            status : row.get("status")
        }
    }

}