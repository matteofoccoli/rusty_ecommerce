use std::fmt;

use uuid::Uuid;

use super::value_objects::Address;

pub struct Customer {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} (uuid: {})",
            self.last_name, self.first_name, self.id
        )
    }
}

pub struct Order {
    pub id: Uuid,
    pub customer: Customer,
    pub items: Vec<Item>,
}

impl Order {
    pub fn new(id: Uuid, customer: Customer) -> Self {
        Self {
            id,
            customer,
            items: vec!()
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn total_price(self) -> f32{
        let total: f32 = self.items.iter().map(|x| x.price * x.quantity as f32).sum();
        (total * 100.0).round() / 100.0
    }
}

pub struct Item {
    pub price: f32,
    pub quantity: u32,
}

impl Item {
    pub fn new(price: f32, quantity: u32) -> Self {
        Self {
            price,
            quantity
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::Address;

    use super::{Customer, Order, Item};
    use uuid::Uuid;

    #[test]
    fn create_a_customer() {
        let id = new_uuid();
        
        let customer = new_customer(id);
        
        assert_eq!(id, customer.id);
        assert_eq!("John".to_string(), customer.first_name);
        assert_eq!("Appleseed".to_string(), customer.last_name);
    }

    #[test]
    fn create_an_order() {
        let order_id = new_uuid();
        let customer_id = new_uuid();
        
        let order = new_order(order_id, customer_id);

        assert_eq!(order_id, order.id);
        assert_eq!(customer_id, order.customer.id);
    }

    #[test]
    fn add_items_to_order() {
        let mut order = new_order(new_uuid(), new_uuid());

        order.add(Item::new(9.99, 1));

        assert_eq!(1, order.items.len());
    }

    #[test]
    fn calculate_total_price_of_an_order() {
        let mut order = new_order(new_uuid(), new_uuid());

        order.add(Item::new(9.99, 10));
        order.add(Item::new(5.55, 2));

        assert_eq!(111.0, order.total_price());
    }

    fn new_order(id: Uuid, customer_id: Uuid) -> Order {
        Order::new(id, new_customer(customer_id))
    }

    fn new_customer(id: Uuid) -> Customer {
        Customer {
            id,
            first_name: "John".to_string(),
            last_name: "Appleseed".to_string(),
            address: Address {
                street: "22, Acacia Avenue".to_string(),
                city: "Minneapolis".to_string(),
                zip_code: "12345".to_string(),
                state: "Usa".to_string(),
            },
        }
    }

    fn new_uuid() -> Uuid {
        Uuid::new_v4()        
    }
}