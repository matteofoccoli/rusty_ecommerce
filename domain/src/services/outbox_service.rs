use crate::{
    publishers::outbox_publisher::OutboxMessagePublisher,
    repositories::outbox_repository::OutboxMessageRepository,
};

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

    pub async fn publish(&self) -> Result<(), String> {
        let messages = self
            .outbox_message_repository
            .find_not_sent()
            .await
            .map_err(|_| "Error")?;

        if let Some(messages) = messages {
            messages.into_iter().for_each(|m| {
                let _ = self.outbox_message_publisher.publish(m);
            });
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
        publishers::outbox_publisher::MockOutboxMessagePublisher,
        repositories::outbox_repository::MockOutboxMessageRepository,
        value_objects::{Address, CustomerId},
    };

    use super::*;

    #[tokio::test]
    pub async fn publishes_message() {
        let message = OutboxMessage::customer_created_event(&create_customer());

        let mut publisher = MockOutboxMessagePublisher::new();
        publisher.expect_publish().with(eq(message.clone())).once();

        let mut repository = MockOutboxMessageRepository::new();
        repository
            .expect_find_not_sent()
            .return_once(|| Ok(Some(vec![message])))
            .once();

        let service = OutboxService::new(Box::new(repository), Box::new(publisher));

        let result = service.publish().await;

        assert!(result.is_ok())
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
