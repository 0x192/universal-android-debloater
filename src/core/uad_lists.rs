use serde::{Deserialize};
use serde_json;
//use std::fs;
use std::{collections::HashMap};


#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    id: String,
    pub list: UadList,
    pub description: Option<String>,
    dependencies: Option<String>,
    needed_by: Option<String>,
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
    Installed,
    Uninstalled
}

impl Default for PackageState {
    fn default() -> PackageState {
        PackageState::Installed
    }
}

impl PackageState {
    pub const ALL: [PackageState; 3] = [
        PackageState::All,
        PackageState::Installed,
        PackageState::Uninstalled,
    ];
}


impl std::fmt::Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PackageState::All => "All packages",
                PackageState::Installed => "installed",
                PackageState::Uninstalled => "uninstalled",
            }
        )
    }
}


impl std::str::FromStr for PackageState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "installed" => Ok(PackageState::Installed),
            "uninstalled" => Ok(PackageState::Uninstalled),
            _ => Err(format!("'{}' is not a valid value for WSType", s)),
        }
    }
}

// Bad names. To be changed!
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Removal {
    All,
    Recommanded,
    Advanced,
    Expert,
    Unsafe,
    Unlisted,
}

impl Default for Removal {
    fn default() -> Removal {
        Removal::Recommanded
    }
}

impl Removal {
    pub const ALL: [Removal; 6] = [
        Removal::All,
        Removal::Recommanded,
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
                Removal::Recommanded => "Recommanded",
                Removal::Advanced => "Advanced",
                Removal::Expert => "Expert",
                Removal::Unsafe => "Unsafe",
                Removal::Unlisted => "Unlisted",
            }
        )
    }
}


pub fn load_debloat_lists() -> HashMap<String, Package> {
    const DATA: &str = include_str!("../../ressources/assets/uad_lists.json");
    let mut package_lists = HashMap::new();
    //let data = fs::read_to_string("ressources/assets/uad_lists.json").expect("Unable to read file");

    // TODO: Do it without intermediary Vec?
    let list: Vec<Package> = serde_json::from_str(&DATA).expect("Unable to parse");

    for p in list {
        let name = p.id.clone();
        package_lists.insert(name, p);
    }

    return package_lists;
}



