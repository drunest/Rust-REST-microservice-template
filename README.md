# Rust REST Webserver Template

This template repo is used for the fast setup of a rust REST API.

**The following technologies are used:**

- Rust
- Axum
- SeaOrm
- Postgres

**For a fast setup:**

1. Run `cargo install sea-orm-cli`
2. Create your .env file based on the example
3. Run `docker-compose up -d`
4. Optional 1: Add new migrations ([see here](./migration/README.md))
5. Optional 2: Update the entities ([see here](./entity/README.md))
6. Start the REST API webserver with `cargo run` (remember to make the environment variables provided in your `.env` accessible)
7. Open your browser and go to [localhost:8000](http://localhost:8000)
