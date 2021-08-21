use std::process::Command;
// TODO: Refactor -> adb function

// Mettres Package dans une table de Hashage


pub fn list_phone_packages() -> String {
    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("list")
                .arg("packages")
                .output()
                .expect("adb command failed to start");

/*    let output = Command::new("ls")
                .output()
                .expect("ls command failed to start");
*/
    String::from_utf8(output.stdout).unwrap().replace("package:", "")
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

pub fn get_phone_brand() -> String {
    let output = Command::new("adb")
            .arg("shell")
            .arg("getprop")
            .arg("ro.product.brand")
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