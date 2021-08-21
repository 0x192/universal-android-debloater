use std::process::Command;
// TODO: Refactor -> adb function

// Mettres Package dans une table de Hashage


pub fn list_phone_packages() -> String {
/*    let output = Command::new("adb")
                .arg("shell")
                .arg("pm")
                .arg("list")
                .arg("packages")
                .output()
                .expect("adb command failed to start");*/

/*    let output = Command::new("ls")
                .output()
                .expect("ls command failed to start");
*/
/*    String::from_utf8(output.stdout).unwrap().replace("package:", "")*/
    let temp = "com.samsung.android.MtpApplication\ncom.samsung.android.provider.filterprovider\ncom.samsung.android.provider.shootingmodeprovider\ncom.samsung.android.provider.stickerprovider\ncom.samsung.android.SettingsReceiver\ncom.samsung.android.sm.policy\ncom.samsung.android.timezone.autoupdate_O\ncom.samsung.app.slowmotion\ncom.samsung.cmh\ncom.samsung.dcmservice\ncom.samsung.networkui\ncom.samsung.sec.android.application.csc\ncom.samsung.upsmtheme\ncom.sec.android.app.camera\ncom.sec.android.app.clockpackage\ncom.sec.android.app.launcher\ncom.sec.android.app.myfiles\ncom.sec.android.app.personalization\ncom.sec.android.app.popupcalculator\ncom.sec.android.app.simsettingmgr\ncom.sec.android.app.soundalive\ncom.sec.android.emergencylauncher\ncom.sec.android.emergencymode.service\ncom.sec.android.gallery3d\ncom.sec.android.gallery3d.panorama360view\ncom.sec.android.ofviewer\ncom.sec.android.provider.badge\ncom.sec.android.provider.emergencymode\ncom.sec.android.wallpapercropper2\ncom.sec.automation\ncom.sec.epdg\ncom.sec.ims\ncom.sec.imsservice";
    return temp.to_string();
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