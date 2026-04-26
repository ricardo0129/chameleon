use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub source: String,
    pub description: Option<String>,
}

impl Config {
    pub fn add_profile(&mut self, name: &str, profile: Profile) {
        self.profiles.insert(name.to_string(), profile);
    }
}

pub fn load_profiles(file_path: &str) -> Result<Config, Box<dyn Error>> {
    // TODO Add some custom error types
    // Also clean up error handling here
    println!("Loading profiles from: {}", file_path);
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(config)
}

pub fn append_config_toml(file_path: &str, name: &str, profile: Profile) {
    let mut config = match load_profiles(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    config.add_profile(name, profile);
    write_config_toml(&config, file_path);
}

pub fn write_config_toml(config: &Config, file_path: &str) {
    let path = std::path::Path::new(file_path);
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    let toml_string = toml::to_string_pretty(config).unwrap();
    fs::write(file_path, toml_string).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_load_profiles_valid() {
        let config = load_profiles("tests/resources/config.toml");
        assert!(config.is_ok());
    }

    #[test]
    fn test_load_profiles_invalid() {
        let config = load_profiles("tests/resources/bad_config.toml");
        assert!(config.is_err());
    }

    #[test]
    fn test_write_config_toml() {
        let profile = Profile {
            source: String::from("test_profile"),
            description: "test profile description".to_string().into(),
        };
        let mut config = Config {
            profiles: HashMap::new(),
        };
        config.add_profile("test", profile);
        let mut temp_dir = env::temp_dir();
        temp_dir.push("test_config.toml");
        write_config_toml(&config, temp_dir.to_str().unwrap());
        assert!(temp_dir.exists());
    }
}
