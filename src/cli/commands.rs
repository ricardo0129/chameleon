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
