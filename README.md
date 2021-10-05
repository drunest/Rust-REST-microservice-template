# Rust REST Webserver Template

This template repo is used for the fast setup of a actix-web server with REST, SQLx and Postgres under the hood.

The following technologies are used:

- Rust
- Actix-Web
- GraphQL
- SQLX
- Postgres

For a fast setup:

- Clone the repo
- Run cargo install sqlx-cli
- Run docker-compose up -d
- Create your .env file based on the example
- Optional edit or add migrations and apply them with sqlx migrate run
- Starte the server with cargo run
- Go to http://localhost:8080/api/example

To ensure compilation without a connected postgres instance use this command:

- `cargo sqlx prepare`

This will generate the [sqlx-data.json](./sqlx-data.json) for your queries. Push this file to version control and cargo will compile the application even without a connected postgres instance.
