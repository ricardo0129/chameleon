use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub source: String,
}

pub fn switch_profile(profile: &str) {
    println!("Switching {profile}");
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
