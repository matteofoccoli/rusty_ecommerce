use async_trait::async_trait;
use mockall::automock;

use crate::entities::outbox::OutboxMessage;

#[derive(Debug)]
pub struct OutboxMessagePublisherError(pub String);

impl std::fmt::Display for OutboxMessagePublisherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error publishing outbox message: {}", self.0)
    }
}

impl std::error::Error for OutboxMessagePublisherError {}

#[automock]
#[async_trait]
pub trait OutboxMessagePublisher {
    async fn publish(&self, message: OutboxMessage) -> Result<(), OutboxMessagePublisherError>;
}

pub struct FakeOutboxMessagePublisher;

#[async_trait]
impl OutboxMessagePublisher for FakeOutboxMessagePublisher {
    async fn publish(&self, message: OutboxMessage) -> Result<(), OutboxMessagePublisherError> {
        println!("{:?}", message);
        Ok(())
    }
}
