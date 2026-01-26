use crate::handlers::create_job::create_job;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/jobs").route(web::post().to(create_job)));
}
