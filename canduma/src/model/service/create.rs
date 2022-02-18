use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceResult;
use crate::model::model::{InsertableModel, Model, ModelData};
use actix_web::web;
use diesel::prelude::*;

pub fn create(model_data: ModelData, pool: web::Data<Pool>) -> ServiceResult<Model> {
    let conn: &PooledConnection = &db_connection(&pool)?;
    create_model(model_data, conn)
}

pub fn create_model(model_data: ModelData, conn: &PooledConnection) -> ServiceResult<Model> {
    use crate::schema::models::dsl::models;

    let model: InsertableModel = model_data.into();
    let inserted_model: Model = diesel::insert_into(models).values(&model).get_result(conn)?;
    Ok(inserted_model.into())
}
