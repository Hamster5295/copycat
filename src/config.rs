use std::sync::{Arc, OnceLock};

use anyhow::Result;
use kovi::{
    RuntimeBot,
    log::{info, warn},
    tokio::fs,
};
use serde::Deserialize;

pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize)]
struct ConfigFile {
    repeat_after: Option<u32>,
    allow_groups: Option<Vec<i64>>,
}

pub(crate) struct Config {
    pub(crate) repeat_after: u32,
    pub(crate) allow_groups: Option<Vec<i64>>,
}

pub(crate) async fn init(bot: &Arc<RuntimeBot>) -> Result<&Config> {
    let config_path = bot.get_data_path().join("config.toml");

    let config_txt = match fs::read_to_string(&config_path).await {
        Ok(txt) => txt,
        Err(e) => {
            warn!("[copycat] Failed to read config file: {}", e);
            info!("[copycat] Using default config");
            String::new()
        }
    };

    // .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;
    let config: ConfigFile = toml::from_str(&config_txt)?;
    Ok(CONFIG.get_or_init(|| Config {
        repeat_after: config.repeat_after.unwrap_or(2),
        allow_groups: config.allow_groups,
    }))
}
