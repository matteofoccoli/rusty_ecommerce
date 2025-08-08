use async_trait::async_trait;
use mockall::automock;

use crate::{entities::customer::Customer, value_objects::CustomerId};

#[derive(Debug)]
pub enum CustomerRepositoryError {
    CustomerNotFoundError,
    CustomerNotSavedError,
    ConnectionNotCreatedError,
}

impl std::fmt::Display for CustomerRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerRepositoryError::ConnectionNotCreatedError => {
                write!(f, "Connection not created error")
            }
            CustomerRepositoryError::CustomerNotFoundError => write!(f, "Customer not found error"),
            CustomerRepositoryError::CustomerNotSavedError => write!(f, "Customer not saved error"),
        }
    }
}

impl std::error::Error for CustomerRepositoryError {}

#[automock]
#[async_trait]
pub trait CustomerRepository {
    async fn find_by_id(&self, id: CustomerId)
        -> Result<Option<Customer>, CustomerRepositoryError>;

    async fn save(&self, customer: Customer) -> Result<Customer, CustomerRepositoryError>;
}
