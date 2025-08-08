use async_trait::async_trait;
use mockall::automock;

use crate::entities::outbox::OutboxMessage;

#[derive(Debug)]
pub enum OutboxMessageRepositoryError {
    OutboxMessageNotSavedError(String),
}

impl std::fmt::Display for OutboxMessageRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboxMessageRepositoryError::OutboxMessageNotSavedError(message) => {
                write!(f, "Outbox message not saved error {}", message)
            }
        }
    }
}

impl std::error::Error for OutboxMessageRepositoryError {}

#[automock]
#[async_trait]
pub trait OutboxMessageRepository {
    async fn save(
        &self,
        message: OutboxMessage,
    ) -> Result<OutboxMessage, OutboxMessageRepositoryError>;
}
