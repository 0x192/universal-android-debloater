use crate::gui::style;
use crate::core::uad_lists::{ UadLists, PackageState, Package };
use std::{collections::HashMap};

use iced::{
    scrollable, Align, Column, Command, Container, Element, Space,
    Length, Row, Scrollable, Text, text_input, TextInput, Svg,
    PickList, pick_list, Button, button, HorizontalAlignment,
};

use crate::core::sync::{ list_phone_packages, uninstall_package };

#[derive(Default, Debug, Clone)]
pub struct List {
    p_row: Vec<PackageRow>,
    packages: String,
    input: text_input::State,
    package_scrollable_state: scrollable::State,
    package_state_picklist: pick_list::State<PackageState>,
    list_picklist: pick_list::State<UadLists>,
    selected_package_state: Option<PackageState>,
    selected_list: Option<UadLists>,
    pub input_value: String,
}

/*impl Default for List {
    fn default() -> Self {
        List { ..List::default() }
    }
}*/


#[derive(Debug, Clone)]
pub enum Message {
    ListInputChanged(String),
    LoadPackages(&'static HashMap<String, Package>),
    ListSelected(UadLists),
    PackageStateSelected(PackageState),
    List(usize, RowMessage),
    NoEvent,
}


impl List {
    pub fn update(&mut self, message: Message) -> Command<Message> {
       match message {
            Message::ListInputChanged(_letter) => {
                Command::none()
            },

            Message::LoadPackages(uad_lists) => {
                self.packages = list_phone_packages();
                self.p_row = Vec::new();
                let mut description = "";

                for p_name in self.packages.lines() {

                    if uad_lists.contains_key(p_name) {
                        description = uad_lists.get(p_name).unwrap().description.as_ref().unwrap();
                    } else {
                        description = "No description";
                    }

                    let package_row = PackageRow::new(
                        &p_name,
                        "Installed",
                        &description,
                    );
                    self.p_row.push(package_row)
                }
                self.p_row.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

                Command::none()
            }

            Message::ListSelected(list) => {
                self.selected_list = Some(list);
                Command::none()
            }

            Message::PackageStateSelected(package_state) => {
                self.selected_package_state = Some(package_state);
                Command::none()
            },
            Message::List(i, row_message) => self.p_row[i]
                .update(row_message)
                .map(move |row_message| Message::List(i, row_message)),
            Message::NoEvent => Command::none(),
        }
    }
    pub fn view(&mut self) -> Element<Message> {

                let search_packages = TextInput::new(
                    &mut self.input,
                    "Search packages...",
                    &mut self.input_value,
                    Message::ListInputChanged,
                )
                .padding(5);

                // let package_amount = Text::new(format!("{} packages found", packages.len()));

                let divider = Space::new(Length::Fill, Length::Shrink);

                let list_picklist = PickList::new(
                            &mut self.list_picklist,
                            &UadLists::ALL[..],
                            self.selected_list,
                            Message::ListSelected,
                        );

                let package_state_picklist = PickList::new(
                            &mut self.package_state_picklist,
                            &PackageState::ALL[..],
                            self.selected_package_state,
                            Message::PackageStateSelected,
                        );

                let control_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(search_packages)
                    .push(divider)
                    .push(package_state_picklist)
                    .push(list_picklist);

                let package_name = Text::new("Package").width(Length::FillPortion(6));
                let package_state = Text::new("State").width(Length::FillPortion(3));
                let advice = Text::new("Advice").width(Length::FillPortion(3));

                let package_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(package_name)
                    .push(package_state)
                    .push(advice);
                    
                // let mut packages_v: Vec<&str> = self.packages.lines().collect();

/*                let description_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(package_name)*/

                let test = self.p_row
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(5), |col, (i, p)| {
                        col.push(p.view().map(move |msg| Message::List(i, msg)))
                    });

                let packages_scrollable = Scrollable::new(&mut self.package_scrollable_state)
                    .push(test)
                    .spacing(5)
                    .align_items(Align::Center)
                    .style(style::Scrollable);

                let content = Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(control_panel)
                    .push(package_panel)
                    .push(packages_scrollable);

                Container::new(content)
                    .height(Length::Fill)
                    .padding(10)
                    .style(style::Content)
                    .into()
    }
}

#[derive(Clone, Debug)]
pub struct PackageRow {
    pub name: String,
    pub state: String,
    pub description: String,

    remove_restore_btn_state: button::State,
}

#[derive(Clone, Debug)]
pub enum RowMessage {
    NoEvent,
    RemovePressed(PackageRow),
    RestorePressed(PackageRow),
    Uninstall(String)
}

impl PackageRow {
    pub fn new(
        name: &str,
        state: &str,
        description: &str,

    ) -> Self {
        Self {
            name: name.to_string(),
            state: "Installed".to_string(),
            description: description.to_string(),
            remove_restore_btn_state: button::State::default(),
        }
    }

    pub fn update(&mut self, message: RowMessage) -> Command<RowMessage> {
        match message {
            RowMessage::RemovePressed(package) => Command::perform(
                uninstall_package(package.name),
                RowMessage::Uninstall
            ),
            RowMessage::RestorePressed(package) => Command::none(),
            RowMessage::NoEvent => Command::none(),
            RowMessage::Uninstall(_) => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<RowMessage> {
        let package = self.clone();
        let add_svg_path = format!("{}/assets/trash.svg", env!("CARGO_MANIFEST_DIR"));

        let content = Row::new()
            .align_items(Align::Center)
            .push(Text::new(&self.name).width(Length::FillPortion(6)))
            .push(Text::new(&self.state).width(Length::FillPortion(3)))
            .push(Text::new(&self.description).width(Length::FillPortion(3)))
            .push(if self.state == "Installed" {
                                        Button::new(
                                            &mut self.remove_restore_btn_state,
                                            Svg::from_path(add_svg_path)
                                                .width(Length::Fill)
                                                .height(Length::Fill),
                                        )
                                        .on_press(RowMessage::RemovePressed(package))
                                        .style(style::PrimaryButton::Enabled)
                                    } else {
                                        Button::new(
                                            &mut self.remove_restore_btn_state,
                                            Text::new("Restore")
                                                .width(Length::Fill)
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .on_press(RowMessage::RestorePressed(package))
                                        .style(style::PrimaryButton::Enabled)
                                    });

        let p_row = Container::new(content)
            .padding(10)
            .style(style::PackageRow);

        Column::new().push(p_row).into()


    }
}