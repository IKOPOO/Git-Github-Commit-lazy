use anyhow::Ok;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

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
    const FILE_PATH: &'static str = ".config/user.json";
    let json_data = serde_json::to_string_pretty(&user).unwrap();
    let mut file = fs::File::create(FILE_PATH)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
  }
}
