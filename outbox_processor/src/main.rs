use std::env;

use adapters::{
    kafka::KafkaOutboxMessagePublisher,
    sqlx::pg_outbox_message_repository::PgOutboxMessageRepository,
};
use domain::services::outbox_service::OutboxService;

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
    let boostrap_servers =
        env::var("BOOTSTRAP_SERVERS=").expect("pass BOOTSTRAP_SERVERS= to the script");
    let topic = env::var("TOPIC").expect("pass TOPIC to the script");
    let pool = create_sqlx_connection_pool(&db_connection_url).await;
    let repository = PgOutboxMessageRepository { pool };
    let publisher = KafkaOutboxMessagePublisher::new(boostrap_servers, topic);

    let outbox_service = OutboxService::new(Box::new(repository), Box::new(publisher));

    outbox_service.publish().await.map_err(|e| e.to_string())
}

pub(crate) async fn create_sqlx_connection_pool(db_connection_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_url)
        .await
        .expect("Error connecting to DB")
}
