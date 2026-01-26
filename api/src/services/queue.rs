use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerJob {
    pub job_id: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub user_encryption_pubkey: String,
    pub webhook_url: String,
}

pub async fn enqueue_job(
    redis_client: &redis::Client, // Using redis::Client to get async connection
    payload: &WorkerJob,
) -> Result<(), redis::RedisError> {
    // Note: redis::Client::get_multiplexed_async_connection() is preferred for usage in handlers usually,
    // or just get_async_connection.
    let mut conn = redis_client.get_async_connection().await?;

    let msg = serde_json::to_string(payload).unwrap();
    conn.rpush::<_, _, ()>("jobs_queue", msg).await?;

    Ok(())
}
