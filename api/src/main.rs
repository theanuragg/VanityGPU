use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

mod handlers;
mod models;
mod routes;
mod schema;
mod services;
// mod errors; // if exists

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub db: DbPool,
    pub redis: redis::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");

    let state = web::Data::new(AppState {
        db: pool,
        redis: redis_client,
    });

    println!("Starting API server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .configure(routes::jobs::routes)
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().body("OK") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
