use crate::helpers::TestContext;

#[actix_web::test]
async fn create_a_customer() {
    let test_context = TestContext::new().await;
    let client = reqwest::Client::new();

    let first_name = "John";
    let last_name = "Doe";
    let street = "123 Elm St";
    let city = "Springfield";
    let zip_code = "12345";
    let state = "IL";

    let body = format!(
        "first_name={}&last_name={}&street={}&city={}&zip_code={}&state={}",
        first_name, last_name, street, city, zip_code, state
    );

    let response = client
        .post(&format!("{}/customers", test_context.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to create a customer");

    assert!(response.status().is_success());

    test_context.cleanup().await;
}
