use crate::core::uad_lists::PackageState;
use crate::gui::widgets::package_row::PackageRow;
use crate::gui::views::list::Selection;

use std::fs; 
use std::io::{self, prelude::*, BufReader};

pub fn update_selection_count(selection: &mut Selection, p_state: PackageState, add: bool) {
    match p_state {
        PackageState::Enabled => {
            if add { selection.enabled += 1 } else { selection.enabled -= 1 };
        },
        PackageState::Disabled => {
            if add { selection.disabled += 1 } else { selection.disabled -= 1 };
        },
        PackageState::Uninstalled => {
            if add { selection.uninstalled += 1 } else { selection.uninstalled -= 1 };
        },
        PackageState::All => {}
    };
}

pub async fn export_selection(packages: Vec<PackageRow>) -> Result<bool, String> {
    let selected = packages
        .iter()
        .filter(|p| p.selected)
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join("\n");

    match fs::write("uad_exported_selection.txt", selected) {
        Ok(_) => Ok(true),
        Err(err) => Err(err.to_string())
    }
}

pub fn import_selection(packages: &mut Vec<PackageRow>, selection: &mut Selection) -> io::Result<()> {
    let file = fs::File::open("uad_exported_selection.txt")?;
    let reader = BufReader::new(file);
    let imported_selection: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    *selection = Selection::default(); // should already be empty normally

    for (i,p) in packages.iter_mut().enumerate() {
        if imported_selection.contains(&p.name) {
            p.selected = true;
            selection.selected_packages.push(i);
            update_selection_count(selection, p.state, true);
        } else {
            p.selected = false;
        }
    }

    Ok(())
}