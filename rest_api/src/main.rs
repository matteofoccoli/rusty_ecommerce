use std::net::TcpListener;

use rest_api::{settings::get_settings, startup::run};
use sqlx::{PgPool, Pool, Postgres};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_settings().expect("Failed to read settings");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address and port");

    run(listener, create_connection_pool().await)?.await
}

async fn create_connection_pool() -> Pool<Postgres> {
    let settings = get_settings().expect("Failed to read settings");
    let db_url = settings.database.connection_string();
    PgPool::connect(&db_url)
        .await
        .expect("Failed to build connection pool")
}
