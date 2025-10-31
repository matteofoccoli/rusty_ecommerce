use async_trait::async_trait;
use mockall::automock;

#[derive(Debug)]
pub enum TransactionalRepositoryError {
    BeginTransactionError(String),
    CommitTransactionError(String),
    RollbackTransactionError(String),
}

impl std::fmt::Display for TransactionalRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionalRepositoryError::BeginTransactionError(message) => {
                write!(f, "Begin transaction error: {}", message)
            }
            TransactionalRepositoryError::CommitTransactionError(message) => {
                write!(f, "Commit transaction error: {}", message)
            }
            TransactionalRepositoryError::RollbackTransactionError(message) => {
                write!(f, "Rollback transaction error {}", message)
            }
        }
    }
}

impl std::error::Error for TransactionalRepositoryError {}

#[automock]
#[async_trait]
pub trait TransactionalRepository {
    async fn begin_transaction(&self) -> Result<(), TransactionalRepositoryError>;
    async fn commit_transaction(&self) -> Result<(), TransactionalRepositoryError>;
    async fn rollback_transaction(&self) -> Result<(), TransactionalRepositoryError>;
}
