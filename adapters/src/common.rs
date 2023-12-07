#[cfg(test)]
pub mod test {
    use diesel::{
        r2d2::{ConnectionManager, Pool},
        PgConnection,
    };
    use dotenvy::dotenv;
    use std::env;

    pub(crate) fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }
}
