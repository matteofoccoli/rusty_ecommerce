use uuid::Uuid;

use crate::domain::entities::Customer;
use crate::domain::value_objects::Address;

mod domain;

fn main() {
    let customer = Customer {
        id: Uuid::new_v4(),
        first_name: "Clark".to_string(),
        last_name: "Kent".to_string(),
        address: Address {
            street: "22, Acacia Avenue".to_string(),
            city: "Minneapolis".to_string(),
            zip_code: "12345".to_string(),
            state: "Usa".to_string(),
        },
    };

    println!("My first customer is: {}", customer);
    println!("He lives in: {}", customer.address);
}
