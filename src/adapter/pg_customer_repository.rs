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
    id: Uuid,
    first_name: String,
    last_name: String,
    #[diesel(embed)]
    address: Address,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    state: String,
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
    fn find_by_id(
        &self,
        id: value_objects::CustomerId,
    ) -> Result<Option<entities::customer::Customer>, String> {
        use crate::schema::customers;
        match &mut self.connection_pool.get() {
            Ok(connection) => {
                match customers::dsl::customers
                    .find(id.0)
                    .select(Customer::as_select())
                    .first(connection)
                {
                    Ok(customer) => Ok(Some(customer.into())),
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Err("Error getting a connection from pool".to_string()),
        }
    }
}

#[cfg(test)]
mod test {

    use diesel::{
        pg::PgConnection,
        r2d2::{ConnectionManager, Pool},
        RunQueryDsl,
    };
    use uuid::Uuid;

    use crate::{
        adapter::{pg_customer_repository::{Address, Customer, PgCustomerRepository}, common},
        domain::{repositories::CustomerRepository, value_objects::CustomerId},
    };

    #[test]
    pub fn find_customer_by_id() {
        let connection_pool = common::create_connection_pool();
        let customer_id = save_a_customer_on_db(&connection_pool);
        let repository = PgCustomerRepository { connection_pool };

        let customer = repository.find_by_id(CustomerId(customer_id)).unwrap().unwrap();
        
        assert_eq!(CustomerId(customer_id), customer.id);
        assert_eq!("John", customer.first_name);
        assert_eq!("Appleseed", customer.last_name);
        assert_eq!("22 Elm Street".to_string(), customer.address.street);
        assert_eq!("Castle Rock".to_string(), customer.address.city);
        assert_eq!("666".to_string(), customer.address.zip_code);
        assert_eq!("US".to_string(), customer.address.state);
    }

    fn save_a_customer_on_db(connection_pool: &Pool<ConnectionManager<PgConnection>>) -> Uuid {
        use crate::schema::customers;
        let customer_id = Uuid::new_v4();
        diesel::insert_into(customers::table)
            .values(&Customer {
                id: customer_id,
                first_name: "John".to_string(),
                last_name: "Appleseed".to_string(),
                address: Address {
                    street: "22 Elm Street".to_string(),
                    city: "Castle Rock".to_string(),
                    zip_code: "666".to_string(),
                    state: "US".to_string(),
                },
            })
            .execute(&mut connection_pool.get().unwrap())
            .expect("Error saving customer on DB");
        customer_id
    }

    
}
