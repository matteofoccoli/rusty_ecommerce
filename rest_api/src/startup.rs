use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::routes::{create_order, health_check};

pub fn run(
    listener: TcpListener,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(create_order)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
