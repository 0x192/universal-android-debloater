use crate::core::config::{BackupSettings, Config, DeviceSettings, GeneralSettings};
use crate::core::save::{
    backup_phone, list_available_backup_user, list_available_backups, restore_backup, BACKUP_DIR,
};
use crate::core::sync::{get_android_sdk, perform_adb_commands, CommandType, Phone};
use crate::core::theme::Theme;
use crate::core::utils::{open_url, string_to_theme, DisplayablePath};
use crate::gui::style;
use crate::gui::views::list::PackageInfo;
use crate::gui::widgets::package_row::PackageRow;

use iced::widget::{button, checkbox, column, container, pick_list, radio, row, text, Space};
use iced::{alignment, Alignment, Command, Element, Length, Renderer};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Settings {
    pub general: GeneralSettings,
    pub device: DeviceSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: Config::load_configuration_file().general,
            device: DeviceSettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadDeviceSettings,
    ExpertMode(bool),
    DisableMode(bool),
    MultiUserMode(bool),
    ApplyTheme(Theme),
    UrlPressed(PathBuf),
    BackupSelected(DisplayablePath),
    BackupDevice,
    RestoreDevice,
    RestoringDevice(Result<CommandType, ()>),
    DeviceBackedUp(Result<(), String>),
}

impl Settings {
    pub fn update(
        &mut self,
        phone: &Phone,
        packages: &[Vec<PackageRow>],
        nb_running_async_adb_commands: &mut u32,
        msg: Message,
    ) -> Command<Message> {
        match msg {
            Message::ExpertMode(toggled) => {
                self.general.expert_mode = toggled;
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
                Command::none()
            }
            Message::DisableMode(toggled) => {
                if phone.android_sdk >= 23 {
                    self.device.disable_mode = toggled;
                    debug!("Config change: {:?}", self);
                    Config::save_changes(self, &phone.adb_id);
                }
                Command::none()
            }
            Message::MultiUserMode(toggled) => {
                self.device.multi_user_mode = toggled;
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
                Command::none()
            }
            Message::ApplyTheme(theme) => {
                self.general.theme = theme.to_string();
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
                Command::none()
            }
            Message::UrlPressed(url) => {
                open_url(url);
                Command::none()
            }
            Message::LoadDeviceSettings => {
                let backups = list_available_backups(&BACKUP_DIR.join(phone.adb_id.clone()));
                match Config::load_configuration_file()
                    .devices
                    .iter()
                    .find(|d| d.device_id == phone.adb_id)
                {
                    Some(device) => {
                        self.device = device.clone();
                        self.device.backup = BackupSettings {
                            backups: backups.clone(),
                            selected: backups.first().cloned(),
                            users: phone.user_list.clone(),
                            selected_user: phone.user_list.first().copied(),
                            backup_state: String::new(),
                        };
                    }
                    None => {
                        self.device = DeviceSettings {
                            device_id: phone.adb_id.clone(),
                            multi_user_mode: phone.android_sdk > 21,
                            disable_mode: false,
                            backup: BackupSettings {
                                backups: backups.clone(),
                                selected: backups.first().cloned(),
                                users: phone.user_list.clone(),
                                selected_user: phone.user_list.first().copied(),
                                backup_state: String::new(),
                            },
                        }
                    }
                };
                Command::none()
            }
            Message::BackupSelected(d_path) => {
                self.device.backup.selected = Some(d_path.clone());
                self.device.backup.users = list_available_backup_user(d_path);
                Command::none()
            }
            Message::BackupDevice => Command::perform(
                backup_phone(
                    phone.user_list.clone(),
                    self.device.device_id.clone(),
                    packages.to_vec(),
                ),
                Message::DeviceBackedUp,
            ),
            Message::DeviceBackedUp(_) => {
                info!("[BACKUP] Backup successfully created");
                self.device.backup.backups =
                    list_available_backups(&BACKUP_DIR.join(phone.adb_id.clone()));
                self.device.backup.selected = self.device.backup.backups.first().cloned();
                Command::none()
            }
            Message::RestoreDevice => match restore_backup(phone, packages, &self.device) {
                Ok(r_packages) => {
                    let mut commands = vec![];
                    *nb_running_async_adb_commands = 0;
                    for p in &r_packages {
                        let p_info = PackageInfo {
                            i_user: 0,
                            index: p.index,
                            removal: "RESTORE".to_string(),
                        };
                        for command in p.commands.clone() {
                            *nb_running_async_adb_commands += 1;
                            commands.push(Command::perform(
                                perform_adb_commands(
                                    command,
                                    CommandType::PackageManager(p_info.clone()),
                                ),
                                Message::RestoringDevice,
                            ));
                        }
                    }
                    if r_packages.is_empty() {
                        if get_android_sdk() == 0 {
                            self.device.backup.backup_state = "Device is not connected".to_string();
                        } else {
                            self.device.backup.backup_state =
                                "Device state is already restored".to_string();
                        }
                    }
                    info!(
                        "[RESTORE] Restoring backup {}",
                        self.device.backup.selected.as_ref().unwrap()
                    );
                    Command::batch(commands)
                }
                Err(e) => {
                    self.device.backup.backup_state = e.to_string();
                    error!("{} - {}", self.device.backup.selected.as_ref().unwrap(), e);
                    Command::none()
                }
            },
            // Trigger an action in mod.rs (Message::SettingsAction(msg))
            Message::RestoringDevice(_) => Command::none(),
        }
    }

