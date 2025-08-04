use anyhow::{anyhow, Result};
use dirs;
use octocrab::Octocrab;
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::Path;

use crate::github::client::*;
use crate::github::github;

pub async fn get_token_create_client(user_token: Option<String>) -> Result<Octocrab> {
  let token = match user_token {
    Some(t) => {
      let folder_path = dirs::config_dir().unwrap().join("myapp");
      fs::create_dir_all(&folder_path)?;

      let file_path = Path::new(&folder_path).join("token.txt");
      let mut file = File::create(file_path)?;

      file.write_all(t.as_bytes())?;
      t
    }
    None => {
      let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("Path tidak bisa didapatkan"))?;
      let folder_path = config_dir.join("myapp");
      let file_path = folder_path.join("token.txt");
      let isi = fs::read_to_string(&file_path)?;
      isi
    }
  };

  let octocrab = github::GithubClient::new(token).await?.client();
  Ok(octocrab)
}

pub async fn get_auth_user(client: Octocrab) -> Result<UserGithub, anyhow::Error> {
  let author = client.current().user().await?;
  Ok(UserGithub::new(author))
}

pub async fn auth_user() -> Result<UserGithub> {
  let mut client = match get_token_create_client(None).await {
    Ok(client) => client,
    Err(_) => {
      println!("Token tidak ADA");
      print!("Masukkan TOKEN PAT : ");
      stdout().flush().unwrap();
      let mut input = String::new();
      stdin().read_line(&mut input).expect("Cannot read line");
      let token = input.trim().to_string();

      get_token_create_client(Some(token)).await?
    }
  };
  loop {
    match get_auth_user(client.clone()).await {
      Ok(user) => {
        println!("Token Valid, Selamat Bekerja!!");
        break Ok(user);
      }
      Err(e) => {
        // minta user masukkan lagi token yang baru
        println!("Token anda {e}");
        print!("Input ulang token anda : ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("cannot read line");
        let token = input.trim().to_string();

        client = get_token_create_client(Some(token)).await?;
      }
    }
  }
}
