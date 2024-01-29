use mockall::automock;

use crate::value_objects::OrderId;

use super::{
    entities::{customer::Customer, order::Order},
    value_objects::CustomerId,
};

#[automock]
pub trait CustomerRepository {
    fn find_by_id(&self, id: CustomerId) -> Result<Option<Customer>, String>;
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
            OrderRepositoryError::ConnectionNotCreatedError => write!(f, "Connection not created error"),
        }
    }
}

impl std::error::Error for OrderRepositoryError {}

#[automock]
pub trait OrderRepository {
    fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, OrderRepositoryError>;

    fn save(&self, order: Order) -> Result<Order, OrderRepositoryError>;

    fn update(&self, order: Order) -> Result<Order, OrderRepositoryError>;
}
