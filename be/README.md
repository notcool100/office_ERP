# Backend

This directory contains the Rust API service built with [axum](https://crates.io/crates/axum).

## Prerequisites

- Rust toolchain (stable)
- PostgreSQL database
- `DATABASE_URL` environment variable pointing to the database

## Running

```bash
cd be
cargo run
```

The server listens on `127.0.0.1:3117` by default.

## Development Tasks

- Format code: `cargo fmt`
- Lint: `cargo clippy`
- Run tests: `cargo test` (none are currently implemented)
- Apply migrations with `sqlx migrate run` if you modify the schema.

## Project Structure

- `src/` – application source code
- `migrations/` – SQLx migration files

The available API endpoints are under `/auth` for basic authentication (register, login, refresh, etc.).

## Seeding

Seed the initial `superadmin` user with:

```bash
cargo run --bin seed -- --password YOUR_PASSWORD
```

Or generate a random password which is printed once:

```bash
cargo run --bin seed -- --generate
```

The generated password is only shown in the console, so make sure to save it.
