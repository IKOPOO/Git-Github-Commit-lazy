use crate::core::{new, process::add_project_to_monitor};

pub fn handle_new_project() {
  // user input data dan membuat project
  let project = match new::create_project() {
    Some(p) => p,   // jika berhasil maka data disimpan
    None => return, // jika gagal maka proses di berhentikan
  };

  // kirim data ke process monitor
  add_project_to_monitor(project);
}
