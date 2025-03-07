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

    pub fn total_price(self) -> f64 {
        let total: f64 = self
            .order_items
            .iter()
            .map(|x| x.price * x.quantity as f64)
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

        let order = Order::create(OrderId(order_id), CustomerId(customer_id));

        assert_eq!(OrderId(order_id), order.id);
        assert_eq!(CustomerId(customer_id), order.customer_id);
    }

    #[test]
    fn add_items_to_order() {
        let mut order = Order::create(OrderId(Uuid::new_v4()), CustomerId(Uuid::new_v4()));

        order.add_multiple(vec![
            OrderItem {
                price: 9.99,
                quantity: 1,
                product_id: ProductId(Uuid::new_v4()),
            },
            OrderItem {
                price: 5.55,
                quantity: 2,
                product_id: ProductId(Uuid::new_v4()),
            },
            OrderItem {
                price: 7.77,
                quantity: 3,
                product_id: ProductId(Uuid::new_v4()),
            },
        ]);
        assert_eq!(3, order.order_items.len());
    }

    #[test]
    fn calculate_total_price_of_an_order() {
        let mut order = Order::create(OrderId(Uuid::new_v4()), CustomerId(Uuid::new_v4()));

        order.add_multiple(vec![
            OrderItem {
                price: 9.99,
                quantity: 10,
                product_id: ProductId(Uuid::new_v4()),
            },
            OrderItem {
                price: 5.55,
                quantity: 2,
                product_id: ProductId(Uuid::new_v4()),
            },
        ]);

        assert_eq!(111.0, order.total_price());
    }
}
