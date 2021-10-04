# Rust REST Webserver Template

This template repo is used for the fast setup of a actix-web server with REST, SQLx and Postgres under the hood.

The following technologies are used:

* Rust
* Actix-Web
* GraphQL
* SQLX
* Postgres

For a fast setup:

* Clone the repo
* Run cargo install sqlx-cli
* Run docker-compose up -d
* Create your .env file based on the example
* Optional edit or add migrations and apply them with sqlx migrate run
* Starte the server with cargo run
* Go to http://localhost:8080/api/example

