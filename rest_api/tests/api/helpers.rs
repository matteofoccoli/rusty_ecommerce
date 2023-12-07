use std::net::TcpListener;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection, RunQueryDsl,
};
use rest_api::settings::{get_settings, Settings};
use uuid::Uuid;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub struct TestContext {
    pub address: String,
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
    db_name: String,
}

impl TestContext {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
        let port = listener.local_addr().unwrap().port();
        let db_name = format!("rusty_ecommerce_test_{}", Uuid::new_v4());
        let connection_pool = setup_test_db(db_name.clone());
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

impl Drop for TestContext {
    fn drop(&mut self) {
        drop_test_db(self.db_name.clone());
    }
}

fn setup_test_db(db_name: String) -> Pool<ConnectionManager<PgConnection>> {
    let settings = get_settings().expect("Failed to read settings");
    create_test_db(&db_name, &settings);
    run_migrations_on_test_db(&db_name, &settings);
    create_connection_pool(&db_name, &settings)
}

fn create_test_db(db_name: &str, settings: &Settings) {
    let mut connection =
        PgConnection::establish(&settings.database.connection_string_without_db_name())
            .expect("Failed to connect to DB");
    diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, db_name))
        .execute(&mut connection)
        .expect(format!("Failed to create test DB {}", db_name).as_str());
}

fn run_migrations_on_test_db(db_name: &str, settings: &Settings) {
    let mut connection = PgConnection::establish(&get_test_db_url(settings, db_name)).expect("Failed to connect to DB");
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../adapters/migrations/");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed running migration on test DB");
}

fn create_connection_pool(
    db_name: &str,
    settings: &Settings,
) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(get_test_db_url(settings, db_name));
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build connection pool")
}

fn get_test_db_url(settings: &Settings, db_name: &str) -> String {
    format!(
        "{}/{}",
        settings.database.connection_string_without_db_name(),
        db_name
    )
}

fn drop_test_db(db_name: String) {
    let config = get_settings().expect("Failed to read settings");
    let mut connection =
        PgConnection::establish(&config.database.connection_string_without_db_name())
            .expect("Failed to connect to DB");

    diesel::sql_query(format!(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
        db_name
    ))
    .execute(&mut connection)
    .unwrap();

    diesel::sql_query(format!(r#"DROP DATABASE "{}""#, db_name))
        .execute(&mut connection)
        .expect(format!("Failed to create test DB {}", db_name).as_str());
}
