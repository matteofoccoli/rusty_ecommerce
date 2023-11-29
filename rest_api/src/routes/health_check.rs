use actix_web::{get, Responder, HttpResponse};


#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

