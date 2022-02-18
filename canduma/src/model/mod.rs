mod handler;
pub mod model;
pub(crate) mod service;
use crate::model::handler::{all, create, get_by_name};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/models")
            .service(web::resource("/create").route(web::post().to(create)))
            .service(web::resource("").route(web::get().to(all)))
            .service(web::resource("/{name}").route(web::get().to(get_by_name))),
    );
}


