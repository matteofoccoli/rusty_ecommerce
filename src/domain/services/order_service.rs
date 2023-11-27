use mockall::predicate::*;
use mockall::*;

use uuid::Uuid;

use crate::domain::{
    entities::{customer::Customer, order::Order},
    value_objects::{CustomerId, OrderId},
};

struct OrderService {
    pub customer_repository: Box<dyn CustomerRepository>,
}

#[automock]
trait CustomerRepository {
    fn find_by_id(&self, id: CustomerId) -> Option<Customer>;
}

impl OrderService {
    fn create_order(&self, order_id: Uuid, customer_id: Uuid) -> Result<Order, String> {
        if self
            .customer_repository
            .find_by_id(CustomerId(customer_id))
            .is_none()
        {
            return Err("Not existing customer".to_string());
        }
        Ok(Order::create(OrderId(order_id), CustomerId(customer_id)))
    }
}

#[cfg(test)]
mod test {
    use std::any;

    use mockall::predicate;
    use uuid::Uuid;

    use crate::domain::{
        entities::customer::Customer,
        services::order_service::{CustomerRepository, MockCustomerRepository},
        value_objects::{Address, CustomerId, OrderId},
    };

    use super::OrderService;

    #[test]
    fn create_a_new_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository.expect_find_by_id().returning(move |_| {
            Some(Customer {
                id: CustomerId(customer_id),
                first_name: "Mario".to_string(),
                last_name: "Luigi".to_string(),
                address: Address {
                    street: "street".to_string(),
                    city: "city".to_string(),
                    zip_code: "zip_code".to_string(),
                    state: "state".to_string(),
                },
            })
        });
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
        };

        let result = order_service.create_order(order_id, customer_id);

        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(OrderId(order_id), order.id);
        assert_eq!(CustomerId(customer_id), order.customer_id);
        assert_eq!(0, order.items.len());
    }

    #[test]
    fn return_error_if_customer_does_not_exist() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_find_by_id()
            .returning(move |_| None);
        let order_service = OrderService {
            customer_repository: Box::new(customer_repository),
        };

        let result = order_service.create_order(order_id, customer_id);

        assert!(result.is_err());
    }
}
