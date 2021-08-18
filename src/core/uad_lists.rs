use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UadLists {
    All,
    Aosp,
    Carrier,
    Google,
    Misc,
    Oem
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Installed,
    Uninstalled
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    packageId: String,
    oem: Option<String>,
    description: String,
    dependencies: Option<String>,
    neededBy: Option<String>,
    labels: Option<Vec<String>>,

    #[serde(skip)]
    list: UadLists,

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


// TODO oem value can change
pub fn load_debloat_lists(uad_list: UadLists) -> Vec<Package> {
    let path = "debloat_lists/".to_string() + &uad_list.to_string() + ".json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let list: Vec<Package> = serde_json::from_str(&data).expect("Unable to parse");
    println!("LIST: {:?}", list[0].packageId);
    return list;
}
