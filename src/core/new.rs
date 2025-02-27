use rfd::FileDialog;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::config::Project;
use crate::core::utils;

fn select_path() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn create_project() -> Option<Project> {
    utils::clear();
    println!("Silahkan isi :)");
    println!("{}", "=".repeat(60));
    println!("");
    // user input
    // folder name
    print!("Nama Folder: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("cannot read line");
    let name_folder = name.trim();
    // folder path
    let path = match select_path() {
        Some(mut path) => {
            path.push(&name_folder);
            println!("path lengkap: {}", path.display());
            path
        }
        None => {
            println!("folder yang dipilih tidak ada, operasi di batalkan ");
            return None;
        }
    };

    // email github
    print!("Email Github: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("cannot read line");
    // username github
    print!("Username Github: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("cannot read line");
    // url repo
    print!("Url repo: ");
    io::stdout().flush().unwrap();
    let mut url_repo = String::new();
    io::stdin()
        .read_line(&mut url_repo)
        .expect("cannot read line");
    // token github
    print!("Token Github: ");
    io::stdout().flush().unwrap();
    let mut token = String::new();
    io::stdin().read_line(&mut token).expect("cannot read line");

    let mut data_project = Project::load_data_project();
    let project_data = Project {
        name: name.trim().to_string(),
        path: path.to_string_lossy().to_string(),
        username_github: username.trim().to_string(),
        email_github: email.trim().to_string(),
        url_repo: url_repo.trim().to_string(),
        token_github: token.trim().to_string(),
        // last_edit: date.to_string(),
        last_edit: chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
    };
    project_data.create_dir();
    project_data.open_project();
    data_project.push(project_data.clone());

    if let Err(e) = Project::save_project_data(&data_project) {
        eprintln!("Gagal menyimpan data ke dalam file: {e}")
    } else {
        println!("Proyek baru berhasil ditambahkan dan disimpan!")
    }

    Some(project_data)
}
