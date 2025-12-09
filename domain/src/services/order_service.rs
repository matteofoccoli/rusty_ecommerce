use uuid::Uuid;

use crate::{
    entities::{order::Order, outbox::OutboxMessage},
    repositories::{
        customer_repository::CustomerRepository, order_repository::OrderRepository,
        outbox_repository::OutboxMessageRepository,
    },
    value_objects::{CustomerId, OrderId, OrderItem, ProductId},
};

#[derive(Debug)]
pub enum OrderServiceError {
    CustomerNotFoundError,
    CustomerNotReadError,
    OrderNotFoundError,
    OrderNotReadError,
    OrderNotSavedError,
    GenericError(String),
}

impl std::fmt::Display for OrderServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderServiceError::CustomerNotFoundError => write!(f, "Customer not found error"),
            OrderServiceError::CustomerNotReadError => write!(f, "Customer not read error"),
            OrderServiceError::OrderNotFoundError => write!(f, "Order not found error"),
            OrderServiceError::OrderNotReadError => write!(f, "Order not read error"),
            OrderServiceError::OrderNotSavedError => write!(f, "Order not saved error"),
            OrderServiceError::GenericError(error) => write!(f, "Generic error: ${error}"),
        }
    }
}

impl std::error::Error for OrderServiceError {}

pub struct OrderService {
    customer_repository: Box<dyn CustomerRepository>,
    order_repository: Box<dyn OrderRepository>,
    outbox_message_repository: Box<dyn OutboxMessageRepository>,
}

pub struct AddProductRequestObject {
    order_id: String,
    product_id: String,
    price: f64,
    quantity: i32,
}

pub struct CreateOrderRequestObject {
    pub order_id: String,
    pub customer_id: String,
}

impl OrderService {
    pub fn new(
        customer_repository: Box<dyn CustomerRepository>,
        order_repository: Box<dyn OrderRepository>,
        outbox_message_repository: Box<dyn OutboxMessageRepository>,
    ) -> Self {
        Self {
            customer_repository,
            order_repository,
            outbox_message_repository,
        }
    }

