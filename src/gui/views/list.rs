use crate::core::config::DeviceSettings;
use crate::core::sync::{apply_pkg_state_commands, perform_adb_commands, CommandType, Phone, User};
use crate::core::theme::Theme;
use crate::core::uad_lists::{
    load_debloat_lists, Opposite, Package, PackageState, Removal, UadList, UadListState,
};
use crate::core::utils::fetch_packages;
use crate::gui::style;
use crate::gui::widgets::navigation_menu::ICONS;
use std::collections::HashMap;
use std::env;

use crate::gui::views::settings::Settings;
use crate::gui::widgets::modal::Modal;
use crate::gui::widgets::package_row::{Message as RowMessage, PackageRow};
use iced::widget::{
    button, column, container, horizontal_space, pick_list, radio, row, scrollable, text,
    text_input, tooltip, vertical_rule, Space,
};
use iced::{alignment, Alignment, Command, Element, Length, Renderer};

#[derive(Debug, Default, Clone)]
pub struct PackageInfo {
    pub i_user: usize,
    pub index: usize,
    pub removal: String,
}

#[derive(Debug, Clone)]
pub enum LoadingState {
    DownloadingList(String),
    FindingPhones(String),
    LoadingPackages(String),
    _UpdatingUad(String),
    Ready(String),
    RestoringDevice(String),
}

impl Default for LoadingState {
    fn default() -> Self {
        Self::FindingPhones(String::new())
    }
}

#[derive(Default, Debug, Clone)]
pub struct List {
    pub loading_state: LoadingState,
    pub uad_lists: HashMap<String, Package>,
    pub phone_packages: Vec<Vec<PackageRow>>, // packages of all users of the phone
    filtered_packages: Vec<usize>, // phone_packages indexes of the selected user (= what you see on screen)
    selected_packages: Vec<(usize, usize)>, // Vec of (user_index, pkg_index)
    selected_package_state: Option<PackageState>,
    selected_removal: Option<Removal>,
    selected_list: Option<UadList>,
    selected_user: Option<User>,
    pub input_value: String,
    description: String,
    selection_modal: bool,
    current_package_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadUadList(bool),
    LoadPhonePackages((HashMap<String, Package>, UadListState)),
    RestoringDevice(Result<CommandType, ()>),
    ApplyFilters(Vec<Vec<PackageRow>>),
    SearchInputChanged(String),
    ToggleAllSelected(bool),
    ListSelected(UadList),
    UserSelected(User),
    PackageStateSelected(PackageState),
    RemovalSelected(Removal),
    ApplyActionOnSelection,
    List(usize, RowMessage),
    ChangePackageState(Result<CommandType, ()>),
    Nothing,
    ModalHide,
    ModalUserSelected(User),
    ModalValidate,
}

