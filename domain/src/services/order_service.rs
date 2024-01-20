use uuid::Uuid;

use crate::{
    entities::order::Order,
    repositories::{CustomerRepository, OrderRepository},
    value_objects::{CustomerId, OrderId, OrderItem, ProductId},
};

pub struct OrderService {
    pub customer_repository: Box<dyn CustomerRepository>,
    pub order_repository: Box<dyn OrderRepository>,
}

impl OrderService {
    pub fn create_order(&self, order_id: &str, customer_id: &str) -> Result<Order, String> {
        let order_id =
            Uuid::parse_str(order_id).map_err(|_| "Unable to parse order id".to_string())?;
        let customer_id =
            Uuid::parse_str(customer_id).map_err(|_| "Unable to parse customer id".to_string())?;
        match self.customer_repository.find_by_id(CustomerId(customer_id)) {
            Ok(option) => match option {
                Some(_) => {
                    let order = Order::create(OrderId(order_id), CustomerId(customer_id));
                    match self.order_repository.save(order) {
                        Ok(order) => Ok(order),
                        Err(_) => Err("Error saving order".to_string()),
                    }
                }
                None => Err("Cannot find customer".to_string()),
            },
            Err(_) => Err("Error reading customer".to_string()),
        }
    }

    pub fn add_product(
        &self,
        order_id: &str,
        product_id: &str,
        price: f64,
        quantity: i32,
    ) -> Result<Order, String> {
        let order_id =
            Uuid::parse_str(order_id).map_err(|_| "Unable to parse order id".to_string())?;
        let product_id =
            Uuid::parse_str(product_id).map_err(|_| "Unable to parse product id".to_string())?;
        let mut order = match self.order_repository.find_by_id(OrderId(order_id)) {
            Ok(option) => match option {
                Some(order) => order,
                None => return Err("Cannot find order".to_string()),
            },
            Err(_) => return Err("Error reading order".to_string()),
        };
        order.add(OrderItem {
            price,
            quantity,
            product_id: ProductId(product_id),
        });
        self.order_repository.update(order)
    }
}

#[cfg(test)]
mod test {

    use uuid::Uuid;

    use crate::{
        entities::{customer::Customer, order::Order},
        repositories::{MockCustomerRepository, MockOrderRepository},
        value_objects::{Address, CustomerId, OrderId},
    };

    use super::OrderService;

    const ORDER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
    const CUSTOMER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
    const PRODUCT_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";

    #[test]
    fn creates_an_order_for_a_customer() {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository.expect_find_by_id().returning(move |_| {
            Ok(Some(Customer {
                id: CustomerId(Uuid::parse_str(CUSTOMER_ID).unwrap()),
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
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_save().once().returning(move |_| {
            Ok(Order::create(
                OrderId(Uuid::parse_str(ORDER_ID).unwrap()),
                CustomerId(Uuid::parse_str(CUSTOMER_ID).unwrap()),
            ))
        });
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
            order_repository: Box::new(order_repository),
        };

        let result = order_service.create_order(ORDER_ID, CUSTOMER_ID);

        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(OrderId(Uuid::parse_str(ORDER_ID).unwrap()), order.id);
        assert_eq!(
            CustomerId(Uuid::parse_str(CUSTOMER_ID).unwrap()),
            order.customer_id
        );
        assert_eq!(0, order.order_items.len());
    }

    #[test]
    fn cannot_create_an_order_without_a_customer() {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_find_by_id()
            .returning(move |_| Ok(None));
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_save().never();
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
            order_repository: Box::new(order_repository),
        };

        let result = order_service.create_order(ORDER_ID, CUSTOMER_ID);

        assert!(result.is_err());
    }

    #[test]
    fn adds_a_product_to_an_order() {
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_find_by_id().returning(move |_| {
            Ok(Some(Order {
                id: OrderId(Uuid::parse_str(ORDER_ID).unwrap()),
                customer_id: CustomerId(Uuid::new_v4()),
                order_items: vec![],
            }))
        });
        order_repository.expect_update().return_once(move |_| {
            Ok(Order {
                id: OrderId(Uuid::parse_str(ORDER_ID).unwrap()),
                customer_id: CustomerId(Uuid::new_v4()),
                order_items: vec![],
            })
        });
        let order_service = OrderService {
            customer_repository: Box::new(MockCustomerRepository::new()),
            order_repository: Box::new(order_repository),
        };

        let result = order_service.add_product(ORDER_ID, PRODUCT_ID, 10.0, 1);

        assert!(result.is_ok());
    }

    #[test]
    fn cannot_add_a_product_to_a_not_existing_order() {
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_find_by_id().returning(|_| Ok(None));
        let order_service = OrderService {
            customer_repository: Box::new(MockCustomerRepository::new()),
            order_repository: Box::new(order_repository),
        };

        let result = order_service.add_product(ORDER_ID, PRODUCT_ID, 10.0, 1);

        assert!(result.is_err());
    }

    #[test]
    fn cannot_add_a_product_if_there_is_an_infrastructural_failure() {
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_find_by_id().returning(|_| Err("Error".to_string()));
        let order_service = OrderService {
            customer_repository: Box::new(MockCustomerRepository::new()),
            order_repository: Box::new(order_repository)
        };

        let result = order_service.add_product(ORDER_ID, PRODUCT_ID, 10.0, 1);

        assert!(result.is_err());
    }


}
