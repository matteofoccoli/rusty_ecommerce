use async_trait::async_trait;
use domain::repositories::transactional_repository::TransactionalRepositoryError;
use sqlx::{Pool, Postgres};

pub struct PgTransactionalRepository {
    pool: Pool<Postgres>,
}

impl PgTransactionalRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl domain::repositories::transactional_repository::TransactionalRepository
    for PgTransactionalRepository
{
    async fn begin_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("BEGIN")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::BeginTransactionError(e.to_string()))
            .map(|_| ())
    }

    async fn commit_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::CommitTransactionError(e.to_string()))
            .map(|_| ())
    }

    async fn rollback_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("ROLLBACK")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::RollbackTransactionError(e.to_string()))
            .map(|_| ())
    }
}
