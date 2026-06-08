use actix_web::{App, HttpServer, web};
use clap::Parser;

use crate::client::ProxyClient;
use crate::cmd::NetimitorConfig;

pub mod client;
pub mod cmd;
pub mod server;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  let config = NetimitorConfig::parse();
  let emu = config.resolve_emulation();
  let client = web::Data::new(ProxyClient::new(emu));

  log::info!("welcome to netimitor");
  log::info!("emulation profile: {:?}", emu);
  log::info!("starting proxy server...");

  HttpServer::new(move || {
    App::new()
      .app_data(client.clone())
      .wrap(actix_web::middleware::Logger::default())
      .configure(server::configure_app)
  })
  .bind(&config.address)?
  .run()
  .await?;

  Ok(())
}