impl List {
    pub fn update(
        &mut self,
        settings: &mut Settings,
        selected_device: &mut Phone,
        list_update_state: &mut UadListState,
        message: Message,
    ) -> Command<Message> {
        let i_user = self.selected_user.unwrap_or_default().index;
        match message {
            Message::ModalHide => {
                self.selection_modal = false;
                Command::none()
            }
            Message::ModalValidate => {
                let mut commands = vec![];
                self.selected_packages.sort_unstable();
                self.selected_packages.dedup();
                for selection in &self.selected_packages {
                    commands.append(&mut build_action_pkg_commands(
                        &self.phone_packages,
                        selected_device,
                        &settings.device,
                        *selection,
                    ));
                }
                self.selection_modal = false;
                Command::batch(commands)
            }
            Message::RestoringDevice(output) => {
                if let Ok(res) = output {
                    if let CommandType::PackageManager(p) = res {
                        self.loading_state = LoadingState::RestoringDevice(
                            self.phone_packages[i_user][p.index].name.clone(),
                        );
                    }
                } else {
                    self.loading_state = LoadingState::RestoringDevice("Error [TODO]".to_string());
                }
                Command::none()
            }
            Message::LoadUadList(remote) => {
                info!("{:-^65}", "-");
                info!(
                    "ANDROID_SDK: {} | DEVICE: {}",
                    selected_device.android_sdk, selected_device.model
                );
                info!("{:-^65}", "-");
                self.loading_state = LoadingState::DownloadingList(String::new());
                Command::perform(
                    Self::init_apps_view(remote, selected_device.clone()),
                    Message::LoadPhonePackages,
                )
            }
            Message::LoadPhonePackages(list_box) => {
                let (uad_list, list_state) = list_box;
                self.loading_state = LoadingState::LoadingPackages(String::new());
                self.uad_lists = uad_list.clone();
                *list_update_state = list_state;
                Command::perform(
                    Self::load_packages(uad_list, selected_device.user_list.clone()),
                    Message::ApplyFilters,
                )
            }
            Message::ApplyFilters(packages) => {
                self.phone_packages = packages;
                self.filtered_packages = (0..self.phone_packages[i_user].len()).collect();
                self.selected_package_state = Some(PackageState::Enabled);
                self.selected_removal = Some(Removal::Recommended);
                self.selected_list = Some(UadList::All);
                self.selected_user = Some(User::default());
                Self::filter_package_lists(self);
                self.loading_state = LoadingState::Ready(String::new());
                Command::none()
            }
            Message::ToggleAllSelected(selected) => {
                #[allow(unused_must_use)]
                for i in self.filtered_packages.clone() {
                    if self.phone_packages[i_user][i].selected != selected {
                        self.update(
                            settings,
                            selected_device,
                            list_update_state,
                            Message::List(i, RowMessage::ToggleSelection(selected)),
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
                #[allow(unused_must_use)]
                {
                    self.phone_packages[i_user][i_package]
                        .update(&row_message)
                        .map(move |row_message| Message::List(i_package, row_message));
                }

                let package = &mut self.phone_packages[i_user][i_package];

                match row_message {
                    RowMessage::ToggleSelection(toggle) => {
                        if package.removal == Removal::Unsafe && !settings.general.expert_mode {
                            package.selected = false;
                            return Command::none();
                        }

                        if settings.device.multi_user_mode {
                            for u in selected_device.user_list.iter().filter(|&u| !u.protected) {
                                self.phone_packages[u.index][i_package].selected = toggle;
                                if toggle {
                                    self.selected_packages.push((u.index, i_package));
                                }
                            }
                            if !toggle {
                                self.selected_packages.retain(|&x| x.1 != i_package);
                            }
                        } else {
                            package.selected = toggle;
                            if toggle {
                                self.selected_packages.push((i_user, i_package));
                            } else {
                                self.selected_packages
                                    .retain(|&x| x.1 != i_package || x.0 != i_user);
                            }
                        }
                        Command::none()
                    }
                    RowMessage::ActionPressed => {
                        self.phone_packages[i_user][i_package].selected = true;
                        Command::batch(build_action_pkg_commands(
                            &self.phone_packages,
                            selected_device,
                            &settings.device,
                            (i_user, i_package),
                        ))
                    }
                    RowMessage::PackagePressed => {
                        self.description = package.clone().description;
                        package.current = true;
                        if self.current_package_index != i_package {
                            self.phone_packages[i_user][self.current_package_index].current = false;
                        }
                        self.current_package_index = i_package;
                        Command::none()
                    }
                }
            }
            Message::ApplyActionOnSelection => {
                self.selection_modal = true;
                Command::none()
            }
            Message::UserSelected(user) => {
                self.selected_user = Some(user);
                self.filtered_packages = (0..self.phone_packages[user.index].len()).collect();
                Self::filter_package_lists(self);
                Command::none()
            }
            Message::ChangePackageState(res) => {
                if let Ok(CommandType::PackageManager(p)) = res {
                    let package = &mut self.phone_packages[p.i_user][p.index];
                    package.state = package.state.opposite(settings.device.disable_mode);
                    package.selected = false;
                    self.selected_packages
                        .retain(|&x| x.1 != p.index && x.0 != p.i_user);
                    Self::filter_package_lists(self);
                }
                Command::none()
            }
            Message::ModalUserSelected(user) => {
                self.selected_user = Some(user);
                self.update(
                    settings,
                    selected_device,
                    list_update_state,
                    Message::UserSelected(user),
                )
            }
            Message::Nothing => Command::none(),
        }
    }

    pub fn view(
        &self,
        settings: &Settings,
        selected_device: &Phone,
    ) -> Element<Message, Renderer<Theme>> {
        match &self.loading_state {
            LoadingState::DownloadingList(_) => {
                let text = "Downloading latest UAD lists from Github. Please wait...";
                waiting_view(settings, text, true)
            }
            LoadingState::FindingPhones(_) => {
                let text = "Finding connected devices...";
                waiting_view(settings, text, false)
            }
            LoadingState::LoadingPackages(_) => {
                let text = "Pulling packages from the device. Please wait...";
                waiting_view(settings, text, false)
            }
            LoadingState::_UpdatingUad(_) => {
                let text = "Updating UAD. Please wait...";
                waiting_view(settings, text, false)
            }
            LoadingState::RestoringDevice(output) => {
                let text = format!("Restoring device: {output}");
                waiting_view(settings, &text, false)
            }
            LoadingState::Ready(_) => {
                let search_packages = text_input(
                    "Search packages...",
                    &self.input_value,
                    Message::SearchInputChanged,
                )
                .padding(5);

                let user_picklist = pick_list(
                    selected_device.user_list.clone(),
                    self.selected_user,
                    Message::UserSelected,
                )
                .width(85);

                let divider = Space::new(Length::Fill, Length::Shrink);

                let list_picklist =
                    pick_list(&UadList::ALL[..], self.selected_list, Message::ListSelected);
                let package_state_picklist = pick_list(
                    &PackageState::ALL[..],
                    self.selected_package_state,
                    Message::PackageStateSelected,
                );

                let removal_picklist = pick_list(
                    &Removal::ALL[..],
                    self.selected_removal,
                    Message::RemovalSelected,
                );

                let control_panel = row![
                    search_packages,
                    user_picklist,
                    divider,
                    removal_picklist,
                    package_state_picklist,
                    list_picklist,
                ]
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
                .padding([0, 16, 0, 0]);

                let packages =
                    self.filtered_packages
                        .iter()
                        .fold(column![].spacing(6), |col, i| {
                            col.push(
                                self.phone_packages[self.selected_user.unwrap().index][*i]
                                    .view(settings, selected_device)
                                    .map(move |msg| Message::List(*i, msg)),
                            )
                        });

                let packages_scrollable = scrollable(packages)
                    .height(Length::FillPortion(6))
                    .style(style::Scrollable::Packages);

                let description_scroll = scrollable(text(&self.description).width(Length::Fill))
                    .style(style::Scrollable::Description);

                let description_panel = container(description_scroll)
                    .padding(6)
                    .height(Length::FillPortion(2))
                    .width(Length::Fill)
                    .style(style::Container::Frame);

                let review_selection = if !self.selected_packages.is_empty() {
                    button(text(format!(
                        "Review selection ({})",
                        self.selected_packages.len()
                    )))
                    .on_press(Message::ApplyActionOnSelection)
                    .padding(5)
                    .style(style::Button::Primary)
                } else {
                    button(text(format!(
                        "Review selection ({})",
                        self.selected_packages.len()
                    )))
                    .padding(5)
                };

                let select_all_btn = button("Select all")
                    .padding(5)
                    .on_press(Message::ToggleAllSelected(true))
                    .style(style::Button::Primary);

                let unselect_all_btn = button("Unselect all")
                    .padding(5)
                    .on_press(Message::ToggleAllSelected(false))
                    .style(style::Button::Primary);

                let action_row = row![
                    select_all_btn,
                    unselect_all_btn,
                    Space::new(Length::Fill, Length::Shrink),
                    review_selection,
                ]
                .width(Length::Fill)
                .spacing(10)
                .align_items(Alignment::Center);

                let unavailable = container(
                    column![
                        text("ADB is not authorized to access this user!").size(22)
                            .style(style::Text::Danger),
                        text("The most likely reason is that it is the user of your work profile (also called Secure Folder on Samsung devices). There's really no solution, other than completely disabling your work profile in your device settings.")
                            .style(style::Text::Commentary)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center)
                )
                .padding(10)
                .center_x()
                .style(style::Container::BorderedFrame);

                let content = if selected_device.user_list.is_empty()
                    || !self.phone_packages[self.selected_user.unwrap().index].is_empty()
                {
                    column![
                        control_panel,
                        packages_scrollable,
                        description_panel,
                        action_row,
                    ]
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                } else {
                    column![
                        control_panel,
                        container(unavailable).height(Length::Fill).center_y(),
                    ]
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                };
                if self.selection_modal {
                    Modal::new(
                        content.padding(10),
                        self.apply_selection_modal(
                            selected_device,
                            settings,
                            &self.phone_packages[self.selected_user.unwrap().index],
                        ),
                    )
                    .on_blur(Message::ModalHide)
                    .into()
                } else {
                    container(content).height(Length::Fill).padding(10).into()
                }
            }
        }
    }

    fn apply_selection_modal(
        &self,
        device: &Phone,
        settings: &Settings,
        packages: &[PackageRow],
    ) -> Element<Message, Renderer<Theme>> {
        // (nb_to_restore, nb_to_remove)
        let mut h_recap: HashMap<Removal, (u8, u8)> = HashMap::new();
        for p in packages.iter().filter(|p| p.selected) {
            if p.state == PackageState::Uninstalled || p.state == PackageState::Disabled {
                h_recap.entry(p.removal).or_insert((0, 0)).1 += 1;
            } else {
                h_recap.entry(p.removal).or_insert((0, 0)).0 += 1;
            }
        }

        let radio_btn_users = device.user_list.iter().filter(|&u| !u.protected).fold(
            row![].spacing(10),
            |row, user| {
                row.push(
                    radio(
                        format!("{}", user.clone()),
                        *user,
                        self.selected_user,
                        Message::ModalUserSelected,
                    )
                    .size(23),
                )
            },
        );

        let title_ctn =
            container(row![text("Review your selection").size(25)].align_items(Alignment::Center))
                .width(Length::Fill)
                .style(style::Container::Frame)
                .padding([10, 0, 10, 0])
                .center_y()
                .center_x();

        let users_ctn = container(radio_btn_users)
            .padding(10)
            .center_x()
            .style(style::Container::Frame);

        let explaination_ctn = container(
            row![
                text("The action for the selected user will be applied to all other users")
                    .style(style::Text::Danger),
                tooltip(
                    text("\u{EA0C}")
                        .font(ICONS)
                        .width(17)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .style(style::Text::Commentary)
                        .size(17),
                    "Let's say you choose user 0. If a selected package on user 0\n\
                        is set to be uninstalled and if this same package is disabled on user 10,\n\
                        then the package on both users will be uninstalled.",
                    tooltip::Position::Top,
                )
                .gap(20)
                .padding(10)
                .size(17)
                .style(style::Container::Tooltip)
            ]
            .spacing(10),
        )
        .center_x()
        .padding(10)
        .style(style::Container::BorderedFrame);

        let modal_btn_row = row![
            button(text("Cancel")).on_press(Message::ModalHide),
            horizontal_space(Length::Fill),
            button(text("Apply")).on_press(Message::ModalValidate),
        ]
        .padding([0, 15, 10, 10]);

        let recap_view = Removal::ALL
            .iter()
            .filter(|&&r| r != Removal::All)
            .fold(column![].spacing(6).width(Length::Fill), |col, r| {
                col.push(recap(settings, &mut h_recap, *r))
            });

        let selected_pkgs_ctn = container(
            container(
                scrollable(
                    container(
                        if !self
                            .selected_packages
                            .iter()
                            .any(|s| s.0 == self.selected_user.unwrap().index)
                        {
                            column![text("No packages selected for this user")]
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                        } else {
                            self.selected_packages
                                .iter()
                                .filter(|s| s.0 == self.selected_user.unwrap().index)
                                .fold(
                                    column![].spacing(6).width(Length::Fill),
                                    |col, selection| {
                                        col.push(
                                            row![
                                                row![text(
                                                    self.phone_packages[selection.0][selection.1]
                                                        .removal
                                                )]
                                                .width(100),
                                                row![text(
                                                    self.phone_packages[selection.0][selection.1]
                                                        .uad_list
                                                )]
                                                .width(60),
                                                row![text(
                                                    self.phone_packages[selection.0][selection.1]
                                                        .name
                                                        .clone()
                                                ),],
                                                horizontal_space(Length::Fill),
                                                row![match self.phone_packages[selection.0]
                                                    [selection.1]
                                                    .state
                                                {
                                                    PackageState::Enabled =>
                                                        if settings.device.disable_mode {
                                                            text("Disable")
                                                                .style(style::Text::Danger)
                                                        } else {
                                                            text("Uninstall")
                                                                .style(style::Text::Danger)
                                                        },
                                                    PackageState::Disabled =>
                                                        text("Enable").style(style::Text::Ok),
                                                    PackageState::Uninstalled =>
                                                        text("Restore").style(style::Text::Ok),
                                                    PackageState::All => text("Impossible")
                                                        .style(style::Text::Danger),
                                                },]
                                                .width(60),
                                            ]
                                            .width(Length::Fill)
                                            .spacing(20),
                                        )
                                    },
                                )
                        },
                    )
                    .padding(10)
                    .width(Length::Fill),
                )
                .style(style::Scrollable::Description),
            )
            .width(Length::Fill)
            .style(style::Container::Frame),
        )
        .width(Length::Fill)
        .max_height(150)
        .padding([0, 10, 0, 10]);

        container(
            if device.user_list.iter().filter(|&u| !u.protected).count() > 1
                && settings.device.multi_user_mode
            {
                column![
                    title_ctn,
                    users_ctn,
                    row![explaination_ctn].padding([0, 10, 0, 10]),
                    container(recap_view).padding(10),
                    selected_pkgs_ctn,
                    modal_btn_row,
                ]
                .spacing(10)
                .align_items(Alignment::Center)
            } else if !settings.device.multi_user_mode {
                column![
                    title_ctn,
                    users_ctn,
                    container(recap_view).padding(10),
                    selected_pkgs_ctn,
                    modal_btn_row,
                ]
                .spacing(10)
                .align_items(Alignment::Center)
            } else {
                column![
                    title_ctn,
                    container(recap_view).padding(10),
                    selected_pkgs_ctn,
                    modal_btn_row,
                ]
                .spacing(10)
                .align_items(Alignment::Center)
            },
        )
        .width(800)
        .height(Length::Shrink)
        .max_height(700)
        .style(style::Container::Background)
        .into()
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

    async fn load_packages(
        uad_list: HashMap<String, Package>,
        user_list: Vec<User>,
    ) -> Vec<Vec<PackageRow>> {
        let mut phone_packages = vec![];

        if user_list.len() <= 1 {
            phone_packages.push(fetch_packages(&uad_list, None));
        } else {
            phone_packages.extend(
                user_list
                    .iter()
                    .map(|user| fetch_packages(&uad_list, Some(user))),
            );
        };
        phone_packages
    }

    async fn init_apps_view(
        remote: bool,
        phone: Phone,
    ) -> (HashMap<String, Package>, UadListState) {
        let (uad_lists, _) = load_debloat_lists(remote);
        match uad_lists {
            Ok(list) => {
                env::set_var("ANDROID_SERIAL", phone.adb_id.clone());
                if phone.adb_id.is_empty() {
                    error!("AppsView ready but no phone found");
                }
                (list, UadListState::Done)
            }
            Err(local_list) => {
                error!("Error loading remote debloat list for the phone. Fallback to embedded (and outdated) list");
                (local_list, UadListState::Failed)
            }
        }
    }
}

fn waiting_view<'a>(
    _settings: &Settings,
    displayed_text: &str,
    btn: bool,
) -> Element<'a, Message, Renderer<Theme>> {
    let col = if btn {
        let no_internet_btn = button("No internet?")
            .padding(5)
            .on_press(Message::LoadUadList(false))
            .style(style::Button::Primary);

        column![]
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text(displayed_text).size(20))
            .push(no_internet_btn)
    } else {
        column![]
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text(displayed_text).size(20))
    };

