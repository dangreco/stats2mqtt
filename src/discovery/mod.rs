use crate::Config;
use anyhow::Result;
use rumqttc::{AsyncClient, QoS};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryConfig {
  pub device: Device,
  pub device_class: String,
  pub enabled_by_default: bool,
  pub name: String,
  pub state_class: String,
  pub state_topic: String,
  pub value_template: String,
  pub unit_of_measurement: String,
  pub unique_id: String,
}

#[derive(Debug, Clone)]
pub struct Announcment {
  pub domain: String,
  pub object_id: String,
  pub config: DiscoveryConfig,
}

#[derive(Debug, Clone, Serialize)]
pub struct Device {
  pub identifiers: Vec<String>,
  pub name: String,
  pub manufacturer: String,
  pub sw_version: String,
}

pub async fn announce(cfg: &Config, mqtt: &AsyncClient, announcment: &Announcment) -> Result<()> {
  let ha = cfg.homeassistant.clone().unwrap();
  let data = serde_json::to_string(&announcment.config)?;

  mqtt
    .publish(
      format!(
        "{}/{}/{}/{}/config",
        ha.discovery_topic, announcment.domain, cfg.mqtt.client_id, announcment.object_id
      ),
      QoS::AtLeastOnce,
      true,
      data,
    )
    .await?;

  Ok(())
}
