use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub source: String,
}

pub fn switch_profile(profile: &str) {
    println!("Switching {profile}");
}

impl Config {
    pub fn add_profile(&mut self, name: &str, profile: Profile) {
        self.profiles.insert(name.to_string(), profile);
    }
}

pub fn load_profiles(file_path: &str) -> Result<Config, Box<dyn Error>> {
    // TODO Add some custom error types
    // Also clean up error handling here
    println!("Loading profiles from: {}", file_path);
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(config)
}

pub fn append_config_toml(file_path: &str, name: &str, profile: Profile) {
    let mut config = match load_profiles(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    config.add_profile(name, profile);
    write_config_toml(&config, file_path);
}

pub fn write_config_toml(config: &Config, file_path: &str) {
    let toml_string = toml::to_string_pretty(config).unwrap();
    fs::write(file_path, toml_string).unwrap();
}
