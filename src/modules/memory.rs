use anyhow::Result;
use async_trait::async_trait;
use systemstat::{Memory as MemoryStat, Platform, System};

use super::Module;
use crate::config::Config;
use crate::discovery::{Announcment, DiscoveryConfig};

pub struct Memory;

#[async_trait]
impl Module for Memory {
  type State = MemoryStat;

  #[inline]
  fn topic() -> &'static str {
    "memory"
  }

  fn announcments(cfg: &Config) -> Vec<Announcment> {
    let ha = cfg.homeassistant.clone().unwrap();

    vec![
      Announcment {
        domain: "sensor".to_string(),
        object_id: "memory_free".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "data_size".to_string(),
          enabled_by_default: true,
          name: format!("{} Memory Free", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/memory", cfg.mqtt.client_id),
          value_template: "{{ value_json.free.split(' ')[0] | float }}".to_string(),
          unit_of_measurement: "GB".to_string(),
          unique_id: format!("{}_memory_free", cfg.mqtt.client_id),
        },
      },
      Announcment {
        domain: "sensor".to_string(),
        object_id: "memory_total".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "data_size".to_string(),
          enabled_by_default: true,
          name: format!("{} Memory Total", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/memory", cfg.mqtt.client_id),
          value_template: "{{ value_json.total.split(' ')[0] | float }}".to_string(),
          unit_of_measurement: "GB".to_string(),
          unique_id: format!("{}_memory_total", cfg.mqtt.client_id),
        },
      },
    ]
  }

  async fn get_state(system: &System) -> Result<Self::State> {
    system.memory().map_err(anyhow::Error::msg)
  }
}
