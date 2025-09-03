use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref CONFIG: Config = load_config();
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub languages: Vec<LanguageConfig>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub file_name: String,
    pub command: Vec<String>,
}

fn load_config() -> Config {
    let config_file = std::fs::read_to_string("config.json").expect("Failed to read config file");
    serde_json::from_str::<Config>(&config_file).expect("Failed to parse config file.")
}
