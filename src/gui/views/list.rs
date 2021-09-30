use crate::gui::style;
use crate::core::uad_lists::{ UadList, PackageState, Package, Removal, Opposite};
use crate::core::utils::{import_selection, export_selection, update_selection_count};
use std::{collections::HashMap};

use crate::gui::views::settings::Settings;
use crate::gui::widgets::package_row::{ PackageRow, Message as RowMessage };
use iced::{
    scrollable, Alignment, alignment, Column, Container, Element, Space,
    Length, Row, Scrollable, Text, text_input, TextInput, Command,//Svg,
    PickList, pick_list, Button, button, 
};

use crate::core::sync::{ 
    list_all_system_packages, hashset_system_packages, 
    Phone, action_handler,
};

#[derive(Debug, Clone)]
pub struct Selection {
    pub uninstalled: u16,
    pub enabled: u16,
    pub disabled: u16,
    pub selected_packages: Vec<usize>, // phone_packages indexes (= what you've selected)
}

#[derive(Debug, Clone)]
pub enum Action {
    Remove,
    Restore,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            uninstalled: 0,
            enabled: 0,
            disabled: 0,
            selected_packages: Vec::new(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct List {
    ready: bool,
    phone_packages: Vec<PackageRow>, // packages of the phone
    filtered_packages: Vec<usize>, // phone_packages indexes (= what you see on screen)
    pub selection: Selection,
    search_input: text_input::State,
    select_all_btn_state: button::State,
    export_selection_btn_state: button::State,
    apply_remove_selection: button::State,
    apply_restore_selection: button::State,
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
    ApplyActionOnSelection(Action),
    SelectAllPressed,
    ExportSelectionPressed,
    List(usize, RowMessage),
    ExportedSelection(Result<bool, String>),
}


impl List {
    pub fn update(&mut self, settings: &Settings, phone: &Phone, message: Message) -> Command<Message> {
        match message {
            Message::LoadPackages(uad_lists) => {
                self.selected_package_state = Some(PackageState::Enabled);
                self.selected_list = Some(UadList::All);
                self.selected_removal = Some(Removal::Recommended);

                let all_system_packages = list_all_system_packages(); // installed and uninstalled packages
                let enabled_system_packages = hashset_system_packages(PackageState::Enabled);
                let disabled_system_packages = hashset_system_packages(PackageState::Disabled);
                let mut description;
                let mut uad_list;
                let mut state;
                let mut removal;

                for p_name in all_system_packages.lines() {
                    state = PackageState::Uninstalled;
                    description = "[No description]";
                    uad_list = UadList::Unlisted;
                    removal = Removal::Unlisted;

                    if uad_lists.contains_key(p_name) {
                        description = match &uad_lists.get(p_name).unwrap().description {
                            Some(descr) => &descr,
                            None => "[No description]",
                        };
                        uad_list = uad_lists.get(p_name).unwrap().list;
                        removal = uad_lists.get(p_name).unwrap().removal;
                    }

                    if enabled_system_packages.contains(p_name) {
                       state = PackageState::Enabled;
                    } else if disabled_system_packages.contains(p_name) {
                        state = PackageState::Disabled;
                    }

                    let package_row = PackageRow::new(
                        &p_name,
                        state,
                        &description,
                        uad_list,
                        removal,
                        false,
                    );
                    self.phone_packages.push(package_row)
                }
                self.phone_packages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                self.filtered_packages = (0..self.phone_packages.len()).collect();
                Self::filter_package_lists(self);

                match import_selection(&mut self.phone_packages, &mut self.selection) {
                    Ok(_) => info!("Custom selection has been successfully imported"),
                    Err(err) => warn!("No custom selection imported: {}", err),
                };
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
            }
            Message::RemovalSelected(removal) => {
                self.selected_removal = Some(removal);
                Self::filter_package_lists(self);
                Command::none()
            }
            Message::List(i, row_message) => {
                self.phone_packages[i].update(row_message.clone()).map(move |row_message| Message::List(i, row_message));
                
                match row_message {
                    RowMessage::ToggleSelection(toggle) => {
                        self.phone_packages[i].selected = toggle;
                        if self.phone_packages[i].selected {
                            self.selection.selected_packages.push(i);
                            update_selection_count(&mut self.selection, self.phone_packages[i].state, true);
                        } else {
                            self.selection.selected_packages.drain_filter(|s_i| *s_i == i);
                            update_selection_count(&mut self.selection, self.phone_packages[i].state, false);
                        }
                    },
                    RowMessage::ActionPressed => {
                        let success = action_handler(
                                &self.phone_packages[i],
                                phone,
                                settings.disable_mode,
                            ).unwrap_or_else(|err| err);
                            
                        if success {
                            let i = self.phone_packages
                                .iter()
                                .position(|p| p.name == self.phone_packages[i].name)
                                .unwrap();
                            if self.phone_packages[i].selected {
                                update_selection_count(&mut self.selection, self.phone_packages[i].state, false);
                            }
                            self.phone_packages[i].state = self.phone_packages[i].state.opposite(settings.disable_mode);
                            self.selection.selected_packages.drain_filter(|s_i| *s_i == i);
                            self.phone_packages[i].selected = false;
                            Self::filter_package_lists(self);
                        }
                    },
                    RowMessage::PackagePressed => {
                        self.description = self.phone_packages[i].clone().description;
                    },
                }
                Command::none()
            },
            Message::ApplyActionOnSelection(action) => {
                let mut selected_packages = self.selection.selected_packages.clone();
                
                match action {
                    Action::Remove => {
                        selected_packages.drain_filter(|i| self.phone_packages[*i].state != PackageState::Enabled);
                    }
                    Action::Restore => {
                        selected_packages.drain_filter(|i| self.phone_packages[*i].state == PackageState::Enabled);
                    }
                }

                for i in selected_packages {
                    let success = action_handler(
                        &self.phone_packages[i],
                        phone,
                        settings.disable_mode
                    ).unwrap_or_else(|err| err);
                    
                    if success {
                        update_selection_count(&mut self.selection, self.phone_packages[i].state, false);
                        self.phone_packages[i].state = self.phone_packages[i].state.opposite(settings.disable_mode);
                        self.phone_packages[i].selected = false;
                        self.selection.selected_packages.drain_filter(|s_i| *s_i == i);
                    }
                }
                Self::filter_package_lists(self);
                Command::none()
            },
            Message::SelectAllPressed => {
                for i in self.filtered_packages.clone() {
                    self.phone_packages[i].selected = true;
                    if !self.selection.selected_packages.contains(&i) {
                        self.selection.selected_packages.push(i);
                        update_selection_count(&mut self.selection, self.phone_packages[i].state, true);
                    }
                }
                Command::none()
            },
            Message::ExportSelectionPressed => {
                Command::perform(export_selection(self.phone_packages.clone()), Message::ExportedSelection)
            }
            Message::ExportedSelection(export) => {
                match export {
                    Ok(_) => info!("Selection exported"),
                    Err(err) => error!("Selection export: {}", err),
                };
               Command::none() 
            }
        }
    }

    pub fn view(&mut self, settings: &Settings, phone: &Phone) -> Element<Message> {
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
                .align_items(Alignment::Center)
                .spacing(10)
                .push(search_packages)
                .push(divider)
                .push(removal_picklist)
                .push(package_state_picklist)
                .push(list_picklist);

            let packages = self.phone_packages
                .iter_mut()
                .enumerate()
                .filter(|(i,_)| self.filtered_packages.contains(i))
                .fold(Column::new().spacing(6), |col, (i,p)| {
                    col.push(p.view(settings, phone).map(move |msg| Message::List(i, msg)))
                });

            /*
            // Seems better to me but I can't fix the lifetime issues
            let packages = self.filtered_packages
                .into_iter()
                .fold(Column::new().spacing(6), |col, i| {
                    col.push(self.phone_packages[i].view().map(move |msg| Message::List(i, msg)))
                });
            */

            let packages_scrollable = Scrollable::new(&mut self.package_scrollable_state)
                .push(packages)
                .spacing(2)
                .align_items(Alignment::Start)
                .height(Length::FillPortion(6))
                .style(style::Scrollable);

            // let mut packages_v: Vec<&str> = self.packages.lines().collect();
            let description_panel = Container::new(
                Row::new()
                .align_items(Alignment::Center)
                .push(Text::new(&self.description))
            )
            .style(style::Description)
            .padding(10)
            .height(Length::FillPortion(2))
            .width(Length::Fill);


            let restore_action = if settings.disable_mode {"Enable/Restore"} else {"Restore"};
            let remove_action = if settings.disable_mode {"Disable"} else {"Uninstall"};

            let apply_restore_selection = Button::new(
                &mut self.apply_restore_selection, 
                Text::new(format!("{} selection ({})", 
                        restore_action, 
                        self.selection.uninstalled + self.selection.disabled)
                    )
                )
                .on_press(Message::ApplyActionOnSelection(Action::Restore))
                .padding(5)
                .style(style::PrimaryButton::Enabled);

            let apply_remove_selection = Button::new(
                &mut self.apply_remove_selection, 
                Text::new(format!("{} selection ({})", remove_action, self.selection.enabled)))
                .on_press(Message::ApplyActionOnSelection(Action::Remove))
                .padding(5)
                .style(style::PrimaryButton::Enabled);

            let select_all_btn = Button::new(&mut self.select_all_btn_state, Text::new("Select all"))
                .padding(5)
                .on_press(Message::SelectAllPressed)
                .style(style::PrimaryButton::Enabled);

            let export_selection_btn = Button::new(
                &mut self.export_selection_btn_state, 
                Text::new(format!("Export current selection ({})", self.selection.selected_packages.len())))
                .padding(5)
                .on_press(Message::ExportSelectionPressed)
                .style(style::PrimaryButton::Enabled);

            let action_row = Row::new()
                .width(Length::Fill)
                .spacing(10)
                .align_items(Alignment::Center)
                .push(select_all_btn)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(export_selection_btn)
                .push(apply_restore_selection)
                .push(apply_remove_selection);

            let content = Column::new()
                .width(Length::Fill)
                .spacing(10)
                .align_items(Alignment::Center)
                .push(control_panel)
                .push(Space::new(Length::Fill, Length::Shrink))
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

        self.filtered_packages = self.phone_packages
            .iter()
            .enumerate()
            .filter(
                |(_,p)|
                (list_filter == UadList::All || p.uad_list == list_filter) &&
                (package_filter == PackageState::All || p.state == package_filter) &&
                (removal_filter == Removal::All || p.removal == removal_filter) &&
                (self.input_value.is_empty() || p.name.contains(&self.input_value))
            )
            .map(|(i,_)| i)
            .collect();
    }
}


fn loading_data<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Pulling packages from the phone. Please wait...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .size(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .style(style::Content)
    .into()
}