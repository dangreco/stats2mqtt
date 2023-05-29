use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::time::Duration;
use systemstat::{CPULoad, Platform, System};

use super::Module;
use crate::config::Config;
use crate::discovery::{Announcment, DiscoveryConfig};

#[derive(Debug, Clone, Serialize)]
pub struct CpuState {
  pub temperature: f32,
  pub load: CPULoad,
}

pub struct Cpu;

#[async_trait]
impl Module for Cpu {
  type State = CpuState;

  #[inline]
  fn topic() -> &'static str {
    "cpu"
  }

  fn announcments(cfg: &Config) -> Vec<Announcment> {
    let ha = cfg.homeassistant.clone().unwrap();

    vec![
      Announcment {
        domain: "sensor".to_string(),
        object_id: "cpu_temperature".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "temperature".to_string(),
          enabled_by_default: true,
          name: format!("{} CPU Temperature", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/cpu", cfg.mqtt.client_id),
          value_template: "{{ value_json.temperature }}".to_string(),
          unit_of_measurement: "°C".to_string(),
          unique_id: format!("{}_cpu_temperature", cfg.mqtt.client_id),
        },
      },
      Announcment {
        domain: "sensor".to_string(),
        object_id: "cpu_load_user".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "power_factor".to_string(),
          enabled_by_default: true,
          name: format!("{} CPU Load User", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/cpu", cfg.mqtt.client_id),
          value_template: "{{ value_json.load.user * 100.0 }}".to_string(),
          unit_of_measurement: "%".to_string(),
          unique_id: format!("{}_cpu_load_user", cfg.mqtt.client_id)
        },
      },
      Announcment {
        domain: "sensor".to_string(),
        object_id: "cpu_load_system".to_string(),
        config: DiscoveryConfig {
          device: cfg.device(),
          device_class: "power_factor".to_string(),
          enabled_by_default: true,
          name: format!("{} CPU Load System", ha.friendly_name),
          state_class: "measurement".to_string(),
          state_topic: format!("stats2mqtt/{}/cpu", cfg.mqtt.client_id),
          value_template: "{{ value_json.load.system * 100.0 }}".to_string(),
          unit_of_measurement: "%".to_string(),
          unique_id: format!("{}_cpu_load_system", cfg.mqtt.client_id),
        },
      },
    ]
  }

  async fn get_state(system: &System) -> Result<Self::State> {
    /* CPU Temp */
    let temperature = system.cpu_temp()?;

    /* CPU Load */
    let load_delayed = system.cpu_load_aggregate()?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let load = load_delayed.done()?;

    Ok(CpuState { temperature, load })
  }
}
