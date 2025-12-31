use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WorkerJob {
    pub job_id: String,
    pub solana_pubkey: String,
    pub desired_prefix: String,
    pub desired_suffix: String,
}

pub fn enqueue_job(job: WorkerJob) {
    // Implementation to enqueue the job
    let payload = WorkerJob {
        job_id: Job.id.to_string(),
        solana_pubkey: job.solana_pubkey.clone(),
        desired_prefix: job.desired_prefix.clone(),
        desired_suffix: job.desired_suffix.clone(),
    };
    
    redis::cmd("LPUSH")
        .arg("queue")
        .arg(serde_json::to_string(&payload))
        .execute(&mut conn);
        
        Ok(())
}