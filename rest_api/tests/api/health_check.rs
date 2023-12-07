use crate::helpers::TestContext;

#[actix_web::test]
async fn health_check_works() {
    let test_context = TestContext::new();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", test_context.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
