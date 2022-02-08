use crate::core::update::SelfUpdateStatus;
use crate::core::utils::{format_diff_time_from_now, last_modified_date, open_url};
use crate::gui::style;
use crate::gui::views::settings::Settings;
use crate::CACHE_DIR;
use iced::{button, Alignment, Button, Column, Container, Element, Length, Row, Space, Text};
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct About {
    website_btn: button::State,
    issue_btn: button::State,
    wiki_btn: button::State,
    log_btn: button::State,
    lists_btn: button::State,
    self_update_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlPressed(PathBuf),
    UpdateUadLists,
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
        }
    }
    pub fn view(&mut self, settings: &Settings) -> Element<Message> {
        let about_text = Text::new(
            "Universal Android Debloater (UAD) is a Free and Open-Source community project aiming at simplifying \
            the removal of pre-installed apps on any Android device.",
        );

        let descr_container = Container::new(about_text)
            .width(Length::Fill)
            .padding(25)
            .style(style::NavigationContainer(settings.theme.palette));

        let date = last_modified_date(CACHE_DIR.join("uad_lists.json"));
        let uad_list_text = Text::new(format!("Documentation: v{}", date.format("%Y%m%d")))
            .width(Length::Units(250));
        let last_update_text = Text::new(format!("(last was {})", format_diff_time_from_now(date)))
            .color(settings.theme.palette.normal.surface);
        let uad_lists_btn = Button::new(&mut self.lists_btn, Text::new("Update"))
            .on_press(Message::UpdateUadLists)
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let self_update_btn = Button::new(&mut self.self_update_btn, Text::new("Update"))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/releases",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let uad_version_text = Text::new(format!("UAD version: v{}", env!("CARGO_PKG_VERSION")))
            .width(Length::Units(250));

        let self_update_text = match &settings.self_update_state.latest_release {
            Some(r) => format!("(v{} available)", r.tag_name),
            None => {
                if settings.self_update_state.status == SelfUpdateStatus::Done {
                    "(No update available)".to_string()
                } else {
                    settings.self_update_state.status.to_string()
                }
            }
        };

        let last_self_update_text =
            Text::new(self_update_text).color(settings.theme.palette.normal.surface);

        let uad_list_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .width(iced::Length::Units(550))
            .push(uad_list_text)
            .push(uad_lists_btn)
            .push(last_update_text);

        let self_update_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .width(iced::Length::Units(550))
            .push(uad_version_text)
            .push(self_update_btn)
            .push(last_self_update_text);

        let update_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(uad_list_row)
            .push(self_update_row);

        let update_container = Container::new(update_column)
            .width(Length::Fill)
            .center_x()
            .padding(10)
            .style(style::NavigationContainer(settings.theme.palette));

        let website_btn = Button::new(&mut self.website_btn, Text::new("Github page"))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let issue_btn = Button::new(&mut self.issue_btn, Text::new("Have an issue?"))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/issues",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let log_btn = Button::new(&mut self.log_btn, Text::new("Locate the logfiles"))
            .on_press(Message::UrlPressed(CACHE_DIR.to_path_buf()))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let wiki_btn = Button::new(&mut self.wiki_btn, Text::new("Wiki"))
            .on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/0x192/universal-android-debloater/wiki",
            )))
            .padding(5)
            .style(style::PrimaryButton(settings.theme.palette));

        let row = Row::new()
            .spacing(20)
            .push(website_btn)
            .push(wiki_btn)
            .push(issue_btn)
            .push(log_btn);

        let content = Column::new()
            .width(Length::Fill)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(descr_container)
            .push(update_container)
            .push(row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(style::Content(settings.theme.palette))
            .into()
    }
}
