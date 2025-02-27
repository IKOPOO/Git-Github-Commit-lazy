use crate::core::{self, new, process};
use std::thread;

pub fn handle_new_project() {
    // user input data dan membuat project
    let project = match new::create_project() {
        Some(p) => p,   // jika berhasil maka data disimpan
        None => return, // jika gagal maka proses di berhentikan
    };
    // mulai memantau project tersebut
    let project_baru = process::ProcessMonitor::add_project(&self, project);
}
