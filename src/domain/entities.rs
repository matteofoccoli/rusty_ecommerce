use std::fmt;

use uuid::Uuid;

use super::value_objects::Address;

pub struct Customer {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} (uuid: {})",
            self.last_name, self.first_name, self.id
        )
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::Address;

    use super::Customer;
    use uuid::Uuid;

    #[test]
    fn create_a_customer() {
        let id = Uuid::new_v4();
        let customer = Customer {
            id,
            first_name: "John".to_string(),
            last_name: "Appleseed".to_string(),
            address: Address {
                street: "22, Acacia Avenue".to_string(),
                city: "Minneapolis".to_string(),
                zip_code: "12345".to_string(),
                state: "Usa".to_string(),
            },
        };

        assert_eq!(id, customer.id);
        assert_eq!("John".to_string(), customer.first_name);
        assert_eq!("Appleseed".to_string(), customer.last_name);
    }
}
