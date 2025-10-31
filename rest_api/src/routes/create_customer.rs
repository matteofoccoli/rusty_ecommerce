use actix_web::{post, web, HttpResponse, Responder};
use domain::services::customer_service::CreateCustomerRequestObject;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[post("/customers")]
async fn create_customer(
    data: web::Form<CustomerData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let customer_repository =
        adapters::sqlx::pg_customer_repository::PgCustomerRepository::new(pool.get_ref().clone());
    let outbox_message_repository =
        adapters::sqlx::pg_outbox_message_repository::PgOutboxMessageRepository::new(
            pool.get_ref().clone(),
        );
    let common_repository =
        adapters::sqlx::pg_common_repository::PgCommonRepository::new(pool.get_ref().clone());

    let customer_service = domain::services::customer_service::CustomerService::new(
        Box::new(customer_repository),
        Box::new(outbox_message_repository),
        Box::new(common_repository),
    );

    match customer_service
        .create_customer(CreateCustomerRequestObject {
            first_name: data.first_name.clone(),
            last_name: data.last_name.clone(),
            street: data.street.clone(),
            city: data.city.clone(),
            zip_code: data.zip_code.clone(),
            state: data.state.clone(),
        })
        .await
    {
        Ok(customer) => HttpResponse::Ok().json(CustomerResponse {
            customer_id: customer.id.0.to_string(),
        }),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[derive(Deserialize)]
struct CustomerData {
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    zip_code: String,
    state: String,
}

#[derive(Serialize)]
struct CustomerResponse {
    customer_id: String,
}
