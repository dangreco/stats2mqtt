use anyhow::Result;
use dotenv::dotenv;

mod config;
use config::Config;

mod modules;
use modules::*;

mod discovery;
use discovery::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let cfg = Config::load()?;
  let (mqtt, mut eventloop) = cfg.mqtt.create_client();

  let base_topic = format!("stats2mqtt/{}", cfg.mqtt.client_id);

  Cpu::init(&cfg, &base_topic, &mqtt);
  Battery::init(&cfg, &base_topic, &mqtt);
  Power::init(&cfg, &base_topic, &mqtt);
  Memory::init(&cfg, &base_topic, &mqtt);

  while let Ok(_) = eventloop.poll().await {}

  Ok(())
}
