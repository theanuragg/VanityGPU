use redis::Client;

#[derive(Clone, Debug)]
struct AppStore {
  pub redis: Client,
  pub db: PgPool,
}