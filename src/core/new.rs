use rfd::FileDialog;
use std::fs::{create_dir, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::config::Project;
use crate::core::log_global::log_global;
use crate::core::utils::{self, validate_github_token};
use crate::log::LogType;

fn select_path() -> Option<PathBuf> {
  FileDialog::new().pick_folder()
}

pub fn create_project() -> Option<Project> {
  utils::clear();
  println!("Silahkan isi :)");
  println!("{}", "=".repeat(60));
  println!("");
  // user Input

  // folder name
  print!("Nama Proyek : ");
  io::stdout().flush().unwrap();
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("cannot read line");
  let name_folder = name.trim();

  // folder path
  let path = loop {
    match select_path() {
      Some(mut path) => {
        path.push(&name_folder);
        println!("Folder yang dipilih: {path:?}");
        break path;
      }
      None => {
        println!("Tidak ada folder yang dipilih");
        continue;
      }
    };
  };

  // email github
  print!("Email Github : ");
  io::stdout().flush().unwrap();
  let mut email = String::new();
  io::stdin().read_line(&mut email).expect("cannot read line");

  // username github
  print!("Username Github : ");
  io::stdout().flush().unwrap();
  let mut username = String::new();
  io::stdin()
    .read_line(&mut username)
    .expect("cannot read line");

  // url repo
  print!("URL Repo : ");
  io::stdout().flush().unwrap();
  let mut url_repo = String::new();
  io::stdin()
    .read_line(&mut url_repo)
    .expect("cannot read line");

  // token github
  let token = loop {
    print!("Token Github : ");
    io::stdout().flush().unwrap();
    let mut token = String::new();
    io::stdin().read_line(&mut token).expect("cannot read line");
    // validate token github
    let trimmed = token.trim();
    match validate_github_token(trimmed, username.trim()) {
      Ok(_) => {
        break trimmed.to_string();
      }
      Err(e) => {
        log_global(LogType::Error, e.to_string().as_str(), true);
        continue;
      }
    }
  };

  let mut data_project = Project::load_data_project();
  let project_data = Project {
    name: name.trim().to_string(),
    path: path.to_string_lossy().to_string(),
    username_github: username.trim().to_string(),
    email_github: email.trim().to_string(),
    url_repo: url_repo.trim().to_string(),
    token_github: token.trim().to_string(),
    logger: path.join("logs/program.log"),
    // last_edit: date.to_string(),
    last_edit: chrono::offset::Local::now()
      .format("%Y-%m-%d %H:%M:%S")
      .to_string(),
  };

  project_data.create_dir();

  let Some(logger_dir) = project_data.logger.parent() else {
    log_global(
      LogType::Error,
      "Gagal mendapatkan direktori logger {logger_dir:?}",
      true,
    );
    return None;
  };

  match create_dir(logger_dir).and_then(|_| File::create(&project_data.logger)) {
    Ok(_) => {}
    Err(e) => {
      log_global(
        LogType::Error,
        &format!("gagal menyiapkan log file : {e:?}"),
        true,
      );
      return None;
    }
  }
  project_data.open_project();
  data_project.push(project_data.clone());

  if let Err(e) = Project::save_project_data(&data_project) {
    log_global(
      LogType::Error,
      &format!("Gagal menyimpan data proyek: {e}"),
      true,
    );
  } else {
    log_global(
      LogType::Info,
      "Proyek baru berhasil ditambahkan dan disimpan!",
      true,
    );

    log_global(
      LogType::Path,
      &format!("path folder {}", project_data.path),
      false,
    );
  }

  Some(project_data)
}
