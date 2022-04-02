use crate::core::config::Config;
use crate::core::sync::{get_android_sdk, Phone as CorePhone};
use crate::core::theme::Theme;
use crate::core::uad_lists::UadListState;
use crate::core::update::{Release, SelfUpdateState, SelfUpdateStatus};
use crate::core::utils::{open_url, string_to_theme};
use crate::gui::style;
use crate::IN_FILE_CONFIGURATION;

use iced::pure::widget::Text;
use iced::pure::{button, checkbox, column, container, pick_list, row, text, Element};
use iced::{Length, Space};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Settings {
    pub phone: Phone,
    pub theme: Theme,
    pub self_update_state: SelfUpdateState,
    pub list_update_state: UadListState,
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
            self_update_state: SelfUpdateState::default(),
            list_update_state: UadListState::default(),
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
    GetLatestRelease(Result<Option<Release>, ()>),
}

impl Settings {
    pub fn update(&mut self, phone: &CorePhone, msg: Message) {
        match msg {
            Message::GetLatestRelease(release) => {
                match release {
                    Ok(r) => {
                        self.self_update_state.status = SelfUpdateStatus::Done;
                        self.self_update_state.latest_release = r
                    }
                    Err(_) => self.self_update_state.status = SelfUpdateStatus::Failed,
                };
            }
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

    pub fn view(&mut self, phone: &CorePhone) -> Element<Message> {
        let general_category_text = text("General").size(25);

        let theme_picklist = pick_list(Theme::all(), Some(self.theme.clone()), Message::ApplyTheme)
            .style(style::PickList(self.theme.palette));

        let uad_category_text = text("Non-persistent settings").size(25);

        let expert_mode_descr =
            text("Most of unsafe packages are known to bootloop the device if removed.")
                .size(15)
                .color(self.theme.palette.normal.surface);

        let expert_mode_checkbox = checkbox(
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)",
            self.phone.expert_mode,
            Message::ExpertMode,
        )
        .style(style::SettingsCheckBox::Enabled(self.theme.palette));

        let multi_user_mode_descr =
            text("Disabling this setting will typically prevent affecting your work profile")
                .size(15)
                .color(self.theme.palette.normal.surface);

        let multi_user_mode_checkbox = checkbox(
            "Affect all the users of the phone (not only the selected user)",
            self.phone.multi_user_mode,
            Message::MultiUserMode,
        )
        .style(style::SettingsCheckBox::Enabled(self.theme.palette));

        let disable_color = if phone.android_sdk >= 23 {
            self.theme.palette.normal.surface
        } else {
            self.theme.palette.normal.primary
        };

        let disable_checkbox_style = if phone.android_sdk >= 23 {
            style::SettingsCheckBox::Enabled(self.theme.palette)
        } else {
            style::SettingsCheckBox::Disabled(self.theme.palette)
        };

        let disable_mode_descr =
            text("In some cases, it can be better to disable a package instead of uninstalling it")
                .size(15)
                .color(disable_color);

        let _unavailable_text = Text::new("[Unavailable before Android 8.0]")
            .size(16)
            .color(self.theme.palette.bright.error);

        let unavailable_btn = button(text("Unavailable").size(13))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki/FAQ#\
                    why-is-the-disable-mode-setting-not-available-for-my-device",
            )))
            .height(Length::Units(22))
            .style(style::UnavailableButton(self.theme.palette));

        // Disabling package without root isn't really possible before Android Oreo (8.0)
        // see https://github.com/0x192/universal-android-debloater/wiki/ADB-reference
        let disable_mode_checkbox = checkbox(
            "Clear and disable packages instead of uninstalling them",
            self.phone.disable_mode,
            Message::DisableMode,
        )
        .style(disable_checkbox_style);

        let disable_setting_row = if phone.android_sdk >= 23 {
            row()
                .width(Length::Fill)
                .push(disable_mode_checkbox)
                .push(Space::new(Length::Fill, Length::Shrink))
        } else {
            row()
                .width(Length::Fill)
                .push(disable_mode_checkbox)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(unavailable_btn)
        };

        let content = column()
            .width(Length::Fill)
            .spacing(10)
            .push(general_category_text)
            .push("Theme")
            .push(theme_picklist)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(uad_category_text)
            .push(expert_mode_checkbox)
            .push(expert_mode_descr)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(multi_user_mode_checkbox)
            .push(multi_user_mode_descr)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(disable_setting_row)
            .push(disable_mode_descr);

        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Content(self.theme.palette))
            .into()
    }
}
