use crate::gui::style;

use iced::{Checkbox, Column, Container, Element, Length, Text};

#[derive(Debug, Clone)]
pub struct Settings {
    pub expert_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            expert_mode: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ExpertModeToogle(bool),
}

impl Settings {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ExpertModeToogle(toggled) => {
                info!("Expert mode {}", if toggled {"enabled"} else {"disabled"});
                self.expert_mode = toggled;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let about_text = Text::new("General");

        let checkbox = Checkbox::new(
            self.expert_mode, 
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)", 
            Message::ExpertModeToogle
        );

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