use mockall::automock;

use crate::value_objects::OrderId;

use super::{
    entities::{customer::Customer, order::Order},
    value_objects::CustomerId,
};

#[automock]
pub trait CustomerRepository {
    fn find_by_id(&self, id: CustomerId) -> Result<Option<Customer>, String>;
}

#[automock]
pub trait OrderRepository {
    fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, String>;

    fn save(&self, order: Order) -> Result<Order, String>;

    fn update(&self, order: Order) -> Result<Order, String>;
}
