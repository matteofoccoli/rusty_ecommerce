use uuid::Uuid;

use crate::{
    entities::{customer::Customer, outbox::OutboxMessage},
    repositories::{
        customer_repository::CustomerRepository, outbox_repository::OutboxMessageRepository,
    },
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
    customer_repository: Box<dyn CustomerRepository>,
    outbox_message_repository: Box<dyn OutboxMessageRepository>,
}

impl CustomerService {
    pub fn new(
        customer_repository: Box<dyn CustomerRepository>,
        outbox_message_repository: Box<dyn OutboxMessageRepository>,
    ) -> Self {
        Self {
            customer_repository,
            outbox_message_repository,
        }
    }

    async fn begin_transaction(&self) -> Result<(), CustomerServiceError> {
        self.customer_repository
            .begin_transaction()
            .await
            .map_err(|e| CustomerServiceError::GenericError(e.to_string()))
    }

    async fn commit_transaction(&self) -> Result<(), CustomerServiceError> {
        self.customer_repository
            .commit_transaction()
            .await
            .map_err(|e| CustomerServiceError::GenericError(e.to_string()))
    }

    async fn rollback_transaction(&self) -> Result<(), CustomerServiceError> {
        self.customer_repository
            .rollback_transaction()
            .await
            .map_err(|e| CustomerServiceError::GenericError(e.to_string()))
    }

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

        self.begin_transaction().await?;

        let saved_customer = match self.customer_repository.save(customer).await {
            Ok(customer) => customer,
            Err(_) => {
                self.rollback_transaction().await?;
                return Err(CustomerServiceError::CustomerNotSavedError);
            }
        };

        match self
            .outbox_message_repository
            .save(OutboxMessage::customer_created_event(&saved_customer))
            .await
        {
            Ok(_) => (),
            Err(_) => {
                self.rollback_transaction().await?;
                return Err(CustomerServiceError::GenericError(
                    "Outbox message not saved".to_string(),
                ));
            }
        };

        self.commit_transaction().await?;
        return Ok(saved_customer);
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        entities::{customer::Customer, outbox::OutboxMessage},
        repositories::{
            customer_repository::{CustomerRepositoryError, MockMyCustomerRepository},
            outbox_repository::{MockOutboxMessageRepository, OutboxMessageRepositoryError},
        },
        services::customer_service::{
            CreateCustomerRequestObject, CustomerService, CustomerServiceError,
        },
        value_objects::{Address, CustomerId},
    };

    const CUSTOMER_ID: &str = "2585491a-8e05-11ee-af1c-9bfe41ffe61f";

    #[tokio::test]
    async fn creates_a_customer() {
        let saved_customer = create_customer();
        let saved_outbox_message = OutboxMessage::customer_created_event(&saved_customer);
        let expected_event_payload = saved_outbox_message.event_payload();

        let mut customer_repository = MockMyCustomerRepository::new();
        customer_repository
            .expect_save()
            .once()
            .return_once(|_| Ok(saved_customer));
        customer_repository
            .expect_begin_transaction()
            .once()
            .returning(|| Ok(()));
        customer_repository
            .expect_commit_transaction()
            .once()
            .returning(|| Ok(()));

        let mut outbox_message_repository = MockOutboxMessageRepository::new();
        outbox_message_repository
            .expect_save()
            .withf(move |m| {
                m.event_type() == "customer_created".to_string()
                    && m.processed_at().is_none()
                    && m.event_payload() == expected_event_payload
            })
            .once()
            .return_once(|_| Ok(saved_outbox_message));

        let customer_service = CustomerService::new(
            Box::new(customer_repository),
            Box::new(outbox_message_repository),
        );
        let saved_customer = customer_service
            .create_customer(create_customer_request_object())
            .await
            .unwrap();

        assert_eq!(
            CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
            saved_customer.id
        );
    }

    #[tokio::test]
    async fn has_an_error_while_saving_customer() {
        let mut customer_repository = MockMyCustomerRepository::new();
        customer_repository
            .expect_save()
            .returning(move |_| Err(CustomerRepositoryError::ConnectionNotCreatedError))
            .once();
        customer_repository
            .expect_begin_transaction()
            .once()
            .returning(|| Ok(()));

        customer_repository
            .expect_rollback_transaction()
            .once()
            .returning(|| Ok(()));
        let outbox_message_repository = MockOutboxMessageRepository::new();

        let customer_service = CustomerService::new(
            Box::new(customer_repository),
            Box::new(outbox_message_repository),
        );
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

    #[tokio::test]
    async fn has_an_error_while_saving_outbox_message() {
        let mut customer_repository = MockMyCustomerRepository::new();
        customer_repository
            .expect_save()
            .returning(move |_| Ok(create_customer()))
            .once();
        customer_repository
            .expect_begin_transaction()
            .once()
            .returning(|| Ok(()));

        customer_repository
            .expect_rollback_transaction()
            .once()
            .returning(|| Ok(()));

        let mut outbox_message_repository = MockOutboxMessageRepository::new();
        outbox_message_repository
            .expect_save()
            .returning(move |_| {
                Err(OutboxMessageRepositoryError::OutboxMessageNotSavedError(
                    "Database error".to_string(),
                ))
            })
            .once();

        let customer_service = CustomerService::new(
            Box::new(customer_repository),
            Box::new(outbox_message_repository),
        );
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

        assert!(matches!(result, Err(CustomerServiceError::GenericError(_))));
    }

    fn create_customer_request_object() -> CreateCustomerRequestObject {
        CreateCustomerRequestObject {
            first_name: "my_customer_first_name".to_string(),
            last_name: "my_customer_last_name".to_string(),
            street: "my_customer_street".to_string(),
            city: "my_customer_city".to_string(),
            zip_code: "my_customer_zip_code".to_string(),
            state: "my_customer_state".to_string(),
        }
    }

    fn create_customer() -> Customer {
        Customer {
            id: CustomerId(Uuid::try_parse(CUSTOMER_ID).unwrap()),
            first_name: "my_customer_first_name".to_string(),
            last_name: "my_customer_last_name".to_string(),
            address: Address {
                street: "my_customer_street".to_string(),
                city: "my_customer_city".to_string(),
                zip_code: "my_customer_zip_code".to_string(),
                state: "my_customer_state".to_string(),
            },
        }
    }
}
