mod cli;
mod core;
mod models;
mod utils;
use crate::cli::commands::App;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::core::state::{DotfileService, ProfileService, SqlDotfileRepo, SqlProfileRepo};
use crate::utils::db_helpers;
use clap::Parser;
use log::info;
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    env_logger::init();
    let conf = conf::Config::load();
    info!("Loaded config");
    let cli = Cli::parse();
    let mut conn = Connection::open(conf.db_location).unwrap();
    let _ = db_helpers::run_migrations(&mut conn);
    let conn = Rc::new(RefCell::new(conn));
    let profile_service = ProfileService {
        repo: SqlProfileRepo { conn: conn.clone() },
    };
    let dotfile_service = DotfileService {
        repo: SqlDotfileRepo { conn: conn.clone() },
    };

    let mut app = App {
        profile_service,
        dotfile_service,
    };

    app.run(cli);
}
