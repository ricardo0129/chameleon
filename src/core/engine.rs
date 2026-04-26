use crate::config::profile;
use crate::state;

pub fn create(profile_name: &str) {
    let profile = profile::Profile {
        source: String::from("123"),
        description: None,
    };
    println!("Initializing new profile: {}", profile_name);
    profile::append_config_toml("examples/config.toml", profile_name, profile);
}

pub fn list(db: &mut sled::Db, config_path: &str) {
    let config = profile::load_profiles(config_path);
    if let Ok(config) = config {
        for profile in config.profiles.keys() {
            println!("{}", profile);
        }
        state::mem::save_config(db, &config);
    }
}

pub fn describe(profile_name: &str) {
    println!("Describe profile: {}", profile_name);
    let config = profile::load_profiles("examples/config.toml");
    if let Ok(config) = config {
        if let Some(profile) = config.profiles.get(profile_name) {
            println!("Source: {}", profile.source);
        } else {
            println!("Profile not found");
        }
    }
}

pub fn add(_profile_name: &str) {}

pub fn remove(_profile_name: &str) {}

pub fn swap(profile_name: &str, new_profile_name: &str) {
    remove(profile_name);
    add(new_profile_name);
}
