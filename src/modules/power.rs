use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use systemstat::{Platform, System};

use super::Module;
use crate::config::Config;
use crate::discovery::{Announcment, DiscoveryConfig};

#[derive(Debug, Clone, Serialize)]
pub struct PowerState {
  ac_connected: bool,
}

pub struct Power;

#[async_trait]
impl Module for Power {
  type State = PowerState;

  #[inline]
  fn topic() -> &'static str {
    "power"
  }

  fn announcments(cfg: &Config) -> Vec<Announcment> {
    let ha = cfg.homeassistant.clone().unwrap();

    let h = System::new();
    if h.on_ac_power().is_err() {
      return vec![];
    }

    vec![Announcment {
      domain: "binary_sensor".to_string(),
      object_id: "ac_power".to_string(),
      config: DiscoveryConfig {
        device: cfg.device(),
        device_class: "plug".to_string(),
        enabled_by_default: true,
        name: format!("{} AC Power", ha.friendly_name),
        state_class: "measurement".to_string(),
        state_topic: format!("stats2mqtt/{}/power", cfg.mqtt.client_id),
        value_template: "{{ 'ON' if value_json.ac_connected else 'OFF' }}".to_string(),
        unit_of_measurement: "".to_string(),
        unique_id: format!("{}_ac_power", cfg.mqtt.client_id),
      },
    }]
  }

  async fn get_state(system: &System) -> Result<Self::State> {
    let ac_connected = system.on_ac_power()?;
    Ok(PowerState { ac_connected })
  }
}
