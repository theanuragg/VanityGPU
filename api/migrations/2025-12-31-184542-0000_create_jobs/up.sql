-- Your SQL goes here
CREATE TABLE jobs (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- user inputs
    solana_pubkey TEXT NOT NULL,
    desired_prefix TEXT NOT NULL,
    desired_suffix TEXT,

    -- where result goes
    result_db_url TEXT NOT NULL,
    result_table TEXT NOT NULL,

    -- webhook
    webhook_url TEXT NOT NULL,

    -- execution
    status TEXT NOT NULL, -- pending | running | completed | failed
    attempts INTEGER NOT NULL DEFAULT 0,

    -- results
    matched_address TEXT,
    encrypted_private_key BYTEA,

    -- attestation
    worker_id TEXT,
    attestation_hash TEXT
);
