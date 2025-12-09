use async_trait::async_trait;
use domain::repositories::transactional_repository::TransactionalRepositoryError;
use sqlx::{Pool, Postgres, Transaction};

pub struct PgTransactionalRepository<'a> {
    pool: Pool<Postgres>,
    transaction: Option<Transaction<'a, Postgres>>,
}

impl<'a> PgTransactionalRepository<'a> {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }
}

#[async_trait]
impl<'a> domain::repositories::transactional_repository::TransactionalRepository
    for PgTransactionalRepository<'a>
{
    async fn begin_transaction(&mut self) -> Result<(), TransactionalRepositoryError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionalRepositoryError::BeginTransactionError(e.to_string()))?;
        self.transaction = Some(tx);
        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), TransactionalRepositoryError> {
        if let Some(tx) = self.transaction.take() {
            tx.commit()
                .await
                .map_err(|e| TransactionalRepositoryError::CommitTransactionError(e.to_string()))?;
        } else {
            return Err(TransactionalRepositoryError::CommitTransactionError(
                "No transaction started".to_string(),
            ));
        }
        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), TransactionalRepositoryError> {
        if let Some(tx) = self.transaction.take() {
            tx.rollback().await.map_err(|e| {
                TransactionalRepositoryError::RollbackTransactionError(e.to_string())
            })?;
        } else {
            return Err(TransactionalRepositoryError::RollbackTransactionError(
                "No transaction started".to_string(),
            ));
        }
        Ok(())
    }
}
