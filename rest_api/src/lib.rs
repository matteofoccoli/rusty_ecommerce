use std::net::TcpListener;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, dev::Server};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};

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

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/orders")]
async fn create_order(data: web::Form<OrderData>) -> impl Responder {
    let connection_pool = create_connection_pool();

    let customer_repository = adapters::pg_customer_repository::PgCustomerRepository {
        connection_pool: connection_pool.clone(),
    };

    let order_repository = adapters::pg_order_repository::PgOrderRepository {
        connection_pool: connection_pool.clone(),
    };

    let order_service = domain::services::order_service::OrderService {
        customer_repository: Box::new(customer_repository),
        order_repository: Box::new(order_repository),
    };

    match order_service.create_order(&data.order_id, &data.customer_id) {
        Ok(_) => HttpResponse::Ok().json(OrderResponse {
            order_id: data.order_id.clone(),
            customer_id: data.customer_id.clone(),
        }),
        Err(error_message) => HttpResponse::BadRequest().body(error_message),
    }
}

fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let db_url = "postgres://postgres@localhost/rusty_ecommerce";
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(create_order))
        .listen(listener)?
        .run();
    Ok(server)
}
