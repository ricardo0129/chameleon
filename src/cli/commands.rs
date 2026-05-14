use crate::core::engine;
use crate::core::state::Database;
use crate::core::state::{DotfileRepo, DotfileService, ProfileRepo, ProfileService};
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
    profile_service: ProfileService<Box<dyn ProfileRepo + Send + Sync>>,
    dotfile_service: DotfileService<Box<dyn DotfileRepo + Send + Sync>>,
}

#[allow(dead_code)]
impl App {
    pub fn run(&self, cli: Cli, _db: &mut Database) {
        match cli.command {
            Commands::AddProfile {
                profile_name: _,
                dotfile_list: _,
            } => {
                engine::add_profile(&self.profile_service).expect("Error");
            }
            Commands::AddDotfile {
                dotfile_name: _,
                source: _,
                description: _,
            } => {
                /*
                engine::add_dotfile(db, dotfile_name, source, description).expect("Error");
                */
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
