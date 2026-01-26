VanityGPU is designed as a high-performance, verifiable compute utility within the AIDP (Artificial Intelligence Decentralized Protocol) ecosystem. It leverages the massive parallel processing power of GPUs to solve the entropy-intensive task of vanity address generation while ensuring a non-custodial, "depth of computing" security model.

Here is how the project integrates GPU usage and the concept of "Depth of Computing":

1. GPU Usage on AIDP: Massive Parallelism
The core of VanityGPU is the search for a specific cryptographic pattern (prefix/suffix). This is a "brute-force" problem that is perfectly suited for GPU architecture:

Throughput: While a CPU might check thousands of keys per second, a single GPU (like an RTX 4090) can check millions.
in Redis, download the job criteria, and saturate their CUDA cores with Ed25519 curve operations.Stateless Scaling: Within the AIDP protocol, workers act as stateless compute nodes. They poll thejobs_queue

Energy Efficiency: By offloading these highly repetitive mathematical operations to specialized hardware, the energy cost per "match" is significantly reduced compared to traditional CPU clusters.
2. Depth of Computing: The Secure Pipeline
"Depth of Computing" in VanityGPU refers to the multi-layered lifecycle of a single job. It isn't just about finding a result; it's about the verifiable and secure path that result takes.

Layer 1: The Search (Compute)
Workers generate random seeds and derive Solana public keys. This is the raw "work" layer.

Layer 2: The Sealed Transformation (X25519 Encryption)
This is the most critical security layer.

The worker uses ECDH (Elliptic Curve Diffie-Hellman) over Curve25519 (X25519).
It takes the user's provided public key and generates an ephemeral secret to derive a shared AES-256-GCM key.
The seed is encrypted while still in the worker's RAM.
Layer 3: Zero-Knowledge Zeroization
Immediately after encryption, the worker uses the

zeroize
crate to physically wipe the plaintext seed and the AES key from memory. In the "Depth of Computing" model, this ensures that even if a worker is compromised post-computation, the private keys of users are gone.

Layer 4: Cryptographic Attestation (Proof of Work)
Every result delivered to AIDP includes an Attestation.
.The worker signs a payload containing thejob_id
, theresult_hash
, and its ownworker_pubkey

This provides a verifiable proof that the compute was performed by a registered node in the cluster and hasn't been tampered with in transit.

Layer 5: Direct Delivery & Webhook Dispatch
The result is delivered directly to the user's infrastructure (Postgres) and triggered via Webhooks. This completes the "depth" by ensuring the data reaches its destination without manual intervention or intermediary storage exposure.

By integrating with AIDP, VanityGPU transforms a simple search tool into a verifiable, distributed utility that prioritizes user sovereignty over their private entropy.