use crate::gui::style;

use iced::{Checkbox, Column, Container, Element, Length, Text};

#[derive(Debug, Clone)]
pub struct Settings {
    description: String,
    backup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            description: "Enable Backup".to_string(),
            backup: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackupTriggered(bool),
}

impl Settings {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::BackupTriggered(toggled) => {
                self.backup = toggled;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let about_text = Text::new("General");

        let checkbox = Checkbox::new(self.backup, &self.description, Message::BackupTriggered);

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(about_text)
            .push(checkbox);

        Container::new(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Content)
            .into()
    }
}