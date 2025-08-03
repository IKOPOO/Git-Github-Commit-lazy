use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::{self, PathBuf};
use std::sync::Arc;
use std::{error, fs, io::Write, path::Path, process::Command, vec};

#[derive(Debug, Serialize, Deserialize, Clone)]
// saving data project
pub struct Project {
  pub name: String,
  pub path: String,
  pub username_github: String,
  pub email_github: String,
  pub url_repo: String,
  pub token_github: String,
  pub last_edit: String,
  pub logger: PathBuf,
}

impl Project {
  // membuat direktori
  pub fn create_dir(&self) {
    match fs::create_dir(&self.path) {
      Ok(_) => {
        println!("proyek berhasil dibuat di: {}", self.path)
      }
      Err(e) => {
        println!("gagal membuat directory: {}", e);
      }
    }
  }

  // membuka project
  pub fn open_project(&self) {
    Command::new("code")
      .arg(&self.path)
      .spawn()
      .expect("gagal dalam membuka proyek");
  }

  // load project/membaca project yang sudah pernah dibuat
  pub fn load_data_project() -> Vec<Project> {
    const FILE_PATH: &'static str = "project_data.json";
    if Path::new(FILE_PATH).exists() {
      let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
      serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
      vec![]
    }
  }

  // menyimpan informasi project
  pub fn save_project_data(project: &[Project]) -> Result<(), Box<dyn error::Error>> {
    const FILE_PATH: &'static str = "project_data.json";
    let json_data = serde_json::to_string_pretty(project).unwrap();
    let mut file = fs::File::create(FILE_PATH)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ProcessStatus {
  Running,
  NotRunning,
  NoPermission,
  Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
  pub repo_name: String,
  pub description: String,
  pub private: bool,
}

impl RepoConfig {
  pub fn load() -> anyhow::Result<Self> {
    const JSON_PATH: &str = "repo_config.json";
    if Path::new(JSON_PATH).exists() {
      let content = fs::read_to_string(JSON_PATH)?;
      let config = serde_json::from_str(&content)?;
      Ok(config)
    } else {
      Err(anyhow::anyhow!("Configuration file not found"))
    }
  }
}
