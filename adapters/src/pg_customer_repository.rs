use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use domain::repositories::CustomerRepositoryError;
use uuid::Uuid;

use crate::schema;

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
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl From<Customer> for domain::entities::customer::Customer {
    fn from(value: Customer) -> Self {
        domain::entities::customer::Customer {
            id: domain::value_objects::CustomerId(value.id),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            address: value.address.into(),
        }
    }
}

impl From<Address> for domain::value_objects::Address {
    fn from(value: Address) -> Self {
        domain::value_objects::Address {
            street: value.street,
            city: value.city,
            zip_code: value.zip_code,
            state: value.state,
        }
    }
}

impl domain::repositories::CustomerRepository for PgCustomerRepository {
    fn find_by_id(
        &self,
        id: domain::value_objects::CustomerId,
    ) -> Result<Option<domain::entities::customer::Customer>, CustomerRepositoryError> {
        let mut connection = self.create_connection()?;

        let customer = schema::customers::dsl::customers
            .find(id.0)
            .select(Customer::as_select())
            .first(&mut connection)
            .map_err(|_| CustomerRepositoryError::CustomerNotFoundError)?;

        Ok(Some(customer.into()))
    }
}

impl PgCustomerRepository {
    fn create_connection(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>,
        CustomerRepositoryError,
    > {
        Ok(self
            .connection_pool
            .get()
            .map_err(|_| CustomerRepositoryError::ConnectionNotCreatedError)?)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        common,
        pg_customer_repository::{Address, Customer, PgCustomerRepository},
        schema,
    };
    use diesel::{
        pg::PgConnection,
        r2d2::{ConnectionManager, Pool},
        RunQueryDsl,
    };
    use domain::{repositories::CustomerRepository, value_objects::CustomerId};
    use uuid::Uuid;

    #[test]
    pub fn find_customer_by_id() {
        let connection_pool = common::test::create_connection_pool();
        let customer_id = save_a_customer_on_db(&connection_pool);
        let repository = PgCustomerRepository { connection_pool };

        let customer = repository
            .find_by_id(CustomerId(customer_id))
            .unwrap()
            .unwrap();

        assert_eq!(CustomerId(customer_id), customer.id);
        assert_eq!("John", customer.first_name);
        assert_eq!("Appleseed", customer.last_name);
        assert_eq!("22 Elm Street".to_string(), customer.address.street);
        assert_eq!("Castle Rock".to_string(), customer.address.city);
        assert_eq!("666".to_string(), customer.address.zip_code);
        assert_eq!("US".to_string(), customer.address.state);
    }

    fn save_a_customer_on_db(connection_pool: &Pool<ConnectionManager<PgConnection>>) -> Uuid {
        let customer_id = Uuid::new_v4();
        diesel::insert_into(schema::customers::table)
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
