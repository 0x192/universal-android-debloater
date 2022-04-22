use crate::core::sync::{
    adb_shell_command, hashset_system_packages, list_all_system_packages, User,
};
use crate::core::theme::Theme;
use crate::core::uad_lists::{Package, PackageState, Removal, UadList};
use crate::gui::views::list::Selection;
use crate::gui::widgets::package_row::PackageRow;
use crate::gui::ICONS;
use chrono::offset::Utc;
use chrono::DateTime;
use iced::{alignment, Length, Text};
use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
use std::process::Command;

pub fn fetch_packages(
    uad_lists: &HashMap<String, Package>,
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

        let package_row =
            PackageRow::new(p_name, state, description, uad_list, removal, false, false);
        user_package.push(package_row);
    }
    user_package.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    user_package
}

pub fn update_selection_count(selection: &mut Selection, p_state: PackageState, add: bool) {
    match p_state {
        PackageState::Enabled => {
            if add {
                selection.enabled += 1
            } else if selection.enabled > 0 {
                selection.enabled -= 1
            };
        }
        PackageState::Disabled => {
            if add {
                selection.disabled += 1
            } else if selection.disabled > 0 {
                selection.disabled -= 1
            };
        }
        PackageState::Uninstalled => {
            if add {
                selection.uninstalled += 1
            } else if selection.uninstalled > 0 {
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
pub fn import_selection(packages: &mut [PackageRow], selection: &mut Selection) -> io::Result<()> {
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

pub fn setup_uad_dir(dir: Option<PathBuf>) -> PathBuf {
    let dir = dir.unwrap().join("uad");
    fs::create_dir_all(&dir).expect("Can't create cache directory");
    dir
}

pub fn open_url(dir: PathBuf) {
    #[cfg(target_os = "windows")]
    let output = Command::new("explorer").args([dir]).output();

    #[cfg(target_os = "macos")]
    let output = Command::new("open").args([dir]).output();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let output = Command::new("xdg-open").args([dir]).output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                let stderr = String::from_utf8(o.stderr).unwrap().trim_end().to_string();
                error!("Can't open the following URL: {}", stderr)
            }
        }
        Err(e) => error!("Failed to run command to open the file explorer: {}", e),
    }
}

pub fn request_builder(commands: Vec<&str>, package: &str, users: &[User]) -> Vec<String> {
    if !users.is_empty() {
        users
            .iter()
            .flat_map(|u| {
                commands
                    .iter()
                    .map(|c| format!("{} --user {} {}", c, u.id, package))
            })
            .collect()
    } else {
        commands
            .iter()
            .map(|c| format!("{} {}", c, package))
            .collect()
    }
}

pub fn last_modified_date(file: PathBuf) -> DateTime<Utc> {
    let metadata = fs::metadata(file).unwrap();

    match metadata.modified() {
        Ok(time) => time.into(),
        Err(_) => Utc::now(),
    }
}

pub fn format_diff_time_from_now(date: DateTime<Utc>) -> String {
    let now: DateTime<Utc> = Utc::now();
    let last_update = now - date;
    if last_update.num_days() == 0 {
        if last_update.num_hours() == 0 {
            last_update.num_minutes().to_string() + " min(s) ago"
        } else {
            last_update.num_hours().to_string() + " hour(s) ago"
        }
    } else {
        last_update.num_days().to_string() + " day(s) ago"
    }
}

pub async fn perform_commands(action: String, i: usize, label: String) -> Result<usize, ()> {
    match adb_shell_command(true, &action) {
        Ok(o) => {
            // On old devices, adb commands can return the '0' exit code even if there
            // is an error. On Android 4.4, ADB doesn't check if the package exists.
            // It does not return any error if you try to `pm block` a non-existent package.
            // Some commands are even killed by ADB before finishing and UAD can't catch
            // the output.
            if ["Error", "Failure"].iter().any(|&e| o.contains(e)) {
                error!("[{}] {} -> {}", label, action, o);
                Err(())
            } else {
                info!("[{}] {} -> {}", label, action, o);
                Ok(i)
            }
        }
        Err(err) => {
            if !err.contains("[not installed for") {
                error!("[{}] {} -> {}", label, action, err);
            }
            Err(())
        }
    }
}
