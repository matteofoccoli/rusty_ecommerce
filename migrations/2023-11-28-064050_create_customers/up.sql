-- Your SQL goes here
CREATE TABLE customers (
    id UUID PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    street VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    zip_code VARCHAR NOT NULL,
    state VARCHAR NOT NULL
)