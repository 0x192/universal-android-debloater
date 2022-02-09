use crate::core::sync::{action_handler, adb_shell_command, Phone, User};
use crate::core::uad_lists::{
    load_debloat_lists, Opposite, Package, PackageState, Removal, UadList,
};
use crate::core::utils::{
    export_selection, fetch_packages, import_selection, update_selection_count,
};
use crate::gui::style;
use std::collections::HashMap;
use std::env;

use crate::gui::views::settings::{Phone as SettingsPhone, Settings};
use crate::gui::widgets::package_row::{Message as RowMessage, PackageRow};
use iced::{
    button, pick_list, scrollable, text_input, Alignment, Button, Column, Command, Container,
    Element, Length, PickList, Row, Scrollable, Space, Text, TextInput,
};

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    DownloadingList,
    LoadingPackages,
    _UpdatingUad,
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Ready,
    Loading(LoadingState),
}

impl Default for State {
    fn default() -> Self {
        State::Loading(LoadingState::LoadingPackages)
    }
}

#[derive(Default, Debug, Clone)]
pub struct List {
    pub state: State,
    pub uad_lists: HashMap<String, Package>,
    phone_packages: Vec<Vec<PackageRow>>, // packages of all users of the phone
    filtered_packages: Vec<usize>, // phone_packages indexes of the selected user (= what you see on screen)
    pub selection: Selection,
    select_all_btn_state: button::State,
    unselect_all_btn_state: button::State,
    search_input: text_input::State,
    user_picklist: pick_list::State<User>,
    export_selection_btn_state: button::State,
    no_internet_btn: button::State,
    apply_remove_selection: button::State,
    apply_restore_selection: button::State,
    package_scrollable_state: scrollable::State,
    description_scrollable_state: scrollable::State,
    package_state_picklist: pick_list::State<PackageState>,
    list_picklist: pick_list::State<UadList>,
    removal_picklist: pick_list::State<Removal>,
    selected_package_state: Option<PackageState>,
    selected_removal: Option<Removal>,
    selected_list: Option<UadList>,
    selected_user: Option<User>,
    pub input_value: String,
    description: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchInputChanged(String),
    ToggleAllSelected(bool),
    InitUadList(bool),
    ListsIsInitialized(HashMap<String, Package>),
    LoadPackages,
    ListSelected(UadList),
    UserSelected(User),
    PackageStateSelected(PackageState),
    RemovalSelected(Removal),
    ApplyActionOnSelection(Action),
    ExportSelectionPressed,
    List(usize, RowMessage),
    ExportedSelection(Result<bool, String>),
    ChangePackageState(Result<usize, ()>),
    PackagesLoaded(Vec<Vec<PackageRow>>),
    Nothing,
}

