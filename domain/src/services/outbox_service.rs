use chrono::Utc;

use crate::{
    entities::outbox::OutboxMessage, publishers::outbox_publisher::OutboxMessagePublisher,
    repositories::outbox_repository::OutboxMessageRepository,
};

#[derive(Debug)]
pub enum OutboxServiceError {
    MessageNotPublishedError(OutboxMessage, String),
    MessageNotSetToProcessedError(OutboxMessage, String),
    MessagesNotReadError(String),
    GenericError(String),
}

impl std::fmt::Display for OutboxServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboxServiceError::MessagesNotReadError(error) => {
                write!(f, "Error reading messages: {}", error)
            }
            OutboxServiceError::GenericError(error) => {
                write!(f, "Error handling outbox messages: {}", error)
            }
            OutboxServiceError::MessageNotPublishedError(message, error) => {
                write!(
                    f,
                    "Error publishing message (id: {}, type: {}): {}",
                    message.id(),
                    message.event_type(),
                    error
                )
            }
            OutboxServiceError::MessageNotSetToProcessedError(message, error) => {
                write!(
                    f,
                    "Error setting message message to processed (id: {}, type: {}): {}",
                    message.id(),
                    message.event_type(),
                    error
                )
            }
        }
    }
}

impl std::error::Error for OutboxServiceError {}

pub struct OutboxService {
    outbox_message_repository: Box<dyn OutboxMessageRepository>,
    outbox_message_publisher: Box<dyn OutboxMessagePublisher>,
}

impl OutboxService {
    pub fn new(
        outbox_message_repository: Box<dyn OutboxMessageRepository>,
        outbox_message_publisher: Box<dyn OutboxMessagePublisher>,
    ) -> Self {
        Self {
            outbox_message_repository,
            outbox_message_publisher,
        }
    }

    pub async fn publish(&self) -> Result<(), OutboxServiceError> {
        let messages = self
            .outbox_message_repository
            .find_unprocessed()
            .await
            .map_err(|e| OutboxServiceError::MessagesNotReadError(e.to_string()))?;

        if let Some(messages) = messages {
            for message in messages.into_iter() {
                if let Err(error) = self.outbox_message_publisher.publish(message.clone()).await {
                    return Err(OutboxServiceError::MessageNotPublishedError(
                        message,
                        error.to_string(),
                    ));
                }
                self.outbox_message_repository
                    .set_processed(message.id(), Utc::now())
                    .await
                    .map_err(|e| {
                        OutboxServiceError::MessageNotSetToProcessedError(message, e.to_string())
                    })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::{
        entities::{customer::Customer, outbox::OutboxMessage},
        publishers::outbox_publisher::{MockOutboxMessagePublisher, OutboxMessagePublisherError},
        repositories::outbox_repository::MockOutboxMessageRepository,
        value_objects::{Address, CustomerId},
    };

    use super::*;

    #[tokio::test]
    pub async fn publishes_message() {
        let message = OutboxMessage::customer_created_event(&create_customer()).unwrap();
        let message_id = message.id();

        let mut publisher = MockOutboxMessagePublisher::new();
        publisher
            .expect_publish()
            .with(eq(message.clone()))
            .return_once(|_| Ok(()))
            .once();

        let mut repository = MockOutboxMessageRepository::new();
        repository
            .expect_find_unprocessed()
            .return_once(|| Ok(Some(vec![message])))
            .once();
        repository
            .expect_set_processed()
            .withf(move |id, _| *id == message_id)
            .return_once(|_, _| Ok(()))
            .once();

        let service = OutboxService::new(Box::new(repository), Box::new(publisher));

        let result = service.publish().await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    pub async fn handled_publisher_errors() {
        let message = OutboxMessage::customer_created_event(&create_customer()).unwrap();

        let mut publisher = MockOutboxMessagePublisher::new();
        publisher
            .expect_publish()
            .return_once(|_| {
                Err(OutboxMessagePublisherError(
                    "Publishing went wrong :(".to_string(),
                ))
            })
            .once();

        let mut repository = MockOutboxMessageRepository::new();
        repository
            .expect_find_unprocessed()
            .return_once(|| Ok(Some(vec![message])))
            .once();
        repository.expect_set_processed().never();

        let service = OutboxService::new(Box::new(repository), Box::new(publisher));

        let result = service.publish().await.unwrap_err();

        assert!(matches!(
            result,
            OutboxServiceError::MessageNotPublishedError(_, _)
        ));
    }

    fn create_customer() -> Customer {
        Customer {
            id: CustomerId(Uuid::new_v4()),
            first_name: "my_customer_first_name".to_string(),
            last_name: "my_customer_last_name".to_string(),
            address: Address {
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            },
        }
    }
}
