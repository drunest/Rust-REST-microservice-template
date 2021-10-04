
FROM lukemathwalker/cargo-chef as planner
WORKDIR /service
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /service
COPY --from=planner /service/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as builder
WORKDIR /service
COPY . .
COPY --from=cacher /service/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin service

FROM debian:buster-slim as runtime
WORKDIR /service
COPY --from=builder /service/target/release/service service
CMD [ "./service" ]