use crate::core::config::{Config, DeviceSettings, GeneralSettings};
use crate::core::sync::Phone;
use crate::core::theme::Theme;
use crate::core::utils::{open_url, string_to_theme};
use crate::gui::style;

use iced::widget::{button, checkbox, column, container, pick_list, row, text, Space};
use iced::{Element, Length, Renderer};
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
}

impl Settings {
    pub fn update(&mut self, phone: &Phone, msg: Message) {
        match msg {
            Message::ExpertMode(toggled) => {
                self.general.expert_mode = toggled;
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
            }
            Message::DisableMode(toggled) => {
                if phone.android_sdk >= 23 {
                    self.device.disable_mode = toggled;
                    debug!("Config change: {:?}", self);
                    Config::save_changes(self, &phone.adb_id);
                }
            }
            Message::MultiUserMode(toggled) => {
                self.device.multi_user_mode = toggled;
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
            }
            Message::ApplyTheme(theme) => {
                self.general.theme = theme.to_string();
                debug!("Config change: {:?}", self);
                Config::save_changes(self, &phone.adb_id);
            }
            Message::UrlPressed(url) => {
                open_url(url);
            }
            Message::LoadDeviceSettings => {
                match Config::load_configuration_file()
                    .devices
                    .iter()
                    .find(|d| d.device_id == phone.adb_id)
                {
                    Some(device) => self.device = device.clone(),
                    None => {
                        self.device = DeviceSettings {
                            device_id: phone.adb_id.clone(),
                            multi_user_mode: phone.android_sdk > 21,
                            disable_mode: false,
                        }
                    }
                };
            }
        }
    }

    pub fn view(&self, phone: &Phone) -> Element<Message, Renderer<Theme>> {
        let general_category_text = text("General").size(25);

        let theme_picklist = pick_list(
            Theme::all(),
            Some(string_to_theme(self.general.theme.clone())),
            Message::ApplyTheme,
        );

        let expert_mode_descr =
            text("Most of unsafe packages are known to bootloop the device if removed.").size(15);

        let expert_mode_checkbox = checkbox(
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)",
            self.general.expert_mode,
            Message::ExpertMode,
        )
        .style(style::CheckBox::SettingsEnabled);

        let uad_category_text = text(phone.model.to_owned()).size(25);

        let warning = container(
            text("The following settings only affect the currently selected device")
                .style(style::Text::Danger),
        )
        .padding(10)
        .width(Length::Fill)
        .style(style::Container::Description);

        let multi_user_mode_descr =
            text("Disabling this setting will typically prevent affecting your work profile")
                .size(15);

        let multi_user_mode_checkbox = checkbox(
            "Affect all the users of the phone (not only the selected user)",
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
                .size(15);

        /*        let _unavailable_text = text("[Unavailable before Android 8.0]")
        .size(16);*/

        let unavailable_btn = button(text("Unavailable").size(13))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki/FAQ#\
                    why-is-the-disable-mode-setting-not-available-for-my-device",
            )))
            .height(Length::Units(22))
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

        let content = column![
            general_category_text,
            "Theme",
            theme_picklist,
            Space::new(Length::Fill, Length::Shrink),
            expert_mode_checkbox,
            expert_mode_descr,
            Space::new(Length::Fill, Length::Shrink),
            uad_category_text,
            warning,
            Space::new(Length::Fill, Length::Shrink),
            multi_user_mode_checkbox,
            multi_user_mode_descr,
            Space::new(Length::Fill, Length::Shrink),
            disable_setting_row,
            disable_mode_descr,
        ]
        .width(Length::Fill)
        .spacing(10);

        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container::Content)
            .into()
    }
}
