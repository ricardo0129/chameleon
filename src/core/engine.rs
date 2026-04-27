use crate::{config::profile, core::state::StateRepository};

pub fn create(_dotfile_name: &str) {
    let dotfile = profile::Dotfile {
        source: String::from("123"),
        description: None,
    };
    println!("Initializing new dotfile: {}", dotfile.source);
}

pub fn list(_state_repository: &mut StateRepository) {
    /*
        let config = profile::load_profiles();
        if let Ok(config) = config {
            for profile in config.profiles.keys() {
                println!("{}", profile);
            }
            core::mem::save_config(db, &config);
        }
    */
}

pub fn describe(profile_name: &str) {
    println!("Describe profile: {}", profile_name);
    /*
    let config = profile::load_profiles("examples/config.toml");
    if let Ok(config) = config {
        if let Some(profile) = config.profiles.get(profile_name) {
            println!("Source: {}", profile.source);
        } else {
            println!("Profile not found");
        }
    }
    */
}

pub fn add(_profile_name: &str) {}

pub fn remove(_profile_name: &str) {}

pub fn swap(profile_name: &str, new_profile_name: &str) {
    remove(profile_name);
    add(new_profile_name);
}
