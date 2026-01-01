mod job;
mod gpu;

use std::str::from_boxed_utf8_unchecked;

use gpu::VanityEngine;
use gpu::cpu::CpuVanityEngine;
use job::WorkerJob;

fn main() {
    let job = fetch_job_from_redis(); // todo add fetch job from redis to excute
    
    println!("Job received: {:?}", job.job_id);
    
    let engine =  CpuVanityEngine::new();
    
    let matches = engine.search(
        &job.desired_prefix,
        job.desired_suffix.as_deref(),
        10,
    );
    for m in matches{
        print!("match found: {}", m.pubkey.to_string());
        
    }
    
    //todo: add  encryption seeds and sign attestation callback API
}