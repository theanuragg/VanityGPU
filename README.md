# VanityGPU: Solana Vanity Address Generation with AIDP Compute

VanityGPU is a Solana‑native vanity address generation platform that offloads the heavy brute‑force key search to AIDP GPUs. Teams submit desired Base58 patterns (prefix/suffix/regex), and receive batches of matched public keys with secure, non‑custodial delivery of private key material. The system provides verifiable proof of GPU compute, enterprise‑grade webhooks for “store in your DB,” and optional on‑chain attestation for provenance.

## Table of Contents
1. Overview
2. Why VanityGPU
3. Architecture
4. Security Model
5. Proof of Compute (Attestation)
6. Solana Integration
7. API Reference
8. Client SDK & Examples
9. Operational Model (Performance, Pricing, SLAs)
10. “Store in Your DB” Webhooks
11. Deployment & AIDP Integration
12. Demo & Submission Checklist
13. Risk, Limitations, and Future Work
14. License

---

## 1) Overview

- Product: GPU‑accelerated vanity address generation for Solana (Ed25519).
- Users: Protocols, brands, creators, and teams needing recognizable addresses.
- Compute: Heavy brute‑force runs on AIDP GPUs; CPU stubs available for local dev.
- Delivery: Non‑custodial by default (encrypted seeds), enterprise custodial optional.
- Proof: Signed batch attestation and optional on‑chain verification.
- Storage: Results delivered into each customer’s own DB via signed webhooks, or stored in per‑tenant tables for teams without infra.

Supported patterns:
- Prefix: e.g., “ANU”, “DAO”, “SOL”
- Suffix: e.g., “…XYZ”
- Regex (bounded): e.g., `^(ANU|DAO)[1-9A-HJ-NP-Za-km-z]{2}`

---

## 2) Why VanityGPU

- GPU fit: Brute‑force Ed25519 key search is embarrassingly parallel and maps perfectly to GPUs.
- Verifiable compute: Each batch includes a signed attestation and AIDP job metadata.
- Security first: Non‑custodial default; keys encrypted for the user; zeroization on workers.
- Solana‑native: Base58 normalization, wallet flows, optional on‑chain verifier.
- Enterprise features: Per‑tenant webhooks/HMAC, retries, audit logs, rate limits.

Use cases:
- Branded public addresses for treasuries and dApps
- Campaign addresses for mints or donations
- Short vanity patterns for social discoverability

---

## 3) Architecture

Components:
- Web App: Next.js/React for pattern submission and status tracking
- API: Node.js (Fastify/Express) + Prisma/Postgres for jobs, tenants, matches
- Queue: Redis/SQS for job orchestration
- GPU Worker: CUDA/OpenCL worker (Rust or Python wrapper) running on AIDP
- Attestation Service: Signs batch summaries with a worker keypair
- Optional On‑Chain Verifier: Solana program validating worker signatures (secp/ed25519 syscall)
- Webhook Dispatcher: Signed delivery to customer DB endpoints

Data flow:
1. User submits pattern, count ≤ 100, delivery mode, and encryption pubkey (for non‑custodial).
2. API creates job, enqueues to AIDP.
3. GPU worker brute‑forces seeds → derives Ed25519 pubkeys → Base58 encodes → pattern match → collect up to N hits.
4. For non‑custodial, encrypt private keys with user’s provided pubkey; for custodial, encrypt with tenant KMS key and immediately zeroize plaintext.
5. Worker signs batch attestation containing job_id, pattern, count, hit digests, and AIDP run metadata.
6. API persists matches and attestation, then POSTs webhook to customer DB with HMAC.
7. Optional: client submits attestation to on‑chain verifier; program emits “AttestedGeneration” event.

---

## 4) Security Model

Principles:
- Non‑custodial by default: Worker never stores plaintext private keys beyond immediate encryption for the user.
- Zeroization: Plaintext seeds are zeroized in worker memory after encryption.
- Encryption: Use user‑provided public key (or tenant KMS) to encrypt seeds before leaving the worker.
- Access control: Wallet authentication, API keys, rate limits, pattern policy checks (anti‑phishing).
- Auditability: Per‑batch logs, signed attestations, and job metadata; optional on‑chain verification.
- Privacy: No public disclosure of customer patterns unless opted in.

Threats & mitigations:
- Custody risk → Avoid by default; if required, use KMS + deletion SLAs + audits.
- Phishing look‑alikes → Pattern policy and warnings; optional blocklist for brand names.
- Replay attacks → Signed message schema includes nonce and job_id; on-chain verify checks message digest.
- Worker compromise → Rotate worker keys, isolated containers, minimal privilege, encrypted temp storage.

---

## 5) Proof of Compute (Attestation)

Each batch includes:
- job_id
- pattern, count, delivery mode
- AIDP job metadata (IDs, timestamps, node class)
- hit digests (pubkey hash, encrypted seed hash if applicable)
- worker_pubkey
- signature over the above

Schema (JSON):
{
  "job_id": "cuid",
  "pattern": "ANU",
  "count": 100,
  "delivery": "non_custodial",
  "aidp": { "job": "aidp-123", "started_at": "...", "completed_at": "...", "gpu": "A100" },
  "hits": [{ "pubkey_hash": "sha256(base58(pubkey))", "seed_ct_hash": "sha256(ciphertext?)" }],
  "worker_pubkey": "ed25519:...",
  "signature": "ed25519:..."
}

