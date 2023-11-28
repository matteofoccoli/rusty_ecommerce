use std::fmt;

use crate::value_objects::{Address, CustomerId};

pub struct Customer {
    pub id: CustomerId,
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} (uuid: {})",
            self.last_name, self.first_name, self.id.0
        )
    }
}

#[cfg(test)]
mod test {

    use crate::value_objects::{Address, CustomerId};

    use super::Customer;
    use uuid::Uuid;

    #[test]
    fn create_a_customer() {
        let id = Uuid::new_v4();

        let customer = customer_fixture(id);

        assert_eq!(CustomerId(id), customer.id);
        assert_eq!("John".to_string(), customer.first_name);
        assert_eq!("Appleseed".to_string(), customer.last_name);
    }

    fn customer_fixture(id: Uuid) -> Customer {
        Customer {
            id: CustomerId(id),
            first_name: "John".to_string(),
            last_name: "Appleseed".to_string(),
            address: Address {
                street: "22, Acacia Avenue".to_string(),
                city: "Minneapolis".to_string(),
                zip_code: "12345".to_string(),
                state: "Usa".to_string(),
            },
        }
    }
}
