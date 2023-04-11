use crate::core::uad_lists::PackageState;
use crate::gui::views::list::PackageInfo;
use crate::gui::widgets::package_row::PackageRow;
use regex::Regex;
use retry::{delay::Fixed, retry, OperationResult};
use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::collections::HashSet;
use std::env;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[dynamic]
static RE: Regex = Regex::new(r"\n(\S+)\s+device").unwrap();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phone {
    pub model: String,
    pub android_sdk: u8,
    pub user_list: Vec<User>,
    pub adb_id: String,
}

impl Default for Phone {
    fn default() -> Self {
        Self {
            model: "fetching devices...".to_string(),
            android_sdk: 0,
            user_list: vec![],
            adb_id: String::new(),
        }
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.model)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Copy)]
pub struct User {
    pub id: u16,
    pub index: usize,
    pub protected: bool,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user {}", self.id)
    }
}

pub fn adb_shell_command(shell: bool, args: &str) -> Result<String, String> {
    let adb_command = if shell {
        vec!["shell", args]
    } else {
        vec![args]
    };

    let mut command = Command::new("adb");
    command.args(adb_command);

    #[cfg(target_os = "windows")]
    let command = command.creation_flags(0x08000000); // do not open a cmd window

    match command.output() {
        Err(e) => {
            error!("ADB: {}", e);
            Err("ADB was not found".to_string())
        }
        Ok(o) => {
            if o.status.success() {
                Ok(String::from_utf8(o.stdout)
                    .map_err(|e| e.to_string())?
                    .trim_end()
                    .to_string())
            } else {
                let stdout = String::from_utf8(o.stdout)
                    .map_err(|e| e.to_string())?
                    .trim_end()
                    .to_string();
                let stderr = String::from_utf8(o.stderr)
                    .map_err(|e| e.to_string())?
                    .trim_end()
                    .to_string();

                // ADB does really weird things. Some errors are not redirected to stderr
                let err = if stdout.is_empty() { stderr } else { stdout };
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandType {
    PackageManager(PackageInfo),
    Shell,
}
pub async fn perform_adb_commands(
    action: String,
    command_type: CommandType,
) -> Result<CommandType, ()> {
    let label = match command_type {
        CommandType::PackageManager(ref p) => p.removal.to_string(),
        CommandType::Shell => "Shell".to_string(),
    };

    match adb_shell_command(true, &action) {
        Ok(o) => {
            // On old devices, adb commands can return the '0' exit code even if there
            // is an error. On Android 4.4, ADB doesn't check if the package exists.
            // It does not return any error if you try to `pm block` a non-existent package.
            // Some commands are even killed by ADB before finishing and UAD can't catch
            // the output.
            if ["Error", "Failure"].iter().any(|&e| o.contains(e)) {
                error!("[{}] {} -> {}", label, action, o);
                Err(())
            } else {
                info!("[{}] {} -> {}", label, action, o);
                Ok(command_type)
            }
        }
        Err(err) => {
            if !err.contains("[not installed for") {
                error!("[{}] {} -> {}", label, action, err);
            }
            Err(())
        }
    }
}

#[allow(clippy::option_if_let_else)]
pub fn user_flag(user_id: Option<&User>) -> String {
    match user_id {
        Some(user_id) => format!(" --user {}", user_id.id),
        None => "".to_string(),
    }
}

pub fn list_all_system_packages(user_id: Option<&User>) -> String {
    let action = format!("pm list packages -s -u{}", user_flag(user_id));

    adb_shell_command(true, &action)
        .unwrap_or_else(|_| String::new())
        .replace("package:", "")
}

pub fn hashset_system_packages(state: PackageState, user_id: Option<&User>) -> HashSet<String> {
    let user = user_flag(user_id);
    let action = match state {
        PackageState::Enabled => format!("pm list packages -s -e{user}"),
        PackageState::Disabled => format!("pm list package -s -d{user}"),
        _ => String::new(), // You probably don't need to use this function for anything else
    };

    adb_shell_command(true, &action)
        .unwrap_or_default()
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect()
}

// Minimum information for processing adb commands
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CorePackage {
    pub name: String,
    pub state: PackageState,
}

impl From<&mut PackageRow> for CorePackage {
    fn from(pr: &mut PackageRow) -> Self {
        Self {
            name: pr.name.clone(),
            state: pr.state,
        }
    }
}
impl From<PackageRow> for CorePackage {
    fn from(pr: PackageRow) -> Self {
        Self {
            name: pr.name.clone(),
            state: pr.state,
        }
    }
}

impl From<&PackageRow> for CorePackage {
    fn from(pr: &PackageRow) -> Self {
        Self {
            name: pr.name.clone(),
            state: pr.state,
        }
    }
}

pub fn apply_pkg_state_commands(
    package: &CorePackage,
    wanted_state: PackageState,
    selected_user: &User,
    phone: &Phone,
) -> Vec<String> {
    // https://github.com/0x192/universal-android-debloater/wiki/ADB-reference
    // ALWAYS PUT THE COMMAND THAT CHANGES THE PACKAGE STATE FIRST!
    let commands = match wanted_state {
        PackageState::Enabled => {
            match package.state {
                PackageState::Disabled => match phone.android_sdk {
                    i if i >= 23 => vec!["pm enable"],
                    _ => vec!["pm enable"],
                },
                PackageState::Uninstalled => match phone.android_sdk {
                    i if i >= 23 => vec!["cmd package install-existing"],
                    21 | 22 => vec!["pm unhide"],
                    19 | 20 => vec!["pm unblock", "pm clear"],
                    _ => vec![], // Impossible action already prevented by the GUI
                },
                _ => vec![],
            }
        }
        PackageState::Disabled => match package.state {
            PackageState::Uninstalled | PackageState::Enabled => match phone.android_sdk {
                sdk if sdk >= 23 => vec!["pm disable-user", "am force-stop", "pm clear"],
                _ => vec![],
            },
            _ => vec![],
        },
        PackageState::Uninstalled => match package.state {
            PackageState::Enabled | PackageState::Disabled => match phone.android_sdk {
                sdk if sdk >= 23 => vec!["pm uninstall"], // > Android Marshmallow (6.0)
                21 | 22 => vec!["pm hide", "pm clear"],   // Android Lollipop (5.x)
                19 | 20 => vec!["pm block", "pm clear"],  // Android KitKat (4.4/4.4W)
                _ => vec!["pm uninstall"], // Disable mode is unavailable on older devices because the specific ADB commands need root
            },
            _ => vec![],
        },
        PackageState::All => vec![],
    };
    if phone.android_sdk < 21 {
        request_builder(&commands, &package.name, None)
    } else {
        request_builder(&commands, &package.name, Some(selected_user))
    }
}

pub fn request_builder(commands: &[&str], package: &str, user: Option<&User>) -> Vec<String> {
    #[allow(clippy::option_if_let_else)]
    match user {
        Some(u) => commands
            .iter()
            .map(|c| format!("{} --user {} {}", c, u.id, package))
            .collect(),
        None => commands.iter().map(|c| format!("{c} {package}")).collect(),
    }
}

pub fn get_phone_model() -> String {
    adb_shell_command(true, "getprop ro.product.model").unwrap_or_else(|err| {
        println!("ERROR: {err}");
        if err.contains("adb: no devices/emulators found") {
            "no devices/emulators found".to_string()
        } else {
            err
        }
    })
}

pub fn get_android_sdk() -> u8 {
    adb_shell_command(true, "getprop ro.build.version.sdk").map_or(0, |sdk| sdk.parse().unwrap())
}

pub fn get_phone_brand() -> String {
    format!(
        "{} {}",
        adb_shell_command(true, "getprop ro.product.brand")
            .map(|s| s.trim().to_string())
            .unwrap_or_default(),
        get_phone_model()
    )
}

pub fn is_protected_user(user_id: &str) -> bool {
    adb_shell_command(true, &format!("pm list packages --user {user_id}")).is_err()
}

pub fn get_user_list() -> Vec<User> {
    #[dynamic]
    static RE: Regex = Regex::new(r"\{([0-9]+)").unwrap();
    adb_shell_command(true, "pm list users")
        .map(|users| {
            RE.find_iter(&users)
                .enumerate()
                .map(|(i, u)| User {
                    id: u.as_str()[1..].parse().unwrap(),
                    index: i,
                    protected: is_protected_user(&u.as_str()[1..]),
                })
                .collect()
        })
        .unwrap_or_default()
}

// getprop ro.serialno
pub async fn get_devices_list() -> Vec<Phone> {
    retry(
        Fixed::from_millis(500).take(120),
        || match adb_shell_command(false, "devices") {
            Ok(devices) => {
                let mut device_list: Vec<Phone> = vec![];
                if !RE.is_match(&devices) {
                    return OperationResult::Retry(vec![]);
                }
                for device in RE.captures_iter(&devices) {
                    env::set_var("ANDROID_SERIAL", &device[1]);
                    device_list.push(Phone {
                        model: get_phone_brand(),
                        android_sdk: get_android_sdk(),
                        user_list: get_user_list(),
                        adb_id: device[1].to_string(),
                    });
                }
                OperationResult::Ok(device_list)
            }
            Err(err) => {
                error!("get_device_list() -> {}", err);
                let test: Vec<Phone> = vec![];
                OperationResult::Retry(test)
            }
        },
    )
    .unwrap_or_default()
}
