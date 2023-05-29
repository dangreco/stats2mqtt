use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct MqttConfig {
  pub server: String,

  #[serde(default = "Defaults::port")]
  pub port: u16,

  #[serde(default = "Defaults::client_id")]
  pub client_id: String,

  #[serde(default = "Defaults::keepalive")]
  pub keepalive: Duration,

  pub username: Option<String>,
  pub password: Option<String>,
}

struct Defaults;
impl Defaults {
  pub fn port() -> u16 {
    1883
  }

  pub fn client_id() -> String {
    hardware_id::get_id().unwrap()
  }

  pub fn keepalive() -> Duration {
    Duration::from_secs(60)
  }
}

impl MqttConfig {
  pub fn create_client(&self) -> (AsyncClient, EventLoop) {
    let mut opts = MqttOptions::new(&self.client_id, &self.server, self.port);
    opts.set_keep_alive(self.keepalive);

    match (&self.username, &self.password) {
      (Some(username), Some(password)) => {
        opts.set_credentials(username, password);
      }
      _ => {}
    }

    AsyncClient::new(opts, 10)
  }
}
