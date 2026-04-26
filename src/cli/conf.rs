use crate::cli::defaults;

pub struct Config {
    pub db_location: String,
}

impl Config {
    pub fn load() -> Self {
        Config {
            db_location: std::env::var("DB_LOCATION")
                .unwrap_or(defaults::DEFAULT_DB_LOCATION.to_string()),
        }
    }
}
