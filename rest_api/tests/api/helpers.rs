use std::{net::TcpListener, path::Path};

use rest_api::settings::{get_settings, Settings};
use sqlx::{migrate::Migrator, Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

pub struct TestContext {
    pub address: String,
    pub connection_pool: Pool<Postgres>,
    db_name: String,
}

impl TestContext {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
        let port = listener.local_addr().unwrap().port();
        let db_name = format!("rusty_ecommerce_test_{}", Uuid::new_v4());
        let connection_pool = setup_test_db(db_name.clone()).await;
        let server = rest_api::startup::run(listener, connection_pool.clone())
            .expect("Failed to start server");
        let _ = actix_web::rt::spawn(server);
        TestContext {
            address: format!("http://127.0.0.1:{}", port),
            connection_pool,
            db_name,
        }
    }
}

impl TestContext {
    pub async fn cleanup(&self) {
        drop_test_db(self.db_name.clone()).await;
    }
}

async fn setup_test_db(db_name: String) -> Pool<Postgres> {
    let settings = get_settings().expect("Failed to read settings");
    create_test_db(&db_name, &settings).await;
    run_migrations_on_test_db(&db_name, &settings).await;
    create_connection_pool(&db_name, &settings).await
}

async fn create_test_db(db_name: &str, settings: &Settings) {
    let mut conn = PgConnection::connect(&settings.database.connection_string_without_db_name())
        .await
        .expect("Failed to connect to DB");

    let create_db_query = format!(r#"CREATE DATABASE "{}""#, db_name);
    conn.execute(create_db_query.as_str())
        .await
        .expect("Failed to create test db");
}

fn get_test_db_url(settings: &Settings, db_name: &str) -> String {
    format!(
        "{}/{}",
        settings.database.connection_string_without_db_name(),
        db_name
    )
}

async fn run_migrations_on_test_db(db_name: &str, settings: &Settings) {
    let pool = create_connection_pool(db_name, settings).await;

    let migrator = Migrator::new(Path::new("../adapters/migrations"))
        .await
        .unwrap();
    migrator
        .run(&pool)
        .await
        .expect("Failed running migration on test DB")
}

async fn create_connection_pool(db_name: &str, settings: &Settings) -> Pool<Postgres> {
    let db_url = get_test_db_url(settings, db_name);
    PgPool::connect(&db_url)
        .await
        .expect("Failed to build connection pool")
}

async fn drop_test_db(db_name: String) {
    let settings = get_settings().expect("Failed to read settings");
    let mut conn = PgConnection::connect(&settings.database.connection_string_without_db_name())
        .await
        .expect("Failed to connect to DB");

    let terminate_query = format!(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
        db_name
    );
    conn.execute(terminate_query.as_str())
        .await
        .expect("Failed to terminate connections to test db");

    let drop_db_query = format!(r#"DROP DATABASE "{}""#, db_name);
    conn.execute(drop_db_query.as_str())
        .await
        .expect("Failed to drop test db");
}

pub async fn insert_customer_on_db(
    customer_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO customers (id, first_name, last_name, street, city, zip_code, state)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(customer_id)
    .bind("John")
    .bind("Doe")
    .bind("John's street")
    .bind("John's city")
    .bind("John's zip code")
    .bind("John's state")
    .execute(pool)
    .await?;
    Ok(())
}
