-- Add migration script here
CREATE TABLE outbox_messages (
    id UUID PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    event_payload VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
)