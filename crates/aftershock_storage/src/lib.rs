use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod error;
pub mod migration;
mod models;
mod pool;
pub mod routes;
mod schema;
mod utils;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(get_connection_pool);
