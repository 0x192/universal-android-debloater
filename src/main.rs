#![windows_subsystem = "windows"]

mod core;
mod gui;

fn main() {
    gui::UadGui::start();
}