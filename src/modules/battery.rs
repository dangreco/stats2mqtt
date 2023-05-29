use anyhow::Result;
use async_trait::async_trait;
use systemstat::{BatteryLife, Platform, System};

use super::Module;
use crate::config::Config;
use crate::discovery::{Announcment, DiscoveryConfig};

pub struct Battery;

#[async_trait]
impl Module for Battery {
  type State = BatteryLife;

  #[inline]
  fn topic() -> &'static str {
    "battery"
  }

  fn announcments(cfg: &Config) -> Vec<Announcment> {
    let ha = cfg.homeassistant.clone().unwrap();

    vec![
      Announcment {
        domain: "sensor".to_string(),
        object_id: "battery_level".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "battery".to_string(),
          enabled_by_default: true,
          name: format!("{} Battery Level", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/battery", cfg.mqtt.client_id),
          value_template: "{{ value_json.remaining_capacity * 100.0 | round(1) }}".to_string(),
          unit_of_measurement: "%".to_string(),
          unique_id: format!("{}_battery_level", cfg.mqtt.client_id),
        },
      },
      Announcment {
        domain: "sensor".to_string(),
        object_id: "battery_time_remaining".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "duration".to_string(),
          enabled_by_default: true,
          name: format!("{} Battery Time Remaining", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/battery", cfg.mqtt.client_id),
          value_template: "{{ value_json.remaining_time.secs }}".to_string(),
          unit_of_measurement: "s".to_string(),
          unique_id: format!("{}_battery_time_remaining", cfg.mqtt.client_id),
        },
      },
    ]
  }

  async fn get_state(system: &System) -> Result<Self::State> {
    system.battery_life().map_err(anyhow::Error::msg)
  }
}
