use anyhow::Ok;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::fs;

use crate::log::{GlobalLogger, LogType, Loggertrait};

pub static GLOBAL_LOGGER: OnceCell<Arc<GlobalLogger>> = OnceCell::new();

pub fn log_global(log_type: LogType, message: &str, show: bool) {
  if let Some(logger) = GLOBAL_LOGGER.get() {
    logger.log_with_type(log_type, message, show);
    logger.write_log(log_type, message, show);
  } else {
    eprintln!("Logger belum diinisialisasi");
    // if let Err(e) = init_global_log().await {
    //   eprintln!("Failed to reinitialize log : {}", e);
    // } else {
    //   if let Some(logger) = GLOBAL_LOGGER.get() {
    //     logger.log_with_type(log_type, message, show);
    //     logger.write_log(log_type, message, show);
    //   }
    // }
  }
}

pub async fn init_global_log() -> Result<(), anyhow::Error> {
  let config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("cannot get config dirs"))?;
  let folder_path = config_dir.join("myapp/GlobalLog");
  fs::create_dir_all(&folder_path).await?;

  let log_file = folder_path.join("GlobalLog.log");
  let logger = GlobalLogger::new(log_file.to_str().unwrap());

  GLOBAL_LOGGER
    .set(logger.into())
    .map_err(|_| anyhow::anyhow!("Logger has been initialized"))?;

  Ok(())
}
