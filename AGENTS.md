# Contributor Guide

This repository is a monorepo that contains both the backend and frontend of the Xenledger accounting demo application.

- **`be/`** – Rust service built with [axum](https://crates.io/crates/axum) and [tokio](https://crates.io/crates/tokio).
- **`fe/`** – SvelteKit web interface managed with Node.

## Workflow

1. Format Rust code with `cargo fmt` and run `cargo clippy` when the toolchain is available.
2. Format frontend files with `biome` and check them using `svelte-check`.
3. Execute `cargo test` and `npm test` when code in `be/` or `fe/` changes.
4. If the environment cannot install dependencies for these tools, note it in the PR testing section.
5. Document new or changed features in the appropriate `README.md` files appropriately.
6. Keep commit messages and pull request descriptions informative.
7. Document new guidelines in the appropriate `AGENTS.md` files appropriately.

The repository contains READMEs in the root and each subproject describing how to build and run the software. Please consult them before making changes.
