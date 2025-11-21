use uuid::Uuid;

use crate::{
    entities::order::Order,
    repositories::customer_repository::CustomerRepository,
    repositories::order_repository::OrderRepository,
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
    pub customer_repository: Box<dyn CustomerRepository>,
    pub order_repository: Box<dyn OrderRepository>,
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
    pub async fn create_order(
        &self,
        create_order: CreateOrderRequestObject,
    ) -> Result<Order, OrderServiceError> {
        let order_id = Uuid::try_parse(&create_order.order_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;
        let customer_id = Uuid::try_parse(&create_order.customer_id)
            .map_err(|err| OrderServiceError::GenericError(err.to_string()))?;

        let customer = self
            .customer_repository
            .find_by_id(CustomerId(customer_id))
            .await
            .map_err(|_| OrderServiceError::CustomerNotReadError)?;

        if customer.is_none() {
            return Err(OrderServiceError::CustomerNotFoundError);
        } else {
            let order = Order::create(OrderId(order_id), CustomerId(customer_id));
            let saved_order = self
                .order_repository
                .save(order)
                .await
                .map_err(|_| OrderServiceError::OrderNotSavedError)?;
            return Ok(saved_order);
        }
    }

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
}

#[cfg(test)]
mod test {

    use uuid::Uuid;

    use crate::{
        entities::{customer::Customer, order::Order},
        repositories::{
            customer_repository::MockMyCustomerRepository,
            order_repository::{MockMyOrderRepository, OrderRepositoryError},
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
        order_repository.expect_save().once().returning(move |_| {
            Ok(Order::create(
                OrderId(Uuid::try_parse(ORDER_ID).unwrap()),
                CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
            ))
        });
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
            order_repository: Box::new(order_repository),
        };

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
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
            order_repository: Box::new(order_repository),
        };

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
        let order_service = OrderService {
            customer_repository: Box::new(MockMyCustomerRepository::new()),
            order_repository: Box::new(order_repository),
        };

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
        let order_service = OrderService {
            customer_repository: Box::new(MockMyCustomerRepository::new()),
            order_repository: Box::new(order_repository),
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

    #[tokio::test]
    async fn cannot_add_a_product_if_there_is_an_infrastructural_failure() {
        let mut order_repository = MockMyOrderRepository::new();
        order_repository
            .expect_find_by_id()
            .returning(|_| Err(OrderRepositoryError::OrderNotFoundError));
        let order_service = OrderService {
            customer_repository: Box::new(MockMyCustomerRepository::new()),
            order_repository: Box::new(order_repository),
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
