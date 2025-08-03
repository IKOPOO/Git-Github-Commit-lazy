use crate::menu;
pub mod lanjut;
pub mod project_baru;

#[derive(Debug, Clone)]
pub enum Menu {
  ProjectBaru,
  Lanjut,
}

impl Menu {
  pub fn pilih_menu(input: &str) -> Result<Self, &'static str> {
    match input.trim() {
      "1" => Ok(Menu::ProjectBaru),
      "2" => Ok(Menu::Lanjut),
      text if text.eq_ignore_ascii_case("project baru") => Ok(Menu::ProjectBaru),
      text if text.eq_ignore_ascii_case("lanjut project") => Ok(Menu::Lanjut),
      _ => Err("input tidak valid. Pilih menu nya cok blok ndlogok jaran"),
    }
  }

  pub fn run(&self) {
    match self {
      Menu::ProjectBaru => menu::project_baru::handle_new_project(),
      Menu::Lanjut => menu::lanjut::handle_continue_project(),
    }
  }
}
