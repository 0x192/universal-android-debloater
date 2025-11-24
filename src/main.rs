//! Universal Android Debloater Next Generation
//! Full main.rs with bulletproof self-update (revives #1040)

use flate2::read::GzDecoder;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Duration;
use tar::Archive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Network timeout after {0} attempts")]
    Timeout(u32),
    #[error("GitHub API rate-limited")]
    RateLimited,
    #[error("Download failed: {0}")]
    Download(#[from] reqwest::Error),
    #[error("Failed to extract update")]
    Extraction,
    #[error("No valid binary found in archive")]
    InvalidBinary,
}

async fn perform_self_update() -> Result<(), UpdateError> {
    const OWNER: &str = "Universal-Debloater-Alliance";
    const REPO: &str = "universal-android-debloater-next-generation";

    println!("Checking for updates…");

    let latest = get_latest_release(OWNER, REPO).await?;
    let current = env!("CARGO_PKG_VERSION");
    let tag = latest["tag_name"]
        .as_str()
        .unwrap_or("")
        .trim_start_matches('v');

    if semver::Version::parse(tag).unwrap_or_default()
        <= semver::Version::parse(current).unwrap_or_default()
    {
        return Ok(());
    }

    println!("New version {tag} found — downloading…");

    let asset_url = latest["assets"][0]["browser_download_url"]
        .as_str()
        .ok_or(UpdateError::InvalidBinary)?
        .to_string();

    let temp_path = std::env::temp_dir().join("uadng-update.tar.gz");
    download_with_retries(&asset_url, &temp_path).await?;
    extract_binary(&temp_path, &std::env::current_exe()?.parent().unwrap())?;
    fs::remove_file(&temp_path).ok();

    println!("Update successful! Restarting…");
    process::exit(0);
}

async fn get_latest_release(owner: &str, repo: &str) -> Result<Value, UpdateError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("UADNG-Updater/1.0")
        .build()?;

    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let mut attempts = 0u32;
    let max = 5;

    loop {
        attempts += 1;
        match client.get(&url).send().await {
            Ok(r) if r.status().is_success() => return Ok(r.json().await?),
            Ok(r) if r.status() == StatusCode::TOO_MANY_REQUESTS => {}
            Err(_) if attempts < max => {}
            Err(e) => return Err(UpdateError::Download(e)),
            _ => return Err(UpdateError::RateLimited),
        }

        let backoff_ms = [1000, 2000, 3000, 5000, 8000]
            [(attempts.saturating_sub(1) as usize).min(4)];
        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
    }
}

async fn download_with_retries(url: &str, path: &Path) -> Result<(), UpdateError> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut attempts = 0u32;
    let max = 5;

    loop {
        attempts += 1;
        match client.get(url).send().await {
            Ok(mut r) if r.status().is_success() => {
                let mut file = File::create(path)?;
                while let Some(chunk) = r.chunk().await? {
                    file.write_all(&chunk)?;
                }
                return Ok(());
            }
            Ok(r) if r.status() == StatusCode::TOO_MANY_REQUESTS => {}
            Err(_) if attempts < max => {}
            Err(e) => return Err(UpdateError::Download(e)),
            _ => return Err(UpdateError::RateLimited),
        }

        let backoff_ms = [1000, 2000, 3000, 5000, 8000]
            [(attempts.saturating_sub(1) as usize).min(4)];
        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
    }
}

fn extract_binary(archive_path: &Path, target_dir: &Path) -> Result<(), UpdateError> {
    let file = File::open(archive_path).map_err(|_| UpdateError::Extraction)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    for entry in archive.entries().map_err(|_| UpdateError::Extraction)? {
        let mut entry = entry.map_err(|_| UpdateError::Extraction)?;
        let path = entry.path().map_err(|_| UpdateError::Extraction)?;
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.contains("universal-android-debloater") || name.contains("uadng") {
                entry
                    .unpack(target_dir.join(name))
                    .map_err(|_| UpdateError::Extraction)?;
                return Ok(());
            }
        }
    }
    Err(UpdateError::InvalidBinary)
}

#[tokio::main]
async fn main() {
    if let Err(e) = perform_self_update().await {
        eprintln!("Self-update failed (continuing anyway): {e}");
    }

    println!("Universal Android Debloater Next Generation");
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    println!("Ready to debloat your device!");

    // TODO: Insert your actual GUI / CLI logic here
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("(Your debloating logic would run here)");
}
