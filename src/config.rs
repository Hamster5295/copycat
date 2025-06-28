use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result};
use kovi::{RuntimeBot, tokio::fs};
use serde::Deserialize;

pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) repeat_after: u32,
    pub(crate) allow_groups : Vec<i64>
}

pub(crate) async fn init(bot: &Arc<RuntimeBot>) -> Result<&Config> {
    let config_path = bot.get_data_path().join("config.toml");
    let config_txt = fs::read_to_string(&config_path)
        .await
        .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;
    let config: Config = toml::from_str(&config_txt)?;
    Ok(CONFIG.get_or_init(|| config))
}
