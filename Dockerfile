FROM rust:1.71-slim-bullseye

WORKDIR /app

RUN apt update && apt install -y gcc libssl-dev libc6-dev pkg-config

COPY src /app/src
COPY migration /app/migration
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY entrypoint.sh /app/entrypoint.sh
COPY .env /app/.env

RUN cargo install --path /app
RUN cd /app && cargo build
RUN chmod -R 0755 /app/entrypoint.sh
RUN chmod -R 0755 /app/target/debug/rust-shortener-url

ENTRYPOINT  [ "/app/entrypoint.sh" ]
