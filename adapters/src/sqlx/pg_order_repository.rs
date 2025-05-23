use async_trait::async_trait;
use domain::{
    entities::order::Order,
    repositories::OrderRepositoryError,
    value_objects::{CustomerId, OrderId},
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

pub struct PgOrderRepository {
    pool: Pool<Postgres>,
}

#[async_trait]
impl domain::repositories::OrderRepository for PgOrderRepository {
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError> {
        let uuid = id.0;
        let order = sqlx::query("SELECT * FROM orders where id = $1")
            .bind(uuid)
            .map(|row: PgRow| Order {
                id: OrderId(uuid),
                customer_id: CustomerId(row.try_get("customer_id").unwrap_or_default()),
                order_items: vec![],
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|_| OrderRepositoryError::OrderNotFoundError)?;
        return Ok(Some(order));
    }

    async fn save(&self, order: Order) -> Result<Order, OrderRepositoryError> {
        sqlx::query("INSERT INTO orders (id, customer_id) VALUES ($1, $2)")
            .bind(order.id.0)
            .bind(order.customer_id.0)
            .execute(&self.pool)
            .await
            .map_err(|_| OrderRepositoryError::OrderNotSavedError)?;
        Ok(order)
    }

    async fn update(&self, _: Order) -> Result<Order, OrderRepositoryError> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::common::test;
    use domain::repositories::OrderRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn saves_a_new_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let order = domain::entities::order::Order::create(
            domain::value_objects::OrderId(order_id),
            domain::value_objects::CustomerId(customer_id),
        );
        let repository = PgOrderRepository {
            pool: test::create_sqlx_connection_pool().await,
        };

        let result = repository.save(order);

        assert!(result.await.is_ok());
        let order_from_db = repository
            .find_by_id(domain::value_objects::OrderId(order_id))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(domain::value_objects::OrderId(order_id), order_from_db.id);
    }
}
