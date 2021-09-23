use std::process::Command;
use std::collections::HashSet;
use crate::core::uad_lists::{PackageState, Opposite};
use crate::gui::widgets::package_row::PackageRow;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;


#[derive(Debug)]
pub struct Phone {
    pub model: String,
    pub android_sdk: u8,
}

impl Default for Phone {
    fn default() -> Self {
        Self {
            model: get_phone_brand(),
            android_sdk: get_android_sdk(),
        }
    }
}
pub fn adb_shell_command(args: &str) -> Result<String,String> {

    #[cfg(target_os = "windows")]
        let output = Command::new("adb")
            .args(&["shell", args])
            .creation_flags(0x08000000) // do not open a cmd window
            .output();

    #[cfg(target_os = "macos")]
        let output = Command::new("adb")
            .args(&["shell", args])
            .output();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let output = Command::new("adb")
            .args(&["shell", args])
            .output();

    match output {
        Err(e) => {
            error!("ADB: {}", e);
            Err("ADB was not found".to_string())
        },
        Ok(o) => {
            if !o.status.success() {
                let stderr = String::from_utf8(o.stderr).unwrap().trim_end().to_string();
                error!("ADB: {}", stderr);
                Err(stderr)
            } else {
                Ok(String::from_utf8(o.stdout).unwrap().trim_end().to_string())
            }
        }
    } 
    
}


pub fn list_all_system_packages() -> String {
    adb_shell_command("pm list packages -s -u")
        .unwrap_or("".to_string())
        .replace("package:", "")
        
}

pub fn hashset_system_packages(state: PackageState) -> HashSet<String> {
    let action = match state {
        PackageState::Enabled => "pm list packages -s -e",
        PackageState::Disabled => "pm list package -s -d",
        _ => "", // You probably don't need to use this function for anything else
    };

    adb_shell_command(action)
        .unwrap_or(String::new())
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect()
}

pub fn action_handler(package: &PackageRow, phone: &Phone, disable_mode: bool) -> Result<bool, bool> {
    let user = if phone.android_sdk < 21 { "" } else { " --user 0"};

    let actions: Vec<String> = match package.state {
        PackageState::Enabled => {
            match disable_mode {
                true => {
                    vec![
                        format!("am force-stop {}", package.name),
                        format!("pm disable-user {}", package.name),
                        format!("pm clear {}", package.name)
                    ]
                }
                false => vec![format!("pm uninstall{} {}", user, package.name)]
            }
        }
        PackageState::Disabled => vec![format!("pm enable {}", package.name)],
        PackageState::Uninstalled => vec![format!("cmd package install-existing {} {}", user, package.name)],
        PackageState::All => vec![], // This can't happen (like... never)
    };

    for action in actions {
        match adb_shell_command(&action) {
            Ok(_) => {}
            Err(_) => {
                error!("{} [{}]: {}", package.state.opposite(disable_mode), package.removal, package.name);
                return Err(false);
            }
        }
    }
    info!("{} [{}]: {}", package.state.opposite(disable_mode), package.removal, package.name);
    Ok(true)
}

pub fn get_phone_model() -> String {
    match adb_shell_command("getprop ro.product.model") {
        Ok(model) => model,
        Err(err) => {
            if err.contains("adb: no devices/emulators found") {
                "adb: no devices/emulators found".to_string()
            } else {
                err
            }
        }
            
    }
}

pub fn get_android_sdk() -> u8 {
    match adb_shell_command("getprop ro.build.version.sdk") {
        Ok(sdk) => sdk.parse().unwrap(),
        Err(_) => 0, 
    }
}


pub fn get_phone_brand() -> String {
    format!("{} {}", adb_shell_command("getprop ro.product.brand").unwrap_or("".to_string()).trim(), get_phone_model())
}