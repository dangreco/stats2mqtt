use serde::{Deserialize, Serialize};
use std::default::Default;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntervalsConfig {
  #[serde(default = "Defaults::cpu")]
  pub cpu: Duration,

  #[serde(default = "Defaults::memory")]
  pub memory: Duration,

  #[serde(default = "Defaults::battery")]
  pub battery: Duration,
  
  #[serde(default = "Defaults::power")]
  pub power: Duration,
}

impl IntervalsConfig {
  pub fn for_topic<'s, S>(&self, topic: S) -> Duration
  where
    S: Into<String>,
  {
    let s: String = topic.into();

    match s.as_str() {
      "cpu" => self.cpu,
      "memory" => self.memory,
      "battery" => self.battery,
      "power" => self.power,
      _ => Duration::MAX,
    }
  }
}

impl Default for IntervalsConfig {
  fn default() -> Self {
    Self {
      cpu: Defaults::cpu(),
      memory: Defaults::memory(),
      battery: Defaults::battery(),
      power: Defaults::power(),
    }
  }
}

struct Defaults;
impl Defaults {
  pub fn cpu() -> Duration {
    Duration::from_secs(15)
  }

  pub fn memory() -> Duration {
    Duration::from_secs(15)
  }

  pub fn battery() -> Duration {
    Duration::from_secs(60)
  }

  pub fn power() -> Duration {
    Duration::from_secs(1)
  }
}