    container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .style(style::Container::default())
        .into()
}

fn build_action_pkg_commands(
    packages: &[Vec<PackageRow>],
    device: &Phone,
    settings: &DeviceSettings,
    selection: (usize, usize),
) -> Vec<Command<Message>> {
    let pkg = &packages[selection.0][selection.1];
    let wanted_state = pkg.state.opposite(settings.disable_mode);

    let mut commands = vec![];
    for u in device.user_list.iter().filter(|&&u| {
        !u.protected && (packages[u.index][selection.1].selected || settings.multi_user_mode)
    }) {
        let u_pkg = packages[u.index][selection.1].clone();
        let actions = if settings.multi_user_mode {
            apply_pkg_state_commands(&u_pkg.into(), wanted_state, u, device)
        } else {
            let wanted_state = u_pkg.state.opposite(settings.disable_mode);
            apply_pkg_state_commands(&u_pkg.into(), wanted_state, u, device)
        };
        for (j, action) in actions.into_iter().enumerate() {
            let p_info = PackageInfo {
                i_user: u.index,
                index: selection.1,
                removal: pkg.removal.to_string(),
            };
            // In the end there is only one package state change
            // even if we run multiple adb commands
            commands.push(Command::perform(
                perform_adb_commands(action, CommandType::PackageManager(p_info)),
                if j == 0 {
                    Message::ChangePackageState
                } else {
                    |_| Message::Nothing
                },
            ));
        }
    }
    commands
}

