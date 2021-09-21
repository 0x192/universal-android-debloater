use std::process::Command;
use std::collections::HashSet;
use crate::core::uad_lists::Removal;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;


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

pub fn hashset_installed_system_packages() -> HashSet<String> {
    adb_shell_command("pm list packages -s")
        .unwrap_or("".to_string())
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect()
}


pub fn uninstall_package(package: String, removal: Removal) -> Result<bool, bool> {
    let arg = "pm uninstall --user 0 ".to_string() + &package;
    let output = adb_shell_command(&arg).unwrap_or_else(|_| "Error".to_string());
    if output.contains("Success") {
        info!("REMOVE  [{}]: {}", removal, package);
        Ok(true)
    } else {
        error!("REMOVE [{}]: {}", removal, output);
        Err(false)
    }

}


pub fn restore_package(package: String, removal: Removal) -> Result<bool, bool> {
    let arg = "cmd package install-existing --user 0 ".to_string() + &package;
    let output = adb_shell_command(&arg).unwrap_or_else(|_| "Error".to_string());

    if output.contains("installed for user") {
        info!("RESTORE [{}]: {}", removal, package);
        Ok(true)
    } else {
        error!("RESTORE: [{}]: {}", removal, output);
        Err(false)
    }

}

pub fn get_phone_model() -> String {
    match adb_shell_command("getprop ro.product.model") {
        Ok(model) => {
            model
        },

        Err(err) => {
            if err.contains("adb: no devices/emulators found") {
                "adb: no devices/emulators found".to_string()
            } else {
                err
            }
        }
            
    }
}

pub fn get_phone_brand() -> String {
    format!("{} {}", adb_shell_command("getprop ro.product.brand").unwrap_or("".to_string()).trim(), get_phone_model())
}