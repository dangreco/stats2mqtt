use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeAssistantConfig {
  #[serde(default = "Defaults::friendly_name")]
  pub friendly_name: String,

  #[serde(default = "Defaults::discovery_topic")]
  pub discovery_topic: String,

  #[serde(default = "Defaults::status_topic")]
  pub status_topic: String,
}

struct Defaults;
impl Defaults {
  pub fn friendly_name() -> String {
    hardware_id::get_id().unwrap()
  }

  pub fn discovery_topic() -> String {
    "homeassistant".to_string()
  }

  pub fn status_topic() -> String {
    "hass/status".to_string()
  }
}
