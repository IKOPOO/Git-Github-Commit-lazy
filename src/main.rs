mod app;
mod config;
mod core;
mod menu;

// use crate::menu::cmd_command;

fn main() {
    let mut aplication = app::App::new();
    aplication.run();
}
