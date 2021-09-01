use crate::gui::style;
use crate::core::uad_lists::{ UadList, PackageState, Package, Removal };
use std::{collections::HashMap};

use crate::gui::views::settings::Settings;

use iced::{
    scrollable, Align, Column, Command, Container, Element, Space,
    Length, Row, Scrollable, Text, text_input, TextInput, Checkbox, //Svg,
    PickList, pick_list, Button, button, HorizontalAlignment, VerticalAlignment
};

use crate::core::sync::{ 
    list_all_system_packages, hashset_installed_system_packages, 
    uninstall_package, restore_package,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionPackage {
    pub name: String,
    pub state: PackageState,
}

#[derive(Default, Debug, Clone)]
pub struct List {
    ready: bool,
    pub settings: Settings,
    filtered_packages: Vec<PackageRow>,
    phone_packages: Vec<PackageRow>,
    selected_packages: Vec<SelectionPackage>,
    search_input: text_input::State,
    select_all_btn_state: button::State,
    apply_selection_btn_state: button::State,
    package_scrollable_state: scrollable::State,
    package_state_picklist: pick_list::State<PackageState>,
    list_picklist: pick_list::State<UadList>,
    removal_picklist: pick_list::State<Removal>,
    selected_package_state: Option<PackageState>,
    selected_removal: Option<Removal>,
    selected_list: Option<UadList>,
    pub input_value: String,
    description: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchInputChanged(String),
    LoadPackages(&'static HashMap<String, Package>),
    ListSelected(UadList),
    PackageStateSelected(PackageState),
    RemovalSelected(Removal),
    ApplyActionOnSelection,
    SelectAllPressed,
    List(usize, RowMessage),
    LoadSettings(Settings),
}


impl List {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadPackages(uad_lists) => {
                self.filtered_packages = Vec::new();
                self.selected_package_state = Some(PackageState::Installed);
                self.selected_list = Some(UadList::All);
                self.selected_removal = Some(Removal::Recommanded);

                let all_system_packages = list_all_system_packages(); // installed and uninstalled packages
                let installed_system_packages = hashset_installed_system_packages();
                let mut description;
                let mut uad_list;
                let mut state;
                let mut removal;
                let selected = false;

                for p_name in all_system_packages.lines() {
                    state = PackageState::Installed;
                    description = "[No description]";
                    uad_list = UadList::Unlisted;
                    removal = Removal::Unlisted;

                    if uad_lists.contains_key(p_name) {
                        description = uad_lists.get(p_name).unwrap().description.as_ref().unwrap();
                        uad_list = uad_lists.get(p_name).unwrap().list;
                        removal = uad_lists.get(p_name).unwrap().removal;
                    }

                    if !installed_system_packages.contains(p_name) {
                        state = PackageState::Uninstalled;
                    }

                    let package_row = PackageRow::new(
                        &p_name,
                        state,
                        &description,
                        uad_list,
                        removal,
                        selected,
                        self.settings.expert_mode,
                    );
                    self.filtered_packages.push(package_row)
                }
                self.filtered_packages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                self.phone_packages = self.filtered_packages.clone();

                Self::filter_package_lists(self);
                self.ready = true;
                Command::none()
            }
            Message::SearchInputChanged(letter) => {
                self.input_value = letter;
                Self::filter_package_lists(self);
                Command::none()
            }
            Message::ListSelected(list) => {

                self.selected_list = Some(list);
                Self::filter_package_lists(self);
                Command::none()
            }

            Message::PackageStateSelected(package_state) => {
                self.selected_package_state = Some(package_state);
                Self::filter_package_lists(self);
                Command::none()
            },
            Message::RemovalSelected(removal) => {
                self.selected_removal = Some(removal);
                Self::filter_package_lists(self);
                Command::none()
            }
            Message::List(i, row_message) => {
                self.filtered_packages[i].update(row_message.clone()).map(move |row_message| Message::List(i, row_message));
                
                match row_message {
                    RowMessage::UpdateSelection(_) => {
                        if self.filtered_packages[i].selected {
                            self.selected_packages.push(
                                SelectionPackage {
                                    name: self.filtered_packages[i].name.clone(),
                                    state: self.filtered_packages[i].state
                                }
                            );
                        } else {
                            let p_name = self.filtered_packages[i].name.clone();
                            self.selected_packages.drain_filter(|p| p.name == p_name.as_str());
                        }
                    },
                    RowMessage::RemovePressed(package)|
                    RowMessage::RestorePressed(package) => {
                        for p in &mut self.phone_packages {
                            if package.name == p.name {
                                if p.state == PackageState::Installed { 
                                    p.state = PackageState::Uninstalled;
                                 } else { 
                                    p.state = PackageState::Installed;
                                }
                                break
                            }
                        }
                        let p_name = self.filtered_packages[i].name.clone();
                        self.selected_packages.drain_filter(|p| p.name == p_name.as_str());
                        Self::filter_package_lists(self);
                    }
                    RowMessage::PackagePressed => self.description = self.filtered_packages[i].clone().description,
                    _ => {}
                }
                Command::none()
            },
            Message::ApplyActionOnSelection => {
                for p in &self.selected_packages {
                    match p.state {
                        PackageState::Installed => uninstall_package(p.name.clone()),
                        PackageState::Uninstalled => restore_package(p.name.clone()),
                        _ => "[DEBUG] ApplySelectionAction: Unknown package state".to_string(), // TODO: this is an error
                    };

                    for phone_p in &mut self.phone_packages {
                        if p.name == phone_p.name {
                            phone_p.state = match phone_p.state {
                                PackageState::Installed => PackageState::Uninstalled,
                                PackageState::Uninstalled => PackageState::Installed, 
                                _ => PackageState::Installed, // TODO: this is an error
                            };
                            break
                        }
                    }
                }
                Self::filter_package_lists(self);
                Command::none()
            },
            Message::SelectAllPressed => {
                let mut package;
                for p in &mut self.filtered_packages {
                    p.selected = true;
                    package = SelectionPackage { name: p.name.clone(), state: p.state };
                    if !self.selected_packages.contains(&package) {
                        self.selected_packages.push(package);
                    }
                }
                Command::none()
            },
            Message::LoadSettings(settings) => {
                self.settings = settings;
                Command::none()
            },
        }


    }
    pub fn view(&mut self) -> Element<Message> {
        if self.ready {
            let search_packages = TextInput::new(
                &mut self.search_input,
                "Search packages...",
                &mut self.input_value,
                Message::SearchInputChanged,
            )
            .padding(5);

            // let package_amount = Text::new(format!("{} packages found", packages.len()));

            let divider = Space::new(Length::Fill, Length::Shrink);

            let list_picklist = PickList::new(
                        &mut self.list_picklist,
                        &UadList::ALL[..],
                        self.selected_list,
                        Message::ListSelected,
                    );

            let package_state_picklist = PickList::new(
                        &mut self.package_state_picklist,
                        &PackageState::ALL[..],
                        self.selected_package_state,
                        Message::PackageStateSelected,
                    );

            let removal_picklist = PickList::new(
                        &mut self.removal_picklist,
                        &Removal::ALL[..],
                        self.selected_removal,
                        Message::RemovalSelected,
                    );

            let control_panel = Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(10)
                .push(search_packages)
                .push(divider)
                .push(removal_picklist)
                .push(package_state_picklist)
                .push(list_picklist);

            let package_name = Text::new("Package").width(Length::FillPortion(6));
            let package_state = Text::new("State").width(Length::FillPortion(3));
            let package_action = Text::new("").width(Length::FillPortion(1));

            let package_panel = Row::new()
                .width(Length::Fill)
                .push(package_name)
                .push(package_state)
                .push(package_action)
                .push(Space::with_width(Length::Units(15)));

            let packages = self.filtered_packages
                .iter_mut()
                .enumerate()
                .fold(Column::new().spacing(6), |col, (i, p)| {
                    col.push(p.view().map(move |msg| Message::List(i, msg)))
                });

            let packages_scrollable = Scrollable::new(&mut self.package_scrollable_state)
                .push(packages)
                .spacing(2)
                .align_items(Align::Start)
                .height(Length::FillPortion(6))
                .style(style::Scrollable);

            // let mut packages_v: Vec<&str> = self.packages.lines().collect();
            let description_panel = Container::new(
                Row::new()
                .align_items(Align::Center)
                .push(Text::new(&self.description))
            )
            .style(style::Description)
            .padding(10)
            .height(Length::FillPortion(2))
            .width(Length::Fill);


            let apply_selection_btn = Button::new(
                &mut self.apply_selection_btn_state, 
                Row::new()
                    .align_items(Align::Center)
                    .push(Text::new(format!("{}{}{}", "Debloat/Restore selection (", self.selected_packages.len(), ")")))
                )
                .on_press(Message::ApplyActionOnSelection)
                .padding(5)
                .height(Length::Units(40))
                .style(style::PrimaryButton::Enabled);

            let select_all_btn = Button::new(
                &mut self.select_all_btn_state, 
                Row::new()
                    .align_items(Align::Center)
                    .push(Text::new("Select all"))
                )
                .on_press(Message::SelectAllPressed)
                .padding(5)
                .height(Length::Units(40))
                .style(style::PrimaryButton::Enabled);

            let action_row = Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .height(Length::FillPortion(1))
                .push(select_all_btn)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(apply_selection_btn);


            let content = Column::new()
                .width(Length::Fill)
                .spacing(10)
                .align_items(Align::Center)
                .push(control_panel)
                .push(package_panel)
                .push(packages_scrollable)
                .push(description_panel)
                .push(action_row);

            Container::new(content)
                .height(Length::Fill)
                .padding(10)
                .style(style::Content)
                .into()
        } else {
            loading_data()
        }
    }

    fn filter_package_lists(&mut self) {

        let list_filter: UadList = self.selected_list.unwrap();
        let package_filter: PackageState = self.selected_package_state.unwrap();
        let removal_filter: Removal = self.selected_removal.unwrap();

        let mut filtered_packages: Vec<PackageRow> = self.phone_packages
            .iter()
            .filter(
                |p|
                (list_filter == UadList::All || p.uad_list == list_filter) &&
                (package_filter == PackageState::All || p.state == package_filter) &&
                (removal_filter == Removal::All || p.removal == removal_filter) &&
                (self.input_value.is_empty() || p.name.contains(&self.input_value))
            )
            .cloned()
            .collect();
            
        filtered_packages.sort_by(|a, b| a.name.cmp(&b.name));

        for p in &mut filtered_packages {
            if self.selected_packages.iter().any(|s_p| s_p.name == p.name) {
                p.selected = true;
            }
        }
        self.filtered_packages = filtered_packages;
    }
}

