use serde::Deserialize;
use std::collections::HashMap;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    name: String,
    dotfile_source: String,
}

pub fn load_profiles(file_location: &str) -> Vec<Profile> {
    let profiles: Vec<Profile> = Vec::new();
    profiles
}

pub fn switch_profile(profile: &str) {
    println!("Switching {profile}");
}
