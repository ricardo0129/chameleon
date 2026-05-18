mod cli;
mod config;
mod core;
mod utils;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::utils::db_helpers;
use clap::Parser;
use log::info;
use rusqlite::Connection;

fn main() {
    env_logger::init();
    let conf = conf::Config::load();
    info!("Loaded config");
    let _cli = Cli::parse();
    let mut conn = Connection::open(conf.db_location).unwrap();
    let _ = db_helpers::run_migrations(&mut conn);

    /*
    let app = App {
        profile_service: conn.clone(),
        dotfile_service: conn.clone(),
    };

    commands::run(cli, &mut db);
    */
}
