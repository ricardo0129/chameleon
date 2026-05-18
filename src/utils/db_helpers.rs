use crate::core::error::AppError;
use log::info;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

pub fn run_migrations(conn: &mut Connection) -> Result<(), AppError> {
    info!("Running migrations");
    const V1: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/migrations/v1__init_tables.sql"
    ));
    let migrations = Migrations::new(vec![M::up(V1)]);
    migrations.to_latest(conn).unwrap();
    Ok(())
}
