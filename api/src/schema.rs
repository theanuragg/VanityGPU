// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> Uuid,
        created_at -> Timestamp,
        solana_pubkey -> Text,
        desired_prefix -> Text,
        desired_suffix -> Nullable<Text>,
        result_db_url -> Text,
        result_table -> Text,
        webhook_url -> Text,
        status -> Text,
        attempts -> Int4,
        matched_address -> Nullable<Text>,
        encrypted_private_key -> Nullable<Bytea>,
        worker_id -> Nullable<Text>,
        attestation_hash -> Nullable<Text>,
    }
}
