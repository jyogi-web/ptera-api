use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub region: String,
    pub table_name: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    if cfg!(debug_assertions) {
        dotenv().unwrap();
    }
    envy::from_env().unwrap()
});
