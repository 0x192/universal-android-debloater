use std::process::Command;
use std::collections::HashSet;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;


pub fn adb_shell_command(args: &str) -> Result<String,String> {

    #[cfg(target_os = "windows")]
        let output = Command::new("adb")
            .args(&["shell", args])
            .creation_flags(0x08000000) // do not open a cmd window
            .output()
            .expect("adb command failed to start. Do you have ADB installed?");

    #[cfg(target_os = "macos")]
        let output = Command::new("adb")
            .args(&["shell", args])
            .output()
            .expect("adb command failed to start. Do you have ADB installed?");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let output = Command::new("adb")
            .args(&["shell", args])
            .output()
            .expect("adb command failed to start. Do you have ADB installed?");

    if !output.status.success() {
        Err(String::from_utf8(output.stderr).unwrap())
    } else {
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}


pub fn list_all_system_packages() -> String {
    adb_shell_command("pm list packages -s -u")
        .unwrap_or("".to_string())
        .replace("package:", "")
        
}

pub fn hashset_installed_system_packages() -> HashSet<String> {
    let hashet: HashSet<String> = adb_shell_command("pm list packages -s")
        .unwrap_or("".to_string())
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect();

    hashet
}


pub fn uninstall_package(package: String) -> String {
    let arg = "pm uninstall --user 0 ".to_string() + &package;

    adb_shell_command(&arg).unwrap()
}


pub fn restore_package(package: String) -> String {
    let arg = "cmd package install-existing --user 0 ".to_string() + &package;

    adb_shell_command(&arg).unwrap()
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


pub fn get_phone_brand() -> String {
    format!("{} {}", adb_shell_command("getprop ro.product.brand").unwrap_or("".to_string()).trim(), get_phone_model())
}