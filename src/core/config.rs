use crate::core::sync::get_android_sdk;
use crate::gui::views::settings::Settings;
use crate::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::fs;
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralSettings,
    pub devices: Vec<DeviceSettings>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    pub theme: String,
    pub expert_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceSettings {
    pub device_id: String,
    pub disable_mode: bool,
    pub multi_user_mode: bool,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            device_id: "".to_string(),
            multi_user_mode: get_android_sdk() > 21,
            disable_mode: false,
        }
    }
}

#[dynamic]
static CONFIG_FILE: PathBuf = CONFIG_DIR.join("config.toml");

impl Config {
    pub fn save_changes(settings: &Settings, device_id: &String) {
        let mut config = Self::load_configuration_file();
        match config
            .devices
            .iter_mut()
            .find(|x| x.device_id == *device_id)
        {
            Some(device) => {
                *device = settings.device.clone();
                config.general = settings.general.clone();
            }
            None => {
                debug!("config: New device settings saved");
                config.devices.push(settings.device.clone());
                config.general = settings.general.clone();
            }
        }
        let toml = toml::to_string(&config).unwrap();
        fs::write(&*CONFIG_FILE, toml).expect("Could not write config file to disk!");
    }

    pub fn load_configuration_file() -> Self {
        match fs::read_to_string(&*CONFIG_FILE) {
            Ok(s) => match toml::from_str(&s) {
                Ok(config) => config,
                Err(e) => {
                    error!("Invalid config file: `{}`", e);
                    Config::default()
                }
            },
            Err(_) => {
                let default_conf = toml::to_string(&Config::default()).unwrap();
                fs::write(&*CONFIG_FILE, default_conf)
                    .expect("Could not write config file to disk!");
                Config::default()
            }
        }
    }
}
