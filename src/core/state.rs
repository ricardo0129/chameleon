use crate::config::profile::Profile;
use crate::core::constants::DOTFILES_NAMESPACE;
use serde_json;
use sled;
use std::collections::{HashMap, HashSet};

pub struct State {
    #[allow(dead_code)]
    pub config: Profile,
    pub _active_dotfiles: HashSet<String>,
}

pub trait StateStore {
    fn load_profile(&mut self) -> Profile;
    fn save_profile(&mut self, config: &Profile);
    fn get_active_dotfiles(&self) -> Vec<String>;
}

pub struct StateRepository<T: StateStore> {
    pub db: T,
}

impl StateStore for sled::Db {
    #[allow(dead_code)]
    fn load_profile(&mut self) -> Profile {
        let v = self.get(DOTFILES_NAMESPACE).unwrap();
        if let Some(encoded_config) = v {
            serde_json::from_slice::<Profile>(&encoded_config).unwrap();
        }
        // No config found returning empty config
        Profile {
            dotfiles: HashMap::new(),
        }
    }
    fn save_profile(&mut self, config: &Profile) {
        let encoded_profile: Vec<u8> = serde_json::to_vec(config).unwrap();
        self.insert(DOTFILES_NAMESPACE, encoded_profile).unwrap();
    }
    fn get_active_dotfiles(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::profile::{Dotfile, Profile};
    use std::collections::HashMap;
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
        let mut state_repository = test_repository();
        let dotfile = Dotfile {
            source: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let test_profile = Profile {
            dotfiles: HashMap::from([("test".to_string(), dotfile)]),
        };
    }

    #[test]
    fn test_load_empty_config() {
        let mut state_repository = test_repository();
        let profile = state_repository.db.load_profile();
        assert_eq!(profile.dotfiles.len(), 0);
    }
}
