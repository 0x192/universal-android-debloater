use crate::core::uad_lists::{ UadList, PackageState, Removal };
use crate::gui::style;

use iced::{
    Alignment, alignment, Command, Element, Space, Length, Row, Text, Button, 
    button, Checkbox,
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
    expert_mode: bool,
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
        expert_mode: bool,

    ) -> Self {
        Self {
            name: name.to_string(),
            state: state,
            description: description.to_string(),
            uad_list: uad_list,
            removal: removal,
            package_btn_state: button::State::default(),
            action_btn_state: button::State::default(),
            selected: selected,
            expert_mode: expert_mode,
        }
    }

    pub fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        //let trash_svg = format!("{}/ressources/assets/trash.svg", env!("CARGO_MANIFEST_DIR"));
        //let restore_svg = format!("{}/ressources/assets/rotate.svg", env!("CARGO_MANIFEST_DIR"));
        let button_style;
        let action_text;
        let action_btn;
        let selection_checkbox;

        if self.state == PackageState::Installed {
            action_text = "Uninstall";
            button_style = style::PackageButton::Uninstall;
        } else {
            action_text = "Restore";
            button_style = style::PackageButton::Restore;
        }

        if self.expert_mode || self.removal != Removal::Unsafe {
            selection_checkbox = Checkbox::new(self.selected, "", Message::ToggleSelection)
                .style(style::SelectionCheckBox::Enabled);

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(Message::ActionPressed);

        } else {
            selection_checkbox = Checkbox::new(self.selected, "", Message::ToggleSelection)
                .style(style::SelectionCheckBox::Disabled);

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(alignment::Horizontal::Center),
            );
        }

        Row::new()
            .push(Button::new(
                &mut self.package_btn_state,
                Row::new()
                    .align_items(Alignment::Center)
                    .push(selection_checkbox)
                    .push(Text::new(&self.name).width(Length::FillPortion(8)))
                    .push(action_btn.width(Length::FillPortion(1))
                                    .style(button_style)
                    )
                )
                .style(style::PackageRow)
                .width(Length::Fill)
                .on_press(Message::PackagePressed)

            )
            .push(Space::with_width(Length::Units(15)))
            .align_items(Alignment::Center)
            .into()
    }
}