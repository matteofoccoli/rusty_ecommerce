use actix_web::{post, web, HttpResponse, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::Deserialize;

#[post("/customers")]
async fn create_customer(
    data: web::Form<CustomerData>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct CustomerData {
    order_id: String,
    customer_id: String,
}
