use diesel::{
    r2d2::{ConnectionManager, Pool},
    sql_query,
    sql_types::{self, Text},
    PgConnection, RunQueryDsl,
};
use uuid::Uuid;

use crate::helpers::TestContext;

#[actix_web::test]
async fn create_an_order() {
    let test_context = TestContext::new();
    let client = reqwest::Client::new();
    let order_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    insert_customer_on_db(customer_id, test_context.connection_pool.clone());
    let body = format!("order_id={}&customer_id={}", order_id, customer_id);

    let response = client
        .post(&format!("{}/orders", test_context.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create an order");

    assert!(response.status().is_success());
}

#[actix_web::test]
async fn return_error_if_customer_does_not_exist() {
    let test_context = TestContext::new();
    let client = reqwest::Client::new();
    let order_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let body = format!("order_id={}&customer_id={}", order_id, customer_id);

    let response = client
        .post(&format!("{}/orders", test_context.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create an order");

    assert!(response.status().is_client_error());
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
