mod app;
mod config;
mod core;
mod github;
mod log;
mod menu;

use std::f32::consts::E;

use nix::libc::user;
use tokio;

use crate::core::log_global::{self, init_global_log, log_global};
use crate::github::{
  auth::{self, auth_user},
  client::UserGithub,
};
use crate::log::{LogType, Logger};
#[tokio::main]
async fn main() {
  if let Err(e) = init_global_log().await {
    eprintln!("Failed to initialize Log : {}", e);
    return;
  }

  log_global(LogType::Info, "Program Dimulai", false);

  match auth_user().await {
    Ok(user) => {
      println!("Hasil autentikasi: {:?}", user);

      if let Err(e) = UserGithub::write_user(user) {
        log_global(LogType::Error, &format!("Gagal Menulis User : {}", e), true);
      }

      let mut application = app::App::new();
      application.run();
    }
    Err(e) => {
      log_global(LogType::Error, &format!("Authentikasi Gagal : {}", e), true);
    }
  }
}
