use serde::{Serialize};
use serde::de::DeserializeOwned;
use postgres::{Row, types::ToSql};
pub type DbError = String;

// should be serializable
pub trait Model : Serialize + DeserializeOwned  {
    fn table_name() -> &'static str;
    fn from_row(row : &Row) -> Self;

    fn insert_query() -> &'static str;
    fn insert_params(&self) -> Vec<&(dyn ToSql + Sync)>;

    fn update_query() -> &'static str;
    fn update_params(&self) -> Vec<&(dyn ToSql + Sync)>;
}
// now for every model we need to add 
// #[derive(Serialize, Deserialize, Clone, Debug)]

// Product : id, price, quantity, category, name
pub trait Repository{
    // enforce model
    type Item : Model;
    
    fn save(&self, item : &Self::Item) -> Result<(), DbError>;
    fn find_by_id(&self, id : i32) -> Result<Self::Item, DbError>;
    fn find_all(&self) -> Result<Vec<Self::Item>, DbError>;
    fn update(&self, item : &Self::Item) -> Result<(), DbError>;
    fn delete(&self, id : i32) -> Result<(), DbError>;
}

// every controller should have one Model and Repository 
pub trait Controller{
    type ModelItem : Model;
    type RepositoryItem : Repository<Item = Self::ModelItem>;
}
