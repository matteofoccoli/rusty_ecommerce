use diesel::{
    r2d2::{ConnectionManager, Pool},
    Insertable, PgConnection, Queryable, RunQueryDsl, Selectable,
};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
}

pub struct PgOrderRepository {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl domain::repositories::OrderRepository for PgOrderRepository {
    fn save(&self, order: domain::entities::order::Order) -> Result<domain::entities::order::Order, String> {
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
}

#[cfg(test)]
mod test {
    use domain::repositories::OrderRepository;
    use uuid::Uuid;
    use crate::common;

    use super::PgOrderRepository;

    #[test]
    fn save_a_new_order() {
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
    }
}
