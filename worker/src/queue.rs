pub fn fetch_job() -> WorkerJob {
    let msg = redis::cmd("BRPOP").arg("aidp:jobs").arg(0).query::<_, String>(&mut conn).unwrap();
    
    serde_json::from_str(&msg).unwrap()
}