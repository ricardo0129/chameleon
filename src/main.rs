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
            let profile = profile::Profile {
                source: String::from("123"),
                description: None,
            };
            println!("Initializing new profile: {}", profile_name);
            profile::append_config_toml("examples/config.toml", &profile_name, profile);
        }
        Commands::List { config_path } => {
            let config = profile::load_profiles(&config_path);
            if let Ok(config) = config {
                for profile in config.profiles.keys() {
                    println!("{}", profile);
                }
            }
        }
        Commands::Describe { profile_name } => {
            println!("Describe profile: {}", profile_name);
            let config = profile::load_profiles("examples/config.toml");
            if let Ok(config) = config {
                if let Some(profile) = config.profiles.get(&profile_name) {
                    println!("Source: {}", profile.source);
                } else {
                    println!("Profile not found");
                }
            }
        }
        Commands::Switch { profile } => {
            println!("Switching to profile: {}", profile);
            profile::switch_profile(&profile);
        }
    }
    //file::create_symlink("./x", "./examples/y");
    //file::remove_all_symlinks("./examples");
}
