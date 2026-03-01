use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

// should be serializable
pub trait Model : Serialize + DeserializeOwned {}
// now for every model we need to add 
// #[derive(Serialize, Deserialize, Clone, Debug)]

// Product : id, price, quantity, category, name
pub trait Repository{
    // enforce model
    type Item : Model;
    fn save(&self, item : &Self::Item);
    fn find_by_id(&self, id : u32) -> Option<Self::Item>;
}

// every controller should have one Model and Repository 
pub trait Controller{
    type ModelItem : Model;
    type RepositoryItem : Repository<Item = Self::ModelItem>;
}
