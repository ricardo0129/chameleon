mod cli;
mod config;
mod state;
mod utils;
use crate::cli::commands::{Cli, Commands};
use crate::config::profile;
use crate::state::mem::Mem;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mem = Mem::new();
    let db = sled::open("/temp").unwrap();

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
                state::mem::save_config(db, &config);
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
            for key in mem.config.profiles {
                println!("{:?}", key);
            }
        }
        Commands::Add { profile_name } => {
            profile::add_profile(&profile_name);
        }
        Commands::Remove { profile_name } => {
            profile::remove_profile(&profile_name);
        }
        Commands::Swap {
            profile_name,
            new_profile_name,
        } => {
            profile::swap_profile(&profile_name, &new_profile_name);
        }
    }
    //file::create_symlink("./x", "./examples/y");
    //file::remove_all_symlinks("./examples");
}
