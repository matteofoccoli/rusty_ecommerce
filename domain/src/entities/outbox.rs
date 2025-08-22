use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::entities::customer::Customer;

#[derive(Clone, Debug)]
pub struct OutboxMessage {
    id: Uuid,
    event_type: String,
    event_payload: String,
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>,
}

impl OutboxMessage {
    pub fn customer_created_event(customer: &Customer) -> OutboxMessage {
        OutboxMessage {
            id: Uuid::new_v4(),
            event_type: "customer_created".to_string(),
            event_payload: customer_created_event_payload(customer),
            created_at: Utc::now(),
            processed_at: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn event_type(&self) -> String {
        self.event_type.clone()
    }
    pub fn event_payload(&self) -> String {
        self.event_payload.clone()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn processed_at(&self) -> Option<DateTime<Utc>> {
        self.processed_at
    }
}

#[derive(Serialize)]
struct CustomerCreatedEvent {
    id: String,
    first_name: String,
    last_name: String,
}

fn customer_created_event_payload(customer: &Customer) -> String {
    let customer_created_event = CustomerCreatedEvent {
        id: customer.id.0.to_string(),
        first_name: customer.first_name.clone(),
        last_name: customer.last_name.clone(),
    };
    serde_json::to_string(&customer_created_event).unwrap()
}
