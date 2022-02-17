use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceResult;
use crate::brand::model::{InsertableBrand, Brand, BrandData};
use actix_web::web;
use diesel::prelude::*;

pub fn create(brand_data: BrandData, pool: web::Data<Pool>) -> ServiceResult<Brand> {
    let conn: &PooledConnection = &db_connection(&pool)?;
    create_brand(brand_data, conn)
}

pub fn create_brand(brand_data: BrandData, conn: &PooledConnection) -> ServiceResult<Brand> {
    use crate::schema::brands::dsl::brands;

    let brand: InsertableBrand = brand_data.into();
    let inserted_brand: Brand = diesel::insert_into(brands).values(&brand).get_result(conn)?;
    Ok(inserted_brand.into())
}
