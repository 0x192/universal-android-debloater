use crate::core::utils::{last_modified_date, open_url};
use crate::gui::style;
use crate::gui::views::settings::Settings;
use crate::CACHE_DIR;
use iced::pure::{button, column, container, row, text, Element};
use iced::{Alignment, Length, Space};
use std::path::PathBuf;

#[cfg(feature = "self-update")]
use crate::core::update::SelfUpdateStatus;

#[derive(Default, Debug, Clone)]
pub struct About {}

#[derive(Debug, Clone)]
pub enum Message {
    UrlPressed(PathBuf),
    UpdateUadLists,
    DoSelfUpdate,
}

impl About {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::UrlPressed(url) => {
                open_url(url);
            }
            Message::UpdateUadLists => {
                // Action is taken by UadGui update()
            }
            Message::DoSelfUpdate => {
                // Action is taken by UadGui update()
            }
        }
    }
    pub fn view(&mut self, settings: &Settings) -> Element<Message> {
        let about_text = text(
            "Universal Android Debloater (UAD) is a Free and Open-Source community project aiming at simplifying \
            the removal of pre-installed apps on any Android device.",
        );

        let descr_container = container(about_text)
            .width(Length::Fill)
            .padding(25)
            .style(style::NavigationContainer(settings.theme.palette));

        let date = last_modified_date(CACHE_DIR.join("uad_lists.json"));
        let uad_list_text =
            text(format!("Documentation: v{}", date.format("%Y%m%d"))).width(Length::Units(250));
        let last_update_text = text(settings.list_update_state.to_string())
            .color(settings.theme.palette.normal.surface);
        let uad_lists_btn = button("Update")
            .on_press(Message::UpdateUadLists)
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        #[cfg(feature = "self-update")]
        let self_update_btn = button("Update")
            .on_press(Message::DoSelfUpdate)
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        #[cfg(feature = "self-update")]
        let uad_version_text =
            text(format!("UAD version: v{}", env!("CARGO_PKG_VERSION"))).width(Length::Units(250));

        #[cfg(feature = "self-update")]
        let self_update_text = match &settings.self_update_state.latest_release {
            Some(r) => {
                if settings.self_update_state.status == SelfUpdateStatus::Updating {
                    settings.self_update_state.status.to_string()
                } else {
                    format!("(v{} available)", r.tag_name)
                }
            }
            None => {
                if settings.self_update_state.status == SelfUpdateStatus::Done {
                    "(No update available)".to_string()
                } else {
                    settings.self_update_state.status.to_string()
                }
            }
        };

        #[cfg(feature = "self-update")]
        let last_self_update_text =
            text(self_update_text).color(settings.theme.palette.normal.surface);

        #[cfg(feature = "self-update")]
        let self_update_row = row()
            .align_items(Alignment::Center)
            .spacing(10)
            .width(iced::Length::Units(550))
            .push(uad_version_text)
            .push(self_update_btn)
            .push(last_self_update_text);

        let uad_list_row = row()
            .align_items(Alignment::Center)
            .spacing(10)
            .width(iced::Length::Units(550))
            .push(uad_list_text)
            .push(uad_lists_btn)
            .push(last_update_text);

        #[cfg(feature = "self-update")]
        let update_column = column()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(uad_list_row)
            .push(self_update_row);

        #[cfg(not(feature = "self-update"))]
        let update_column = column()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(uad_list_row);

        let update_container = container(update_column)
            .width(Length::Fill)
            .center_x()
            .padding(10)
            .style(style::NavigationContainer(settings.theme.palette));

        let website_btn = button("Github page")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let issue_btn = button("Have an issue?")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/issues",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let log_btn = button("Locate the logfiles")
            .on_press(Message::UrlPressed(CACHE_DIR.to_path_buf()))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let wiki_btn = button("Wiki")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let row = row()
            .spacing(20)
            .push(website_btn)
            .push(wiki_btn)
            .push(issue_btn)
            .push(log_btn);

        let content = column()
            .width(Length::Fill)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(descr_container)
            .push(update_container)
            .push(row);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(style::Content(settings.theme.palette))
            .into()
    }
}