fn recap<'a>(
    settings: &Settings,
    recap: &mut HashMap<Removal, (u8, u8)>,
    removal: Removal,
) -> Element<'a, Message, Renderer<Theme>> {
    container(
        row![
            text(removal).size(25).width(Length::FillPortion(1)),
            vertical_rule(5),
            row![
                if settings.device.disable_mode {
                    text("Disable").style(style::Text::Danger)
                } else {
                    text("Uninstall").style(style::Text::Danger)
                },
                horizontal_space(Length::Fill),
                text(recap.entry(removal).or_insert((0, 0)).0.to_string())
                    .style(style::Text::Danger)
            ]
            .width(Length::FillPortion(1)),
            vertical_rule(5),
            row![
                if settings.device.disable_mode {
                    text("Enable").style(style::Text::Ok)
                } else {
                    text("Restore").style(style::Text::Ok)
                },
                horizontal_space(Length::Fill),
                text(recap.entry(removal).or_insert((0, 0)).1.to_string()).style(style::Text::Ok)
            ]
            .width(Length::FillPortion(1))
        ]
        .spacing(20)
        .padding([0, 10, 0, 0])
        .width(Length::Fill)
        .align_items(Alignment::Center),
    )
    .padding(10)
    .width(Length::Fill)
    .height(45)
    .style(style::Container::Frame)
    .into()
}
