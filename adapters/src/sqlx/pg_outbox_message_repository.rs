use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    entities::outbox::OutboxMessage, repositories::outbox_repository::OutboxMessageRepositoryError,
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};
use uuid::Uuid;

pub struct PgOutboxMessageRepository {
    pool: Pool<Postgres>,
}

impl PgOutboxMessageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl domain::repositories::outbox_repository::OutboxMessageRepository
    for PgOutboxMessageRepository
{
    async fn save(
        &self,
        message: OutboxMessage,
    ) -> Result<OutboxMessage, OutboxMessageRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO outbox_messages (id, event_type, event_payload, created_at, processed_at)
            VALUES ($1, $2, $3, $4, $5) 
        "#,
        )
        .bind(&message.id())
        .bind(&message.event_type())
        .bind(&message.event_payload())
        .bind(&message.created_at())
        .bind(&message.processed_at())
        .execute(&self.pool)
        .await
        .map_err(|error| {
            OutboxMessageRepositoryError::OutboxMessageNotSavedError(error.to_string())
        })?;

        Ok(message)
    }

    async fn find_unprocessed(
        &self,
    ) -> Result<Option<Vec<OutboxMessage>>, OutboxMessageRepositoryError> {
        let messages = sqlx::query("SELECT * FROM outbox_messages WHERE processed_at IS NULL")
            .try_map(|row: PgRow| {
                let id: Uuid = row.try_get("id")?;
                let event_type = row.try_get("event_type")?;
                let event_payload = row.try_get("event_payload")?;
                let created_at = row.try_get("created_at")?;
                let processed_at = row.try_get("processed_at")?;
                Ok(OutboxMessage::new(
                    id,
                    event_type,
                    event_payload,
                    created_at,
                    processed_at,
                ))
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|_| OutboxMessageRepositoryError::OutboxMessagesNotReadError)?;

        Ok(Some(messages))
    }

    async fn set_processed(
        &self,
        message_id: Uuid,
        processed_at: DateTime<Utc>,
    ) -> Result<(), OutboxMessageRepositoryError> {
        sqlx::query(
            r#"
            UPDATE outbox_messages
            SET processed_at = $2
            WHERE id = $1
        "#,
        )
        .bind(&message_id)
        .bind(&Some(processed_at))
        .execute(&self.pool)
        .await
        .map_err(|error| {
            OutboxMessageRepositoryError::OutboxMessageNotSavedError(error.to_string())
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use domain::{
        entities::{customer::Customer, outbox::OutboxMessage},
        repositories::outbox_repository::OutboxMessageRepository,
        value_objects::{Address, CustomerId},
    };
    use uuid::Uuid;

    use crate::{common::test, sqlx::pg_outbox_message_repository::PgOutboxMessageRepository};

    #[tokio::test]
    async fn save_message() {
        let pool = test::create_sqlx_connection_pool().await;
        let repository = PgOutboxMessageRepository { pool };

        let message = OutboxMessage::customer_created_event(&create_customer()).unwrap();
        let result = repository.save(message).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn set_processed() {
        let repository = PgOutboxMessageRepository {
            pool: test::create_sqlx_connection_pool().await,
        };
        let message = save_unprocessed_message(&repository).await;
        let unprocessed_messages = count_unprocessed_messages(&repository).await;

        repository
            .set_processed(message.id(), Utc::now())
            .await
            .unwrap();

        assert_eq!(
            unprocessed_messages - 1,
            count_unprocessed_messages(&repository).await
        );
    }

    #[tokio::test]
    async fn find_unprocessed_messages() {
        let repository = PgOutboxMessageRepository {
            pool: test::create_sqlx_connection_pool().await,
        };
        let unsent_message = save_unprocessed_message(&repository).await;
        let sent_message = save_processed_message(&repository).await;

        let unsent_messages = repository.find_unprocessed().await.unwrap().unwrap();

        assert!(unsent_messages
            .iter()
            .any(|m| m.id() == unsent_message.id()));
        assert!(!unsent_messages.iter().any(|m| m.id() == sent_message.id()));
    }

    async fn save_unprocessed_message(repository: &PgOutboxMessageRepository) -> OutboxMessage {
        let message = OutboxMessage::customer_created_event(&create_customer()).unwrap();
        repository
            .save(message.clone())
            .await
            .expect("Error saving messages during test setup");
        message
    }

    async fn save_processed_message(repository: &PgOutboxMessageRepository) -> OutboxMessage {
        let mut message = OutboxMessage::customer_created_event(&create_customer()).unwrap();
        message.set_processed_at(Utc::now());
        repository
            .save(message.clone())
            .await
            .expect("Error saving messages during test setup");
        message
    }

    async fn count_unprocessed_messages(repository: &PgOutboxMessageRepository) -> isize {
        repository
            .find_unprocessed()
            .await
            .unwrap()
            .unwrap()
            .len()
            .try_into()
            .unwrap()
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
