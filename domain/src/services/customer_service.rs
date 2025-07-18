use uuid::Uuid;

use crate::{
    entities::customer::Customer,
    repositories::customer_repository::CustomerRepository,
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
        repositories::customer_repository::{CustomerRepositoryError, MockCustomerRepository},
        services::customer_service::{
            CreateCustomerRequestObject, CustomerService, CustomerServiceError,
        },
        value_objects::{Address, CustomerId},
    };

    const CUSTOMER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";

    #[tokio::test]
    async fn creates_a_customer() {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_save()
            .returning(move |_| {
                Ok(Customer {
                    id: CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
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
        let saved_customer = customer_service
            .create_customer(CreateCustomerRequestObject {
                first_name: "my_customer_first_name".to_string(),
                last_name: "my_customer_last_name".to_string(),
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(
            CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
            saved_customer.id
        );
    }

    #[tokio::test]
    async fn cannot_create_a_customer() {
        let mut customer_repository = MockCustomerRepository::new();
        customer_repository
            .expect_save()
            .returning(move |_| Err(CustomerRepositoryError::ConnectionNotCreatedError))
            .once();

        let customer_service = CustomerService {
            customer_repository: Box::new(customer_repository),
        };
        let result = customer_service
            .create_customer(CreateCustomerRequestObject {
                first_name: "my_customer_first_name".to_string(),
                last_name: "my_customer_last_name".to_string(),
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            })
            .await;

        assert!(matches!(
            result,
            Err(CustomerServiceError::CustomerNotSavedError)
        ));
    }
}
