use std::net::TcpListener;

use diesel::{
    pg::sql_types,
    r2d2::{ConnectionManager, Pool},
    sql_query,
    sql_types::Text,
    PgConnection, RunQueryDsl,
};
use rest_api::configuration::get_configuration;
use uuid::Uuid;

#[actix_web::test]
async fn health_check_works() {
    let test_app = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[actix_web::test]
async fn create_an_order() {
    let test_app = spawn_app();
    let client = reqwest::Client::new();
    let order_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    insert_customer_on_db(customer_id, test_app.connection_pool);
    let body = format!("order_id={}&customer_id={}", order_id, customer_id);

    let response = client
        .post(&format!("{}/orders", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create an order");

    assert!(response.status().is_success());
}

struct TestApp {
    address: String,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let connection_pool = create_connection_pool();
    let server =
        rest_api::startup::run(listener, connection_pool.clone()).expect("Failed to start server");
    let _ = actix_web::rt::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool,
    }
}

fn insert_customer_on_db(
    customer_id: Uuid,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) {
    let mut connection = connection_pool
        .get()
        .expect("Failed to get a connection from pool");
    sql_query(
        r#"
            INSERT INTO customers (id, first_name, last_name, street, city, zip_code, state) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind::<sql_types::Uuid, _>(customer_id)
    .bind::<Text, _>("John")
    .bind::<Text, _>("Doe")
    .bind::<Text, _>("John's street")
    .bind::<Text, _>("John's city")
    .bind::<Text, _>("John's zip code")
    .bind::<Text, _>("John's state")
    .execute(&mut connection)
    .expect("Failed to insert customer on DB");
}

fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_url = configuration.database.connection_string();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build connection pool")
}
