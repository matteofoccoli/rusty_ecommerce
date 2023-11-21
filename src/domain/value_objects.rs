use std::fmt;

pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub state: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}, {} ({})", self.street, self.zip_code, self.city, self.state)
    }
}