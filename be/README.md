# Backend

This directory contains the Rust API service built with [axum](https://crates.io/crates/axum).

## Prerequisites

- Rust toolchain (stable)
- PostgreSQL database
- `DATABASE_URL` environment variable pointing to the database

## Database Setup

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
2. Update the `DATABASE_URL` in `.env` with your PostgreSQL credentials.

## Migrations

This project uses `sqlx` for database migrations.

1. **Install SQLx CLI** (if not already installed):
   ```bash
   cargo install sqlx-cli
   ```

2. **Run Migrations**:
   ```bash
   # Using cargo-sqlx
   cargo sqlx migrate run

   # OR directly if you have the CLI binary in path
   sqlx migrate run
   ```

## Seeding

To populate the database with initial data (Admin user, Roles, Permissions, etc.):

1. **Seed with a custom password**:
   ```bash
   cargo run --bin seed -- --password <YOUR_PASSWORD>
   ```

   Example:
   ```bash
   cargo run --bin seed -- --password admin123
   ```

   This will create a user `admin` (email: `admin@example.com`) with the specified password.

2. **(Optional) Verify Seeding**:
   The seeder creates:
   - Default Departments (Administration, HR, etc.)
   - Positions (System Administrator, etc.)
   - Navigation Menu Items
   - An Admin Employee linked to the 'Administration' department.
