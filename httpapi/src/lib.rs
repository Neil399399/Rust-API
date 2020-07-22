mod auth;
mod models;
pub mod mono;
mod utils;

type PgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
