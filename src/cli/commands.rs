use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create { profile_name: String },
    List {},
    Switch { profile: String },
}
