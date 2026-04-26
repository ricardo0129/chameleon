mod cli;
mod config;
mod core;
mod state;
mod utils;
use crate::cli::commands;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::config::profile;
use clap::Parser;

fn main() {
    let conf = conf::Config::load();
    let cli = Cli::parse();
    let db = sled::open(conf.db_location).unwrap();

    commands::run(cli, db);
}
