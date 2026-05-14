mod cli;
mod config;
mod core;
mod utils;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::core::error::AppError;
use clap::Parser;
use log::info;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

fn run_migrations(conn: &mut Connection) -> Result<(), AppError> {
    info!("Running migrations");
    let migrations = Migrations::new(vec![M::up(include_str!(
        "../migrations/v1__init_tables.sql"
    ))]);
    migrations.to_latest(conn).unwrap();
    Ok(())
}
fn main() {
    env_logger::init();
    let conf = conf::Config::load();
    info!("Loaded config");
    let _cli = Cli::parse();
    let mut conn = Connection::open(conf.db_location).unwrap();
    let _ = run_migrations(&mut conn);

    /*
    let app = App {
        profile_service: conn.clone(),
        dotfile_service: conn.clone(),
    };

    commands::run(cli, &mut db);
    */
}
