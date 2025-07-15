use lazy_static::lazy_static;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// crate yang digunakan
use super::{cmd_command, watch_proces};
use crate::config::Project;

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
    // constructor
    fn new() -> Arc<Self> {
        let (tx, rx): (Sender<MonitorEvent>, Receiver<MonitorEvent>) = mpsc::channel();
        let project = Arc::new(RwLock::new(Vec::new()));
        let pid = Arc::new(RwLock::new(Vec::new()));

        let monitor = Arc::new(Self {
            project: project.clone(),
            pids: pid.clone(),
            sender: tx.clone(),
        });

        let monitor_clone = Arc::clone(&monitor);
        thread::spawn(move || {
            ProcessMonitor::monitor_all(rx, monitor_clone);
        });

        monitor
    }

    // fungsi menambahkan data project baru dan data PID baru
    fn add_project(&self, project: Project) {
        let mut projects = self.project.write().unwrap();
        let mut pids = self.pids.write().unwrap();

        projects.push(project);

        if let Some(p) = projects.last().map(|p| p.path.clone()) {
            let pids_lock = watch_proces::find_pid_by_path(p);
            pids.push(pids_lock);
        };
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

    fn delete_and_push(&self, index: usize) {
        let mut projects = self.project.write().unwrap();
        let mut pids = self.pids.write().unwrap();

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
    fn monitor_all(rx: Receiver<MonitorEvent>, monitor: Arc<Self>) {
        println!("🟢 monitoring thread dimulai");
        loop {
            while let Ok(event) = rx.try_recv() {
                match event {
                    MonitorEvent::AddProject(project) => {
                        monitor.add_project(project);
                    }
                    MonitorEvent::RemoveProject(index) => {
                        monitor.delete_and_push(index);
                    }
                    MonitorEvent::Shutdown => {
                        println!("Shutting down monitoring...");
                        return;
                    }
                }
            }
            {
                let pids = monitor.pids.read().unwrap();
                let projects = monitor.project.read().unwrap();

                let mut index_to_remove = vec![];

                for (i, pid_group) in pids.iter().enumerate() {
                    match watch_proces::monitor_check(i, pid_group) {
                        Ok(true) => {
                            // do nothing because process is still running
                        }
                        Ok(false) => {
                            // hapus pid dan project yang tidak berjalan lagi
                            println!("❌ Proses project index {i} mati. Akan dihapus & push.");
                            index_to_remove.push(i);
                        }
                        Err(e) => {
                            eprintln!("⚠️ Gagal memantau proses di index {i}: {:?}", e);
                        }
                    }
                }

                drop(projects);
                drop(pids);

                for &i in index_to_remove.iter().rev() {
                    monitor.delete_and_push(i);
                }
            }

            thread::sleep(Duration::from_secs(5));
        }
    }
}

// yang di panggil itu kode yang ini
lazy_static! {
    pub(crate) static ref MONITOR_PROCES: Arc<ProcessMonitor> = ProcessMonitor::new();
}

// pub fn start_monitoring() {
//     let monitor = MONITOR_PROCES.clone();
//     thread::spawn(move || {
//         let rx = monitor.sender.clone();
//         ProcessMonitor::monitor_all(rx, monitor);
//     });
// }
pub fn add_project_to_monitor(project: Project) {
    MONITOR_PROCES.handle_add_project(project);
}

pub fn remove_project_from_monitoring(index: usize) {
    MONITOR_PROCES.handle_remove_project(index);
}
