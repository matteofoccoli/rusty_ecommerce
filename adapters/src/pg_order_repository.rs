use diesel::{
    r2d2::{ConnectionManager, Pool},
    Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper, ExpressionMethods,
};
use domain::value_objects::OrderId;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct OrderItem {
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub price: f64,
    pub quantity: i32,
}

pub struct PgOrderRepository {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl domain::repositories::OrderRepository for PgOrderRepository {
    fn save(
        &self,
        order: domain::entities::order::Order,
    ) -> Result<domain::entities::order::Order, String> {
        use crate::schema::orders;
        match &mut self.connection_pool.get() {
            Ok(connection) => {
                match diesel::insert_into(orders::table)
                    .values(&Order {
                        id: order.id.0,
                        customer_id: order.customer_id.0,
                    })
                    .execute(connection)
                {
                    Ok(_) => Ok(order),
                    Err(_) => Err(format!("Error saving order with id {} on DB", order.id.0)),
                }
            }
            Err(_) => Err("Error getting a DB connection from pool".to_string()),
        }
    }

    fn find_by_id(
        &self,
        searched_order_id: OrderId,
    ) -> Result<Option<domain::entities::order::Order>, String> {
        use crate::schema::{order_items, orders};
        let searched_order_id = searched_order_id.0;
        match &mut self.connection_pool.get() {
            Ok(connection) => {
                match orders::dsl::orders
                    .find(searched_order_id)
                    .select(Order::as_select())
                    .first(connection)
                {
                    Ok(order) => {
                        let mut order: domain::entities::order::Order = order.into();
                        match order_items::dsl::order_items
                            .filter(
                                order_items::dsl::order_id.eq(searched_order_id))
                            .select(OrderItem::as_select())
                            .get_results(connection)
                        {
                            Ok(order_items) => {
                                for order_item in order_items.iter() {
                                    println!("Adding order item");
                                    order.add(domain::value_objects::OrderItem {
                                        price: order_item.price,
                                        quantity: order_item.quantity,
                                        product_id: domain::value_objects::ProductId(
                                            order_item.product_id,
                                        ),
                                    })
                                }
                            }
                            _ => (),
                        }
                        Ok(Some(order))
                    }
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Err("Error getting a DB connection from pool".to_string()),
        }
    }

    fn update(
        &self,
        order: domain::entities::order::Order,
    ) -> Result<domain::entities::order::Order, String> {
        use crate::schema::order_items;
        match &mut self.connection_pool.get() {
            Ok(connection) => {
                for order_item in order.order_items.iter() {
                    let _ = diesel::insert_into(order_items::table)
                        .values(&OrderItem {
                            order_id: order.id.0,
                            product_id: order_item.product_id.0,
                            price: order_item.price,
                            quantity: order_item.quantity,
                        })
                        .execute(connection);
                }
            }
            Err(_) => return Err("Error getting a DB connection from pool".to_string()),
        }
        Ok(order)
    }
}

impl From<Order> for domain::entities::order::Order {
    fn from(value: Order) -> Self {
        domain::entities::order::Order {
            id: domain::value_objects::OrderId(value.id),
            customer_id: domain::value_objects::CustomerId(value.customer_id),
            order_items: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common;
    use domain::repositories::OrderRepository;
    use uuid::Uuid;

    use super::PgOrderRepository;

    #[test]
    fn saves_a_new_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let order = domain::entities::order::Order::create(
            domain::value_objects::OrderId(order_id),
            domain::value_objects::CustomerId(customer_id),
        );
        let repository = PgOrderRepository {
            connection_pool: common::test::create_connection_pool(),
        };

        let result = repository.save(order);

        assert!(result.is_ok());
        let order_from_db = repository
            .find_by_id(domain::value_objects::OrderId(order_id))
            .unwrap()
            .unwrap();
        assert_eq!(domain::value_objects::OrderId(order_id), order_from_db.id);
    }

    #[test]
    fn saves_order_items_for_an_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let repository = PgOrderRepository {
            connection_pool: common::test::create_connection_pool(),
        };
        let _ = repository.save(domain::entities::order::Order::create(
            domain::value_objects::OrderId(order_id),
            domain::value_objects::CustomerId(customer_id),
        ));

        let mut order = domain::entities::order::Order::create(
            domain::value_objects::OrderId(order_id),
            domain::value_objects::CustomerId(customer_id),
        );
        order.add_multiple(vec![domain::value_objects::OrderItem {
            price: 10.0,
            quantity: 1,
            product_id: domain::value_objects::ProductId(product_id),
        }]);
        let result = repository.update(order);

        assert!(result.is_ok());
        let order_from_db = repository
            .find_by_id(domain::value_objects::OrderId(order_id))
            .unwrap()
            .unwrap();
        assert_eq!(1, order_from_db.order_items.len());
    }
}
