use crate::gui::style;

use iced::{Checkbox, Column, Container, Element, Length, Text, Space};

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub expert_mode: bool,
    pub disable_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            expert_mode: false,
            disable_mode: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ExpertModeToogle(bool),
    DisableModeToogle(bool),
}

impl Settings {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ExpertModeToogle(toggled) => {
                info!("Expert mode {}", if toggled {"enabled"} else {"disabled"});
                self.expert_mode = toggled;
            },
            Message::DisableModeToogle(toggled) => {
                info!("Disable mode {}", if toggled {"enabled"} else {"disabled"});
                self.disable_mode = toggled;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let about_text = Text::new("General");

        let expert_mode_descr = Text::new("Most of unsafe packages are known to bootloop the device if removed.")
            .size(15)
            .color(style::GREY_SMALL_SETTINGS_COLOR);

        let expert_mode_checkbox = Checkbox::new(
            self.expert_mode, 
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)", 
            Message::ExpertModeToogle
        );

        let disable_mode_descr = Text::new("Default mode on older phone (< Android 8.0) where uninstalled packages can't be restored.")
            .size(15)
            .color(style::GREY_SMALL_SETTINGS_COLOR);

        let disable_mode_checkbox = Checkbox::new(
            self.disable_mode, 
            "Clear and disable packages instead of uninstalling them",
            Message::DisableModeToogle
        );

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(about_text)
            .push(expert_mode_checkbox)
            .push(expert_mode_descr)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(disable_mode_checkbox)
            .push(disable_mode_descr);

        Container::new(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Content)
            .into()
    }
}