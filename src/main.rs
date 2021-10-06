#![windows_subsystem = "windows"]
#![feature(drain_filter)]
#![feature(const_option)]
#[macro_use] extern crate log;

use std::{fs::OpenOptions, fmt::Arguments};
use fern::{FormatCallback, colors::{Color, ColoredLevelConfig}};
use log::Record;

mod core;
mod gui;

fn main() {
    setup_logger().expect("setup logging");
    gui::UadGui::start();
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    
    let colors = ColoredLevelConfig::new().info(Color::Green);

    let make_formatter = |use_colors: bool| {
        move |out: FormatCallback, message: &Arguments, record: &Record| {
            out.finish(format_args!(
                "{} {} [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                if use_colors {
                    format!("{:5}", colors.color(record.level()))
                } else {
                    format!("{:5}", record.level().to_string())
                },
                record.file().unwrap_or("?"),
                record
                    .line()
                    .map(|l| l.to_string())
                    .unwrap_or_default(),
                message
            ))
        }
    };

    let default_log_level = log::LevelFilter::Warn;
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .truncate(true)
        .open("uad.log")?;

    let file_dispatcher = fern::Dispatch::new()
        .format(make_formatter(false))
        .level(default_log_level)
        .level_for("uad_gui", log::LevelFilter::Debug)
        .chain(log_file);

    let stdout_dispatcher = fern::Dispatch::new()
        .format(make_formatter(true))
        .level(default_log_level)
        .level_for("uad_gui", log::LevelFilter::Warn)
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;

    Ok(())
}