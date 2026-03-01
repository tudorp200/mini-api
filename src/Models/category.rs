use crate::traits::Model;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CategoryModel {
    // primary key
    pub id: i32, 
    pub name: String,
}

impl Model for CategoryModel {}