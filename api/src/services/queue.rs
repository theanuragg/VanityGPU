use serde::{Serialize, Deserialize};
use redis::AsyncCommands;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerJob {
    pub job_id: String,
    pub pattern_prefix: String,
    pub pattern_suffix: Option<String>,
    pub user_encryption_pubkey: String,
    pub webhook_url: String,
}

pub async fn enqueue_job(
    redis: &redis::Client,
    payload: &WorkerJob,
) -> anyhow::Result<()> {
    let mut conn = redis.get_async_connection().await?;

    let msg = serde_json::to_string(payload)?;
    conn.lpush::<_, _, ()>("aidp:jobs", msg).await?;

    Ok(())
}