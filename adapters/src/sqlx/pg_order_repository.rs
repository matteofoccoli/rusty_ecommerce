use async_trait::async_trait;
use domain::{
    entities::order::Order,
    repositories::order_repository::OrderRepositoryError,
    value_objects::{CustomerId, OrderId, OrderItem, ProductId},
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

pub struct PgOrderRepository {
    pool: Pool<Postgres>,
}

impl PgOrderRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl domain::repositories::order_repository::OrderRepository for PgOrderRepository {
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError> {
        let uuid = id.0;
        let mut order = sqlx::query("SELECT * FROM orders where id = $1")
            .bind(uuid)
            .try_map(|row: PgRow| {
                Ok(Order {
                    id: OrderId(uuid),
                    customer_id: CustomerId(row.try_get("customer_id")?),
                    order_items: vec![],
                })
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|_| OrderRepositoryError::OrderNotFoundError)?;

        let order_items = sqlx::query("SELECT * FROM order_items WHERE order_id = $1")
            .bind(uuid)
            .try_map(|row: PgRow| {
                Ok(OrderItem {
                    price: row.try_get("price")?,
                    quantity: row.try_get("quantity")?,
                    product_id: ProductId(row.try_get("product_id")?),
                })
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|_| OrderRepositoryError::OrderItemsNotReadError)?;

        order.add_multiple(order_items);

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

    async fn update(&self, order: Order) -> Result<Order, OrderRepositoryError> {
        for order_item in &order.order_items {
            sqlx::query("INSERT INTO order_items (order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4)")
                .bind(order.id.0)
                .bind(order_item.product_id.0)
                .bind(order_item.quantity)
                .bind(order_item.price)
                .execute(&self.pool)
                .await
                .map_err(|_| OrderRepositoryError::OrderNotSavedError)?;
        }
        Ok(order)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::common::test;
    use domain::repositories::order_repository::OrderRepository;
    use uuid::Uuid;

    #[tokio::test]
    async fn saves_an_empty_order() {
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

    #[tokio::test]
    async fn saves_order_items_for_an_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let repository = PgOrderRepository {
            pool: test::create_sqlx_connection_pool().await,
        };
        let _ = repository
            .save(domain::entities::order::Order::create(
                domain::value_objects::OrderId(order_id),
                domain::value_objects::CustomerId(customer_id),
            ))
            .await;

        let mut order = domain::entities::order::Order::create(
            domain::value_objects::OrderId(order_id),
            domain::value_objects::CustomerId(customer_id),
        );
        order.add_multiple(vec![domain::value_objects::OrderItem {
            price: 10.0,
            quantity: 1,
            product_id: domain::value_objects::ProductId(product_id),
        }]);
        let result = repository.update(order).await;

        assert!(result.is_ok());
        let order_from_db = repository
            .find_by_id(domain::value_objects::OrderId(order_id))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(1, order_from_db.order_items.len());
    }
}
