use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Can be PORT, HTML_PATH and REGISTER_SERVICE, RUN_DIRECT, RUN
    pub action: String,

    pub port: u16,
    pub html_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBuilder {
    // Can be PORT, htmlpath and RUN_DIRECT, RUN
    pub action: Option<String>,
    pub port: Option<u16>,
    pub html_path: Option<String>,
}

pub fn get_config() -> &'static Config {
    static CFG: Lazy<Config> = Lazy::new(|| create_config());

    &*CFG
}

pub fn create_config() -> Config {
    let file = std::fs::read_to_string(
        "C:\\Users\\sgfreshidea\\Desktop\\bots\\shopee\\logger_service\\config.toml",
    )
    .expect("Please check config.toml file");
    let cfg: ConfigBuilder = toml::from_str(&file).expect("Invalid config.toml file");

    Config {
        action: cfg.action.unwrap_or_else(|| "RUN".to_string()),
        port: cfg.port.unwrap_or(1729),

        html_path: cfg.html_path.expect("Please specify html path"),
    }
}
