use crate::core::engine;
use crate::core::state::StateRepository;
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
        dotfile_name: String,
    },
    List,
    Remove {
        dotfile_name: String,
    },
    Add {
        dotfile_name: String,
    },
    Swap {
        dotfile_name: String,
        new_dotfile_name: String,
    },
    Describe {
        dotfile_name: String,
    },
}

pub fn run(cli: Cli, state_repository: &mut StateRepository) {
    match cli.command {
        Commands::Create { dotfile_name } => {
            engine::create(&dotfile_name);
        }
        Commands::List => {
            engine::list(state_repository);
        }
        Commands::Describe { dotfile_name } => {
            engine::describe(&dotfile_name);
        }
        Commands::Add { dotfile_name } => {
            engine::add(&dotfile_name);
        }
        Commands::Remove { dotfile_name } => {
            engine::remove(&dotfile_name);
        }
        Commands::Swap {
            dotfile_name,
            new_dotfile_name,
        } => {
            engine::swap(&dotfile_name, &new_dotfile_name);
        }
    }
}