#[derive(Clone, Debug)]
pub struct PackageRow {
    pub name: String,
    pub state: PackageState,
    pub description: String,
    pub uad_list: UadList,
    pub removal: Removal,
    package_btn_state: button::State,
    action_btn_state: button::State,
    selected: bool,
    expert_mode: bool,
}

#[derive(Clone, Debug)]
pub enum RowMessage {
    PackagePressed,
    RemovePressed(PackageRow),
    RestorePressed(PackageRow),
    UpdateSelection(bool),
    NoEvent(bool),
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

    pub fn update(&mut self, message: RowMessage) -> Command<RowMessage> {
        match message {
            RowMessage::RemovePressed(package) => {
                uninstall_package(package.name);
                self.state = PackageState::Uninstalled;
                self.selected = false;
                Command::none()
            }
            RowMessage::RestorePressed(package) => {
                restore_package(package.name);
                self.state = PackageState::Installed;
                self.selected = false;
                Command::none()
            },
            RowMessage::UpdateSelection(toogled) => {
                self.selected = toogled;
                Command::none()
            },
            RowMessage::PackagePressed => Command::none(),
            RowMessage::NoEvent(_) => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<RowMessage> {
        let package = self.clone();
        //let trash_svg = format!("{}/ressources/assets/trash.svg", env!("CARGO_MANIFEST_DIR"));
        //let restore_svg = format!("{}/ressources/assets/rotate.svg", env!("CARGO_MANIFEST_DIR"));
        let button_style;
        let action_text;
        let action_message;
        let action_btn;
        let selection_checkbox;

        if self.state == PackageState::Installed {
            action_text = "Uninstall";
            action_message = RowMessage::RemovePressed(package);
            button_style = style::PackageButton::Uninstall;
        } else {
            action_text = "Restore";
            action_message = RowMessage::RestorePressed(package);
            button_style = style::PackageButton::Restore;
        }

        if self.expert_mode || self.removal != Removal::Unsafe {
            selection_checkbox = Checkbox::new(self.selected, "", RowMessage::UpdateSelection)
                .style(style::SelectionCheckBox::Enabled);

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(action_message);

        } else {
            selection_checkbox = Checkbox::new(self.selected, "", RowMessage::NoEvent)
                .style(style::SelectionCheckBox::Disabled);

            action_btn = Button::new(
                &mut self.action_btn_state,
                Text::new(action_text).horizontal_alignment(HorizontalAlignment::Center),
            );
        }

        Row::new()
            .push(Button::new(
                &mut self.package_btn_state,
                Row::new()
                    .align_items(Align::Center)
                    .push(selection_checkbox)
                    .push(Text::new(&self.name).width(Length::FillPortion(6)))
                    .push(Text::new(&self.state.to_string()).width(Length::FillPortion(3)))
                    .push(action_btn.width(Length::FillPortion(1))
                                    .style(button_style)
                    )
                )
                .style(style::PackageRow)
                .width(Length::Fill)
                .on_press(RowMessage::PackagePressed)

            )
            .push(Space::with_width(Length::Units(15)))
            .align_items(Align::Center)
            .into()
    }
}

fn loading_data<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Pulling packages from the phone. Please wait...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .size(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .style(style::Content)
    .into()
}