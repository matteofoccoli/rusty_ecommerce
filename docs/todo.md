# To Do

- [x] refactor pg_order_repository
- [x] implement domain (entities)
- [x] first commit on Github
- [x] use Postgres
- [x] expose API (rest or graphql)
- [x] implement integration test
- [x] use settings object
- [x] implement domain services using repository and domain logic
- [x] implement application services
- [x] implement sqlx repositories
- [x] move rest api to sqlx
- [x] implement create customer endpoint
- [x] use Docker
- [x] use Kafka
- [x] create Outbox entity

- [] WIP store Outbox entity
- [] Implement PG common repository

- [] handle failures while starting/committing/rollbacking transaction 
- [] publish outbox content to Kafka
- [] outbox table pattern
- [] stop using diesel repositories in rest API
- [] create/drop test DB
- [] apply aggregate rules
- [] apply effective Rust learnings (chapters 1, 2, and 3)
- [] add status to Order using an enumeration
- [] setup github actions 
- [] add linter
- [] faster linking on Mac
- [] optimistic locking an concurrency on entity root
- [] use money to represent amount
- [] Order line items as a type instead of a vector