    pub async fn create_order(
        &mut self,
        create_order: CreateOrderRequestObject,
    ) -> Result<Order, OrderServiceError> {
        let order_id = Uuid::try_parse(&create_order.order_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;
        let customer_id = Uuid::try_parse(&create_order.customer_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;

        self.begin_transaction().await?;

        let customer = self
            .customer_repository
            .find_by_id(CustomerId(customer_id))
            .await
            .map_err(|_| OrderServiceError::CustomerNotReadError)?;
        if customer.is_none() {
            self.rollback_transaction().await?;
            return Err(OrderServiceError::CustomerNotFoundError);
        }

        let order = Order::create(OrderId(order_id), CustomerId(customer_id));
        let saved_order = match self.order_repository.save(order).await {
            Ok(order) => order,
            Err(_) => {
                self.rollback_transaction().await?;
                return Err(OrderServiceError::OrderNotSavedError);
            }
        };

        let message = match OutboxMessage::order_created_event(&saved_order) {
            Ok(message) => message,
            Err(_) => {
                self.rollback_transaction().await?;
                return Err(OrderServiceError::GenericError(
                    "Error serializing outbox message".to_string(),
                ));
            }
        };

        match self.outbox_message_repository.save(message).await {
            Ok(_) => (),
            Err(_) => {
                self.rollback_transaction().await?;
                return Err(OrderServiceError::GenericError(
                    "Outbox message not saved".to_string(),
                ));
            }
        }

        self.commit_transaction().await?;

        return Ok(saved_order);
    }

    // TODO save outbox event also in this case
    pub async fn add_product(
        &self,
        add_product: AddProductRequestObject,
    ) -> Result<Order, OrderServiceError> {
        let order_id = Uuid::try_parse(&add_product.order_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;
        let product_id = Uuid::try_parse(&add_product.product_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;

        match self
            .order_repository
            .find_by_id(OrderId(order_id))
            .await
            .map_err(|_| OrderServiceError::OrderNotReadError)?
        {
            Some(mut order) => {
                order.add(OrderItem {
                    price: add_product.price,
                    quantity: add_product.quantity,
                    product_id: ProductId(product_id),
                });
                return self
                    .order_repository
                    .update(order)
                    .await
                    .map_err(|_| OrderServiceError::OrderNotSavedError);
            }
            None => return Err(OrderServiceError::OrderNotFoundError),
        }
    }

    async fn begin_transaction(&mut self) -> Result<(), OrderServiceError> {
        self.order_repository
            .begin_transaction()
            .await
            .map_err(|e| OrderServiceError::GenericError(e.to_string()))
    }

    async fn commit_transaction(&mut self) -> Result<(), OrderServiceError> {
        self.order_repository
            .commit_transaction()
            .await
            .map_err(|e| OrderServiceError::GenericError(e.to_string()))
    }

    async fn rollback_transaction(&mut self) -> Result<(), OrderServiceError> {
        self.order_repository
            .rollback_transaction()
            .await
            .map_err(|e| OrderServiceError::GenericError(e.to_string()))
    }
}

#[cfg(test)]
mod test {

    use uuid::Uuid;

    use crate::{
        entities::{
            customer::Customer,
            order::Order,
            outbox::{OutboxMessage, OutboxMessageType},
        },
        repositories::{
            customer_repository::MockMyCustomerRepository,
            order_repository::{MockMyOrderRepository, OrderRepositoryError},
            outbox_repository::MockOutboxMessageRepository,
        },
        services::order_service::{AddProductRequestObject, CreateOrderRequestObject},
        value_objects::{Address, CustomerId, OrderId},
    };

    use super::OrderService;

    const ORDER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
    const CUSTOMER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
    const PRODUCT_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";

    #[tokio::test]
    async fn creates_an_order_for_a_customer() {
        let saved_order = Order::create(
            OrderId(Uuid::try_parse(ORDER_ID).unwrap()),
            CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
        );
        let saved_outbox_message = OutboxMessage::order_created_event(&saved_order).unwrap();
        let expected_event_payload = saved_outbox_message.event_payload();

        let mut customer_repository = MockMyCustomerRepository::new();
        customer_repository.expect_find_by_id().returning(move |_| {
            Ok(Some(Customer {
                id: CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
                first_name: "Mario".to_string(),
                last_name: "Luigi".to_string(),
                address: Address {
                    street: "street".to_string(),
                    city: "city".to_string(),
                    zip_code: "zip_code".to_string(),
                    state: "state".to_string(),
                },
            }))
        });

        let mut order_repository = MockMyOrderRepository::new();
        order_repository
            .expect_save()
            .once()
            .return_once(|_| Ok(saved_order));
        order_repository
            .expect_begin_transaction()
            .once()
            .returning(|| Ok(()));
        order_repository
            .expect_commit_transaction()
            .once()
            .returning(|| Ok(()));
        order_repository.expect_rollback_transaction().never();

        let mut outbox_message_repository = MockOutboxMessageRepository::new();
        outbox_message_repository
            .expect_save()
            .withf(move |m| {
                m.event_type() == OutboxMessageType::OrderCreated
                    && m.processed_at().is_none()
                    && m.event_payload() == expected_event_payload
            })
            .once()
            .return_once(|_| Ok(saved_outbox_message));

        let mut order_service = OrderService::new(
            Box::new(customer_repository),
            Box::new(order_repository),
            Box::new(outbox_message_repository),
        );

        let result = order_service
            .create_order(CreateOrderRequestObject {
                order_id: ORDER_ID.to_string(),
                customer_id: CUSTOMER_ID.to_string(),
            })
            .await;

        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(OrderId(Uuid::try_parse(ORDER_ID).unwrap()), order.id);
        assert_eq!(
            CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
            order.customer_id
        );
        assert_eq!(0, order.order_items.len());
    }

    #[tokio::test]
    async fn cannot_create_an_order_without_a_customer() {
        let mut customer_repository = MockMyCustomerRepository::new();
        customer_repository
            .expect_find_by_id()
            .returning(move |_| Ok(None));

        let mut order_repository = MockMyOrderRepository::new();
        order_repository.expect_save().never();
        order_repository
            .expect_begin_transaction()
            .once()
            .returning(|| Ok(()));
        order_repository.expect_commit_transaction().never();
        order_repository
            .expect_rollback_transaction()
            .once()
            .returning(|| Ok(()));

        let mut outbox_message_repository = MockOutboxMessageRepository::new();
        outbox_message_repository.expect_save().never();

        let mut order_service = OrderService::new(
            Box::new(customer_repository),
            Box::new(order_repository),
            Box::new(outbox_message_repository),
        );

        let result = order_service
            .create_order(CreateOrderRequestObject {
                order_id: ORDER_ID.to_string(),
                customer_id: CUSTOMER_ID.to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn adds_a_product_to_an_order() {
        let mut order_repository = MockMyOrderRepository::new();
        order_repository.expect_find_by_id().returning(move |_| {
            Ok(Some(Order {
                id: OrderId(Uuid::try_parse(ORDER_ID).unwrap()),
                customer_id: CustomerId(Uuid::new_v4()),
                order_items: vec![],
            }))
        });
        order_repository.expect_update().return_once(move |_| {
            Ok(Order {
                id: OrderId(Uuid::try_parse(ORDER_ID).unwrap()),
                customer_id: CustomerId(Uuid::new_v4()),
                order_items: vec![],
            })
        });

        let outbox_message_repository = MockOutboxMessageRepository::new();

        let customer_repository = MockMyCustomerRepository::new();

        let order_service = OrderService::new(
            Box::new(customer_repository),
            Box::new(order_repository),
            Box::new(outbox_message_repository),
        );

        let result = order_service
            .add_product(AddProductRequestObject {
                order_id: ORDER_ID.to_string(),
                product_id: PRODUCT_ID.to_string(),
                price: 10.0,
                quantity: 1,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn cannot_add_a_product_to_a_not_existing_order() {
        let mut order_repository = MockMyOrderRepository::new();
        order_repository.expect_find_by_id().returning(|_| Ok(None));

        let order_service = OrderService::new(
            Box::new(MockMyCustomerRepository::new()),
            Box::new(order_repository),
            Box::new(MockOutboxMessageRepository::new()),
        );

        let result = order_service
            .add_product(AddProductRequestObject {
                order_id: ORDER_ID.to_string(),
                product_id: PRODUCT_ID.to_string(),
                price: 10.0,
                quantity: 1,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn cannot_add_a_product_if_there_is_an_infrastructural_failure() {
        let mut order_repository = MockMyOrderRepository::new();
        order_repository
            .expect_find_by_id()
            .returning(|_| Err(OrderRepositoryError::OrderNotFoundError));
        let order_service = OrderService {
            customer_repository: Box::new(MockMyCustomerRepository::new()),
            order_repository: Box::new(order_repository),
            outbox_message_repository: Box::new(MockOutboxMessageRepository::new()),
        };

        let result = order_service
            .add_product(AddProductRequestObject {
                order_id: ORDER_ID.to_string(),
                product_id: PRODUCT_ID.to_string(),
                price: 10.0,
                quantity: 1,
            })
            .await;

        assert!(result.is_err());
    }
}
