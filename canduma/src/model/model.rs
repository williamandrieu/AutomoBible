use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Model {
    pub model_id: i32,
    pub brand_id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "models"]
pub struct InsertableModel {
    pub brand_id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ModelData {
    pub brand_id: i32,
    pub image_url: Option<String>,
    pub name: String,
}

impl From<ModelData> for InsertableModel {
    fn from(brand_data: ModelData) -> Self {
        let ModelData {
            brand_id,
            image_url,
            name ,
            ..
        } = brand_data;

        Self {
            brand_id,
            image_url,
            name,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}