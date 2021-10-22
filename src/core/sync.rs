use crate::core::uad_lists::PackageState;
use crate::gui::views::settings::Settings;
use crate::gui::widgets::package_row::PackageRow;
use regex::Regex;
use static_init::dynamic;
use std::collections::HashSet;
use std::env;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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
            adb_id: "".to_string(),
        }
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.model.to_string(),)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct User {
    pub id: u16,
    pub index: usize,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("user {}", self.id),)
    }
}

pub fn adb_shell_command(shell: bool, args: &str) -> Result<String, String> {
    let adb_command = match shell {
        true => vec!["shell", args],
        false => vec![args],
    };

    #[cfg(target_os = "windows")]
    let output = Command::new("adb")
        .args(adb_command)
        .creation_flags(0x08000000) // do not open a cmd window
        .output();

    #[cfg(target_os = "macos")]
    let output = Command::new("adb").args(adb_command).output();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let output = Command::new("adb").args(adb_command).output();

    match output {
        Err(e) => {
            error!("ADB: {}", e);
            Err("ADB was not found".to_string())
        }
        Ok(o) => {
            if !o.status.success() {
                let stdout = String::from_utf8(o.stdout).unwrap().trim_end().to_string();
                let stderr = String::from_utf8(o.stderr).unwrap().trim_end().to_string();

                // ADB does really weird things. Some errors are not redirected to stderr
                let err = if stdout.is_empty() { stderr } else { stdout };
                Err(err)
            } else {
                Ok(String::from_utf8(o.stdout).unwrap().trim_end().to_string())
            }
        }
    }
}

pub fn list_all_system_packages(user_id: &Option<&User>) -> String {
    let action = match user_id {
        Some(user_id) => format!("pm list packages -s -u --user {}", user_id.id),
        None => "pm list packages -s -u".to_string(),
    };

    adb_shell_command(true, &action)
        .unwrap_or_else(|_| "".to_string())
        .replace("package:", "")
}

pub fn hashset_system_packages(state: PackageState, user_id: &Option<&User>) -> HashSet<String> {
    let user = match user_id {
        Some(user_id) => format!(" --user {}", user_id.id),
        None => "".to_string(),
    };

    let action = match state {
        PackageState::Enabled => format!("pm list packages -s -e{}", user),
        PackageState::Disabled => format!("pm list package -s -d{}", user),
        _ => "".to_string(), // You probably don't need to use this function for anything else
    };

    adb_shell_command(true, &action)
        .unwrap_or_default()
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect()
}

pub fn action_handler(
    selected_user: &User,
    package: &PackageRow,
    phone: &Phone,
    settings: &Settings,
) -> Result<bool, bool> {
    let actions: Vec<String> = match package.state {
        PackageState::Enabled => match settings.disable_mode {
            true => {
                if phone.android_sdk < 21 {
                    vec![
                        format!("am force-stop {}", package.name),
                        format!("pm disable-user {}", package.name),
                        format!("pm clear {}", package.name),
                    ]
                } else if settings.multi_user_mode {
                    phone
                        .user_list
                        .iter()
                        .flat_map(|u| {
                            [
                                format!("am force-stop --user {} {}", u.id, package.name),
                                format!("pm disable-user --user {} {}", u.id, package.name),
                                format!("pm clear --user {} {}", u.id, package.name),
                            ]
                        })
                        .collect()
                } else {
                    vec![
                        format!("am force-stop --user {} {}", selected_user.id, package.name),
                        format!(
                            "pm disable-user --user {} {}",
                            selected_user.id, package.name
                        ),
                        format!("pm clear --user {} {}", selected_user.id, package.name),
                    ]
                }
            }
            false => {
                if phone.android_sdk < 21 {
                    vec![format!("pm uninstall {}", package.name)]
                } else if settings.multi_user_mode {
                    phone
                        .user_list
                        .iter()
                        .map(|u| format!("pm uninstall --user {} {}", u.id, package.name))
                        .collect()
                } else {
                    vec![format!(
                        "pm uninstall --user {} {}",
                        selected_user.id, package.name
                    )]
                }
            }
        },
        PackageState::Uninstalled => {
            if phone.android_sdk < 21 {
                Vec::new() // action already prevented by the GUI
            } else if settings.multi_user_mode {
                phone
                    .user_list
                    .iter()
                    .map(|u| {
                        format!(
                            "cmd package install-existing --user {} {}",
                            u.id, package.name
                        )
                    })
                    .collect()
            } else {
                vec![format!(
                    "cmd package install-existing --user {} {}",
                    selected_user.id, package.name
                )]
            }
        }
        PackageState::Disabled => {
            if phone.android_sdk < 21 {
                vec![format!("pm enable {}", package.name)]
            } else if settings.multi_user_mode {
                phone
                    .user_list
                    .iter()
                    .map(|u| format!("pm enable --user {} {}", u.id, package.name))
                    .collect()
            } else {
                vec![format!(
                    "pm enable --user {} {}",
                    selected_user.id, package.name
                )]
            }
        }
        PackageState::All => vec![], // This can't happen (like... never)
    };

    for action in actions {
        match adb_shell_command(true, &action) {
            Ok(_) => {
                info!("[{}] {}", package.removal, action);
            }
            Err(err) => {
                if err.contains("[not installed for") {
                } else {
                    error!("[{}] {} -> {}", package.removal, action, err);
                    return Err(false);
                }
            }
        }
    }
    Ok(true)
}

pub fn get_phone_model() -> String {
    match adb_shell_command(true, "getprop ro.product.model") {
        Ok(model) => model,
        Err(err) => {
            println!("ERROR: {}", err);
            if err.contains("adb: no devices/emulators found") {
                "no devices/emulators found".to_string()
            } else {
                err
            }
        }
    }
}

pub fn get_android_sdk() -> u8 {
    match adb_shell_command(true, "getprop ro.build.version.sdk") {
        Ok(sdk) => sdk.parse().unwrap(),
        Err(_) => 0,
    }
}

pub fn get_phone_brand() -> String {
    format!(
        "{} {}",
        adb_shell_command(true, "getprop ro.product.brand")
            .unwrap_or_else(|_| "".to_string())
            .trim(),
        get_phone_model()
    )
}

pub fn get_user_list() -> Vec<User> {
    #[dynamic]
    static RE: Regex = Regex::new(r"\{([0-9]+)").unwrap();
    match adb_shell_command(true, "pm list users") {
        Ok(users) => RE
            .find_iter(&users)
            .enumerate()
            .map(|(i, u)| User {
                id: u.as_str()[1..].parse().unwrap(),
                index: i,
            })
            .collect(),
        Err(_) => vec![],
    }
}

pub fn get_device_list() -> Vec<Phone> {
    #[dynamic]
    static RE: Regex = Regex::new(r"\n([[:alnum:]]+)\s+device").unwrap();

    match adb_shell_command(false, "devices") {
        Ok(devices) => {
            let mut device_list: Vec<Phone> = vec![];
            for device in RE.captures_iter(&devices) {
                env::set_var("ANDROID_SERIAL", device[1].to_string());

                device_list.push(Phone {
                    model: get_phone_brand(),
                    android_sdk: get_android_sdk(),
                    user_list: get_user_list(),
                    adb_id: device[1].to_string(),
                });
            }
            device_list
        }

        Err(err) => {
            warn!("get_device_list() -> {}", err);
            vec![]
        }
    }
}
