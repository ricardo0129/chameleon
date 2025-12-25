mod config;
mod utils;
use crate::config::profile;
use crate::utils::file;
use std::env;
use std::fs;
use std::process;

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
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    file::create_symlink("./x", "./examples/y");
    file::remove_all_symlinks("./examples");
}
