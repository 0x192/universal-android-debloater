use serde::Deserialize;

#[cfg(feature = "self-update")]
use {
    retry::{delay::Fibonacci, retry, OperationResult},
    std::fs,
    std::io,
    std::io::copy,
    std::path::Path,
    std::path::PathBuf,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

#[derive(Default, Debug, Clone)]
pub struct SelfUpdateState {
    pub latest_release: Option<Release>,
    pub status: SelfUpdateStatus,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum SelfUpdateStatus {
    Updating,
    #[default]
    Checking,
    Done,
    Failed,
}

impl std::fmt::Display for SelfUpdateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Checking => "Checking updates...",
            Self::Updating => "Updating...",
            Self::Failed => "Failed to check update!",
            Self::Done => "Done",
        };
        write!(f, "{s}")
    }
}

/// Download a file from the internet
#[cfg(feature = "self-update")]
pub async fn download_file<T: ToString + Send>(url: T, dest_file: PathBuf) -> Result<(), String> {
    let url = url.to_string();
    debug!("downloading file from {}", &url);

    match ureq::get(&url).call() {
        Ok(res) => {
            let mut file = fs::File::create(dest_file).map_err(|e| e.to_string())?;

            if let Err(e) = copy(&mut res.into_reader(), &mut file) {
                return Err(e.to_string());
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}

/// Downloads the latest release file that matches `bin_name`, renames the current
/// executable to a temp path, renames the new version as the original file name,
/// then returns both the original file name (new version) and temp path (old version)
#[cfg(feature = "self-update")]
pub async fn download_update_to_temp_file(
    bin_name: String,
    release: Release,
) -> Result<(PathBuf, PathBuf), ()> {
    let current_bin_path = std::env::current_exe().map_err(|_| ())?;

    // Path to download the new version to
    let download_path = current_bin_path
        .parent()
        .ok_or(())?
        .join(format!("tmp_{bin_name}"));

    // Path to temporarily force rename current process to, se we can then
    // rename `download_path` to `current_bin_path` and then launch new version
    // cleanly as `current_bin_path`
    let tmp_path = current_bin_path
        .parent()
        .ok_or(())?
        .join(format!("tmp2_{bin_name}"));

    // MacOS and Linux release are gziped tarball
    #[cfg(not(target_os = "windows"))]
    {
        let asset_name = format!("{bin_name}.tar.gz");

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .cloned()
            .ok_or(())?;

        let archive_path = current_bin_path.parent().ok_or(())?.join(&asset_name);

        if let Err(e) = download_file(asset.download_url, archive_path.clone()).await {
            error!("Couldn't download UAD update: {}", e);
            return Err(());
        }

        if extract_binary_from_tar(&archive_path, &download_path).is_err() {
            error!("Couldn't extract UAD release tarball");
            return Err(());
        }

        std::fs::remove_file(&archive_path).map_err(|_| ())?;
    }

    // For Windows we download the new binary directly
    #[cfg(target_os = "windows")]
    {
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == bin_name)
            .cloned()
            .ok_or(())?;

        if let Err(e) = download_file(asset.download_url, download_path.clone()).await {
            error!("Couldn't download UAD update: {}", e);
            return Err(());
        }
    }

    // Make the file executable
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(&download_path).map_err(|_| ())?.permissions();
        permissions.set_mode(0o755);
        if let Err(e) = fs::set_permissions(&download_path, permissions) {
            error!("[SelfUpdate] Couldn't set permission to temp file: {}", e);
            return Err(());
        }
    }

    if let Err(e) = rename(&current_bin_path, &tmp_path) {
        error!("[SelfUpdate] Couldn't rename binary path: {}", e);
        return Err(());
    }
    if let Err(e) = rename(&download_path, &current_bin_path) {
        error!("[SelfUpdate] Couldn't rename binary path: {}", e);
        return Err(());
    }

    Ok((current_bin_path, tmp_path))
}

#[cfg(not(feature = "self-update"))]
pub fn get_latest_release() -> Result<Option<Release>, ()> {
    Ok(None)
}

// UAD only has pre-releases so we can't use
// https://api.github.com/repos/0x192/universal-android-debloater/releases/latest
// to only get the latest release
#[cfg(feature = "self-update")]
pub fn get_latest_release() -> Result<Option<Release>, ()> {
    debug!("Checking for UAD update");

    match ureq::get("https://api.github.com/repos/0x192/universal-android-debloater/releases")
        .call()
    {
        Ok(res) => {
            let release: Release = serde_json::from_value(
                res.into_json::<serde_json::Value>()
                    .map_err(|_| ())?
                    .get(0)
                    .ok_or(())?
                    .clone(),
            )
            .map_err(|_| ())?;
            if release.tag_name.as_str() != "dev-build"
                && release.tag_name.as_str() > env!("CARGO_PKG_VERSION")
            {
                Ok(Some(release))
            } else {
                Ok(None)
            }
        }
        Err(_) => {
            debug!("Failed to check UAD update");
            Err(())
        }
    }
}

/// Extracts the binary from a `tar.gz` archive to `temp_file` path
#[cfg(feature = "self-update")]
#[cfg(not(target_os = "windows"))]
pub fn extract_binary_from_tar(archive_path: &Path, temp_file: &Path) -> io::Result<()> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use tar::Archive;
    let mut archive = Archive::new(GzDecoder::new(File::open(archive_path)?));

    let mut temp_file = File::create(temp_file)?;

    for file in archive.entries()? {
        let mut file = file?;

        let path = file.path()?;
        if path.to_str().is_some() {
            io::copy(&mut file, &mut temp_file)?;
            return Ok(());
        }
    }
    Err(io::ErrorKind::NotFound.into())
}

/// Hardcoded binary names for each compilation target
/// that gets published to the Github Release
#[cfg(feature = "self-update")]
pub const fn bin_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "uad_gui.exe"
    }

    #[cfg(target_os = "macos")]
    {
        "uad_gui-macos"
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        "uad_gui-linux"
    }
}

/// Rename a file or directory to a new name, retrying if the operation fails because of permissions
///
/// Will retry for ~30 seconds with longer and longer delays between each, to allow for virus scan
/// and other automated operations to complete.
#[cfg(feature = "self-update")]
pub fn rename<F, T>(from: F, to: T) -> Result<(), String>
where
    F: AsRef<Path>,
    T: AsRef<Path>,
{
    // 21 Fibonacci steps starting at 1 ms is ~28 seconds total
    // See https://github.com/rust-lang/rustup/pull/1873 where this was used by Rustup to work around
    // virus scanning file locks
    let from = from.as_ref();
    let to = to.as_ref();

    retry(Fibonacci::from_millis(1).take(21), || {
        match fs::rename(from, to) {
            Ok(_) => OperationResult::Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                _ => OperationResult::Err(e),
            },
        }
    })
    .map_err(|e| e.to_string())
}

/// Remove a file, retrying if the operation fails because of permissions
///
/// Will retry for ~30 seconds with longer and longer delays between each, to allow for virus scan
/// and other automated operations to complete.
#[cfg(feature = "self-update")]
pub fn remove_file<P>(path: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    // 21 Fibonacci steps starting at 1 ms is ~28 seconds total
    // See https://github.com/rust-lang/rustup/pull/1873 where this was used by Rustup to work around
    // virus scanning file locks
    let path = path.as_ref();

    retry(
        Fibonacci::from_millis(1).take(21),
        || match fs::remove_file(path) {
            Ok(_) => OperationResult::Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                _ => OperationResult::Err(e),
            },
        },
    )
    .map_err(|e| e.to_string())
}
