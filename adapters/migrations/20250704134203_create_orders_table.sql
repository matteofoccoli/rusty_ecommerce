-- Add migration script here
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL
)