mod cli;
mod config;
mod core;
mod utils;
use crate::cli::commands;
use crate::cli::commands::Cli;
use crate::cli::conf;
use crate::core::state::StateRepository;
use clap::Parser;
use log::info;

fn main() {
    env_logger::init();
    let conf = conf::Config::load();
    info!("Loaded config");
    let cli = Cli::parse();
    let mut state_repository = StateRepository {
        db: sled::open(&conf.db_location).unwrap(),
    };

    commands::run(cli, &mut state_repository);
}
