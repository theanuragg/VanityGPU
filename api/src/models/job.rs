use crate::schema::jobs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = jobs)]
pub struct Job {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub solana_pubkey: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub result_db_url: String,
    pub result_table: String,
    pub webhook_url: String,
    pub status: String,
    pub attempts: i32,
    pub matched_address: Option<String>,
    pub encrypted_private_key: Option<Vec<u8>>,
    pub worker_id: Option<String>,
    pub attestation_hash: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = jobs)]
pub struct NewJob {
    pub id: Uuid,
    pub solana_pubkey: String,
    pub desired_prefix: String,
    pub desired_suffix: Option<String>,
    pub result_db_url: String,
    pub result_table: String,
    pub webhook_url: String,
    pub status: String,
}
