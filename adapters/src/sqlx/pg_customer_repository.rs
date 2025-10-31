use async_trait::async_trait;
use domain::{
    entities::customer::Customer,
    repositories::{
        transactional_repository::TransactionalRepositoryError, customer_repository::CustomerRepositoryError,
    },
    value_objects::{Address, CustomerId},
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

pub struct PgCustomerRepository {
    pool: Pool<Postgres>,
}

impl PgCustomerRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl domain::repositories::transactional_repository::TransactionalRepository for PgCustomerRepository {
    async fn begin_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("BEGIN")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::BeginTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
    async fn commit_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::CommitTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
    async fn rollback_transaction(&self) -> Result<(), TransactionalRepositoryError> {
        sqlx::query("ROLLBACK")
            .execute(&self.pool)
            .await
            .map_err(|e| TransactionalRepositoryError::RollbackTransactionError(e.to_string()))
            .map(|_| Ok(()))?
    }
}

#[async_trait]
impl domain::repositories::customer_repository::CustomerRepository for PgCustomerRepository {
    async fn find_by_id(
        &self,
        id: CustomerId,
    ) -> Result<Option<Customer>, CustomerRepositoryError> {
        let uuid = id.0;
        let customer = sqlx::query("SELECT * FROM customers where id = $1")
            .bind(uuid)
            .try_map(|row: PgRow| {
                Ok(Customer {
                    id: CustomerId(uuid),
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    address: Address {
                        street: row.try_get("street")?,
                        city: row.try_get("city")?,
                        zip_code: row.try_get("zip_code")?,
                        state: row.try_get("state")?,
                    },
                })
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|_| CustomerRepositoryError::CustomerNotFoundError)?;
        return Ok(Some(customer));
    }

    async fn save(&self, customer: Customer) -> Result<Customer, CustomerRepositoryError> {
        sqlx::query(
            r#"
        INSERT INTO customers (id, first_name, last_name, street, city, zip_code, state)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id) DO UPDATE
        SET first_name = EXCLUDED.first_name,
            last_name = EXCLUDED.last_name,
            street = EXCLUDED.street,
            city = EXCLUDED.city,
            zip_code = EXCLUDED.zip_code,
            state = EXCLUDED.state
        "#,
        )
        .bind(customer.id.0)
        .bind(&customer.first_name)
        .bind(&customer.last_name)
        .bind(&customer.address.street)
        .bind(&customer.address.city)
        .bind(&customer.address.zip_code)
        .bind(&customer.address.state)
        .execute(&self.pool)
        .await
        .map_err(|_| CustomerRepositoryError::CustomerNotSavedError)?;

        Ok(customer)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::common::test;
    use domain::{
        entities::customer::Customer,
        repositories::customer_repository::CustomerRepository,
        value_objects::{Address, CustomerId},
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn save_customer() {
        let pool = test::create_sqlx_connection_pool().await;

        let repository = PgCustomerRepository { pool };

        let customer_id = Uuid::new_v4();
        let result = repository.save(create_sample_customer(customer_id)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn find_customer_by_id() {
        let pool = test::create_sqlx_connection_pool().await;
        let customer_id = Uuid::new_v4();
        let customer = create_sample_customer(customer_id);
        let repository = PgCustomerRepository { pool };
        repository
            .save(customer)
            .await
            .expect("Error saving customer");

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
}
