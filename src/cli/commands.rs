use crate::core::engine;
use crate::core::state::{StateRepository, StateStore};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chameleon")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    AddProfile {
        profile_name: String,
        dotfile_list: Option<Vec<String>>,
    },
    AddDotfile {
        dotfile_name: String,
        source: String,
        description: Option<String>,
    },
    ListProfiles,
    ActiveProfile,
    SwitchProfile {
        profile_name: String,
    },
}

pub fn run<T: StateStore>(cli: Cli, state_repository: &mut StateRepository<T>) {
    match cli.command {
        Commands::AddProfile {
            profile_name,
            dotfile_list,
        } => {
            engine::add_profile(state_repository, &profile_name, dotfile_list).expect("Error");
        }
        Commands::AddDotfile {
            dotfile_name,
            source,
            description,
        } => {
            engine::add_dotfile(state_repository, dotfile_name, source, description)
                .expect("Error");
        }
        Commands::ListProfiles => {}
        Commands::ActiveProfile => {
            engine::active_profile(state_repository);
        }
        Commands::SwitchProfile { profile_name } => {
            engine::switch_profile(state_repository, &profile_name);
        }
    }
}
