use actix_web::{post, web, HttpResponse, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use domain::services::order_service::CreateOrderRequestObject;
use serde::{Deserialize, Serialize};

#[post("/orders")]
async fn create_order(
    data: web::Form<OrderData>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let customer_repository = adapters::diesel::pg_customer_repository::PgCustomerRepository {
        connection_pool: pool.get_ref().clone(),
    };

    let order_repository = adapters::diesel::pg_order_repository::PgOrderRepository {
        connection_pool: pool.get_ref().clone(),
    };

    let order_service = domain::services::order_service::OrderService {
        customer_repository: Box::new(customer_repository),
        order_repository: Box::new(order_repository),
    };

    match order_service.create_order(CreateOrderRequestObject {
        order_id: data.order_id.clone(),
        customer_id: data.customer_id.clone(),
    }) {
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
