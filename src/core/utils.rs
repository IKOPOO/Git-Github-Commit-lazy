use nix::libc;
use std::error;
use std::fs::{self};
use std::io::Write;
use std::os::raw::c_int;
use std::os::unix::fs::PermissionsExt;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use crate::config::ProcessStatus;

pub fn check_git_installed() -> bool {
    let output = Command::new("git").arg("--version").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn is_git_repo(path: String) -> bool {
    let git_dir = Path::new(&path).join(".git");
    return git_dir.exists();
}

pub fn clear() {
    Command::new("clear").status().unwrap();
}

extern "C" {
    fn kill(pid: c_int, sig: c_int) -> c_int;
}

pub fn check_process(pid: i32) -> ProcessStatus {
    match unsafe { kill(pid, 0) } {
        0 => ProcessStatus::Running,
        _ => match std::io::Error::last_os_error().raw_os_error() {
            Some(libc::ESRCH) => ProcessStatus::NotRunning,
            Some(libc::EPERM) => ProcessStatus::NoPermission,
            _ => ProcessStatus::Unknown,
        },
    }
}

fn get_installation_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf()
}

pub fn create_askpass_script(
    username: &str,
    token: &str,
) -> Result<PathBuf, Box<dyn error::Error>> {
    let install_path = get_installation_path();
    let askpass_path = install_path.join("askpass.sh");

    let script = format!(
        r#"#!/bin/bash
if [[ "$1" == *"Username"* ]]; then
    echo "{}"
elif [[ "$1" == *"Password"* ]]; then
    echo "{}"
fi
"#,
        username, token
    );

    let mut file = fs::File::create(&askpass_path)?;
    file.write_all(script.as_bytes())?;
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&askpass_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&askpass_path, permissions)?;
    }
    println!("askPass.sh created as: {:?}", askpass_path);
    Ok(askpass_path)
}
