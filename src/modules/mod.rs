use anyhow::Result;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use systemstat::{Platform, System};
use tokio::{task, time};

use crate::config::Config;
use crate::discovery::{announce, Announcment};

mod cpu;
pub use cpu::*;

mod battery;
pub use battery::*;

mod power;
pub use power::*;

mod memory;
pub use memory::*;

fn hash_str(s: &String) -> u64 {
  let mut h = DefaultHasher::new();
  s.hash(&mut h);
  h.finish()
}

#[async_trait]
pub trait Module {
  type State: Serialize + Send;

  fn topic() -> &'static str;

  async fn get_state(system: &System) -> Result<Self::State>;
  fn announcments(cfg: &Config) -> Vec<Announcment>;

  fn init(config: &Config, base_topic: &String, mqtt: &AsyncClient) {
    let topic = format!("{base_topic}/{}", Self::topic());
    let mqtt = mqtt.clone();
    let interval = config.intervals.for_topic(Self::topic());
    let cfg = config.clone();

    task::spawn(async move {
      let system = System::new();
      let mut last = 0u64;

      // Announce if enabled
      match cfg.homeassistant {
        Some(_) => {
          for announcment in Self::announcments(&cfg) {
            announce(&cfg, &mqtt, &announcment).await;
          }
        }
        None => (),
      }

      'main: loop {
        let state = Self::get_state(&system).await;

        match state {
          Ok(state) => {
            let data = serde_json::to_string(&state).unwrap();
            let hashed = hash_str(&data);

            if last != hashed {
              last = hashed;

              mqtt
                .publish(&topic, QoS::AtLeastOnce, false, data)
                .await
                .unwrap();
            }
          }
          Err(_) => {
            println!("Failed to get data for {}", Self::topic());
            break 'main;
          },
        }

        time::sleep(interval).await;
      }
    });
  }
}
