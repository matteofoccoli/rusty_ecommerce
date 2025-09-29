use mockall::automock;

use crate::entities::outbox::OutboxMessage;

#[automock]
pub trait OutboxMessagePublisher {
    fn publish(&self, message: OutboxMessage) -> Result<(), String>;
}
