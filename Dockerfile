FROM rust:1.50.0 AS develop

EXPOSE 8080

WORKDIR /app

RUN cargo install cargo-watch
RUN cargo install diesel_cli

COPY . .

# builder
FROM rust:1.50.0 AS builder

WORKDIR /app

COPY . .
RUN cargo install diesel_cli
RUN cargo build --release

# production
FROM debian:buster-slim AS production

RUN apt-get update
RUN apt-get install libpq-dev -y

COPY --from=builder /app/blescanner-server/target/release/blescanner-server .

EXPOSE 8080

CMD ["./blescanner-server"]
