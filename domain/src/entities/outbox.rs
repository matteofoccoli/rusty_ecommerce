use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::entities::{customer::Customer, order::Order};

#[derive(Clone, Debug, PartialEq)]
pub struct OutboxMessage {
    id: Uuid,
    event_type: String,
    event_payload: String,
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>,
}

impl OutboxMessage {
    pub fn new(
        id: Uuid,
        event_type: String,
        event_payload: String,
        created_at: DateTime<Utc>,
        processed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            event_type,
            event_payload,
            created_at,
            processed_at,
        }
    }

    pub fn customer_created_event(customer: &Customer) -> OutboxMessage {
        OutboxMessage {
            id: Uuid::new_v4(),
            event_type: "customer_created".to_string(),
            event_payload: customer_created_event_payload(customer),
            created_at: Utc::now(),
            processed_at: None,
        }
    }

    pub fn order_created_event(order: &Order) -> OutboxMessage {
        OutboxMessage {
            id: Uuid::new_v4(),
            event_type: "order_created".to_string(),
            event_payload: order_created_event_payload(order),
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

    pub fn set_processed_at(&mut self, processed_at: DateTime<Utc>) {
        self.processed_at = Some(processed_at);
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
    // TODO remove unwrap!
    serde_json::to_string(&customer_created_event).unwrap()
}

#[derive(Serialize)]
struct OrderCreatedEvent {
    id: String,
    customer_id: String,
}

fn order_created_event_payload(order: &Order) -> String {
    let order_created_event = OrderCreatedEvent {
        id: order.id.0.to_string(),
        customer_id: order.customer_id.0.to_string(),
    };
    // TODO remove unwrap!
    serde_json::to_string(&order_created_event).unwrap()
}
