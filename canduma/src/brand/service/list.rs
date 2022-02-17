use actix_web::web;
use diesel::prelude::*;

use crate::brand::model::Brand;
use crate::database::{db_connection, Pool};
use crate::database::PooledConnection;
use crate::errors::{ServiceResult};


pub fn list_all(pool: web::Data<Pool>,) -> Vec<Brand> {
    use crate::schema::brands::dsl::{brands};

    let conn = &db_connection(&pool).unwrap();

    let _brands: Vec<Brand> = brands
        .load::<Brand>(conn).unwrap();

    println!("{:?}", _brands);

    _brands
}


pub(crate) fn list(
    pool: web::Data<Pool>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Brand>> {
    use crate::schema::brands::dsl::*;
    let conn: &PooledConnection = &db_connection(&pool)?;

    Ok(brands
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Brand>(conn)?)
}
