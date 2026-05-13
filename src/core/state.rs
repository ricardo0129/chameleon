use crate::config::profile::{Dotfile, Profile};
use crate::core::constants::{ACTIVE_PROFILE_KEY, DOTFILES_KEYSPACE, PROFILE_KEYSPACE};
use crate::core::error::AppError;
use serde_json;

pub trait StateStore {
    fn active_profile(&self) -> Result<Option<Profile>, AppError>;
    fn add_profile(&self, profile_name: &str, profile: &Profile) -> Result<(), AppError>;
    #[allow(dead_code)]
    fn remove_profile(&self, profile_name: &str) -> Result<(), AppError>;
    #[allow(dead_code)]
    fn profile_exists(&self, profile_name: &str) -> Result<bool, AppError>;
    fn switch_profile(&self, profile_name: &str) -> Result<(), AppError>;
    fn add_dotfile(&self, dotfile_name: &str, dotfile: &Dotfile) -> Result<(), AppError>;
    #[allow(dead_code)]
    fn remove_dotfile(&self, dotfile_name: &str) -> Result<(), AppError>;
    #[allow(dead_code)]
    fn profile_add_dotfile(&self, profile_name: &str, dotfile_name: &str) -> Result<(), AppError>;
    #[allow(dead_code)]
    fn profile_remove_dotfile(
        &self,
        profile_name: &str,
        dotfile_name: &str,
    ) -> Result<(), AppError>;
}

pub struct StateRepository<T: StateStore> {
    pub db: T,
}

impl StateStore for sled::Db {
    fn active_profile(&self) -> Result<Option<Profile>, AppError> {
        let active_profile_lookup = self
            .get(ACTIVE_PROFILE_KEY)
            .map_err(|_e| AppError::Runtime)?;
        let active_profile = active_profile_lookup.ok_or(AppError::Runtime)?;
        let profile_tree = self
            .open_tree(PROFILE_KEYSPACE)
            .map_err(|_e| AppError::Runtime)?;
        match profile_tree
            .get(active_profile)
            .map_err(|_e| AppError::Runtime)?
        {
            Some(encoded_profile) => Ok(Some(
                serde_json::from_slice::<Profile>(&encoded_profile).unwrap(),
            )),
            None => Ok(None),
        }
    }

    fn switch_profile(&self, profile_name: &str) -> Result<(), AppError> {
        self.insert(ACTIVE_PROFILE_KEY, profile_name)
            .expect("unabel to update activeprofile");
        Ok(())
    }

    fn add_profile(&self, profile_name: &str, profile: &Profile) -> Result<(), AppError> {
        let encoded_profile = serde_json::to_vec(profile).unwrap();
        let profile_tree = self
            .open_tree(PROFILE_KEYSPACE)
            .map_err(|_e| AppError::Runtime)
            .unwrap();
        profile_tree
            .insert(profile_name, encoded_profile)
            .map_err(|_e| AppError::Runtime)
            .unwrap();
        Ok(())
    }

    fn profile_exists(&self, profile_name: &str) -> Result<bool, AppError> {
        let profile_tree = self
            .open_tree(PROFILE_KEYSPACE)
            .expect("Unable to open Profile KeySpace");
        match profile_tree.get(profile_name).expect("Unable to read") {
            None => Ok(true),
            Some(_) => Ok(true),
        }
    }
    fn remove_profile(&self, _profile_name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn add_dotfile(&self, dotfile_name: &str, dotfile: &Dotfile) -> Result<(), AppError> {
        let encoded_dotfile = serde_json::to_vec(dotfile).unwrap();
        let dotfile_tree = self
            .open_tree(DOTFILES_KEYSPACE)
            .map_err(|_e| AppError::Runtime)
            .unwrap();
        dotfile_tree
            .insert(dotfile_name, encoded_dotfile)
            .map_err(|_e| AppError::Runtime)
            .unwrap();
        Ok(())
    }
    fn remove_dotfile(&self, _dotfile_name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn profile_add_dotfile(
        &self,
        _profile_name: &str,
        _dotfile_name: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
    fn profile_remove_dotfile(
        &self,
        _profile_name: &str,
        _dotfile_name: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use uuid::Uuid;

    fn test_repository() -> StateRepository<sled::Db> {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(Uuid::new_v4().to_string());
        let db = sled::open(temp_dir).unwrap();
        StateRepository { db }
    }

    #[test]
    fn test_save_config() {
        /*
                let mut state_repository = test_repository();
                let dotfile = Dotfile {
                    source: "test".to_string(),
                    description: Some("test description".to_string()),
                };
                let test_profile = Profile {
                    dotfiles: HashMap::from([("test".to_string(), dotfile)]),
                };
                state_repository.db.save_profile(&test_profile);
        */
    }

    #[test]
    fn test_load_empty_config() {
        let _ = test_repository();
        //let profile = state_repository.db.load_profile();
        //assert_eq!(profile.dotfiles.len(), 0);
    }
}
