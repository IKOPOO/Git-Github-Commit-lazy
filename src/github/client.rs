use anyhow::Ok;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserGithub {
  pub id: u64,
  pub login: String,
  pub name: String,
  pub email: String,
  pub last_verified: DateTime<chrono::Utc>,
}
impl UserGithub {
  pub fn new(user: octocrab::models::Author) -> Self {
    Self {
      id: user.id.0,
      login: user.login,
      name: user.name.unwrap_or_default(),
      email: user.email.unwrap_or_default(),
      last_verified: chrono::Utc::now(),
    }
  }

  pub fn write_user(user: UserGithub) -> Result<(), anyhow::Error> {
    let config_dir = Path::new(".config");
    fs::create_dir(config_dir)?;

    let file_path = config_dir.join("user.json");
    let json_data = serde_json::to_string_pretty(&user)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
  }
}
