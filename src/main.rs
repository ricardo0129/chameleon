mod cli;
mod config;
mod utils;
use clap::Parser;

use std::fs;
use std::process;

use crate::cli::commands::{Cli, Commands};
use crate::config::profile;
use crate::utils::file;

fn example_load_config(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file {e}");
            process::exit(1);
        }
    };
    let config: profile::Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to load config {e}");
            process::exit(1);
        }
    };
    dbg!(config);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { profile_name } => {
            println!("Initializing new profile: {}", profile_name);
        }
        Commands::List {} => {
            let profiles: Vec<String> = vec![
                String::from("profile1"),
                String::from("profile2"),
                String::from("profile3"),
            ];
            for profile in profiles {
                println!("{}", profile);
            }
        }
        Commands::Switch { profile } => {
            println!("Switching to profile: {}", profile);
        }
    }
    //file::create_symlink("./x", "./examples/y");
    //file::remove_all_symlinks("./examples");
}
