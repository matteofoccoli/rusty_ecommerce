use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct OutboxMessage {
    pub id: Uuid,
    pub event_type: String,
    pub event_payload: String,
    pub created_at: DateTime<Utc>,
    pub processed_at: DateTime<Utc>,
}
