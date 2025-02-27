use serde::{Deserialize, Serialize};
use std::{error, fs, io::Write, path::Path, process::Command, vec};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub username_github: String,
    pub email_github: String,
    pub url_repo: String,
    pub token_github: String,
    pub last_edit: String,
}

impl Project {
    pub fn create_dir(&self) {
        match fs::create_dir(&self.path) {
            Ok(_) => {
                println!("directory sudah selesai dibuat di: {}", &self.path);
            }
            Err(e) => {
                println!("gagal membuat directory: {}", e);
            }
        }
    }

    pub fn open_project(&self) {
        Command::new("code")
            .arg(&self.path)
            .spawn()
            .expect("gagal dalam membuka proyek");
    }

    pub fn load_data_project() -> Vec<Project> {
        const FILE_PATH: &'static str = "project_data.json";
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
            serde_json::from_str(&data).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        }
    }

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
