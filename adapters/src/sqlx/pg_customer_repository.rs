use async_trait::async_trait;
use domain::{
    entities::customer::Customer,
    repositories::CustomerRepositoryError,
    value_objects::{Address, CustomerId},
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

pub struct PgCustomerRepository {
    pool: Pool<Postgres>,
}

#[async_trait]
impl domain::repositories::CustomerRepository for PgCustomerRepository {
    async fn find_by_id(
        &self,
        id: CustomerId,
    ) -> Result<Option<Customer>, CustomerRepositoryError> {
        let uuid = id.0;
        let customer = sqlx::query("SELECT * FROM customers where id = $1")
            .bind(uuid)
            .map(|row: PgRow| Customer {
                id: CustomerId(uuid),
                first_name: row.try_get("first_name").unwrap_or_default(),
                last_name: row.try_get("last_name").unwrap_or_default(),
                address: Address {
                    street: row.try_get("street").unwrap_or_default(),
                    city: row.try_get("city").unwrap_or_default(),
                    zip_code: row.try_get("zip_code").unwrap_or_default(),
                    state: row.try_get("state").unwrap_or_default(),
                },
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|_| CustomerRepositoryError::CustomerNotFoundError)?;
        return Ok(Some(customer));
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::common::test;
    use domain::{
        entities::customer::Customer,
        repositories::CustomerRepository,
        value_objects::{Address, CustomerId},
    };
    use sqlx::{Pool, Postgres};
    use uuid::Uuid;

    #[tokio::test]
    async fn find_customer_by_id() {
        let pool = test::create_sqlx_connection_pool().await;
        let customer_id = save_a_customer_on_db(&pool).await;
        let repository = PgCustomerRepository { pool };

        let customer = repository
            .find_by_id(CustomerId(customer_id))
            .await
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

    fn create_sample_customer(customer_id: Uuid) -> Customer {
        Customer {
            id: CustomerId(customer_id),
            first_name: "John".to_string(),
            last_name: "Appleseed".to_string(),
            address: Address {
                street: "22 Elm Street".to_string(),
                city: "Castle Rock".to_string(),
                zip_code: "666".to_string(),
                state: "US".to_string(),
            },
        }
    }

    async fn save_a_customer_on_db(pool: &Pool<Postgres>) -> Uuid {
        let customer_id = Uuid::new_v4();
        let customer = create_sample_customer(customer_id);

        sqlx::query(
            r#"
        INSERT INTO customers (id, first_name, last_name, street, city, zip_code, state) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(customer.id.0)
        .bind(customer.first_name)
        .bind(customer.last_name)
        .bind(customer.address.street)
        .bind(customer.address.city)
        .bind(customer.address.zip_code)
        .bind(customer.address.state)
        .execute(pool)
        .await
        .expect("Error saving test customer on DB");

        customer_id
    }
}
