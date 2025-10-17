use std::env;

use adapters::sqlx::pg_outbox_message_repository::PgOutboxMessageRepository;
use domain::{
    publishers::outbox_publisher::FakeOutboxMessagePublisher,
    services::outbox_service::OutboxService,
};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    println!("Processing Outbox");
    match process_outbox().await {
        Ok(_) => println!("Outbox processed"),
        Err(error) => println!("Error processing outbox: {:?}", error),
    }
}

async fn process_outbox() -> Result<(), String> {
    let db_connection_url =
        env::var("DB_CONNECTION_URL").expect("pass DB_CONNECTION_URL to the script");
    let pool = create_sqlx_connection_pool(&db_connection_url).await;
    let repository = PgOutboxMessageRepository { pool };
    let publisher = FakeOutboxMessagePublisher;

    let outbox_service = OutboxService::new(Box::new(repository), Box::new(publisher));

    outbox_service.publish().await
}

pub(crate) async fn create_sqlx_connection_pool(db_connection_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_url)
        .await
        .expect("Error connecting to DB")
}
