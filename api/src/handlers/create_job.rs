use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub solana_pubkey: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub result_db_url: String,
    pub result_table: String,
    pub webhook_url: String,
}

pub async fn create_job(
    state: web::Data<AppState>,
    req: web::Json<CreateJobRequest>,
) -> HttpResponse {

    let job = insert_job_into_db(&state.db, &req).await?;

    let worker_job = WorkerJob {
        job_id: job.id.to_string(),
        pattern_prefix: job.pattern_prefix.clone(),
        pattern_suffix: job.pattern_suffix.clone(),
        user_encryption_pubkey: job.user_pubkey.clone(),
        webhook_url: job.webhook_url.clone(),
    };

    enqueue_job(&state.redis, &worker_job).await
        .map_err(|_| HttpResponse::InternalServerError())?;

    HttpResponse::Ok().json(CreateJobResponse {
        job_id: job.id.to_string(),
        status: "queued",
    })
}
