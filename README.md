# rusty_ecommerce

Exploring Redpanda: https://docs.redpanda.com/23.3/get-started/quick-start/#deploy-redpanda

## Run the application

To create DB run `diesel setup`

To create DB tables do: `DATABASE_URL=postgres://foo:bar@127.0.0.1/rusty_ecommerce sqlx migrate run`

Then move to the `rest_api` folder and execute `cargo run`.

## Tests

Run `cargo test` from the root

### Integrations Tests

Integration tests automatically recreates DB and apply migrations
