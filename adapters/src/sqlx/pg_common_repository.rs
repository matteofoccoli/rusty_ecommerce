use async_trait::async_trait;
use domain::repositories::common_repository::{CommonRepository, CommonRepositoryError};

pub struct PgCommonRepository;

#[async_trait]
impl CommonRepository for PgCommonRepository {
    async fn begin_transaction(&self) -> Result<(), CommonRepositoryError> {
        Ok(())
    }
    async fn commit_transaction(&self) -> Result<(), CommonRepositoryError> {
        Ok(())
    }
    async fn rollback_transaction(&self) -> Result<(), CommonRepositoryError> {
        Ok(())
    }
}
