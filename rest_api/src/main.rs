use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
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

#[post("/orders")]
async fn create_order(data: web::Form<OrderData>) -> impl Responder {
    HttpResponse::Ok().json(OrderResponse {
        order_id: data.order_id.clone(),
        customer_id: data.customer_id.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(create_order))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
