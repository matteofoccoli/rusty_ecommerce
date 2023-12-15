use crate::value_objects::{CustomerId, OrderId, OrderItem};

pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub order_items: Vec<OrderItem>,
}

impl Order {
    pub fn create(id: OrderId, customer_id: CustomerId) -> Self {
        Self {
            id,
            customer_id,
            order_items: vec![],
        }
    }

    pub fn add(&mut self, order_item: OrderItem) {
        self.order_items.push(order_item);
    }

    pub fn add_multiple(&mut self, order_items: Vec<OrderItem>) {
        for order_item in order_items {
            self.add(order_item)
        }
    }

    pub fn total_price(self) -> f32 {
        let total: f32 = self
            .order_items
            .iter()
            .map(|x| x.price * x.quantity as f32)
            .sum();
        (total * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::value_objects::{CustomerId, OrderId, OrderItem, ProductId};

    use super::Order;

    #[test]
    fn create_an_order_for_a_customer() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

        let order = order_fixture(order_id, customer_id);

        assert_eq!(OrderId(order_id), order.id);
        assert_eq!(CustomerId(customer_id), order.customer_id);
    }

    #[test]
    fn add_items_to_order() {
        let mut order = order_fixture(Uuid::new_v4(), Uuid::new_v4());

        order.add_multiple(vec![
            order_item_fixture(9.99, 1, Uuid::new_v4()),
            order_item_fixture(5.55, 2, Uuid::new_v4()),
            order_item_fixture(7.77, 3, Uuid::new_v4()),
        ]);
        assert_eq!(3, order.order_items.len());
    }

    #[test]
    fn calculate_total_price_of_an_order() {
        let mut order = order_fixture(Uuid::new_v4(), Uuid::new_v4());

        order.add_multiple(vec![
            order_item_fixture(9.99, 10, Uuid::new_v4()),
            order_item_fixture(5.55, 2, Uuid::new_v4()),
        ]);

        assert_eq!(111.0, order.total_price());
    }

    fn order_fixture(id: Uuid, customer_id: Uuid) -> Order {
        Order::create(OrderId(id), CustomerId(customer_id))
    }

    fn order_item_fixture(price: f32, quantity: u32, product_id: Uuid) -> OrderItem {
        OrderItem {
            price,
            quantity,
            product_id: ProductId(product_id),
        }
    }
}
