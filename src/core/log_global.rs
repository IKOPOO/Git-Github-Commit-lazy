use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::log::{self, GlobalLogger, LogType, Loggertrait};

pub static GLOBAL_LOGGER: OnceCell<Arc<GlobalLogger>> = OnceCell::new();

pub fn log_global(log_type: LogType, message: &str, show: bool) {
  if let Some(logger) = GLOBAL_LOGGER.get() {
    GlobalLogger::log_with_type(logger, log_type, message, show);
  } else {
    eprintln!("Logger belum diinisialisasi");
  }
}
