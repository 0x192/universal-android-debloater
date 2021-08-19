use std::process::Command;
use crate::core::uad_lists::UadLists;
use crate::core::uad_lists::load_debloat_lists;



pub fn list_phone_packages() -> String {
/*    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("list")
                .arg("packages")
                .output()
                .expect("adb command failed to start");*/
    load_debloat_lists(UadLists::Aosp);

    let output = Command::new("ls")
                .output()
                .expect("ls command failed to start");
    return String::from_utf8(output.stdout).unwrap().replace("package:", "");
}


pub async fn uninstall_package(package: String) -> String {
    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("uninstall")
                .arg("--user")
                .arg("0")
                .arg(package)
                .output()
                .expect("adb command failed");

    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        println!("[DEBUG] {:?}", error);
        error
    } else {
        let adb_return = String::from_utf8(output.stdout).unwrap();
        println!("[DEBUG] - Uninstall: {:?}", adb_return);
        adb_return
    }
}