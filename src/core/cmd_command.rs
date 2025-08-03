use std::{
  env,
  error::{self},
  process::{Command, Output},
};

use crate::config::Project;
use crate::core::utils;
fn execute_command(path: &str, command_script: &str) -> Result<Output, Box<dyn std::error::Error>> {
  let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
  let output = Command::new(&shell)
    .arg("-c")
    .current_dir(path)
    .arg(command_script)
    .output()
    .map_err(|e| format!("failed to execute command: {:?}", e))?;

  if !output.status.success() {
    return Err(
      format!(
        "Command execution failed:\nCommand: {}\nExit Code: {}\nError: {}",
        command_script,
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr)
      )
      .into(),
    );
  }
  Ok(output)
}

pub fn push(project: &Project) -> Result<(), Box<dyn error::Error>> {
  if !utils::check_git_installed() {
    return Err("Git is not installed yet, install first you dumb ass".into());
  }

  let path = &project.path;
  let mut command_script = String::new();

  if !utils::is_git_repo(path.to_string()) {
    println!("Initializing new Git Repository....");
    command_script.push_str(&format!(
            "git init && git branch -M main && git config --global user.name '{}' && git config --global user.email '{}' && git remote add origin '{}' &&",
            project.username_github, project.email_github, project.url_repo
        ));
  } else {
    println!("Git Repository already exists. Skipping init...");
  }

  let askpass_script_path =
    utils::create_askpass_script(&project.username_github, &project.token_github)?;

  env::set_var("GIT_ASKPASS", askpass_script_path);
  command_script.push_str(&format!(
    "git add . && git commit -m 'commit message' && git push -u origin main",
  ));

  match execute_command(path, &command_script) {
    Ok(output) => {
      println!(
        "Git commands executed susccesfully: \n{}",
        String::from_utf8_lossy(&output.stdout)
      );
      Ok(())
    }
    Err(e) => {
      eprintln!("Error executing Git command: {}", e);
      Err(e)
    }
  }
}

// pub fn push_git(project: &Project) -> Result<(), Box<dyn std::error::Error>> {
//     let commands = vec![
//         (vec!["config", "--global", "user.name", &project.username_github]),
//         (vec!["config", "--global", "user.email", &project.email_github]),
//         (vec!["remote", "add", "origin", &project.url_repo]),
//         (vec!["add", "."]),
//         (vec!["commit", "-m", "commit message"]),
//         (vec!["push", "-u", "origin", "main"]),
//     ];

//     if !utils::check_git_installed() {
//         return Err("Git is not installed yet, install first you dumb ass".into());
//     }

//     // fungsi untuk cek sudah pernah init atau belum
//     if !utils::is_git_repo(project.path.clone()) {
//         println!("Initializing new Git Repository....");
//         for args in commands {
//             let output = Command::new("git")
//                 .args(&args)
//                 .current_dir(&project.path)
//                 .output()?;

//             if !output.status.success() {
//                 return Err(format!(
//                     "Git command failed : {}\nError: {}",
//                     args.join(" "),
//                     String::from_utf8_lossy(&output.stderr)
//                 )
//                 .into());
//             }
//             println!("Successfully executed : git {}", args.join(" "));
//             println!("Output: {}", String::from_utf8_lossy(&output.stdout))
//         }
//     } else {
//         println!("sudah pernah push")
//     }
//     Ok(())
// }
