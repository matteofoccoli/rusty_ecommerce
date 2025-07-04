use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[post("/customers")]
async fn create_customer(
    data: web::Form<CustomerData>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    HttpResponse::Ok()
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
