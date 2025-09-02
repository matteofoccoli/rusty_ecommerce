-- Add migration script here
ALTER TABLE outbox_messages
ADD COLUMN processed_at TIMESTAMPTZ NULL;
