#![windows_subsystem = "windows"]
#![feature(drain_filter)]

mod core;
mod gui;

fn main() {
    gui::UadGui::start();
}