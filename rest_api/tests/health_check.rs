use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[actix_web::test]
#[ignore]
async fn create_an_order() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let order_id = "d1d0740b-57f4-4df5-a6e8-a11d4ec0a710";
    let customer_id = "d1d0740b-57f4-4df5-a6e8-a11d4ec0a708";
    let body = format!("order_id={}&customer_id={}", order_id, customer_id);

    let response = client
        .post(&format!("{}/orders", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create an order");

    assert!(response.status().is_success());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = rest_api::startup::run(listener).expect("Failed to start server");
    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
