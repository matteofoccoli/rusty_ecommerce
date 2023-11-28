use mockall::automock;

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
    fn save(&self, order: Order) -> Result<Order, String>;
}
