use crate::core::engine;
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

pub fn run(cli: Cli, db: &mut sled::Db) {
    match cli.command {
        Commands::Create { profile_name } => {
            engine::create(&profile_name);
        }
        Commands::List { config_path } => {
            engine::list(db, &config_path);
        }
        Commands::Describe { profile_name } => {
            engine::describe(&profile_name);
        }
        Commands::Add { profile_name } => {
            engine::add(&profile_name);
        }
        Commands::Remove { profile_name } => {
            engine::remove(&profile_name);
        }
        Commands::Swap {
            profile_name,
            new_profile_name,
        } => {
            engine::swap(&profile_name, &new_profile_name);
        }
    }
}
