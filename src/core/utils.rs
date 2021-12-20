use crate::core::sync::{hashset_system_packages, list_all_system_packages, User};
use crate::core::theme::Theme;
use crate::core::uad_lists::{Package, PackageState, Removal, UadList};
use crate::gui::views::list::Selection;
use crate::gui::widgets::package_row::PackageRow;
use crate::gui::ICONS;

use iced::{alignment, Length, Text};
use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*, BufReader};

pub fn fetch_packages(
    uad_lists: &'static HashMap<String, Package>,
    user_id: &Option<&User>,
) -> Vec<PackageRow> {
    let all_system_packages = list_all_system_packages(user_id); // installed and uninstalled packages
    let enabled_system_packages = hashset_system_packages(PackageState::Enabled, user_id);
    let disabled_system_packages = hashset_system_packages(PackageState::Disabled, user_id);
    let mut description;
    let mut uad_list;
    let mut state;
    let mut removal;
    let mut user_package: Vec<PackageRow> = Vec::new();

    for p_name in all_system_packages.lines() {
        state = PackageState::Uninstalled;
        description = "[No description]";
        uad_list = UadList::Unlisted;
        removal = Removal::Unlisted;

        if uad_lists.contains_key(p_name) {
            description = match &uad_lists.get(p_name).unwrap().description {
                Some(descr) => descr,
                None => "[No description]",
            };
            uad_list = uad_lists.get(p_name).unwrap().list;
            removal = uad_lists.get(p_name).unwrap().removal;
        }

        if enabled_system_packages.contains(p_name) {
            state = PackageState::Enabled;
        } else if disabled_system_packages.contains(p_name) {
            state = PackageState::Disabled;
        }

        let package_row = PackageRow::new(p_name, state, description, uad_list, removal, false);
        user_package.push(package_row);
    }
    user_package.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    user_package
}

pub fn update_selection_count(selection: &mut Selection, p_state: PackageState, add: bool) {
    // Selection can't be negative
    if !add && selection.selected_packages.is_empty() {
        return;
    }

    match p_state {
        PackageState::Enabled => {
            if add {
                selection.enabled += 1
            } else {
                selection.enabled -= 1
            };
        }
        PackageState::Disabled => {
            if add {
                selection.disabled += 1
            } else {
                selection.disabled -= 1
            };
        }
        PackageState::Uninstalled => {
            if add {
                selection.uninstalled += 1
            } else {
                selection.uninstalled -= 1
            };
        }
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
        Err(err) => Err(err.to_string()),
    }
}

#[allow(clippy::needless_collect)] // false positive: https://github.com/rust-lang/rust-clippy/issues/6164
pub fn import_selection(
    packages: &mut Vec<PackageRow>,
    selection: &mut Selection,
) -> io::Result<()> {
    let file = fs::File::open("uad_exported_selection.txt")?;
    let reader = BufReader::new(file);
    let imported_selection: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    *selection = Selection::default(); // should already be empty normally

    for (i, p) in packages.iter_mut().enumerate() {
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

pub fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(17))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(17)
}

pub fn string_to_theme(theme: String) -> Theme {
    match theme.as_str() {
        "Dark" => Theme::dark(),
        "Light" => Theme::light(),
        "Lupin" => Theme::lupin(),
        _ => Theme::lupin(),
    }
}
