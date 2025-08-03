mod app;
mod config;
mod core;
mod github;
mod log;
mod menu;

use tokio;

use crate::core::log_global::log_global;
use crate::github::{
  auth::{self, auth_user},
  client::UserGithub,
};
use crate::log::{LogType, Logger};
#[tokio::main]
async fn main() {
  // // inisialisasi logger di awal program
  // let logger = Logger::new("program.log");
  // GLOBAL_LOGGER
  //   .set(Arc::new(logger))
  //   .expect("Failed to set global logger");

  log_global(LogType::Info, "Program dimulai", false);

  let auth_result = auth_user().await;
  println!("hasil authentikasi: {:?}", auth_result);
  let _ = UserGithub::write_user(auth_result.expect("gagal nulis wok tanganku sakit"));
  let mut aplication = app::App::new();
  aplication.run();
}
