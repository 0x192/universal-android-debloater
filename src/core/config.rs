use crate::core::sync::{get_android_sdk, User};
use crate::core::utils::DisplayablePath;
use crate::gui::views::settings::Settings;
use crate::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::fs;
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralSettings,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub devices: Vec<DeviceSettings>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    pub theme: String,
    pub expert_mode: bool,
}

#[derive(Default, Debug, Clone)]
pub struct BackupSettings {
    pub backups: Vec<DisplayablePath>,
    pub selected: Option<DisplayablePath>,
    pub users: Vec<User>,
    pub selected_user: Option<User>,
    pub backup_state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceSettings {
    pub device_id: String,
    pub disable_mode: bool,
    pub multi_user_mode: bool,
    #[serde(skip)]
    pub backup: BackupSettings,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            multi_user_mode: get_android_sdk() > 21,
            disable_mode: false,
            backup: BackupSettings::default(),
        }
    }
}

#[dynamic]
static CONFIG_FILE: PathBuf = CONFIG_DIR.join("config.toml");

impl Config {
    pub fn save_changes(settings: &Settings, device_id: &String) {
        let mut config = Self::load_configuration_file();
        if let Some(device) = config
            .devices
            .iter_mut()
            .find(|x| x.device_id == *device_id)
        {
            *device = settings.device.clone();
        } else {
            debug!("config: New device settings saved");
            config.devices.push(settings.device.clone());
        }
        config.general = settings.general.clone();
        let toml = toml::to_string(&config).unwrap();
        fs::write(&*CONFIG_FILE, toml).expect("Could not write config file to disk!");
    }

    pub fn load_configuration_file() -> Self {
        match fs::read_to_string(&*CONFIG_FILE) {
            Ok(s) => match toml::from_str(&s) {
                Ok(config) => return config,
                Err(e) => error!("Invalid config file: `{}`", e),
            },
            Err(e) => error!("Failed to read config file: `{}`", e),
        }
        error!("Restoring default config file");
        let toml = toml::to_string(&Self::default()).unwrap();
        fs::write(&*CONFIG_FILE, toml).expect("Could not write config file to disk!");
        Self::default()
    }
}
