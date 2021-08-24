use std::process::Command;
use std::collections::HashSet;

pub fn adb_shell_command(args: &str) -> String {

    #[cfg(target_os = "windows")]
        let output = Command::new("./ressources/windows/adb.exe")
            .args(&["/C", "adb shell", args])
            .output()
            .expect("adb command failed to start");

    #[cfg(target_os = "macos")]
        let output = Command::new("./ressources/macos/adb")
            .args(&["shell", args])
            .output()
            .expect("adb command failed to start");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let output = Command::new("adb")
            .args(&["shell", args])
            .output()
            .expect("adb command failed to start");

    if !output.status.success() {
        String::from_utf8(output.stderr).unwrap()
    } else {
        String::from_utf8(output.stdout).unwrap()
    }
}


pub fn list_all_system_packages() -> String {
    adb_shell_command("pm list packages -s -u").replace("package:", "")
}

pub fn hashset_installed_system_packages() -> HashSet<String> {

    let hashet: HashSet<String> = adb_shell_command("pm list packages -s")
        .replace("package:", "")
        .lines()
        .map(String::from)
        .collect();

    hashet
}


pub fn uninstall_package(package: String) -> String {
    let arg = "pm uninstall --user 0 ".to_string() + &package;

    adb_shell_command(&arg)
}


pub fn restore_package(package: String) -> String {
    let arg = "cmd package install-existing --user 0 ".to_string() + &package;

    adb_shell_command(&arg)
}

pub fn get_phone_model() -> String {
    adb_shell_command("getprop ro.product.model")
}


pub fn get_phone_brand() -> String {
    format!("{} {}", adb_shell_command("getprop ro.product.brand").trim(), get_phone_model())
}