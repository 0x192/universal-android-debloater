#![windows_subsystem = "windows"]
#[macro_use]
extern crate log;

use crate::core::utils::setup_uad_dir;
use fern::{
    colors::{Color, ColoredLevelConfig},
    FormatCallback,
};
use log::Record;
use static_init::dynamic;
use std::path::PathBuf;
use std::{fmt::Arguments, fs::OpenOptions};

mod core;
mod gui;

#[dynamic]
static CONFIG_DIR: PathBuf = setup_uad_dir(dirs::config_dir());

#[dynamic]
static CACHE_DIR: PathBuf = setup_uad_dir(dirs::cache_dir());

fn main() -> iced::Result {
    setup_logger().expect("setup logging");
    gui::UadGui::start()
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
                record.line().map(|l| l.to_string()).unwrap_or_default(),
                message
            ));
        }
    };

    let default_log_level = log::LevelFilter::Warn;
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .truncate(false)
        .open(CACHE_DIR.join(format!("UAD_{}.log", chrono::Local::now().format("%Y%m%d"))))?;

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
