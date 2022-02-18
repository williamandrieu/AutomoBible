mod handler;
pub mod model;
pub(crate) mod service;
use crate::brand::handler::{all, create};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/brands")
            .service(web::resource("/create").route(web::post().to(create)))
            .service(web::resource("").route(web::get().to(all))),
    );
}


