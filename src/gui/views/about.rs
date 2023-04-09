use crate::core::theme::Theme;
use crate::core::utils::{last_modified_date, open_url};
use crate::gui::{style, UpdateState};
use crate::CACHE_DIR;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length, Renderer};
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
        if let Message::UrlPressed(url) = msg {
            open_url(url);
        }
        // other events are handled by UadGui update()
    }
    pub fn view(&self, update_state: &UpdateState) -> Element<Message, Renderer<Theme>> {
        let about_text = text(
            "Universal Android Debloater (UAD) is a Free and Open-Source community project aiming at simplifying \
            the removal of pre-installed apps on any Android device.",
        );

        let descr_container = container(about_text)
            .width(Length::Fill)
            .padding(25)
            .style(style::Container::Frame);

        let date = last_modified_date(CACHE_DIR.join("uad_lists.json"));
        let uad_list_text = text(format!("Documentation: v{}", date.format("%Y%m%d"))).width(250);
        let last_update_text = text(update_state.uad_list.to_string());
        let uad_lists_btn = button("Update")
            .on_press(Message::UpdateUadLists)
            .padding(5)
            .style(style::Button::Primary);

        #[cfg(feature = "self-update")]
        let self_update_btn = button("Update")
            .on_press(Message::DoSelfUpdate)
            .padding(5)
            .style(style::Button::Primary);

        #[cfg(feature = "self-update")]
        let uad_version_text =
            text(format!("UAD version: v{}", env!("CARGO_PKG_VERSION"))).width(250);

        #[cfg(feature = "self-update")]
        #[rustfmt::skip]
        let self_update_text = update_state.self_update.latest_release.as_ref().map_or_else(||
            if update_state.self_update.status == SelfUpdateStatus::Done {
                "(No update available)".to_string()
            } else {
                update_state.self_update.status.to_string()
            }, |r| if update_state.self_update.status == SelfUpdateStatus::Updating {
                update_state.self_update.status.to_string()
            } else {
                format!("(v{} available)", r.tag_name)
            });

        #[cfg(feature = "self-update")]
        let last_self_update_text = text(self_update_text).style(style::Text::Default);

        #[cfg(feature = "self-update")]
        let self_update_row = row![uad_version_text, self_update_btn, last_self_update_text,]
            .align_items(Alignment::Center)
            .spacing(10)
            .width(550);

        let uad_list_row = row![uad_list_text, uad_lists_btn, last_update_text,]
            .align_items(Alignment::Center)
            .spacing(10)
            .width(550);

        #[cfg(feature = "self-update")]
        let update_column = column![uad_list_row, self_update_row]
            .align_items(Alignment::Center)
            .spacing(10);

        #[cfg(not(feature = "self-update"))]
        let update_column = column![uad_list_row]
            .align_items(Alignment::Center)
            .spacing(10);

        let update_container = container(update_column)
            .width(Length::Fill)
            .center_x()
            .padding(10)
            .style(style::Container::Frame);

        let website_btn = button("Github page")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater",
            )))
            .padding(5)
            .style(style::Button::Primary);

        let issue_btn = button("Have an issue?")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/issues",
            )))
            .padding(5)
            .style(style::Button::Primary);

        let log_btn = button("Locate the logfiles")
            .on_press(Message::UrlPressed(CACHE_DIR.to_path_buf()))
            .padding(5)
            .style(style::Button::Primary);

        let wiki_btn = button("Wiki")
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki",
            )))
            .padding(5)
            .style(style::Button::Primary);

        let row = row![website_btn, wiki_btn, issue_btn, log_btn,].spacing(20);

        let content = column![
            Space::new(Length::Fill, Length::Shrink),
            descr_container,
            update_container,
            row,
        ]
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
