use crate::gui::style;
use crate::gui::Message;
use crate::gui::views::settings::Settings;
use iced::{Alignment, Column, Container, Element, Length, Text};

#[derive(Default, Debug, Clone)]
pub struct About {}

impl About {
    pub fn view(&self, settings: &Settings) -> Element<Message> {
        let about_text = Text::new(
            "Welcome to UadGui, a completely free project that helps you remove pre-installed apps on your Android device.",
        );

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .align_items(Alignment::Center)
            .push(about_text);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(style::Content(settings.theme.palette))
            .into()
    }
}