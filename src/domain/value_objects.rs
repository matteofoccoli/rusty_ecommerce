use std::fmt;

use uuid::Uuid;

pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub state: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {} ({})",
            self.street, self.zip_code, self.city, self.state
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct CustomerId(pub Uuid);

#[derive(PartialEq, Debug)]
pub struct OrderId(pub Uuid);

#[derive(PartialEq, Debug)]
pub struct ProductId(pub Uuid);

pub struct OrderItem {
    pub price: f32,
    pub quantity: u32,
    pub product_id: ProductId,
}

impl OrderItem {
    pub fn create(price: f32, quantity: u32, product_id: ProductId) -> Self {
        Self {
            price,
            quantity,
            product_id,
        }
    }
}