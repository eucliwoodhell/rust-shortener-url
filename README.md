# URL Shortener

[![Buid](https://github.com/eucliwoodhell/rust-shortener-url/actions/workflows/build.yml/badge.svg)](https://github.com/eucliwoodhell/rust-shortener-url/actions/workflows/build.yml)

This project appears to be a URL shortener written in Rust, as indicated by the repository name "rust-shortener-url". The URL shortener takes long URLs and converts them into manageable short links that are easier to share and remember.

The project includes a migration file, which suggests that it uses a database to store the original URLs and their corresponding short URLs. The migration file, found at migration/src/m20230719_182205_create_url_table.rs, likely sets up the necessary database structure for the application.

## Tech

- Rust
- PostgreSQL
- Sea-orm-cli
- Actix-web

## How to install
Url shortener requires [Rust](https://www.rust-lang.org) and [PostgreSQL](https://www.postgresql.org) to be installed before to run.

```sh
cd rust-shortener-url
cargo install
cargo run
```

For production environments...

```sh
RUST_LOG=debug
HOST=localhost
PORT=9090
DATABASE_URL=postgres://postgres:postgres@localhost:5432/url
ALLOWED_HOSTS=http://localhost:3000

```

## Endpoints

| Endpoint | Description | Type |
| --- | --- | --- |
| /link  | Get all Shorten a URL | GET |
| /link/:id | Get by ID Shorten a URL | GET |
| /link | Save Shorten a URL | POST |
| /link/:id | Delete by ID Shorten a URL | DELETE |


## Docker

Url shortener is very easy to install and deploy in a Docker container, basic as that.

By default, the Docker will expose use file bash as the entrypoint, so change this within the
Dockerfile if necessary. When ready, simply use the Dockerfile to
build the image.

```sh
cd rust-shortener-url
docker build -t rust-shortener-url .
docker run -p 9090:9090 rust-shortener-url
```
