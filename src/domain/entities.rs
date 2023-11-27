use std::fmt;

use uuid::Uuid;

use super::value_objects::{Address, CustomerId, OrderId, OrderItem};

pub struct Customer {
    pub id: CustomerId,
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} (uuid: {})",
            self.last_name, self.first_name, self.id.0
        )
    }
}

pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub items: Vec<OrderItem>,
}

impl Order {
    pub fn create(id: OrderId, customer_id: CustomerId) -> Self {
        Self {
            id,
            customer_id,
            items: vec![],
        }
    }

    pub fn add(&mut self, item: OrderItem) {
        self.items.push(item);
    }

    pub fn total_price(self) -> f32 {
        let total: f32 = self.items.iter().map(|x| x.price * x.quantity as f32).sum();
        (total * 100.0).round() / 100.0
    }
}

pub struct Product {
    pub id: Uuid,
    pub name: String,
}

impl Product {
}

#[cfg(test)]
mod test {
    use crate::domain::{
        entities::{Address},
        value_objects::{CustomerId, OrderId, ProductId},
    };

    use super::{Customer, OrderItem, Order};
    use uuid::Uuid;

    #[test]
    fn create_a_customer() {
        let id = new_uuid();

        let customer = customer_fixture(id);

        assert_eq!(CustomerId(id), customer.id);
        assert_eq!("John".to_string(), customer.first_name);
        assert_eq!("Appleseed".to_string(), customer.last_name);
    }

    #[test]
    fn create_an_order_for_a_customer() {
        let order_id = new_uuid();
        let customer_id = new_uuid();

        let order = order_fixture(order_id, customer_id);

        assert_eq!(OrderId(order_id), order.id);
        assert_eq!(CustomerId(customer_id), order.customer_id);
    }

    #[test]
    fn add_items_to_order() {
        let mut order = order_fixture(new_uuid(), new_uuid());

        order.add(order_item_fixture(9.99, 1, new_uuid()));
        order.add(order_item_fixture(5.55, 2, new_uuid()));
        order.add(order_item_fixture(7.77, 3, new_uuid()));

        assert_eq!(3, order.items.len());
    }

    #[test]
    fn calculate_total_price_of_an_order() {
        let mut order = order_fixture(new_uuid(), new_uuid());

        order.add(order_item_fixture(9.99, 10, new_uuid()));
        order.add(order_item_fixture(5.55, 2, new_uuid()));

        assert_eq!(111.0, order.total_price());
    }

    fn order_item_fixture(price: f32, quantity: u32, product_id: Uuid) -> OrderItem {
        OrderItem {
            price,
            quantity,
            product_id: ProductId(product_id),
        }
    }

    fn order_fixture(id: Uuid, customer_id: Uuid) -> Order {
        Order::create(OrderId(id), CustomerId(customer_id))
    }

    fn customer_fixture(id: Uuid) -> Customer {
        Customer {
            id: CustomerId(id),
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
