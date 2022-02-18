use actix_web::web;
use diesel::prelude::*;

use crate::brand::model::Brand;
use crate::database::{db_connection, Pool};
use crate::model::model::Model;

pub fn get_by_id(_brand_id: i32, pool: web::Data<Pool>) -> Vec<Model> {
    use crate::schema::models::dsl::{brand_id, models};

    let conn = &db_connection(&pool).unwrap();

    let _models: Vec<Model> = models
        .filter(brand_id.eq(_brand_id))
        .load::<Model>(conn).unwrap();

    _models
}


pub fn get_by_name(_name: String, pool: web::Data<Pool>) -> Vec<Model> {
    use crate::schema::models::dsl::{brand_id, models};
    use crate::schema::brands::dsl::{name, brands};

    let conn = &db_connection(&pool).unwrap();

    let brand: Brand = brands.filter(name.eq(_name)).first(conn).unwrap();

    let _models: Vec<Model> = models
        .filter(brand_id.eq(brand.brand_id))
        .load::<Model>(conn).unwrap();

    _models
}
