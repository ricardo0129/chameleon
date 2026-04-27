mod cli;
mod config;
mod core;
mod utils;
use crate::cli::commands;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::core::state::StateRepository;
use clap::Parser;
use std::sync::Arc;

fn main() {
    let conf = conf::Config::load();
    let cli = Cli::parse();
    let mut state_repository = StateRepository {
        db: Arc::new(sled::open(&conf.db_location).unwrap()),
    };

    commands::run(cli, &mut state_repository);
}
