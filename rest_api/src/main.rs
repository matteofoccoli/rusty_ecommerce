use std::net::TcpListener;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use rest_api::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration fail");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address and port");
    run(listener, create_connection_pool())?.await
}

fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_url = configuration.database.connection_string();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build connection pool")
}
