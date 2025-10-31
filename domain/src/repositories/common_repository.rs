use async_trait::async_trait;
use mockall::automock;

#[derive(Debug)]
pub enum CommonRepositoryError {
    BeginTransactionError(String),
    CommitTransactionError(String),
    RollbackTransactionError(String),
}

impl std::fmt::Display for CommonRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonRepositoryError::BeginTransactionError(message) => {
                write!(f, "Begin transaction error: {}", message)
            }
            CommonRepositoryError::CommitTransactionError(message) => {
                write!(f, "Commit transaction error: {}", message)
            }
            CommonRepositoryError::RollbackTransactionError(message) => {
                write!(f, "Rollback transaction error {}", message)
            }
        }
    }
}

impl std::error::Error for CommonRepositoryError {}

#[automock]
#[async_trait]
pub trait CommonRepository {
    async fn begin_transaction(&self) -> Result<(), CommonRepositoryError>;
    async fn commit_transaction(&self) -> Result<(), CommonRepositoryError>;
    async fn rollback_transaction(&self) -> Result<(), CommonRepositoryError>;
}
