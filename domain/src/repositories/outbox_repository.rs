use async_trait::async_trait;
use mockall::automock;

use crate::entities::outbox::OutboxMessage;

#[derive(Debug)]
pub enum OutboxMessageRepositoryError {
    OutboxMessageNotSavedError(String),
    OutboxMessagesNotReadError,
}

impl std::fmt::Display for OutboxMessageRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboxMessageRepositoryError::OutboxMessageNotSavedError(message) => {
                write!(f, "Outbox message not saved error {}", message)
            }
            OutboxMessageRepositoryError::OutboxMessagesNotReadError => {
                write!(f, "Outbox messages not read error")
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

    async fn find_unprocessed(
        &self,
    ) -> Result<Option<Vec<OutboxMessage>>, OutboxMessageRepositoryError>;

    async fn set_processed(
        &self,
        message_id: uuid::Uuid,
        processed_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), OutboxMessageRepositoryError>;
}
