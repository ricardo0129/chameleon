use crate::config::profile::Profile;
use crate::core::error::AppError;
use crate::core::state::StateStore;
use crate::{config::profile, core::state::StateRepository};
use std::collections::HashSet;

pub fn add_profile<T: StateStore>(
    state_repository: &mut StateRepository<T>,
    profile_name: &str,
    dotfile_list: Option<Vec<String>>,
) -> Result<(), AppError> {
    let dotfiles = match dotfile_list {
        None => HashSet::new(),
        Some(dotfiles) => dotfiles.into_iter().collect(),
    };
    let profile = Profile { dotfiles };
    state_repository.db.add_profile(profile_name, &profile)?;
    Ok(())
}

pub fn active_profile<T: StateStore>(state_repository: &mut StateRepository<T>) {
    let profile = state_repository.db.active_profile().unwrap();
    match profile {
        None => {
            println!("No active profile found");
        }
        Some(profile) => {
            println!("{:?}", profile);
        }
    }
}

pub fn switch_profile<T: StateStore>(
    state_repository: &mut StateRepository<T>,
    profile_name: &str,
) {
    println!("Switching profile to {}", profile_name);
    state_repository.db.switch_profile(profile_name);
    println!("finished updating profile");
}

pub fn create<T: StateStore>(
    dotfile_name: &str,
    source: &str,
    description: Option<String>,
    _state_repository: &mut StateRepository<T>,
) {
    let dotfile = profile::Dotfile {
        source: source.to_string(),
        description,
    };
    println!("Initializing new {}: {}", dotfile_name, dotfile.source);
}

pub fn list_profiles<T: StateStore>(state_repository: &mut StateRepository<T>) {
    /*
        println!("Listing profiles..");
        let profile: profile::Profile = state_repository.db.load_profile();
        println!("Found {} profiles", profile.dotfiles.len());
        for dotfile_name in profile.dotfiles {
            println!("{}", dotfile_name);
        }
    */
}

pub fn describe<T: StateStore>(profile_name: &str, state_repository: &mut StateRepository<T>) {
    /*
        let profile: profile::Profile = state_repository.db.load_profile();
        if let Some(dotfile_name) = profile.dotfiles.get(profile_name) {
            println!("{}", dotfile_name);
        } else {
            println!("Dotfile not found");
        }
    */
}

pub fn add<T: StateStore>(_profile_name: &str, _state_repository: &mut StateRepository<T>) {}

pub fn remove<T: StateStore>(_profile_name: &str, _state_repository: &mut StateRepository<T>) {}

pub fn swap<T: StateStore>(
    profile_name: &str,
    new_profile_name: &str,
    state_repository: &mut StateRepository<T>,
) {
    remove(profile_name, state_repository);
    add(new_profile_name, state_repository);
}
