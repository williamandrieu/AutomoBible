
use crate::model::service as model;
use crate::database::Pool;
use crate::errors::ServiceError;
use actix_web::{web, HttpResponse};
use crate::model::model::{ModelData};

pub async fn create(
    model_data: web::Json<ModelData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    println!("Create: {:?}", model_data);
    model::create(model_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}


pub fn all(pool: web::Data<Pool>) -> HttpResponse{
    let vec_model = model::list::list_all(pool);
    HttpResponse::Ok().json(&vec_model)
}

pub fn get_by_name(web::Path(name): web::Path<String>,pool: web::Data<Pool>) -> HttpResponse{
    let vec_model = model::get_by::get_by_name(name, pool);
    HttpResponse::Ok().json(&vec_model)
}

