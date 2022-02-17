
use crate::brand::model::BrandData;
use crate::brand::service as brand;
use crate::database::Pool;
use crate::errors::ServiceError;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::FutureExt;

pub async fn create(
    brand_data: web::Json<BrandData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    println!("Create: {:?}", brand_data);
    brand::create(brand_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}


pub fn all(pool: web::Data<Pool>) -> HttpResponse{
    let vec_brand = brand::list::list_all(pool);
    match vec_brand.len() > 0 {
        true => {
            HttpResponse::Ok().json(&vec_brand)
        }
        false => {
            HttpResponse::NotFound().json("{}")
        }
    }

}


