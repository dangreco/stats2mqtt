use anyhow::Result;
use serde::{Deserialize, Serialize};

mod homeassistant;
mod intervals;
mod mqtt;

pub use homeassistant::HomeAssistantConfig;
pub use intervals::IntervalsConfig;
pub use mqtt::MqttConfig;

use crate::discovery::Device;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub mqtt: MqttConfig,
  pub homeassistant: Option<HomeAssistantConfig>,

  #[serde(default)]
  pub intervals: IntervalsConfig,
}

impl Config {
  pub fn load() -> Result<Config> {
    let path = std::env::var("STATS2MQTT_CONFIG")
      .map_err(|_| anyhow::anyhow!("\"STATS2MQTT_CONFIG\" environment variable not specified."))?;
    let contents = std::fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(anyhow::Error::msg)
  }

  pub fn device(&self) -> Device {
    Device {
      identifiers: vec![format!("stats2mqtt_{}", self.mqtt.client_id)],
      name: self.homeassistant.clone().unwrap().friendly_name,
      manufacturer: "stats2mqtt".to_string(),
      sw_version: env!("CARGO_PKG_VERSION").to_string(),
    }
  }
}
