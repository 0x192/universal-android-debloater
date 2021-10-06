use crate::gui::style;
use crate::core::theme::{Theme};
use crate::core::sync::get_android_sdk;
use iced::{Checkbox, Column, Container, Element, Length, Text, Space, 
    pick_list, PickList,
    };

#[derive(Debug, Clone)]
pub struct Settings {
    pub expert_mode: bool,
    pub disable_mode: bool,
    pub multi_user_mode: bool,
    pub theme: Theme,
    theme_picklist: pick_list::State<Theme>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            expert_mode: false,
            disable_mode: get_android_sdk() < 26,
            multi_user_mode: get_android_sdk() > 21,
            theme: Theme::lupin(),
            theme_picklist: pick_list::State::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ExpertMode(bool),
    DisableMode(bool),
    MultiUserMode(bool),
    ApplyTheme(Theme),
}

impl Settings {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ExpertMode(toggled) => {
                info!("Expert mode {}", if toggled {"enabled"} else {"disabled"});
                self.expert_mode = toggled;
            },
            Message::DisableMode(toggled) => {
                info!("Disable mode {}", if toggled {"enabled"} else {"disabled"});
                self.disable_mode = toggled;
            },
            Message::MultiUserMode(toggled) => {
                info!("Multi-user mode {}", if toggled {"enabled"} else {"disabled"});
                self.multi_user_mode = toggled;
            },
            Message::ApplyTheme(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let general_category_text = Text::new("General").size(25);

        let theme_picklist = PickList::new(
            &mut self.theme_picklist,
            Theme::all(),
            Some(self.theme.clone()),
            Message::ApplyTheme,
            )
            .style(style::PickList(self.theme.palette));

        let uad_category_text = Text::new("UAD").size(25);

        let expert_mode_descr = Text::new("Most of unsafe packages are known to bootloop the device if removed.")
            .size(15)
            .color(self.theme.palette.normal.surface);

        let expert_mode_checkbox = Checkbox::new(
            self.expert_mode, 
            "Allow to uninstall packages marked as \"unsafe\" (I KNOW WHAT I AM DOING)", 
            Message::ExpertMode
        );

        let disable_mode_descr = Text::new("Default mode on older phone (< Android 8.0) where uninstalled packages can't be restored.")
            .size(15)
            .color(self.theme.palette.normal.surface);

        let disable_mode_checkbox = Checkbox::new(
            self.disable_mode, 
            "Clear and disable packages instead of uninstalling them",
            Message::DisableMode
        );

        let multi_user_mode_descr = Text::new("Disabling this setting will typically prevent affecting your work profile")
            .size(15)
            .color(self.theme.palette.normal.surface);

        let multi_user_mode_checkbox = Checkbox::new(
            self.multi_user_mode, 
            "Affect all the users of the phone (not only the selected user)",
            Message::MultiUserMode
        );

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(general_category_text)
            .push(Text::new("Theme"))
            .push(theme_picklist)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(uad_category_text)
            .push(expert_mode_checkbox)
            .push(expert_mode_descr)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(disable_mode_checkbox)
            .push(disable_mode_descr)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(multi_user_mode_checkbox)
            .push(multi_user_mode_descr);

        Container::new(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Content(self.theme.palette))
            .into()
    }
}
