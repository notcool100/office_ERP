pub mod api;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod middlewares;
pub mod models;
pub mod routes;

pub use db::init_pool;
pub use routes::build_routes;

pub const ACCESS_TOKEN_TTL_MINUTES: i64 = 15;
pub const REFRESH_TOKEN_TTL_DAYS: i64 = 30;
pub const VERIFICATION_TOKEN_TTL_HOURS: i64 = 3;
