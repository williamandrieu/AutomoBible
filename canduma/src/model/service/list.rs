use actix_web::web;
use diesel::prelude::*;

use crate::database::{db_connection, Pool};
use crate::database::PooledConnection;
use crate::errors::{ServiceResult};
use crate::model::model::Model;


pub fn list_all(pool: web::Data<Pool>,) -> Vec<Model> {
    use crate::schema::models::dsl::{models};

    let conn = &db_connection(&pool).unwrap();

    let _models: Vec<Model> = models
        .load::<Model>(conn).unwrap();

    _models
}


pub(crate) fn list(
    pool: web::Data<Pool>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Model>> {
    use crate::schema::models::dsl::*;
    let conn: &PooledConnection = &db_connection(&pool)?;

    Ok(models
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Model>(conn)?)
}
