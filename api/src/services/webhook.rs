pub async fn dispatch_webhook(job: &Job) {
    reqwest::Client::new()
        .post(&job.webhook_url)
        .json(&job)
        .send()
        .await
}