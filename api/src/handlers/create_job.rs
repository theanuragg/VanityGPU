use crate::schema::jobs;
use crate::{
    AppState,
    models::job::NewJob,
    services::queue::{self, WorkerJob},
};
use actix_web::{HttpResponse, web};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub solana_pubkey: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub result_db_url: String,
    pub result_table: String,
    pub webhook_url: String,
}

#[derive(Serialize)]
pub struct CreateJobResponse {
    pub job_id: String,
    pub status: String,
}

pub async fn create_job(
    state: web::Data<AppState>,
    req: web::Json<CreateJobRequest>,
) -> HttpResponse {
    let new_job_id = Uuid::new_v4();

    // 1. Create Job in DB
    let new_job = NewJob {
        id: new_job_id,
        solana_pubkey: req.solana_pubkey.clone(),
        desired_prefix: req.desired_prefix.clone(),
        desired_suffix: req.desired_suffix.clone(),
        result_db_url: req.result_db_url.clone(),
        result_table: req.result_table.clone(),
        webhook_url: req.webhook_url.clone(),
        status: "queued".to_string(),
    };

    let pool = state.db.clone();
    let db_res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        diesel::insert_into(jobs::table)
            .values(&new_job)
            .execute(&mut conn)
    })
    .await;

    if let Err(e) = db_res {
        eprintln!("DB Error: {:?}", e);
        return HttpResponse::InternalServerError().body("DB Error");
    }

    // 2. Enqueue to Worker
    let worker_job = WorkerJob {
        job_id: new_job_id.to_string(),
        desired_prefix: req.desired_prefix.clone(),
        desired_suffix: req.desired_suffix.clone(),
        user_encryption_pubkey: req.solana_pubkey.clone(), // Assuming user pubkey is encryption key for now
        webhook_url: req.webhook_url.clone(),
    };

    if let Err(e) = queue::enqueue_job(&state.redis, &worker_job).await {
        eprintln!("Queue Error: {:?}", e);
        // Ideally rollback DB here or mark as failed
        return HttpResponse::InternalServerError().body("Queue Error");
    }

    HttpResponse::Ok().json(CreateJobResponse {
        job_id: new_job_id.to_string(),
        status: "queued".to_string(),
    })
}
