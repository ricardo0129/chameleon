use crate::config::profile::{Config, Profile};
use serde::Serialize;
use serde_json;
use sled;
use std::collections::{HashMap, HashSet};

struct Mem {
    config: Config,
    active_profiles: HashSet<String>,
}

impl Mem {}

pub fn load_profiles(db: sled::Db) -> Config {
    let key = "config";
    let v = db.get(key).unwrap();
    if let Some(encoded_config) = v {
        serde_json::from_slice::<Config>(&encoded_config).unwrap();
    }
    // No config found returning empty config
    Config::new()
}

pub fn save_config(db: sled::Db, config: &Config) {
    let key = "config";
    let value: Vec<u8> = serde_json::to_vec(config).unwrap();
    db.insert(key, value).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use uuid::Uuid;

    #[test]
    fn test_save_config() {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(Uuid::new_v4().to_string());
        println!("{:?}", temp_dir);
        let db = sled::open(temp_dir).unwrap();
        let key = "config";
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
        let mut temp_dir = env::temp_dir();
        temp_dir.push(Uuid::new_v4().to_string());
        println!("{:?}", temp_dir);
        let db = sled::open(temp_dir).unwrap();
        let config = load_profiles(db);
    }
}
