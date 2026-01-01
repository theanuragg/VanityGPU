use actix_web::{get, App, HttpResponse, HttpServer};
use redis::Client;
use vanitygpu::Job;

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    let redis = redis::Client::open(
        std::env::var("REDIS_URL").expect("REDIS_URL must be set")
    ).expect("Failed to connect to Redis");
    
    let state = AppState {
        redis, db
    };  
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
    })
}
