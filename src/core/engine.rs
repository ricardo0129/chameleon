use crate::core::state::StateStore;
use crate::{config::profile, core::state::StateRepository};

pub fn create(_dotfile_name: &str) {
    let dotfile = profile::Dotfile {
        source: String::from("123"),
        description: None,
    };
    println!("Initializing new dotfile: {}", dotfile.source);
}

pub fn list<T: StateStore>(state_repository: &mut StateRepository<T>) {
    println!("Listing profiles..");
    let profile: profile::Profile = state_repository.db.load_profile();
    println!("Found {} profiles", profile.dotfiles.len());
    for dotfile_name in profile.dotfiles.keys() {
        println!("{}", dotfile_name);
    }
}

pub fn describe<T: StateStore>(profile_name: &str, state_repository: &mut StateRepository<T>) {
    let profile: profile::Profile = state_repository.db.load_profile();
    let dotfile = profile.dotfiles.get(profile_name);
    if let Some(dotfile) = profile.dotfiles.get(profile_name) {
        println!("{}", dotfile.source);
    } else {
        println!("Dotfile not found");
    }
}

pub fn add(_profile_name: &str) {}

pub fn remove(_profile_name: &str) {}

pub fn swap(profile_name: &str, new_profile_name: &str) {
    remove(profile_name);
    add(new_profile_name);
}
