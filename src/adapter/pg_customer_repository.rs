use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};

use uuid::Uuid;

use crate::domain::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Customer {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[diesel(embed)]
    pub address: Address,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub state: String,
}

pub struct PgCustomerRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl From<Customer> for entities::customer::Customer {
    fn from(value: Customer) -> Self {
        entities::customer::Customer {
            id: value_objects::CustomerId(value.id),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            address: value.address.into(),
        }
    }
}

impl From<Address> for value_objects::Address {
    fn from(value: Address) -> Self {
        value_objects::Address {
            street: value.street,
            city: value.city,
            zip_code: value.zip_code,
            state: value.state,
        }
    }
}

impl repositories::CustomerRepository for PgCustomerRepository {
    fn find_by_id(&self, id: value_objects::CustomerId) -> Option<entities::customer::Customer> {
        use crate::schema::customers;
        let conn = &mut self.connection_pool.get().unwrap();
        match customers::dsl::customers
            .find(id.0)
            .select(Customer::as_select())
            .first(conn)
        {
            Ok(customer) => Some(customer.into()),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {

    use diesel::{
        pg::PgConnection,
        r2d2::{ConnectionManager, Pool},
        Connection, RunQueryDsl,
    };
    use dotenvy::dotenv;
    use std::env;
    use uuid::Uuid;

    use crate::{
        adapter::pg_customer_repository::{Address, Customer, PgCustomerRepository},
        domain::{repositories::CustomerRepository, value_objects::CustomerId},
    };

    #[test]
    pub fn store_and_retrieve_customer() {
        use crate::schema::customers;

        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let mut conn = PgConnection::establish(&db_url).expect("Error connecting to DB");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let connection_pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");

        let customer_id = Uuid::new_v4();

        let customer = Customer {
            id: customer_id,
            first_name: "John".to_string(),
            last_name: "Appleseed".to_string(),
            address: Address {
                street: "22 Elm Street".to_string(),
                city: "Castle Rock".to_string(),
                zip_code: "666".to_string(),
                state: "US".to_string(),
            }
        };

        diesel::insert_into(customers::table)
            .values(&customer)
            .execute(&mut conn)
            .expect("Error saving customer on DB");

        let repository = PgCustomerRepository { connection_pool };

        let result = repository.find_by_id(CustomerId(customer_id));
        assert!(result.is_some());
        let customer = result.unwrap();
        assert_eq!(customer_id, customer.id.0);
        assert_eq!("John", customer.first_name);
        assert_eq!("Appleseed", customer.last_name);
    }
}
