use uuid::Uuid;

use crate::helpers::{insert_customer_on_db, TestContext};

#[actix_web::test]
async fn create_an_order() {
    let test_context = TestContext::new().await;
    let client = reqwest::Client::new();
    let order_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    insert_customer_on_db(customer_id, &test_context.connection_pool.clone())
        .await
        .expect("Failed to prepare DB content for test");
    let body = format!("order_id={}&customer_id={}", order_id, customer_id);

    let response = client
        .post(&format!("{}/orders", test_context.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create an order");

    assert!(response.status().is_success());

    test_context.cleanup().await;
}

#[actix_web::test]
async fn return_error_if_customer_does_not_exist() {
    let test_context = TestContext::new().await;
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

    test_context.cleanup().await;
}
