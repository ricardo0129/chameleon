use crate::core::engine;
use crate::core::state::{DotfileService, ProfileService, SqlDotfileRepo, SqlProfileRepo};
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

#[allow(dead_code)]
pub struct App {
    pub profile_service: ProfileService<SqlProfileRepo>,
    pub dotfile_service: DotfileService<SqlDotfileRepo>,
}

#[allow(dead_code)]
impl App {
    pub fn run(&mut self, cli: Cli) {
        match cli.command {
            Commands::AddProfile {
                profile_name: _,
                dotfile_list: _,
            } => {
                engine::add_profile(&mut self.profile_service).expect("Error");
            }
            Commands::AddDotfile {
                dotfile_name: _,
                source: _,
                description: _,
            } => {
                engine::add_dotfile(&mut self.dotfile_service).expect("Error");
            }
            Commands::ListProfiles => {
                /*
                engine::list_profiles(db);
                */
            }
            Commands::ActiveProfile => {
                /*
                engine::active_profile(db);
                */
            }
            Commands::SwitchProfile { profile_name: _ } => {
                /*
                engine::switch_profile(db, &profile_name);
                */
            }
        }
    }
}
