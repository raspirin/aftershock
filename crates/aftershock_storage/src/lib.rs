use pool::{DbPool, get_connection_pool};
use std::sync::LazyLock;

pub mod error;
pub mod job;
pub mod migration;
mod models;
mod pool;
mod schema;
mod utils;

type Result<T> = std::result::Result<T, error::Error>;

static POOL: LazyLock<DbPool> = LazyLock::new(get_connection_pool);

pub use job::page::*;
pub use job::post::*;
pub use job::private::*;
