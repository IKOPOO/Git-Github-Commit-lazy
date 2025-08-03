use chrono::Local;
use colored::*;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::sync::{Arc, Mutex};

use crate::log;

// tipe log yang dikeluarkan
pub enum LogType {
  Info,
  Warn,
  Error,
  Input,
  Path,
  Auth,
}

// log per project
pub struct Logger {
  file: Vec<File>,
}

impl Logger {
  pub fn new(log_path: &[String]) -> Self {
    let file = log_path
      .iter()
      .map(|path| {
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(path)
          .expect(&format!("Cannot open log file: {}", path))
      })
      .collect();

    Self { file }
  }
}

#[derive(Debug)]
pub struct GlobalLogger {
  // log global untuk sistem
  file: Mutex<File>,
}

impl GlobalLogger {
  pub fn new(path: &str) -> Self {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(path)
      .expect("cannot open global log file");

    Self {
      file: Mutex::new(file),
    }
  }
}

pub trait Loggertrait {
  fn log_with_type(&self, log_type: LogType, message: &str, show_on_terminal: bool) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let prefix = match log_type {
      LogType::Info => "INFO",
      LogType::Warn => "WARN",
      LogType::Error => "ERROR",
      LogType::Input => "INPUT",
      LogType::Path => "PATH",
      LogType::Auth => "AUTH",
    };

    if show_on_terminal {
      let colored_prefix = match log_type {
        LogType::Info => prefix.green(),
        LogType::Warn => prefix.yellow(),
        LogType::Error => prefix.red(),
        LogType::Input => prefix.blue(),
        LogType::Path => prefix.cyan(),
        LogType::Auth => prefix.magenta(),
      };
      println!("{} {}", colored_prefix, message);
    }

    let line = format!("{timestamp} {:?} {message}\n", prefix);

    line
  }

  fn write_log(&self, log_type: LogType, message: &str, show_on_terminal: bool) {}
}

impl Loggertrait for Logger {
  fn write_log(&self, log_type: LogType, message: &str, show_on_terminal: bool) {
    let log_str = self.log_with_type(log_type, message, show_on_terminal);
    for mut file in &self.file {
      file
        .write_all(log_str.as_bytes())
        .expect("Failed to write log to file");
    }
    if show_on_terminal {
      println!("{log_str}");
    }
  }
}

impl Loggertrait for GlobalLogger {
  fn write_log(&self, log_type: LogType, message: &str, show_on_terminal: bool) {
    let log_str = self.log_with_type(log_type, message, show_on_terminal);
    let mut file = self.file.lock().unwrap();
    file
      .write_all(log_str.as_bytes())
      .expect("failed to write log to global file");

    if show_on_terminal {
      println!("{log_str}");
    }
  }
}
