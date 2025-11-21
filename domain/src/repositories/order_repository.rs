use async_trait::async_trait;
use mockall::mock;

use crate::{
    entities::order::Order,
    repositories::transactional_repository::{
        TransactionalRepository, TransactionalRepositoryError,
    },
    value_objects::OrderId,
};

#[derive(Debug)]
pub enum OrderRepositoryError {
    OrderNotFoundError,
    OrderNotSavedError,
    OrderItemsNotReadError,
    ConnectionError,
}

impl std::fmt::Display for OrderRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderRepositoryError::OrderNotFoundError => write!(f, "Order not found error"),
            OrderRepositoryError::OrderNotSavedError => write!(f, "Order not saved error"),
            OrderRepositoryError::ConnectionError => {
                write!(f, "Connection not created error")
            }
            OrderRepositoryError::OrderItemsNotReadError => {
                write!(f, "Order items not read error")
            }
        }
    }
}

impl std::error::Error for OrderRepositoryError {}

#[async_trait]
pub trait OrderRepository: TransactionalRepository {
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError>;

    async fn save(&self, order: Order) -> Result<Order, OrderRepositoryError>;

    async fn update(&self, order: Order) -> Result<Order, OrderRepositoryError>;
}

mock! {
    pub MyOrderRepository {}

    #[async_trait]
    impl OrderRepository for MyOrderRepository {
        async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError>;
        async fn save(&self, order: Order) -> Result<Order, OrderRepositoryError>;
        async fn update(&self, order: Order) -> Result<Order, OrderRepositoryError>;
    }

    #[async_trait]
    impl TransactionalRepository for MyOrderRepository {
        async fn begin_transaction(&self) -> Result<(), TransactionalRepositoryError>;
        async fn commit_transaction(&self) -> Result<(), TransactionalRepositoryError>;
        async fn rollback_transaction(&self) -> Result<(), TransactionalRepositoryError>;
    }
}
