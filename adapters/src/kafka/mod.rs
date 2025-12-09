use std::time::Duration;

use async_trait::async_trait;
use domain::{
    entities::outbox::OutboxMessage,
    publishers::outbox_publisher::{OutboxMessagePublisher, OutboxMessagePublisherError},
};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;

pub struct KafkaOutboxMessagePublisher {
    bootstrap_servers: String,
    topic: String,
}

impl KafkaOutboxMessagePublisher {
    pub fn new(bootstrap_servers: String, topic: String) -> Self {
        Self {
            bootstrap_servers,
            topic,
        }
    }

    fn to_kafka_message(
        &self,
        outbox_message: &OutboxMessage,
    ) -> Result<String, OutboxMessagePublisherError> {
        serde_json::to_string(&KafkaMessage {
            event_id: outbox_message.id().to_string(),
            event_type: outbox_message.event_type().to_string(),
            event_payload: outbox_message.event_payload(),
        })
        .map_err(|e| OutboxMessagePublisherError(e.to_string()))
    }
}

#[derive(Serialize)]
struct KafkaMessage {
    event_id: String,
    event_type: String,
    event_payload: String,
}

#[async_trait]
impl OutboxMessagePublisher for KafkaOutboxMessagePublisher {
    async fn publish(
        &self,
        outbox_message: OutboxMessage,
    ) -> Result<(), OutboxMessagePublisherError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", self.bootstrap_servers.clone())
            .create()
            .map_err(|_| {
                OutboxMessagePublisherError("Error creating Kafka producer".to_string())
            })?;

        let kafka_message = self.to_kafka_message(&outbox_message)?;
        producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&kafka_message)
                    .key(&outbox_message.id().to_string()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(ke, _)| OutboxMessagePublisherError(ke.to_string()))
            .map(|_| Ok(()))?
    }
}

#[cfg(test)]
mod test {
    use domain::{
        entities::{customer::Customer, outbox::OutboxMessage},
        publishers::outbox_publisher::OutboxMessagePublisher,
        value_objects::{Address, CustomerId},
    };
    use uuid::Uuid;

    use crate::kafka::KafkaOutboxMessagePublisher;

    #[tokio::test]
    async fn publishes_on_kafka() {
        let publisher =
            KafkaOutboxMessagePublisher::new("localhost:19092".to_string(), "topic".to_string());

        let message = OutboxMessage::customer_created_event(&Customer {
            id: CustomerId(Uuid::new_v4()),
            first_name: "Mario".to_string(),
            last_name: "Rossi".to_string(),
            address: Address {
                street: "customer street".to_string(),
                city: "customer city".to_string(),
                zip_code: "customer zip code".to_string(),
                state: "customer state".to_string(),
            },
        })
        .unwrap();

        let result = publisher.publish(message);

        assert!(result.await.is_ok());
    }
}
