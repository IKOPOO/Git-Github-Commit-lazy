use crate::menu::Menu;
use std::io;

pub struct App {
  current_menu: Option<Menu>,
}

impl App {
  pub fn new() -> Self {
    App { current_menu: None }
  }

  pub fn run(&mut self) {
    loop {
      println!("Pilih Menu: ");
      println!("1. Project Baru");
      println!("2. Lanjut Project");
      println!("0. Keluar");

      let mut input = String::new();
      io::stdin().read_line(&mut input).expect("cannot read line");

      if input.trim() == "0" {
        println!("Keluar Dari Program....");
        break;
      }

      match Menu::pilih_menu(&input) {
        Ok(menu) => {
          self.current_menu = Some(menu);
          if let Some(menu) = &self.current_menu {
            menu.run();
          }
        }
        Err(err) => eprintln!("{}", err),
      }

      println!("\n")
    }
  }
}
