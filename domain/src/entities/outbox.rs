use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::entities::{customer::Customer, order::Order};
use crate::value_objects::{OrderId, ProductId};

#[derive(Debug)]
pub enum OutboxMessageError {
    PayloadSerializationError(String),
}

impl std::fmt::Display for OutboxMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboxMessageError::PayloadSerializationError(error) => {
                write!(f, "Payload serialization error: ${error}")
            }
        }
    }
}

impl std::error::Error for OutboxMessageError {}

#[derive(Clone, Debug, PartialEq)]
pub enum OutboxMessageType {
    OrderCreated,
    CustomerCreated,
    ProductAddedToOrder,
}

const ORDER_CREATED: &str = "order_created";
const CUSTOMER_CREATED: &str = "customer_created";
const PRODUCT_ADDED_TO_ORDER: &str = "product_added_to_order";

impl std::fmt::Display for OutboxMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboxMessageType::OrderCreated => write!(f, "{}", ORDER_CREATED),
            OutboxMessageType::CustomerCreated => write!(f, "{}", CUSTOMER_CREATED),
            OutboxMessageType::ProductAddedToOrder => write!(f, "{}", PRODUCT_ADDED_TO_ORDER),
        }
    }
}

impl std::str::FromStr for OutboxMessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ORDER_CREATED => Ok(OutboxMessageType::OrderCreated),
            CUSTOMER_CREATED => Ok(OutboxMessageType::CustomerCreated),
            PRODUCT_ADDED_TO_ORDER => Ok(OutboxMessageType::ProductAddedToOrder),
            _ => Err(format!("Unknown outbox message type: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutboxMessage {
    id: Uuid,
    event_type: OutboxMessageType,
    event_payload: String,
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>,
}

impl OutboxMessage {
    pub fn new(
        id: Uuid,
        event_type: OutboxMessageType,
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

    pub fn customer_created_event(
        customer: &Customer,
    ) -> Result<OutboxMessage, OutboxMessageError> {
        let event_payload = customer_created_event_payload(customer)?;
        Ok(OutboxMessage {
            id: Uuid::new_v4(),
            event_type: OutboxMessageType::CustomerCreated,
            event_payload,
            created_at: Utc::now(),
            processed_at: None,
        })
    }

    pub fn order_created_event(order: &Order) -> Result<OutboxMessage, OutboxMessageError> {
        let event_payload = order_created_event_payload(order)?;
        Ok(OutboxMessage {
            id: Uuid::new_v4(),
            event_type: OutboxMessageType::OrderCreated,
            event_payload,
            created_at: Utc::now(),
            processed_at: None,
        })
    }

    pub fn product_added_to_order_event(
        order_id: &OrderId,
        product_id: &ProductId,
        price: f64,
        quantity: i32,
    ) -> Result<OutboxMessage, OutboxMessageError> {
        let event_payload = product_added_to_order_event_payload(
            order_id.0.to_string(),
            product_id.0.to_string(),
            price,
            quantity,
        )?;
        Ok(OutboxMessage {
            id: Uuid::new_v4(),
            event_type: OutboxMessageType::ProductAddedToOrder,
            event_payload,
            created_at: Utc::now(),
            processed_at: None,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn event_type(&self) -> OutboxMessageType {
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

fn customer_created_event_payload(customer: &Customer) -> Result<String, OutboxMessageError> {
    let event = CustomerCreatedEvent {
        id: customer.id.0.to_string(),
        first_name: customer.first_name.clone(),
        last_name: customer.last_name.clone(),
    };
    serde_json::to_string(&event)
        .map_err(|e| OutboxMessageError::PayloadSerializationError(e.to_string()))
}

#[derive(Serialize)]
struct OrderCreatedEvent {
    id: String,
    customer_id: String,
}

fn order_created_event_payload(order: &Order) -> Result<String, OutboxMessageError> {
    let event = OrderCreatedEvent {
        id: order.id.0.to_string(),
        customer_id: order.customer_id.0.to_string(),
    };
    serde_json::to_string(&event)
        .map_err(|e| OutboxMessageError::PayloadSerializationError(e.to_string()))
}

#[derive(Serialize)]
struct ProductAddedToOrderEvent {
    order_id: String,
    product_id: String,
    quantity: i32,
    price: f64,
}

fn product_added_to_order_event_payload(
    order_id: String,
    product_id: String,
    price: f64,
    quantity: i32,
) -> Result<String, OutboxMessageError> {
    let event = ProductAddedToOrderEvent {
        order_id,
        product_id,
        price,
        quantity,
    };
    serde_json::to_string(&event)
        .map_err(|e| OutboxMessageError::PayloadSerializationError(e.to_string()))
}
