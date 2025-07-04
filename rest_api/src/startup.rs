use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::routes::{create_customer, create_order, health_check};

pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(create_order)
            .service(create_customer)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
