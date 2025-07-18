use async_trait::async_trait;
use mockall::automock;

#[derive(Debug)]
pub enum CommonRepositoryError {
    BeginTransactionError,
    CommitTransactionError,
    RollbackTransactionError,
}

impl std::fmt::Display for CommonRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonRepositoryError::BeginTransactionError => write!(f, "Begin transaction error"),
            CommonRepositoryError::CommitTransactionError => write!(f, "Commit transaction error"),
            CommonRepositoryError::RollbackTransactionError => {
                write!(f, "Rollback transaction error")
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
