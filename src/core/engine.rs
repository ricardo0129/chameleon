use crate::core::error::AppError;
use crate::core::state::{ProfileRepo, ProfileService};

#[allow(dead_code)]
type ProfileServiceType = ProfileService<Box<dyn ProfileRepo + Sync + Send>>;
#[allow(dead_code)]
pub fn add_profile(_service: &ProfileServiceType) -> Result<(), AppError> {
    /*
    let dotfiles = match dotfile_list {
        None => HashSet::new(),
        Some(dotfiles) => dotfiles.into_iter().collect(),
    };
    let profile = Profile { dotfiles };
    state_repository.db.add_profile(profile_name, &profile)?;
    */
    Ok(())
}

/*
pub fn add_dotfile<T: StateStore>(
    state_repository: &mut StateRepository<T>,
    dotfile_name: String,
    source: String,
    description: Option<String>,
) -> Result<(), AppError> {
    /*
    let dotfile = Dotfile {
        source,
        description,
    };
    state_repository.db.add_dotfile(&dotfile_name, &dotfile)?;
    */
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
    info!("Switching profile to {}", profile_name);
    state_repository
        .db
        .switch_profile(profile_name)
        .expect("Error switching profile");
    info!("finished updating profile");
}

#[allow(dead_code)]
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
    info!("Listing profiles..");
    let profiles: Vec<String> = state_repository.db.list_profiles().unwrap();
    info!("Found {} profiles", profiles.len());
    for dotfile_name in profiles {
        println!("{}", dotfile_name);
    }
}

#[allow(dead_code)]
pub fn describe<T: StateStore>(_profile_name: &str, _state_repository: &mut StateRepository<T>) {
    /*
        let profile: profile::Profile = state_repository.db.load_profile();
        if let Some(dotfile_name) = profile.dotfiles.get(profile_name) {
            println!("{}", dotfile_name);
        } else {
            println!("Dotfile not found");
        }
    */
}

#[allow(dead_code)]
pub fn add<T: StateStore>(_profile_name: &str, _state_repository: &mut StateRepository<T>) {}

#[allow(dead_code)]
pub fn remove<T: StateStore>(_profile_name: &str, _state_repository: &mut StateRepository<T>) {}

#[allow(dead_code)]
pub fn swap<T: StateStore>(
    profile_name: &str,
    new_profile_name: &str,
    state_repository: &mut StateRepository<T>,
) {
    remove(profile_name, state_repository);
    add(new_profile_name, state_repository);
}
*/
