use async_trait::async_trait;
use domain::{
    entities::outbox::OutboxMessage, repositories::outbox_repository::OutboxMessageRepositoryError,
};
use sqlx::{Pool, Postgres};

pub struct PgOutboxMessageRepository {
    pub pool: Pool<Postgres>,
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
            INSERT INTO outbox_messages (id, event_type, event_payload, created_at)
            VALUES ($1, $2, $3, $4) 
        "#,
        )
        .bind(&message.id())
        .bind(&message.event_type())
        .bind(&message.event_payload())
        .bind(&message.created_at())
        .execute(&self.pool)
        .await
        .map_err(|error| {
            OutboxMessageRepositoryError::OutboxMessageNotSavedError(error.to_string())
        })?;

        Ok(message)
    }
}

#[cfg(test)]
mod test {
    use domain::{
        entities::{customer::Customer, outbox::OutboxMessage},
        repositories::outbox_repository::OutboxMessageRepository,
        value_objects::{Address, CustomerId},
    };
    use uuid::Uuid;

    use crate::{common::test, sqlx::pg_outbox_message_repository::PgOutboxMessageRepository};

    #[tokio::test]
    async fn save_outbox_message() {
        let pool = test::create_sqlx_connection_pool().await;
        let repository = PgOutboxMessageRepository { pool };

        let customer = Customer {
            id: CustomerId(Uuid::new_v4()),
            first_name: "my_customer_first_name".to_string(),
            last_name: "my_customer_last_name".to_string(),
            address: Address {
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            },
        };
        let message = OutboxMessage::customer_created_event(&customer);

        let result = repository.save(message).await;

        assert!(result.is_ok());
    }
}
