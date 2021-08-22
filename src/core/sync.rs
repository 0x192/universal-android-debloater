use std::process::Command;
use std::collections::HashSet;

// TODO: Refactor -> adb function
pub fn list_all_system_packages() -> String {
    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("list")
                .arg("packages")
                .arg("-s")
                .arg("-u")
                .output()
                .expect("adb command failed to start");

    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        println!("[DEBUG] Error: {:?}", error);
        error
    } else {
        String::from_utf8(output.stdout).unwrap().replace("package:", "")
    }

}

pub fn hashset_installed_system_packages() -> HashSet<String> {
    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("list")
                .arg("packages")
                .arg("-s")
                .output()
                .expect("adb command failed to start");

        let hashset: HashSet<String> = String::from_utf8(output.stdout)
            .unwrap()
            .replace("package:", "")
            .lines()
            .map(String::from)
            .collect();

        return hashset;

    // let temp = "com.samsung.android.MtpApplication\ncom.samsung.android.provider.filterprovider\ncom.samsung.android.provider.shootingmodeprovider\ncom.samsung.android.provider.stickerprovider\ncom.samsung.android.SettingsReceiver\ncom.samsung.android.sm.policy\ncom.samsung.android.timezone.autoupdate_O\ncom.samsung.app.slowmotion\ncom.samsung.cmh\ncom.samsung.dcmservice\ncom.samsung.networkui\ncom.samsung.sec.android.application.csc\ncom.samsung.upsmtheme\ncom.sec.android.app.camera\ncom.sec.android.app.clockpackage\ncom.sec.android.app.launcher\ncom.sec.android.app.myfiles\ncom.sec.android.app.personalization\ncom.sec.android.app.popupcalculator\ncom.sec.android.app.simsettingmgr\ncom.sec.android.app.soundalive\ncom.sec.android.emergencylauncher\ncom.sec.android.emergencymode.service\ncom.sec.android.gallery3d\ncom.sec.android.gallery3d.panorama360view\ncom.sec.android.ofviewer\ncom.sec.android.provider.badge\ncom.sec.android.provider.emergencymode\ncom.sec.android.wallpapercropper2\ncom.sec.automation\ncom.sec.epdg\ncom.sec.ims\ncom.sec.imsservice";
    // return temp.to_string();
}


pub fn uninstall_package(package: String) -> String {
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
        println!("[DEBUG] Error: {:?}", error);
        error
    } else {
        let adb_return = String::from_utf8(output.stdout).unwrap();
        println!("[DEBUG] - Uninstall: {:?}", adb_return);
        adb_return
    }
}


pub fn restore_package(package: String) -> String {
    let output = Command::new("adb")
                .arg("shell")
                .arg("cmd")
                .arg("package")
                .arg("install-existing")
                .arg("--user")
                .arg("0")
                .arg(package)
                .output()
                .expect("adb command failed");

    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        println!("[DEBUG] Error: {:?}", error);
        error
    } else {
        let adb_return = String::from_utf8(output.stdout).unwrap();
        println!("[DEBUG] - Restore: {:?}", adb_return);
        adb_return
    }
}

pub fn get_phone_model() -> String {

    let output = Command::new("adb")
            .arg("shell")
            .arg("getprop")
            .arg("ro.product.model")
            .output()
            .expect("adb command failed");

    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        println!("[DEBUG] Error: {:?}", error);
        error
    } else {
        let adb_return = String::from_utf8(output.stdout).unwrap().trim().to_string();
        println!("[DEBUG] - Phone, detected: {:?}", adb_return);
        adb_return
    }
}


pub fn get_phone_brand() -> String {
    let device: String;

    let output = Command::new("adb")
            .arg("shell")
            .arg("getprop")
            .arg("ro.product.brand")
            .output()
            .expect("adb command failed");
    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        println!("[DEBUG] Error: {:?}", error);
        error
    } else {
        let adb_return = String::from_utf8(output.stdout).unwrap().trim().to_string() + " ";
        println!("[DEBUG] - Phone, detected: {:?}", adb_return);
        adb_return + &get_phone_model()
    }

}