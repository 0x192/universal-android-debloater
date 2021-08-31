use serde::{Deserialize};
use serde_json;
//use std::fs;
use std::{collections::HashMap};


#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    id: String,
    pub list: String,
    pub description: Option<String>,
    dependencies: Option<String>,
    needed_by: Option<String>,
    labels: Option<Vec<String>>,
    pub confidence: String,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UadLists {
    All,
    Aosp,
    Carrier,
    Google,
    Misc,
    Oem,
    Unlisted,
}

impl Default for UadLists {
    fn default() -> UadLists {
        UadLists::All
    }
}

impl UadLists {
    pub const ALL: [UadLists; 7] = [
        UadLists::All,
        UadLists::Aosp,
        UadLists::Carrier,
        UadLists::Google,
        UadLists::Misc,
        UadLists::Oem,
        UadLists::Unlisted,
    ];
}

impl std::fmt::Display for UadLists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UadLists::All => "All lists",
                UadLists::Aosp => "aosp",
                UadLists::Carrier => "carrier",
                UadLists::Google => "google",
                UadLists::Misc => "misc",
                UadLists::Oem => "oem",
                UadLists::Unlisted => "unlisted",
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
        PackageState::All
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preselection {
    All,
    Safe,
    Advanced,
    Expert,
    Unsafe,
    Unlisted,
}

impl Default for Preselection {
    fn default() -> Preselection {
        Preselection::Safe
    }
}

impl Preselection {
    pub const ALL: [Preselection; 6] = [
        Preselection::All,
        Preselection::Safe,
        Preselection::Advanced,
        Preselection::Expert,
        Preselection::Unsafe,
        Preselection::Unlisted,
    ];
}


impl std::fmt::Display for Preselection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Preselection::All => "All",
                Preselection::Safe => "Safe",
                Preselection::Advanced => "Advanced",
                Preselection::Expert => "Expert",
                Preselection::Unsafe => "Unsafe",
                Preselection::Unlisted => "Unlisted",
            }
        )
    }
}


pub fn load_debloat_lists() -> HashMap<String, Package> {
    const DATA: &str = include_str!("../../ressources/assets/uad_lists.json");
    let mut package_lists = HashMap::new();
    //let data = fs::read_to_string("ressources/assets/uad_lists.json").expect("Unable to read file");

    // TODO: Do it without intermediary Vec 
    let list: Vec<Package> = serde_json::from_str(&DATA).expect("Unable to parse");

    for p in list {
        let name = p.id.clone();
        package_lists.insert(name, p);
    }

    return package_lists;
}



