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
    payload: web::Json<CreateJobRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    verify_solana_pubkey(&payload.solana_pubkey)?;

    //todo
    Ok(HttpResponse::Ok().json("Job created successfully"))
}
