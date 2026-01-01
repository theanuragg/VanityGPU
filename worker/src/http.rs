pub fn send_callback(result: jobResult) {
    reqwest::Client::new()
        .post("API/internal/aidp-callback")
        .json(&result)
        .send()
        .await
}
