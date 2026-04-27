mod cli;
mod config;
mod core;
mod utils;
use crate::cli::commands;
use crate::cli::commands::Cli;
use crate::cli::conf;
use clap::Parser;

fn main() {
    let conf = conf::Config::load();
    let cli = Cli::parse();
    let mut db = sled::open(conf.db_location).unwrap();

    commands::run(cli, &mut db);
}
