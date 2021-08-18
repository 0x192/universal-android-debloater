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