    pub fn view(&self, phone: &Phone) -> Element<Message, Renderer<Theme>> {
        let radio_btn_theme = Theme::ALL
            .iter()
            .fold(row![].spacing(10), |column, option| {
                column.push(
                    radio(
                        format!("{}", option.clone()),
                        *option,
                        Some(string_to_theme(&self.general.theme)),
                        Message::ApplyTheme,
                    )
                    .size(23),
                )
            });
        let theme_ctn = container(radio_btn_theme)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(style::Container::Frame);

        let expert_mode_checkbox = checkbox(
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)",
            self.general.expert_mode,
            Message::ExpertMode,
        )
        .style(style::CheckBox::SettingsEnabled);

        let expert_mode_descr =
            text("Most of unsafe packages are known to bootloop the device if removed.")
                .style(style::Text::Commentary)
                .size(15);

        let general_ctn = container(column![expert_mode_checkbox, expert_mode_descr].spacing(10))
            .padding(10)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(style::Container::Frame);

        let warning_ctn = container(
            row![
                text("The following settings only affect the currently selected device :")
                    .style(style::Text::Danger),
                text(phone.model.clone()),
                Space::new(Length::Fill, Length::Shrink),
                text(phone.adb_id.clone()).style(style::Text::Commentary)
            ]
            .spacing(7),
        )
        .padding(10)
        .width(Length::Fill)
        .style(style::Container::BorderedFrame);

