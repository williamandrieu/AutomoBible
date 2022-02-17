use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Brand {
    pub brand_id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub creation_year: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "brands"]
pub struct InsertableBrand {
    pub image_url: Option<String>,
    pub name: String,
    pub creation_year: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct BrandData {
    pub image_url: Option<String>,
    pub name: String,
    pub creation_year: Option<String>,
}

impl From<BrandData> for InsertableBrand {
    fn from(brand_data: BrandData) -> Self {
        let BrandData {
            image_url,
            name ,
            creation_year,
            ..
        } = brand_data;

        Self {
            image_url,
            name,
            creation_year,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}