This attestation can be:
- Stored in your API and shown on the batch page.
- Delivered with the webhook payload for customer DB storage.
- Submitted to an on‑chain verifier program that checks the signature and emits an event.

---

## 6) Solana Integration

- Base58 normalization: Case-insensitive matching over Solana public keys encoded in Base58.
- Wallet auth: Phantom/Solflare adapters for authenticated submissions and tenant settings.
- On‑chain verifier (optional): Minimal program that inspects the prior secp/ed25519 verification instruction via the sysvar-instructions and records an attested flag tied to job_id.

Verifier flow:
1. Client includes a signature verification ix (secp/ed25519) for the attestation message.
2. Calls `verify_and_record(job_id, msg_hash)` on the program.
3. Program confirms the prior verification ran for the expected message and records/emit event.

---

## 7) API Reference

Base URL: https://api.vanitygpu.xyz

Authentication:
- Bearer API key, plus wallet-based session for dashboard
- HMAC for webhooks

Endpoints:
- POST /jobs
  - body: { pattern: string, count: number ≤ 100, delivery: "non_custodial" | "custodial", user_pubkey?: string, webhook_url?: string }
  - returns: { job_id }
- GET /jobs/:id
  - returns: { status: "queued" | "running" | "completed" | "failed", matches: [{ pubkey, encrypted_seed? }], attestation }
- POST /internal/aidp-callback
  - called by worker; authenticated via signed token
- POST /tenants/webhooks/test
  - sends a sample payload to validate customer DB ingestion
- GET /tenants/batches
  - list batches by tenant with pagination

Webhook payload:
{
  "job_id": "cuid",
  "pattern": "ANU",
  "count": 50,
  "matches": [{ "pubkey": "Base58", "encrypted_seed": "base64?" }],
  "attestation": { ... }
}
Headers:
- x-vanity-signature: HMAC-SHA256 over body

Errors:
- 400 Invalid pattern or delivery
- 402 Budget exceeded for pattern complexity
- 429 Rate limit
- 500 Worker failure (retriable)

---

## 8) Client SDK & Examples

TypeScript SDK:
- submitJob({ pattern, count, delivery, userPubkey, webhookUrl })
- getJob(jobId)
- verifyAttestationOnChain(attestation, connection, programId)

Example usage:
import { submitJob, getJob } from "@vanitygpu/sdk";
const { job_id } = await submitJob({ pattern: "ANU", count: 25, delivery: "non_custodial", userPubkey });
const status = await getJob(job_id);

Front-end:
- Patterns validated with client hints on expected time/cost.
- Wallet sign-in, encryption pubkey management, and export to Phantom/Solflare.

---

## 9) Operational Model (Performance, Pricing, SLAs)

Expected search complexity:
- Base58 alphabet (no 0/O/I/l): ~58 chars; naive prefix probability ≈ 58^(-k)
- Time scales exponentially with k; long prefixes may be infeasible.

Pricing tiers:
- Tier A (2–3 char prefix): fixed price, fast SLA
- Tier B (4 char): metered, ETA estimates, capped budget
- Tier C (5+ char): best-effort, progress updates, cancelation options

SLA:
- Tier A: 95% under 24 hours
- Tier B: 72 hours typical, adjustable
- Tier C: negotiated budgets; may not complete

Resource allocation:
- AIDP node class scaled by tier; dynamic parallelism; backpressure on queue

---

## 10) “Store in Your DB” Webhooks

- Tenants configure a webhook endpoint + secret.
- After completion, we POST signed payloads with retries (exponential backoff).
- Dead-letter queue on failures; dashboard shows delivery status.
- For teams without infra, per-tenant storage with export (CSV/JSON) and API access.

DB schema (customer side recommendation):
- vanity_batches(job_id, pattern, count, status, attestation_json)
- vanity_matches(id, job_id, pubkey, encrypted_seed, created_at)

---

## 11) Deployment & AIDP Integration

AIDP steps:
1. Request GPU access at https://aidp.store
2. Containerize worker with minimal runtime dependencies
3. Submit jobs via AIDP API; capture job IDs and logs
4. Callback to API on completion, including attestation

Infra:
- API: Vercel/Render/Fly.io or Kubernetes
- DB: Postgres with Prisma
- Queue: Redis/SQS
- Secrets: Vault/KMS
- Monitoring: Prometheus/Grafana, Sentry

---

## 12) Demo & Submission Checklist

For campaign qualification:
- AIDP Marketplace Project Page
- Public GitHub repository (this README)
- 1–2 minute demo video:
  - Submit pattern in UI
  - Show AIDP job page/logs running
  - Show webhook payload arriving and being stored
  - Optional on‑chain attestation verification transaction
- Clear explanation of GPU usage (Section 3, 5, 11)
- Superteam Earn submission with public links

---

## 13) Risk, Limitations, and Future Work

Known risks:
- Long prefixes may be computationally infeasible within budget/SLA
- Vanity keys can mislead users (phishing); pattern policy and warnings required
- Custodial generation increases liability; keep optional and audited

Limitations:
- Regex support bounded and sanitized for performance
- Non‑custodial requires users to manage encryption keys securely

Future work:
- ZK verifiable compute (prove a hit without revealing seed)
- Collaborative client‑GPU search (WASM verifier)
- Brand registry with on‑chain reservations and attested proofs
- Multi‑chain support (Ed25519/ECDSA variants with chain‑specific constraints)
- Advanced heuristics to bias search toward likely Base58 shapes

---

## 14) License

AGPL‑3.0 for the core worker and API, with commercial licensing available for enterprise custodial modules.