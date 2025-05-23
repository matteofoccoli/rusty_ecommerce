#[cfg(test)]
pub mod test {
    use diesel::{
        r2d2::{ConnectionManager, Pool as DieselPool},
        PgConnection,
    };
    use dotenvy::dotenv;
    use sqlx::{postgres::PgPoolOptions, Pool as SqlxPool, Postgres};
    use std::env;

    pub(crate) fn create_diesel_connection_pool() -> DieselPool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        DieselPool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

    pub(crate) async fn create_sqlx_connection_pool() -> SqlxPool<Postgres> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Error connecting to DB")
    }
}
