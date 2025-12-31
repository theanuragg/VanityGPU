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
    
    let job = Job {
        id: Uuid::new_v4(),
        created_at: Utc::now().naive_utc(),
        solana_pubkey: payload.solana_pubkey.clone(),
        desired_prefix: payload.desired_prefix.clone(),
        desired_suffix: payload.desired_suffix.clone(),
        result_db_url: payload.result_db_url.clone(),
        result_table: payload.result_table.clone(),
        webhook_url: payload.webhook_url.clone(),
        
        status: "pending".info(),
        attempts: 10,
        
        matched_address: None,
        matched_private_key: None,
        worker_id: None,
        attestations: None,
    };
    
    insert_job(&state.db, &job).await?;
    enqueue_job( &job).await?;
    
    Ok(HttpResponse::Ok().json(job.id))
}