impl List {
    pub fn update(
        &mut self,
        settings: &SettingsPhone,
        phone: &mut Phone,
        message: Message,
    ) -> Command<Message> {
        let i_user = &self.selected_user.unwrap_or(User { id: 0, index: 0 }).index;
        match message {
            Message::Nothing => Command::none(),
            Message::InitUadList(remote) => {
                self.state = State::Loading(LoadingState::LoadingPackages);
                if remote {
                    Command::perform(load_debloat_lists(true), Message::ListsIsInitialized)
                } else {
                    Command::perform(load_debloat_lists(false), Message::ListsIsInitialized)
                }
            }
            Message::ListsIsInitialized(uad_lists) => {
                self.uad_lists = uad_lists;
                env::set_var("ANDROID_SERIAL", phone.adb_id.clone());
                Command::perform(Self::do_load_packages(), |_| Message::LoadPackages)
            }
            Message::LoadPackages => {
                self.state = State::Loading(LoadingState::LoadingPackages);
                self.selected_package_state = Some(PackageState::Enabled);
                self.selected_list = Some(UadList::All);
                self.selected_removal = Some(Removal::Recommended);
                self.selected_user = Some(User { id: 0, index: 0 });
                Command::perform(
                    Self::load_packages(self.uad_lists.clone(), phone.user_list.clone()),
                    Message::PackagesLoaded,
                )
            }
            Message::PackagesLoaded(packages) => {
                self.phone_packages = packages;
                self.filtered_packages = (0..self.phone_packages[*i_user].len()).collect();
                Self::filter_package_lists(self);

                match import_selection(&mut self.phone_packages[*i_user], &mut self.selection) {
                    Ok(_) => info!("Custom selection has been successfully imported"),
                    Err(err) => warn!("No custom selection imported: {}", err),
                };
                self.state = State::Ready;
                Command::none()
            }
            Message::ToggleAllSelected(selected) => {
                for i in self.filtered_packages.clone() {
                    self.phone_packages[*i_user][i].selected = selected;

                    if !selected {
                        if self.selection.selected_packages.contains(&i) {
                            update_selection_count(
                                &mut self.selection,
                                self.phone_packages[*i_user][i].state,
                                false,
                            );
                            self.selection
                                .selected_packages
                                .drain_filter(|s_i| *s_i == i);
                        }
                    } else if !self.selection.selected_packages.contains(&i) {
                        self.selection.selected_packages.push(i);
                        update_selection_count(
                            &mut self.selection,
                            self.phone_packages[*i_user][i].state,
                            true,
                        );
                    }
                }
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
            Message::List(i_package, row_message) => {
                self.phone_packages[*i_user][i_package]
                    .update(row_message.clone())
                    .map(move |row_message| Message::List(i_package, row_message));

                let package = &mut self.phone_packages[*i_user][i_package];

                match row_message {
                    RowMessage::ToggleSelection(toggle) => {
                        if package.removal == Removal::Unsafe && !settings.expert_mode {
                            package.selected = false;
                        } else {
                            package.selected = toggle;

                            if package.selected {
                                self.selection.selected_packages.push(i_package);
                            } else {
                                self.selection
                                    .selected_packages
                                    .drain_filter(|s_i| *s_i == i_package);
                            }
                            update_selection_count(
                                &mut self.selection,
                                package.state,
                                package.selected,
                            );
                        }
                        Command::none()
                    }
                    RowMessage::ActionPressed => {
                        let mut commands = vec![];
                        let actions =
                            action_handler(&self.selected_user.unwrap(), package, phone, settings);

                        for (i, action) in actions.into_iter().enumerate() {
                            // Only the first command can change the package state
                            if i != 0 {
                                commands.push(Command::perform(
                                    Self::perform_commands(action, i_package, package.removal),
                                    |_| Message::Nothing,
                                ));
                            } else {
                                commands.push(Command::perform(
                                    Self::perform_commands(action, i_package, package.removal),
                                    Message::ChangePackageState,
                                ));
                            }
                        }
                        Command::batch(commands)
                    }
                    RowMessage::PackagePressed => {
                        self.description = package.clone().description;
                        Command::none()
                    }
                }
            }
            Message::ApplyActionOnSelection(action) => {
                let mut selected_packages = self.selection.selected_packages.clone();

                match action {
                    Action::Remove => {
                        selected_packages.drain_filter(|i| {
                            self.phone_packages[*i_user][*i].state != PackageState::Enabled
                        });
                    }
                    Action::Restore => {
                        selected_packages.drain_filter(|i| {
                            self.phone_packages[*i_user][*i].state == PackageState::Enabled
                        });
                    }
                }
                let mut commands = vec![];
                for i in selected_packages {
                    let actions = action_handler(
                        &self.selected_user.unwrap(),
                        &self.phone_packages[*i_user][i],
                        phone,
                        settings,
                    );
                    for (j, action) in actions.into_iter().enumerate() {
                        // Only the first command can change the package state
                        if j != 0 {
                            commands.push(Command::perform(
                                Self::perform_commands(
                                    action,
                                    i,
                                    self.phone_packages[*i_user][i].removal,
                                ),
                                |_| Message::Nothing,
                            ));
                        } else {
                            commands.push(Command::perform(
                                Self::perform_commands(
                                    action,
                                    i,
                                    self.phone_packages[*i_user][i].removal,
                                ),
                                Message::ChangePackageState,
                            ));
                        }
                    }
                }
                Command::batch(commands)
            }
            Message::ExportSelectionPressed => Command::perform(
                export_selection(self.phone_packages[*i_user].clone()),
                Message::ExportedSelection,
            ),
            Message::ExportedSelection(export) => {
                match export {
                    Ok(_) => info!("Selection exported"),
                    Err(err) => error!("Selection export: {}", err),
                };
                Command::none()
            }
            Message::UserSelected(user) => {
                for p in &mut self.phone_packages[*i_user] {
                    p.selected = false;
                }
                self.selected_user = Some(user);
                for i_package in &self.selection.selected_packages {
                    self.phone_packages[user.index][*i_package].selected = true;
                }
                self.filtered_packages = (0..self.phone_packages[user.index].len()).collect();
                Self::filter_package_lists(self);
                Command::none()
            }
            Message::ChangePackageState(res) => {
                if let Ok(i) = res {
                    let package = &mut self.phone_packages[*i_user][i];
                    update_selection_count(&mut self.selection, package.state, false);

                    if !settings.multi_user_mode {
                        package.state = package.state.opposite(settings.disable_mode);
                        package.selected = false;
                    } else {
                        for u in &phone.user_list {
                            self.phone_packages[u.index][i].state = self.phone_packages[u.index][i]
                                .state
                                .opposite(settings.disable_mode);
                            self.phone_packages[u.index][i].selected = false;
                        }
                    }
                    self.selection
                        .selected_packages
                        .drain_filter(|s_i| *s_i == i);
                    Self::filter_package_lists(self);
                }
                Command::none()
            }
        }
    }

    pub fn view(&mut self, settings: &Settings, phone: &Phone) -> Element<Message> {
        match self.state {
            State::Loading(LoadingState::DownloadingList) => {
                let text = "Downloading latest UAD lists from Github. Please wait...";
                waiting_view(settings, text, Some(&mut self.no_internet_btn))
            }
            State::Loading(LoadingState::LoadingPackages) => {
                let text = "Pulling packages from the phone. Please wait...";
                waiting_view(settings, text, None)
            }
            State::Loading(LoadingState::_UpdatingUad) => {
                let text = "Updating UAD. Please wait...";
                waiting_view(settings, text, None)
            }
            State::Ready => {
                let search_packages = TextInput::new(
                    &mut self.search_input,
                    "Search packages...",
                    &self.input_value,
                    Message::SearchInputChanged,
                )
                .padding(5)
                .style(style::SearchInput(settings.theme.palette));

                // let package_amount = Text::new(format!("{} packages found", packages.len()));

                let user_picklist = PickList::new(
                    &mut self.user_picklist,
                    phone.user_list.clone(),
                    self.selected_user,
                    Message::UserSelected,
                )
                .width(Length::Units(85))
                .style(style::PickList(settings.theme.palette));

                let divider = Space::new(Length::Fill, Length::Shrink);

                let list_picklist = PickList::new(
                    &mut self.list_picklist,
                    &UadList::ALL[..],
                    self.selected_list,
                    Message::ListSelected,
                )
                .style(style::PickList(settings.theme.palette));

                let package_state_picklist = PickList::new(
                    &mut self.package_state_picklist,
                    &PackageState::ALL[..],
                    self.selected_package_state,
                    Message::PackageStateSelected,
                )
                .style(style::PickList(settings.theme.palette));

                let removal_picklist = PickList::new(
                    &mut self.removal_picklist,
                    &Removal::ALL[..],
                    self.selected_removal,
                    Message::RemovalSelected,
                )
                .style(style::PickList(settings.theme.palette));

                let control_panel = Row::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding([0, 16, 0, 0])
                    .push(search_packages)
                    .push(user_picklist)
                    .push(divider)
                    .push(removal_picklist)
                    .push(package_state_picklist)
                    .push(list_picklist);

                let packages = self.phone_packages[self.selected_user.unwrap().index]
                    .iter_mut()
                    .enumerate()
                    .filter(|(i, _)| self.filtered_packages.contains(i))
                    .fold(Column::new().spacing(6), |col, (i, p)| {
                        col.push(
                            p.view(settings, phone)
                                .map(move |msg| Message::List(i, msg)),
                        )
                    });

                /*
                // Seems better to me but I can't fix the lifetime issues
                let packages = self.filtered_packages
                    .into_iter()
                    .fold(Column::new().spacing(6), |col, i| {
                        col.push(self.phone_packages[self.selected_user.unwrap().index][i].view().map(move |msg| Message::List(i, msg)))
                    });
                */

                let packages_scrollable = Scrollable::new(&mut self.package_scrollable_state)
                    .push(packages)
                    .spacing(2)
                    .align_items(Alignment::Start)
                    .height(Length::FillPortion(6))
                    .style(style::Scrollable(settings.theme.palette));

                // let mut packages_v: Vec<&str> = self.packages.lines().collect();

                let description_scroll = Scrollable::new(&mut self.description_scrollable_state)
                    .push(Text::new(&self.description))
                    .padding(7)
                    .style(style::DescriptionScrollable(settings.theme.palette));

                let description_panel = Container::new(description_scroll)
                    .height(Length::FillPortion(2))
                    .width(Length::Fill)
                    .style(style::Description(settings.theme.palette));

                let restore_action = if settings.phone.disable_mode {
                    "Enable/Restore"
                } else {
                    "Restore"
                };
                let remove_action = if settings.phone.disable_mode {
                    "Disable"
                } else {
                    "Uninstall"
                };

                let apply_restore_selection = Button::new(
                    &mut self.apply_restore_selection,
                    Text::new(format!(
                        "{} selection ({})",
                        restore_action,
                        self.selection.uninstalled + self.selection.disabled
                    )),
                )
                .on_press(Message::ApplyActionOnSelection(Action::Restore))
                .padding(5)
                .style(style::PrimaryButton(settings.theme.palette));

                let apply_remove_selection = Button::new(
                    &mut self.apply_remove_selection,
                    Text::new(format!(
                        "{} selection ({})",
                        remove_action, self.selection.enabled
                    )),
                )
                .on_press(Message::ApplyActionOnSelection(Action::Remove))
                .padding(5)
                .style(style::PrimaryButton(settings.theme.palette));

                let select_all_btn =
                    Button::new(&mut self.select_all_btn_state, Text::new("Select all"))
                        .padding(5)
                        .on_press(Message::ToggleAllSelected(true))
                        .style(style::PrimaryButton(settings.theme.palette));

                let unselect_all_btn =
                    Button::new(&mut self.unselect_all_btn_state, Text::new("Unselect all"))
                        .padding(5)
                        .on_press(Message::ToggleAllSelected(false))
                        .style(style::PrimaryButton(settings.theme.palette));

                let export_selection_btn = Button::new(
                    &mut self.export_selection_btn_state,
                    Text::new(format!(
                        "Export current selection ({})",
                        self.selection.selected_packages.len()
                    )),
                )
                .padding(5)
                .on_press(Message::ExportSelectionPressed)
                .style(style::PrimaryButton(settings.theme.palette));

                let action_row = Row::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(select_all_btn)
                    .push(unselect_all_btn)
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(export_selection_btn)
                    .push(apply_restore_selection)
                    .push(apply_remove_selection);

                let content = Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(control_panel)
                    .push(packages_scrollable)
                    .push(description_panel)
                    .push(action_row);

                Container::new(content)
                    .height(Length::Fill)
                    .padding(10)
                    .style(style::Content(settings.theme.palette))
                    .into()
            }
        }
    }

    fn filter_package_lists(&mut self) {
        let list_filter: UadList = self.selected_list.unwrap();
        let package_filter: PackageState = self.selected_package_state.unwrap();
        let removal_filter: Removal = self.selected_removal.unwrap();

        self.filtered_packages = self.phone_packages[self.selected_user.unwrap().index]
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                (list_filter == UadList::All || p.uad_list == list_filter)
                    && (package_filter == PackageState::All || p.state == package_filter)
                    && (removal_filter == Removal::All || p.removal == removal_filter)
                    && (self.input_value.is_empty() || p.name.contains(&self.input_value))
            })
            .map(|(i, _)| i)
            .collect();
    }

    async fn perform_commands(
        action: String,
        i: usize,
        recommendation: Removal,
    ) -> Result<usize, ()> {
        match adb_shell_command(true, &action) {
            Ok(o) => {
                // On old devices, adb commands can return the '0' exit code even if there
                // is an error. On Android 4.4, ADB doesn't check if the package exists.
                // It does not return any error if you try to `pm block` a non-existent package.
                // Some commands are even killed by ADB before finishing and UAD can't catch
                // the output.
                if ["Error", "Failure"].iter().any(|&e| o.contains(e)) {
                    error!("[{}] {} -> {}", recommendation, action, o);
                    Err(())
                } else {
                    info!("[{}] {} -> {}", recommendation, action, o);
                    Ok(i)
                }
            }
            Err(err) => {
                if !err.contains("[not installed for") {
                    error!("[{}] {} -> {}", recommendation, action, err);
                }
                Err(())
            }
        }
    }

    async fn do_load_packages() -> Message {
        Message::LoadPackages
    }

    async fn load_packages(
        uad_lists: HashMap<String, Package>,
        user_list: Vec<User>,
    ) -> Vec<Vec<PackageRow>> {
        let mut temp = vec![];
        match user_list.len() {
            0 | 1 => temp.push(fetch_packages(&uad_lists, &None)),
            _ => {
                for user in &user_list {
                    temp.push(fetch_packages(&uad_lists, &Some(user)));
                }
            }
        }
        temp
    }
}

fn waiting_view<'a>(
    settings: &Settings,
    text: &str,
    btn: Option<&'a mut button::State>,
) -> Element<'a, Message> {
    let col = if let Some(btn) = btn {
        let no_internet_btn = Button::new(btn, Text::new("No internet?"))
            .padding(5)
            .on_press(Message::InitUadList(false))
            .style(style::PrimaryButton(settings.theme.palette));

        Column::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Text::new(text).size(20))
            .push(no_internet_btn)
    } else {
        Column::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Text::new(text).size(20))
    };

    Container::new(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .style(style::Content(settings.theme.palette))
        .into()
}
