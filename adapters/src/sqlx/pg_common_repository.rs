use async_trait::async_trait;
use domain::repositories::common_repository::{CommonRepository, CommonRepositoryError};
use sqlx::{Pool, Postgres};

pub struct PgCommonRepository {
    pool: Pool<Postgres>,
}

impl PgCommonRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommonRepository for PgCommonRepository {
    async fn begin_transaction(&self) -> Result<(), CommonRepositoryError> {
        sqlx::query("BEGIN")
            .execute(&self.pool)
            .await
            .map_err(|e| CommonRepositoryError::BeginTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
    async fn commit_transaction(&self) -> Result<(), CommonRepositoryError> {
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await
            .map_err(|e| CommonRepositoryError::CommitTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
    async fn rollback_transaction(&self) -> Result<(), CommonRepositoryError> {
        sqlx::query("ROLLBACK")
            .execute(&self.pool)
            .await
            .map_err(|e| CommonRepositoryError::RollbackTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
}
