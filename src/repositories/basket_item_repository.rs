use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use crate::models::basket_item::BasketItem; 
use crate::traits::{Model, DbError}; 

pub struct BasketItemRepository {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl BasketItemRepository {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { pool }
    }

    pub fn save(&self, item: &BasketItem) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        conn.execute(BasketItem::insert_query(), &BasketItem::insert_params(item)[..])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&self, item: &BasketItem) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        conn.execute(BasketItem::update_query(), &BasketItem::update_params(item)[..])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn find_item(&self, basket_id: i32, product_id: i32) -> Result<Option<BasketItem>, DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let query = "SELECT * FROM basket_items WHERE basket_id = $1 AND product_id = $2";
        
        let row_opt = conn.query_opt(query, &[&basket_id, &product_id]).map_err(|e| e.to_string())?;
        Ok(row_opt.map(|row| BasketItem::from_row(&row)))
    }

    pub fn find_by_basket_id(&self, basket_id: i32) -> Result<Vec<BasketItem>, DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let query = "SELECT * FROM basket_items WHERE basket_id = $1";
        
        let rows = conn.query(query, &[&basket_id]).map_err(|e| e.to_string())?;
        Ok(rows.iter().map(|row| BasketItem::from_row(row)).collect())
    }

    pub fn delete_item(&self, basket_id: i32, product_id: i32) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let query = "DELETE FROM basket_items WHERE basket_id = $1 AND product_id = $2";
        
        conn.execute(query, &[&basket_id, &product_id]).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn clear_basket(&self, basket_id: i32) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let query = "DELETE FROM basket_items WHERE basket_id = $1";
        
        conn.execute(query, &[&basket_id]).map_err(|e| e.to_string())?;
        Ok(())
    }
}