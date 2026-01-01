use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyJob {
    pub job_id: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub user_encryption_pubkey: String,
    pub webhook_url: String,
}