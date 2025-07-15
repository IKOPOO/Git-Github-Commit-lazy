use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::config::ProcessStatus;
use crate::core::utils;
pub fn find_pid_by_path(target_path: String) -> Vec<u32> {
    let path = target_path.trim();

    println!("path : {}", path);
    let mut attempt = false;
    loop {
        let output = Command::new("lsof").arg("+D").arg(path).output();
        let mut pids = Vec::new();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 && parts[0].contains("code") {
                    if let Ok(pid) = parts[1].parse::<u32>() {
                        pids.push(pid);
                    }
                }
            }

            if !pids.is_empty() {
                return pids;
            } else {
                eprintln!("❌ pid tidak di temukan");
            }
            attempt = true;
        } else {
            println!("⚠️ error saat menjalankan command LSOF");
        }
        if attempt == true {
            println!("proses sudah ditemukan")
        }

        thread::sleep(Duration::from_secs(10));
    }
}

pub fn monitor_check(group_id: usize, pids: &Vec<u32>) -> Result<bool, Box<dyn Error>> {
    for &pid in pids {
        let current_status = utils::check_process(pid as i32);

        match current_status {
            ProcessStatus::Running => {
                println!("Pocess {pid} sedang berjalan ");
            }
            ProcessStatus::NotRunning => {
                println!("process tidak lagi berjalan, {pid},{group_id} ");
                return Ok(false);
            }
            ProcessStatus::NoPermission => {
                println!("tidak ada permisi untuk mengecek proses {}", pid);
                return Err("permission denied".into());
            }
            ProcessStatus::Unknown => {
                println!("error tidak diketahui proses {}", pid);
                return Err("Unknown error".into());
            }
        }
    }
    Ok(true)
}
