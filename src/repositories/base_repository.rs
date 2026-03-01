use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::marker::PhantomData;

use crate::traits::Model;
use crate::models::product::Product;
use crate::traits::Repository;
use crate::traits::DbError;

pub struct BaseRepository<T: Model> {
    pool: Pool<PostgresConnectionManager<NoTls>>,
    _marker: PhantomData<T>, 
}

impl<T: Model> BaseRepository<T> {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
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
        
        let rows_affected = conn.execute(query, &params[..]).map_err(|e| e.to_string())?;
        
        if rows_affected == 0 {
            
            return Err("Not Found".to_string());
        }
        Ok(())
    }

    fn find_by_id(&self, id: i32) -> Result<T, DbError> {
    
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        let query = format!("SELECT * FROM {} WHERE id = $1", T::table_name());
        
        let row_opt = conn.query_opt(&query, &[&id]).map_err(|e| e.to_string())?; 

        match row_opt {
            Some(row) => Ok(T::from_row(&row)),
            None => Err(format!("Record with id {} not found in {}", id, T::table_name())),
        }
    }

    fn find_all(&self) -> Result<Vec<T>, DbError> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        let query = format!("SELECT * FROM {}", T::table_name());
        
        let rows = conn.query(&query, &[]).map_err(|e| e.to_string())?;
        
        Ok(rows.iter().map(|row| T::from_row(row)).collect())
    }

    fn delete(&self, id: i32) -> Result<(), String> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        
        let query = format!("DELETE FROM {} WHERE id = $1", T::table_name());
        
        conn.execute(&query, &[&id]).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl BaseRepository<Product> {
    // We now make category_id a strict requirement (i32 instead of Option<i32>)
    pub fn find_by_category_with_filters(
        &self,
        category_id: i32,
        min_price: Option<f32>,
        max_price: Option<f32>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        let query = "
            SELECT * FROM products 
            WHERE category_id = $1
              AND ($2::real IS NULL OR price >= $2)
              AND ($3::real IS NULL OR price <= $3)
            ORDER BY id
            LIMIT $4 OFFSET $5
        ";

        let rows = conn
            .query(
                query,
                &[&category_id, &min_price, &max_price, &limit, &offset],
            )
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(|row| Product::from_row(row)).collect())
    }

    pub fn find_all_paginated(&self, limit: i64, offset: i64) -> Result<Vec<Product>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        
        let query = "SELECT * FROM products ORDER BY id LIMIT $1 OFFSET $2";
        
        let rows = conn.query(query, &[&limit, &offset]).map_err(|e| e.to_string())?;
        Ok(rows.iter().map(|row| Product::from_row(row)).collect())
    }
}
