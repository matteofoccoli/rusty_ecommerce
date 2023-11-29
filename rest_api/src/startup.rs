use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer };

use crate::routes::{create_order, health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check::health_check)
            .service(create_order::create_order)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
