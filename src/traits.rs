use serde::{Serialize};
use serde::de::DeserializeOwned;

pub type DbError = String;

// should be serializable
pub trait Model : Serialize + DeserializeOwned  {}
// now for every model we need to add 
// #[derive(Serialize, Deserialize, Clone, Debug)]

// Product : id, price, quantity, category, name
pub trait Repository{
    // enforce model
    type Item : Model;
    
    fn save(&self, item : &Self::Item) -> Result<(), DbError>;
    fn find_by_id(&self, id : u32) -> Result<Self::Item, DbError>;
    fn find_all(&self) -> Result<Vec<Self::Item>, DbError>;
    fn count(&self) -> Result<u32, DbError>;
    fn delete(&self, id : u32) -> Result<(), DbError>;
}

// every controller should have one Model and Repository 
pub trait Controller{
    type ModelItem : Model;
    type RepositoryItem : Repository<Item = Self::ModelItem>;
}
