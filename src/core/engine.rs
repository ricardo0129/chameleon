use crate::core::state::StateStore;
use crate::{config::profile, core::state::StateRepository};

pub fn create<T: StateStore>(_dotfile_name: &str, state_repository: &mut StateRepository<T>) {
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

pub fn add<T: StateStore>(_profile_name: &str, state_repository: &mut StateRepository<T>) {}

pub fn remove<T: StateStore>(_profile_name: &str, state_repository: &mut StateRepository<T>) {}

pub fn swap<T: StateStore>(
    profile_name: &str,
    new_profile_name: &str,
    state_repository: &mut StateRepository<T>,
) {
    remove(profile_name, state_repository);
    add(new_profile_name, state_repository);
}
