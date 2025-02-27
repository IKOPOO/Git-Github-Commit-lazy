use lazy_static::lazy_static;
use std::error::Error;
use std::os::unix::raw::pid_t;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock, RwLock};
use std::time::Duration;
use std::{thread, vec};

// crate yang digunakan
use super::{cmd_command, watch_proces};
use crate::config::Project;
use crate::core;

pub enum MonitorEvent {
    AddProject(Project),
    RemoveProject(usize),
    Shutdown,
}

pub(crate) struct ProcessMonitor {
    pub project: Arc<RwLock<Vec<Project>>>,
    pub pids: Arc<RwLock<Vec<Vec<u32>>>>,
    sender: Sender<MonitorEvent>,
}

impl ProcessMonitor {
    fn new() -> Self {
        let (tx, rx): (Sender<MonitorEvent>, Receiver<MonitorEvent>) = mpsc::channel();
        let project = Arc::new(RwLock::new(Vec::new()));
        let pid = Arc::new(RwLock::new(Vec::new()));

        let monitor = Arc::new(Self {
            project: project.clone(),
            pids: pid.clone(),
            sender: tx,
        });

        let monitor_clone = monitor.clone();
        thread::spawn(move || ProcessMonitor::monitor_all(rx, monitor_clone));

        monitor
    }

    // fungsi menambahkan data project baru dan data PID baru
    fn add_project(project: Project, projects: &mut Vec<Project>, pids: &mut Vec<Vec<u32>>) {
        projects.push(project);

        if let Some(p) = projects.last().map(|p| p.path.clone()) {
            let pids_lock = watch_proces::find_pid_by_path(p);
            pids.push(pids_lock);
        }

        // sekedar validasi untuk melihat apakah data berhasi disimpan atau tidak
        println!("daftar proyek saat ini ");
        for p in projects.iter() {
            println!("->{}", p.name);
        }
        println!("daftar pid saat ini");
        for p in pids.iter() {
            println!("->{:?}", p);
        }
    }

    fn delete_and_push(index: usize, monitor: &Arc<Self>) {
        let mut projects = monitor.project.write().unwrap();
        let mut pids = monitor.pids.write().unwrap();

        if index < projects.len() {
            // menghapus project dan mengembalikan datanya untuk di push
            let proj = projects.remove(index);
            match cmd_command::push(&proj) {
                Ok(_) => println!("Push ke Github untuk project {} selesai", proj.name),
                Err(e) => eprintln!("Terjadi kesalahan dalam PUSH ke Github: {:?}", e),
            }
        }

        // hapus PID setelah push
        if index < pids.len() {
            pids.remove(index);
        }
    }

    pub fn handle_add_project(&self, project: Project) {
        if let Err(e) = self.sender.send(MonitorEvent::AddProject(project)) {
            eprintln!("Gagal mengirim event RemoveProject: {:?}", e);
        }
    }

    pub fn handle_remove_project(&self, index: usize) {
        if let Err(e) = self.sender.send(MonitorEvent::RemoveProject(index)) {
            eprintln!("Gagal menghapus data: {:?}", e);
        }
    }

    pub fn shutdown(&self) {
        if let Err(e) = self.sender.send(MonitorEvent::Shutdown) {
            eprintln!("error mematikan kau {:?}", e);
        }
    }

    // fungsi untuk memantau semua group PID
    pub fn monitor_all(rx: Receiver<MonitorEvent>, monitor: &Arc<Self>) {
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    MonitorEvent::AddProject(project) => {
                        Self::handle_add_project(project, &monitor);
                    }
                },
            }
        }
    }
}
