use uuid::Uuid;

use crate::{repositories::{CustomerRepository, OrderRepository}, entities::order::Order, value_objects::{CustomerId, OrderId}};

pub struct OrderService {
    pub customer_repository: Box<dyn CustomerRepository>,
    pub order_repository: Box<dyn OrderRepository>,
}

impl OrderService {
    pub fn create_order(&self, order_id: &str, customer_id: &str) -> Result<Order, String> {
        let order_id = Uuid::parse_str(order_id).map_err(|_| "Unable to parse order id".to_string())?;
        let customer_id = Uuid::parse_str(customer_id).map_err(|_| "Unable to parse customer id".to_string())?;
        match self.customer_repository.find_by_id(CustomerId(customer_id)) {
            Ok(result) => match result {
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

    #[test]
    fn create_a_new_order() {
        let order_id = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
        let customer_id = "2dfdeb10-8e05-11ee-8841-130a6fed61ad";

        let order_service = OrderService {
            customer_repository: Box::new(mock_customer_repository_returning_a_customer(
                Uuid::parse_str(customer_id).unwrap(),
            )),
            order_repository: Box::new(mock_order_repository_saving_an_order(
                Uuid::parse_str(order_id).unwrap(),
                Uuid::parse_str(customer_id).unwrap(),
            )),
        };

        let result = order_service.create_order(order_id, customer_id);
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(OrderId(Uuid::parse_str(order_id).unwrap()), order.id);
        assert_eq!(CustomerId(Uuid::parse_str(customer_id).unwrap()), order.customer_id);
        assert_eq!(0, order.items.len());
    }

    #[test]
    fn return_error_if_customer_does_not_exist() {
        let order_id = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";
        let customer_id = "2dfdeb10-8e05-11ee-8841-130a6fed61ad";

        let order_service = OrderService {
            customer_repository: Box::new(mock_customer_repository_returning_none()),
            order_repository: Box::new(mock_order_repository_not_saving_an_order()),
        };

        let result = order_service.create_order(order_id, customer_id);
        assert!(result.is_err());
    }

    fn mock_customer_repository_returning_none() -> MockCustomerRepository {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_find_by_id()
            .returning(move |_| Ok(None));
        customer_repository
    }

    fn mock_customer_repository_returning_a_customer(customer_id: Uuid) -> MockCustomerRepository {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository.expect_find_by_id().returning(move |_| {
            Ok(Some(Customer {
                id: CustomerId(customer_id),
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
        customer_repository
    }

    fn mock_order_repository_saving_an_order(
        order_id: Uuid,
        customer_id: Uuid,
    ) -> MockOrderRepository {
        let mut order_repository = MockOrderRepository::new();
        order_repository
            .expect_save()
            .once()
            .returning(move |_| Ok(Order::create(OrderId(order_id), CustomerId(customer_id))));
        order_repository
    }

    fn mock_order_repository_not_saving_an_order() -> MockOrderRepository {
        let mut order_repository = MockOrderRepository::new();
        order_repository.expect_save().never();
        order_repository
    }
}
