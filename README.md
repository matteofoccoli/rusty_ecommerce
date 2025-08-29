# rusty_ecommerce

Exploring Redpanda: https://docs.redpanda.com/23.3/get-started/quick-start/#deploy-redpanda

## Run the application

To create DB run `diesel setup`
To create DB tables do: `DATABASE_URL=postgres://foo:bar@127.0.0.1/rusty_ecommerce sqlx migrate run`
Then move to the `rest_api` folder and execute `cargo run`.

## Adapters Unit Tests

Start required containers with `docker compose up -d`

To run tests in the adapters sub-project you need to have a running DB and apply migrations, move to the `adapters` folder and just run `sqlx database create` followed by `sqlx migrate run`. Then you can run `cargo test`.

### Rest APIs Integrations Tests

Integration tests automatically recreates DB and apply migrations, just launch `cargo test` from the `rest_api` folder.

### Process Outbox

Just run the outbox processor via `cargo run -p outbox_processor` from the workspace root.