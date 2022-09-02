use crate::core::config::Config;
use crate::core::sync::{get_android_sdk, Phone as CorePhone};
use crate::core::theme::Theme;
use crate::core::utils::{open_url, string_to_theme};
use crate::gui::style;
use crate::IN_FILE_CONFIGURATION;

use iced::widget::{button, checkbox, column, container, pick_list, row, text, Space};
use iced::{Element, Length, Renderer};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Settings {
    pub phone: Phone,
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub struct Phone {
    pub expert_mode: bool,
    pub disable_mode: bool,
    pub multi_user_mode: bool,
}

impl Default for Phone {
    fn default() -> Self {
        Self {
            expert_mode: false,
            disable_mode: false,
            multi_user_mode: get_android_sdk() > 21,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            phone: Phone::default(),
            theme: string_to_theme(IN_FILE_CONFIGURATION.theme.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ExpertMode(bool),
    DisableMode(bool),
    MultiUserMode(bool),
    ApplyTheme(Theme),
    UrlPressed(PathBuf),
}

impl Settings {
    pub fn update(&mut self, phone: &CorePhone, msg: Message) {
        match msg {
            Message::ExpertMode(toggled) => {
                info!(
                    "Expert mode {}",
                    if toggled { "enabled" } else { "disabled" }
                );
                self.phone.expert_mode = toggled;
            }
            Message::DisableMode(toggled) => {
                if phone.android_sdk >= 23 {
                    info!(
                        "Disable mode {}",
                        if toggled { "enabled" } else { "disabled" }
                    );
                    self.phone.disable_mode = toggled;
                }
            }
            Message::MultiUserMode(toggled) => {
                info!(
                    "Multi-user mode {}",
                    if toggled { "enabled" } else { "disabled" }
                );
                self.phone.multi_user_mode = toggled;
            }
            Message::ApplyTheme(theme) => {
                self.theme = theme;
                Config::save_changes(self);
            }
            Message::UrlPressed(url) => {
                open_url(url);
            }
        }
    }

    pub fn view(&self, phone: &CorePhone) -> Element<Message, Renderer<Theme>> {
        let general_category_text = text("General").size(25);

        let theme_picklist = pick_list(Theme::all(), Some(self.theme.clone()), Message::ApplyTheme);

        let uad_category_text = text("Non-persistent settings").size(25);

        let expert_mode_descr =
            text("Most of unsafe packages are known to bootloop the device if removed.").size(15);

        let expert_mode_checkbox = checkbox(
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)",
            self.phone.expert_mode,
            Message::ExpertMode,
        )
        .style(style::CheckBox::SettingsEnabled);

        let multi_user_mode_descr =
            text("Disabling this setting will typically prevent affecting your work profile")
                .size(15);

        let multi_user_mode_checkbox = checkbox(
            "Affect all the users of the phone (not only the selected user)",
            self.phone.multi_user_mode,
            Message::MultiUserMode,
        )
        .style(style::CheckBox::SettingsEnabled);

        let _disable_color = if phone.android_sdk >= 23 {
            self.theme.palette.normal.surface
        } else {
            self.theme.palette.normal.primary
        };

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
            self.phone.disable_mode,
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
            uad_category_text,
            expert_mode_checkbox,
            expert_mode_descr,
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
