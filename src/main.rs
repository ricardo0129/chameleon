mod cli;
mod config;
mod utils;
use crate::cli::commands::{Cli, Commands};
use crate::config::profile;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { profile_name } => {
            println!("Initializing new profile: {}", profile_name);
        }
        Commands::List { config_path } => {
            let config = profile::load_profiles(&config_path);
            if let Ok(config) = config {
                for profile in config.profiles.keys() {
                    println!("{}", profile);
                }
            }
        }
        Commands::Switch { profile } => {
            println!("Switching to profile: {}", profile);
        }
    }
    //file::create_symlink("./x", "./examples/y");
    //file::remove_all_symlinks("./examples");
}
