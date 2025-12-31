use actix_web::{web, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/jobs")
            .route(web::post().to(create_job))
    );
}
