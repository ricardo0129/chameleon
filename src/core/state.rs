use crate::config::profile::Profile;
use serde_json;
use sled;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct State {
    #[allow(dead_code)]
    pub config: Profile,
    pub _active_profiles: HashSet<String>,
}

pub trait StateStore {
    fn load_profiles(&mut self) -> Profile;
    fn save_config(&mut self, config: &Profile);
}

struct StateRepository {
    pub db: Arc<dyn StateStore>,
}

impl StateStore for sled::Db {
    #[allow(dead_code)]
    fn load_profiles(&mut self) -> Profile {
        let key = "config";
        let v = self.get(key).unwrap();
        if let Some(encoded_config) = v {
            serde_json::from_slice::<Profile>(&encoded_config).unwrap();
        }
        // No config found returning empty config
        Profile {
            dotfiles: HashMap::new(),
        }
    }
    fn save_config(&mut self, config: &Profile) {
        let key = "config";
        let value: Vec<u8> = serde_json::to_vec(config).unwrap();
        self.insert(key, value).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::profile::{Dotfile, Profile};
    use std::collections::HashMap;
    use std::env;
    use uuid::Uuid;

    fn test_db() -> sled::Db {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(Uuid::new_v4().to_string());
        sled::open(temp_dir).unwrap()
    }

    #[test]
    fn test_save_config() {
        let mut db = test_db();
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
        let mut db = test_db();
        let config = load_profiles(&mut db);
        assert_eq!(config.profiles.len(), 0);
    }
}
