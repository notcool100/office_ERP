//! Personal API tokens for the project API.
//!
//! Tokens are managed with a normal login session (`/api-tokens`) and
//! accepted as `Authorization: Bearer ubk_...` on `/projects` only — see
//! `middlewares::auth` for the enforcement.
pub mod handlers;
pub mod routes;
pub mod service;
