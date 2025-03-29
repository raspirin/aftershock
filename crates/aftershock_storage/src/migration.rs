use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::POOL;
use crate::Result;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub fn run_migrations() -> Result<()> {
    let mut conn = POOL.clone().get()?;
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}