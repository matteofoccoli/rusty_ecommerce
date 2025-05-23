use async_trait::async_trait;
use mockall::automock;

use crate::value_objects::OrderId;

use super::{
    entities::{customer::Customer, order::Order},
    value_objects::CustomerId,
};

#[derive(Debug)]
pub enum CustomerRepositoryError {
    CustomerNotFoundError,
    ConnectionNotCreatedError,
}

impl std::fmt::Display for CustomerRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerRepositoryError::ConnectionNotCreatedError => {
                write!(f, "Connection not created error")
            }
            CustomerRepositoryError::CustomerNotFoundError => write!(f, "Customer not found error"),
        }
    }
}

impl std::error::Error for CustomerRepositoryError {}

#[automock]
#[async_trait]
pub trait CustomerRepository {
    async fn find_by_id(&self, id: CustomerId)
        -> Result<Option<Customer>, CustomerRepositoryError>;
}

#[derive(Debug)]
pub enum OrderRepositoryError {
    OrderNotFoundError,
    OrderNotReadError,
    OrderNotSavedError,
    ConnectionNotCreatedError,
}

impl std::fmt::Display for OrderRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderRepositoryError::OrderNotFoundError => write!(f, "Order not found error"),
            OrderRepositoryError::OrderNotReadError => write!(f, "Order not read error"),
            OrderRepositoryError::OrderNotSavedError => write!(f, "Order not saved error"),
            OrderRepositoryError::ConnectionNotCreatedError => {
                write!(f, "Connection not created error")
            }
        }
    }
}

impl std::error::Error for OrderRepositoryError {}

#[automock]
#[async_trait]
pub trait OrderRepository {
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError>;

    async fn save(&self, order: Order) -> Result<Order, OrderRepositoryError>;

    async fn update(&self, order: Order) -> Result<Order, OrderRepositoryError>;
}
