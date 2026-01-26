use crate::job::VerifyJob;
use redis::Commands;
use std::env;

pub struct JobQueue {
    client: redis::Client,
}

impl JobQueue {
    pub fn new() -> Self {
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
        Self { client }
    }

    pub fn pop_job(&self) -> Option<VerifyJob> {
        let mut con = self.client.get_connection().ok()?;
        // blocking pop from "jobs_queue", timeout 5 seconds
        let result: Option<(String, String)> = con.blpop("jobs_queue", 5.0).ok()?;

        if let Some((_list, job_json)) = result {
            serde_json::from_str(&job_json).ok()
        } else {
            None
        }
    }
}
