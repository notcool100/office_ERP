# Xenledger Accounting Suite

This repository contains the code for a small accounting demo. It is organised as a monorepo with separate folders for the backend API and the Svelte frontend.

## Repository Structure

- **`be/`** – Rust service using `axum` and `tokio`.
- **`fe/`** – SvelteKit web application.

## Getting Started

### Backend

1. Ensure the Rust toolchain is installed.
2. Set the `DATABASE_URL` environment variable to a PostgreSQL connection string.
3. Run the service:

```bash
cd be
cargo run
```

### Frontend

1. Install Node (v18 or later recommended).
2. Install dependencies and start the dev server:

```bash
cd fe
npm install
npm run dev
```

### Running Tests

- Backend tests: `cd be && cargo test`
- Frontend tests: `cd fe && npm test`

See the READMEs inside each subdirectory for more details about development, testing, and additional commands.

For contribution guidelines refer to [AGENTS.md](./AGENTS.md).
