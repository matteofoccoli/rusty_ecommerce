use uuid::Uuid;

use crate::{
    entities::customer::Customer,
    repositories::CustomerRepository,
    value_objects::{Address, CustomerId},
};

#[derive(Debug)]
pub enum CustomerServiceError {
    CustomerNotSavedError,
    GenericError(String),
}

impl std::fmt::Display for CustomerServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerServiceError::CustomerNotSavedError => write!(f, "Customer not saved error"),
            CustomerServiceError::GenericError(error) => write!(f, "Generic error: ${error}"),
        }
    }
}

impl std::error::Error for CustomerServiceError {}

pub struct CreateCustomerRequestObject {
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub state: String,
}

pub struct CustomerService {
    pub customer_repository: Box<dyn CustomerRepository>,
}

impl CustomerService {
    pub async fn create_customer(
        &self,
        request: CreateCustomerRequestObject,
    ) -> Result<Customer, CustomerServiceError> {
        let customer_id = CustomerId(Uuid::new_v4());
        let first_name = request.first_name;
        let last_name = request.last_name;
        let address = Address {
            street: request.street,
            city: request.city,
            zip_code: request.zip_code,
            state: request.state,
        };
        let customer = Customer::new(customer_id, first_name, last_name, address);

        let saved_customer = self
            .customer_repository
            .save(customer)
            .await
            .map_err(|_| CustomerServiceError::CustomerNotSavedError)?;

        return Ok(saved_customer);
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        entities::customer::Customer,
        repositories::MockCustomerRepository,
        services::customer_service::{CreateCustomerRequestObject, CustomerService},
        value_objects::{Address, CustomerId},
    };

    #[tokio::test]
    async fn creates_a_customer() {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_save()
            .returning(|_| {
                Ok(Customer {
                    id: CustomerId(Uuid::new_v4()),
                    first_name: "my_customer_first_name".to_string(),
                    last_name: "my_customer_last_name".to_string(),
                    address: Address {
                        street: "my_customer_street".to_string(),
                        city: "my_customer_city".to_string(),
                        zip_code: "my_customer_zip_code".to_string(),
                        state: "my_customer_state".to_string(),
                    },
                })
            })
            .once();

        let customer_service = CustomerService {
            customer_repository: Box::new(customer_repository),
        };
        let _ = customer_service
            .create_customer(CreateCustomerRequestObject {
                first_name: "my_customer_first_name".to_string(),
                last_name: "my_customer_last_name".to_string(),
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            })
            .await;
    }
}
