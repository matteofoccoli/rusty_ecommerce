use actix_web::{post, web, HttpResponse, Responder};
use domain::services::order_service::CreateOrderRequestObject;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[post("/orders")]
async fn create_order(
    data: web::Form<OrderData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let customer_repository =
        adapters::sqlx::pg_customer_repository::PgCustomerRepository::new(pool.get_ref().clone());

    let order_repository =
        adapters::sqlx::pg_order_repository::PgOrderRepository::new(pool.get_ref().clone());

    let outbox_message_repository =
        adapters::sqlx::pg_outbox_message_repository::PgOutboxMessageRepository::new(
            pool.get_ref().clone(),
        );

    let order_service = domain::services::order_service::OrderService::new(
        Box::new(customer_repository),
        Box::new(order_repository),
        Box::new(outbox_message_repository),
    );

    match order_service
        .create_order(CreateOrderRequestObject {
            order_id: data.order_id.clone(),
            customer_id: data.customer_id.clone(),
        })
        .await
    {
        Ok(_) => HttpResponse::Ok().json(OrderResponse {
            order_id: data.order_id.clone(),
            customer_id: data.customer_id.clone(),
        }),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[derive(Deserialize)]
struct OrderData {
    order_id: String,
    customer_id: String,
}

#[derive(Serialize)]
struct OrderResponse {
    order_id: String,
    customer_id: String,
}
