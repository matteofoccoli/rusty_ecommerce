use diesel::{
    r2d2::{ConnectionManager, Pool},
    Insertable, PgConnection, Queryable, RunQueryDsl, Selectable,
};
use uuid::Uuid;

use crate::domain::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
}

pub struct PgOrderRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl repositories::OrderRepository for PgOrderRepository {
    fn save(&self, order: entities::order::Order) -> Result<entities::order::Order, String> {
        use crate::schema::orders;
        match diesel::insert_into(orders::table)
            .values(&Order {
                id: order.id.0,
                customer_id: order.customer_id.0,
            })
            .execute(&mut self.connection_pool.get().unwrap())
        {
            Ok(_) => Ok(order),
            Err(_) => Err("Error saving order on DB".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        adapter::common,
        domain::{repositories::OrderRepository, *},
    };

    use super::PgOrderRepository;

    #[test]
    fn save_a_new_order() {
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let order = entities::order::Order::create(
            value_objects::OrderId(order_id),
            value_objects::CustomerId(customer_id),
        );
        let repository = PgOrderRepository {
            connection_pool: common::create_connection_pool(),
        };

        let result = repository.save(order);

        assert!(result.is_ok());
    }
}
