use crate::profile;
use crate::state;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        profile_name: String,
    },
    List {
        config_path: String,
    },
    Remove {
        profile_name: String,
    },
    Add {
        profile_name: String,
    },
    Swap {
        profile_name: String,
        new_profile_name: String,
    },
    Describe {
        profile_name: String,
    },
}

pub fn run(cli: Cli, db: sled::Db) {
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
}
