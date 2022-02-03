use crate::core::sync::Phone;
use crate::core::uad_lists::{PackageState, Removal, UadList};

use crate::gui::style;
use crate::gui::views::settings::Settings;

use iced::{
    alignment, button, Alignment, Button, Checkbox, Command, Element, Length, Row, Space, Text,
};

#[derive(Clone, Debug)]
pub struct PackageRow {
    pub name: String,
    pub state: PackageState,
    pub description: String,
    pub uad_list: UadList,
    pub removal: Removal,
    package_btn_state: button::State,
    action_btn_state: button::State,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    PackagePressed,
    ActionPressed,
    ToggleSelection(bool),
}

impl PackageRow {
    pub fn new(
        name: &str,
        state: PackageState,
        description: &str,
        uad_list: UadList,
        removal: Removal,
        selected: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            state,
            description: description.to_string(),
            uad_list,
            removal,
            package_btn_state: button::State::default(),
            action_btn_state: button::State::default(),
            selected,
        }
    }

    pub fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self, settings: &Settings, _phone: &Phone) -> Element<Message> {
        //let trash_svg = format!("{}/resources/assets/trash.svg", env!("CARGO_MANIFEST_DIR"));
        //let restore_svg = format!("{}/resources/assets/rotate.svg", env!("CARGO_MANIFEST_DIR"));
        let button_style;
        let action_text;
        let action_btn;
        let selection_checkbox;

        match self.state {
            PackageState::Enabled => {
                action_text = if settings.phone.disable_mode {
                    "Disable"
                } else {
                    "Uninstall"
                };
                button_style = style::PackageButton::Uninstall(settings.theme.palette);
            }
            PackageState::Disabled => {
                action_text = "Enable";
                button_style = style::PackageButton::Restore(settings.theme.palette);
            }
            PackageState::Uninstalled => {
                action_text = "Restore";
                button_style = style::PackageButton::Restore(settings.theme.palette);
            }
            PackageState::All => {
                action_text = "Error";
                button_style = style::PackageButton::Restore(settings.theme.palette);
                warn!("Incredible! Something impossible happenned!");
            }
        }
        // Disable any removal action for unsafe packages if expert_mode is disabled
        if self.removal != Removal::Unsafe
            || self.state != PackageState::Enabled
            || settings.phone.expert_mode
        {
            selection_checkbox = Checkbox::new(self.selected, "", Message::ToggleSelection)
                .style(style::SelectionCheckBox::Enabled(settings.theme.palette));

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(alignment::Horizontal::Center).width(Length::Units(100)),
            )
            .on_press(Message::ActionPressed);
        } else {
            selection_checkbox = Checkbox::new(self.selected, "", Message::ToggleSelection)
                .style(style::SelectionCheckBox::Disabled(settings.theme.palette));

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(alignment::Horizontal::Center).width(Length::Units(100)),
            );
        }

        Row::new()
            .push(
                Button::new(
                    &mut self.package_btn_state,
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(selection_checkbox)
                        .push(Text::new(&self.name).width(Length::FillPortion(8)))
                        .push(action_btn.style(button_style))
                )
                .padding(8)
                .style(style::PackageRow(settings.theme.palette))
                .width(Length::Fill)
                .on_press(Message::PackagePressed),
            )
            .push(Space::with_width(Length::Units(15)))
            .align_items(Alignment::Center)
            .into()
    }
}
