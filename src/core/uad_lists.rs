use crate::core::utils::{format_diff_time_from_now, last_modified_date};
use crate::CACHE_DIR;
use retry::{delay::Fixed, retry, OperationResult};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    id: String,
    pub list: UadList,
    pub description: Option<String>,
    dependencies: Option<Vec<String>>,
    needed_by: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    pub removal: Removal,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UadList {
    All,
    Aosp,
    Carrier,
    Google,
    Misc,
    Oem,
    Pending,
    Unlisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UadListState {
    Downloading,
    Done,
    Failed,
}

impl Default for UadListState {
    fn default() -> Self {
        UadListState::Downloading
    }
}

impl std::fmt::Display for UadListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = last_modified_date(CACHE_DIR.join("uad_lists.json"));
        let s = match self {
            UadListState::Downloading => "Checking updates...".to_string(),
            UadListState::Done => format!("Done (last was {})", format_diff_time_from_now(date)),
            UadListState::Failed => "Failed to check update!".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Default for UadList {
    fn default() -> UadList {
        UadList::All
    }
}

impl UadList {
    pub const ALL: [UadList; 8] = [
        UadList::All,
        UadList::Aosp,
        UadList::Carrier,
        UadList::Google,
        UadList::Misc,
        UadList::Oem,
        UadList::Pending,
        UadList::Unlisted,
    ];
}

impl std::fmt::Display for UadList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UadList::All => "All lists",
                UadList::Aosp => "aosp",
                UadList::Carrier => "carrier",
                UadList::Google => "google",
                UadList::Misc => "misc",
                UadList::Oem => "oem",
                UadList::Pending => "pending",
                UadList::Unlisted => "unlisted",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    All,
    Enabled,
    Uninstalled,
    Disabled,
}

impl Default for PackageState {
    fn default() -> PackageState {
        PackageState::Enabled
    }
}

impl PackageState {
    pub const ALL: [PackageState; 4] = [
        PackageState::All,
        PackageState::Enabled,
        PackageState::Uninstalled,
        PackageState::Disabled,
    ];
}

impl std::fmt::Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PackageState::All => "All packages",
                PackageState::Enabled => "Enabled",
                PackageState::Uninstalled => "Uninstalled",
                PackageState::Disabled => "Disabled",
            }
        )
    }
}

pub trait Opposite {
    fn opposite(&self, disable: bool) -> PackageState;
}

impl Opposite for PackageState {
    fn opposite(&self, disable: bool) -> PackageState {
        match self {
            PackageState::Enabled => {
                if disable {
                    PackageState::Disabled
                } else {
                    PackageState::Uninstalled
                }
            }
            PackageState::Uninstalled | PackageState::Disabled => PackageState::Enabled,
            PackageState::All => PackageState::All,
        }
    }
}

// Bad names. To be changed!
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Removal {
    All,
    Recommended,
    Advanced,
    Expert,
    Unsafe,
    Unlisted,
}

impl Default for Removal {
    fn default() -> Removal {
        Removal::Recommended
    }
}

impl Removal {
    pub const ALL: [Removal; 6] = [
        Removal::All,
        Removal::Recommended,
        Removal::Advanced,
        Removal::Expert,
        Removal::Unsafe,
        Removal::Unlisted,
    ];
}

impl std::fmt::Display for Removal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Removal::All => "All",
                Removal::Recommended => "Recommended",
                Removal::Advanced => "Advanced",
                Removal::Expert => "Expert",
                Removal::Unsafe => "Unsafe",
                Removal::Unlisted => "Unlisted",
            }
        )
    }
}

pub async fn load_debloat_lists(remote: bool) -> (Result<HashMap<String, Package>, ()>, bool) {
    let cached_uad_lists: PathBuf = CACHE_DIR.join("uad_lists.json");
    let mut error = false;
    let list: Vec<Package> = if remote {
        match retry(Fixed::from_millis(500).take(120), || {
            match ureq::get(
                "https://raw.githubusercontent.com/0x192/universal-android-debloater/\
            main/resources/assets/uad_lists.json",
            )
            .call()
            {
                Ok(data) => {
                    let text = data.into_string().unwrap();
                    fs::write(cached_uad_lists.clone(), &text).expect("Unable to write file");
                    let list = serde_json::from_str(&text).expect("Unable to parse");
                    OperationResult::Ok(list)
                }
                Err(e) => {
                    warn!("Could not load remote debloat list: {}", e);
                    error = true;
                    OperationResult::Retry(Vec::<Package>::new())
                }
            }
        }) {
            Ok(list) => list,
            Err(_) => vec![],
        }
    } else {
        warn!("Could not load remote debloat list");
        get_local_lists()
    };

    // TODO: Do it without intermediary Vec?
    let mut package_lists = HashMap::new();
    for p in list {
        let name = p.id.clone();
        package_lists.insert(name, p);
    }
    if error {
        (Err(()), remote)
    } else {
        (Ok(package_lists), remote)
    }
}

fn get_local_lists() -> Vec<Package> {
    const DATA: &str = include_str!("../../resources/assets/uad_lists.json");
    let cached_uad_lists = CACHE_DIR.join("uad_lists.json");

    if Path::new(&cached_uad_lists).exists() {
        let data = fs::read_to_string(cached_uad_lists).unwrap();
        serde_json::from_str(&data).expect("Unable to parse")
    } else {
        serde_json::from_str(DATA).expect("Unable to parse")
    }
}
