use crate::config::profile::Config;
use serde_json;
use sled;
use std::collections::{HashMap, HashSet};

pub struct Mem {
    #[allow(dead_code)]
    pub config: Config,
    pub _active_profiles: HashSet<String>,
}

impl Mem {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let db = sled::open("/temp").unwrap();
        let config = load_profiles(db);
        Mem {
            config,
            _active_profiles: HashSet::new(),
        }
    }
}

#[allow(dead_code)]
pub fn load_profiles(db: sled::Db) -> Config {
    let key = "config";
    let v = db.get(key).unwrap();
    if let Some(encoded_config) = v {
        serde_json::from_slice::<Config>(&encoded_config).unwrap();
    }
    // No config found returning empty config
    Config {
        profiles: HashMap::new(),
    }
}

pub fn save_config(db: sled::Db, config: &Config) {
    let key = "config";
    let value: Vec<u8> = serde_json::to_vec(config).unwrap();
    db.insert(key, value).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::profile::Profile;
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
        let db = test_db();
        let profile = Profile {
            source: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let test_profile = Config {
            profiles: HashMap::from([("test".to_string(), profile)]),
        };
        save_config(db, &test_profile);
    }

    #[test]
    fn test_load_empty_config() {
        let db = test_db();
        let config = load_profiles(db);
        assert_eq!(config.profiles.len(), 0);
    }
}
