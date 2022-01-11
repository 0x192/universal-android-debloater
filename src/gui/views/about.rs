use crate::core::utils::open_url;
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
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlPressed(PathBuf),
}

impl About {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::UrlPressed(url) => {
                open_url(url);
            }
        }
    }
    pub fn view(&mut self, settings: &Settings) -> Element<Message> {
        let about_text = Text::new(
            "Universal Android Debloater (UAD) is a Free and Open-Source community project aiming at simplifying \
            the removal of pre-installed apps on any Android device.",
        );

        let container = Container::new(about_text)
            .width(Length::Fill)
            .padding(25)
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
            .push(container)
            .push(row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(style::Content(settings.theme.palette))
            .into()
    }
}
