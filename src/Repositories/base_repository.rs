use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::marker::PhantomData;

use crate::traits::Model;
use crate::traits::Repository;
use crate::traits::DbError;

pub struct BaseRepository<T: Model> {
    pool: Pool<PostgresConnectionManager<NoTls>>,
    _marker: PhantomData<T>, 
}

impl<T: Model> Repository for BaseRepository<T> {
    type Item = T;

    fn save(&self, item: &Self::Item) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        let query = T::insert_query();
        let params = item.insert_params();
        
        conn.execute(query, &params[..]).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn update(&self, item: &Self::Item) -> Result<(), DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        let query = T::update_query();
        let params = item.update_params();
        
        conn.execute(query, &params[..]).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn find_by_id(&self, id: i32) -> Option<T> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        
        let query = format!("SELECT * FROM {} WHERE id = $1", T::table_name());
        
        let row_opt = conn.query_opt(&query, &[&id]).ok()?; 

        row_opt.map(|row| T::from_row(&row))
    }

    pub fn find_all(&self) -> Vec<T> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        
        let query = format!("SELECT * FROM {}", T::table_name());
        
        let rows = conn.query(&query, &[]).unwrap_or_default();
        
        rows.iter().map(|row| T::from_row(row)).collect()
    }

    pub fn delete(&self, id: i32) -> Result<(), String> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        
        let query = format!("DELETE FROM {} WHERE id = $1", T::table_name());
        
        conn.execute(&query, &[&id]).map_err(|e| e.to_string())?;
        Ok(())
    }
}