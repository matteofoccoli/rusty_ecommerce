use std::env;

use adapters::sqlx::pg_outbox_message_repository::PgOutboxMessageRepository;
use domain::repositories::outbox_repository::OutboxMessageRepository;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    println!("Processing Outbox");
    process_outbox().await;
    println!("Outbox processed");
}

async fn process_outbox() {
    let db_connection_url =
        env::var("DB_CONNECTION_URL").expect("pass DB_CONNECTION_URL to the script");
    let pool = create_sqlx_connection_pool(&db_connection_url).await;
    let outbox_repository = PgOutboxMessageRepository { pool };

    if let Some(messages) = outbox_repository
        .find_not_sent()
        .await
        .expect("Error reading messages from outbox table")
    {
        messages
            .iter()
            .for_each(|message| println!("{:?}", message));
    }
}

pub(crate) async fn create_sqlx_connection_pool(db_connection_url: &str) -> Pool<Postgres> {
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_url)
        .await
        .expect("Error connecting to DB")
}
