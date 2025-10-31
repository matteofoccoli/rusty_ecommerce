use async_trait::async_trait;
use mockall::mock;

use crate::{
    entities::customer::Customer,
    repositories::transactional_repository::{
        TransactionalRepository, TransactionalRepositoryError,
    },
    value_objects::CustomerId,
};

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

#[async_trait]
pub trait CustomerRepository: TransactionalRepository {
    async fn find_by_id(&self, id: CustomerId)
        -> Result<Option<Customer>, CustomerRepositoryError>;

    async fn save(&self, customer: Customer) -> Result<Customer, CustomerRepositoryError>;
}

mock! {
    pub MyCustomerRepository {}

    #[async_trait]
    impl CustomerRepository for MyCustomerRepository {
        async fn find_by_id(&self, id: CustomerId) -> Result<Option<Customer>, CustomerRepositoryError>;
        async fn save(&self, customer: Customer) -> Result<Customer, CustomerRepositoryError>;
    }

    #[async_trait]
    impl TransactionalRepository for MyCustomerRepository {
        async fn begin_transaction(&self) -> Result<(), TransactionalRepositoryError>;
        async fn commit_transaction(&self) -> Result<(), TransactionalRepositoryError>;
        async fn rollback_transaction(&self) -> Result<(), TransactionalRepositoryError>;
    }
}
