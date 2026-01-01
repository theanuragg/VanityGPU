use actix_web::{web, HttpResponse};

pub async fn aidp_callback(
    payload: web::Json<WorkerCallback>,
) {
    verify_attestation(&payload)?;
    
    update_job_result(payload)?;
    dispatch_webhook(payload)?;
    
    HttpResponse::Ok().finish()
}