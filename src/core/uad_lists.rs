use serde::{Deserialize};
use serde_json;
use std::fs;
use std::{collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UadLists {
    All,
    Aosp,
    Carrier,
    Google,
    Misc,
    Oem
}

/*pub enum Oem {
    Asus,
    Huawei,
    Lg,
    Motorola,
    Nokia,
    OnePlus,
    Oppo,
    Samsung,
    Sony,
    Tcl,
    Xiaomi,
    Zte,
}*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Installed,
    Uninstalled
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Package {
    id: String,
    list: String,
    pub description: Option<String>,
    dependencies: Option<String>,
    neededBy: Option<String>,
    labels: Option<Vec<String>>,
}


impl UadLists {
    pub const ALL: [UadLists; 6] = [
        UadLists::All,
        UadLists::Aosp,
        UadLists::Carrier,
        UadLists::Google,
        UadLists::Misc,
        UadLists::Oem,
    ];

/*    fn matches(&self, list: &UadLists) -> bool {
        match self {
            UadLists::All => true,
            UadLists::Active => !task.completed,
            UadLists::Completed => task.completed,
        }
    }*/
}

impl Default for UadLists {
    fn default() -> UadLists {
        UadLists::All
    }
}

impl std::fmt::Display for UadLists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UadLists::All => "all",
                UadLists::Aosp => "aosp",
                UadLists::Carrier => "carrier",
                UadLists::Google => "google",
                UadLists::Misc => "misc",
                UadLists::Oem => "oem",
            }
        )
    }
}


impl PackageState {
    pub const ALL: [PackageState; 2] = [
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
                PackageState::Installed => "installed",
                PackageState::Uninstalled => "uninstalled",
            }
        )
    }
}


pub fn load_debloat_lists() -> HashMap<String, Package> {
    let mut package_lists = HashMap::new();
    let data = fs::read_to_string("debloat_lists/all.json").expect("Unable to read file");

    // TODO: Do it without intermediary Vec 
    let list: Vec<Package> = serde_json::from_str(&data).expect("Unable to parse");

    for p in list {
        let name = p.id.clone();
        package_lists.insert(name, p);
    }

    return package_lists;
}



