use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub dotfiles: HashMap<String, Dotfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dotfile {
    pub source: String,
    pub description: Option<String>,
}

/*
pub fn load_profile(db: &mut sled::Db) -> Result<Config, Box<dyn Error>> {
    // TODO Add some custom error types
    // Also clean up error handling here
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

pub fn append_config_toml(db: &mut sled::Db, name: &str, profile: Profile) {
    let mut config = match load_profiles(db) {
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
*/