        let multi_user_mode_descr = row![
            text("This will not affect the following protected work profile users: ")
                .size(15)
                .style(style::Text::Commentary),
            text(
                phone
                    .user_list
                    .iter()
                    .filter(|&u| u.protected)
                    .map(|u| u.id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .size(15)
            .style(style::Text::Danger)
        ];

        let multi_user_mode_checkbox = checkbox(
            "Affect all the users of the device (not only the selected user)",
            self.device.multi_user_mode,
            Message::MultiUserMode,
        )
        .style(style::CheckBox::SettingsEnabled);

        let disable_checkbox_style = if phone.android_sdk >= 23 {
            style::CheckBox::SettingsEnabled
        } else {
            style::CheckBox::SettingsDisabled
        };

        let disable_mode_descr =
            text("In some cases, it can be better to disable a package instead of uninstalling it")
                .style(style::Text::Commentary)
                .size(15);

        let unavailable_btn = button(text("Unavailable").size(13))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki/FAQ#\
                    why-is-the-disable-mode-setting-not-available-for-my-device",
            )))
            .height(22)
            .style(style::Button::Unavailable);

        // Disabling package without root isn't really possible before Android Oreo (8.0)
        // see https://github.com/0x192/universal-android-debloater/wiki/ADB-reference
        let disable_mode_checkbox = checkbox(
            "Clear and disable packages instead of uninstalling them",
            self.device.disable_mode,
            Message::DisableMode,
        )
        .style(disable_checkbox_style);

        let disable_setting_row = if phone.android_sdk >= 23 {
            row![
                disable_mode_checkbox,
                Space::new(Length::Fill, Length::Shrink),
            ]
            .width(Length::Fill)
        } else {
            row![
                disable_mode_checkbox,
                Space::new(Length::Fill, Length::Shrink),
                unavailable_btn,
            ]
            .width(Length::Fill)
        };

        let device_specific_ctn = container(
            column![
                multi_user_mode_checkbox,
                multi_user_mode_descr,
                disable_setting_row,
                disable_mode_descr,
            ]
            .spacing(10),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(style::Container::Frame);

        let backup_pick_list = pick_list(
            self.device.backup.backups.clone(),
            self.device.backup.selected.clone(),
            Message::BackupSelected,
        )
        .padding(6);

        let backup_btn = button(text("Backup").horizontal_alignment(alignment::Horizontal::Center))
            .padding(5)
            .on_press(Message::BackupDevice)
            .style(style::Button::Primary)
            .width(77);

        let restore_btn = |enabled| {
            if enabled {
                button(text("Restore").horizontal_alignment(alignment::Horizontal::Center))
                    .padding(5)
                    .on_press(Message::RestoreDevice)
                    .width(77)
            } else {
                button(text("No backup").horizontal_alignment(alignment::Horizontal::Center))
                    .padding(5)
                    .width(77)
            }
        };

        let locate_backup_btn = if self.device.backup.backups.is_empty() {
            button("Open backup directory")
                .padding(5)
                .style(style::Button::Primary)
        } else {
            button("Open backup directory")
                .on_press(Message::UrlPressed(BACKUP_DIR.join(phone.adb_id.clone())))
                .padding(5)
                .style(style::Button::Primary)
        };

        let backup_row = row![
            backup_btn,
            "Backup the current state of the phone",
            Space::new(Length::Fill, Length::Shrink),
            locate_backup_btn,
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let restore_row = if self.device.backup.backups.is_empty() {
            row![restore_btn(false), "Restore the state of the device",]
                .spacing(10)
                .align_items(Alignment::Center)
        } else {
            row![
                restore_btn(true),
                "Restore the state of the device",
                Space::new(Length::Fill, Length::Shrink),
                text(self.device.backup.backup_state.clone()).style(style::Text::Danger),
                backup_pick_list,
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        };

        let backup_restore_ctn = container(column![backup_row, restore_row].spacing(10))
            .padding(10)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(style::Container::Frame);

        let no_device_ctn = || {
            container(text("No device detected").style(style::Text::Danger))
                .padding(10)
                .width(Length::Fill)
                .style(style::Container::BorderedFrame)
        };

        let content = if phone.adb_id.clone().is_empty() {
            column![
                text("Theme").size(25),
                theme_ctn,
                text("General").size(25),
                general_ctn,
                text("Current device").size(25),
                no_device_ctn(),
                text("Backup / Restore").size(25),
                no_device_ctn(),
            ]
            .width(Length::Fill)
            .spacing(20)
        } else {
            column![
                text("Theme").size(25),
                theme_ctn,
                text("General").size(25),
                general_ctn,
                text("Current device").size(25),
                warning_ctn,
                device_specific_ctn,
                backup_restore_ctn,
            ]
            .width(Length::Fill)
            .spacing(20)
        };

        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
