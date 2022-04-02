pub mod style;
pub mod views;
pub mod widgets;

pub use crate::core::sync::{get_device_list, Phone};
pub use crate::core::uad_lists::Package;
use crate::core::uad_lists::{load_debloat_lists, UadListState};
use crate::core::update::{get_latest_release, SelfUpdateState, SelfUpdateStatus};
use crate::core::utils::{icon, perform_commands};
use iced::pure::widget::Text;
use iced::pure::{button, column, container, pick_list, row, text, Pure, State};
use iced::{
    window::Settings as Window, Alignment, Application, Command, Element, Font, Length, Settings,
    Space,
};

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
pub use views::about::{About as AboutView, Message as AboutMessage};
pub use views::list::{
    List as AppsView, LoadingState as ListLoadingState, Message as AppsMessage, State as ListState,
};
pub use views::settings::{
    Message as SettingsMessage, Phone as SettingsPhone, Settings as SettingsView,
};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../resources/assets/icons.ttf"),
};

#[cfg(feature = "self-update")]
use crate::core::update::{bin_name, download_update_to_temp_file, remove_file};

#[derive(Debug, Clone)]
pub enum View {
    List,
    About,
    Settings,
}

impl Default for View {
    fn default() -> Self {
        Self::List
    }
}

#[derive(Default)]
pub struct UadGui {
    ready: bool,
    view: View,
    state: State,
    apps_view: AppsView,
    about_view: AboutView,
    settings_view: SettingsView,
    device_list: Vec<Phone>,
    selected_device: Option<Phone>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    AppsPress,
    DeviceSelected(Phone),
    AboutAction(AboutMessage),
    AppsAction(AppsMessage),
    SettingsAction(SettingsMessage),
    RefreshButtonPressed,
    RebootButtonPressed,
    UadListsDownloaded((Result<HashMap<String, Package>, ()>, bool)),
    InitList,
    InitDevice(Vec<Phone>),
    _NewReleaseDownloaded(Result<(PathBuf, PathBuf), ()>),
    Nothing,
}

impl Application for UadGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::batch([
                Command::perform(Self::download_uad_list(), |_| Message::InitList),
                Command::perform(get_device_list(), Message::InitDevice),
                Command::perform(Self::send_self_update_message(), Message::SettingsAction),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Universal Android Debloater")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InitList => {
                warn!("Trying to download remote UAD list");
                self.apps_view.state = ListState::Loading(ListLoadingState::DownloadingList);
                Command::perform(load_debloat_lists(true), Message::UadListsDownloaded)
            }
            Message::UadListsDownloaded((uad_lists, _)) => match uad_lists {
                Ok(list) => {
                    if !list.is_empty() {
                        self.apps_view.uad_lists = list;
                    }
                    self.settings_view.list_update_state = UadListState::Done;

                    if self.ready {
                        Command::perform(
                            Self::load_phone_packages(
                                self.selected_device.clone().unwrap_or_default(),
                            ),
                            Message::AppsAction,
                        )
                    } else {
                        self.ready = true;
                        self.apps_view.state = ListState::Loading(ListLoadingState::FindingPhones);
                        Command::none()
                    }
                }
                Err(_) => {
                    self.settings_view.list_update_state = UadListState::Failed;
                    if let ListState::Ready = self.apps_view.state {
                    } else {
                        self.apps_view.state =
                            ListState::Loading(ListLoadingState::DownloadingList);
                    }
                    Command::none()
                }
            },
            Message::InitDevice(device_list) => {
                self.device_list = device_list;
                self.settings_view.phone = SettingsPhone::default();

                // Save the current selected device
                let i = self
                    .device_list
                    .iter()
                    .position(|phone| *phone == self.selected_device.clone().unwrap_or_default())
                    .unwrap_or(0);

                // Try to reload last selected phone
                if !self.device_list.is_empty() {
                    self.selected_device = match i < self.device_list.len() {
                        true => Some(self.device_list[i].clone()),
                        false => match self.device_list.last() {
                            Some(last) => Some(last.clone()),
                            None => Some(Phone::default()),
                        },
                    };
                    self.view = View::List;
                } else {
                    self.selected_device = None;
                }
                if self.settings_view.list_update_state != UadListState::Downloading || self.ready {
                    Command::perform(
                        Self::load_phone_packages(self.selected_device.clone().unwrap_or_default()),
                        Message::AppsAction,
                    )
                } else {
                    self.ready = true;
                    Command::none()
                }
            }
            Message::RefreshButtonPressed => {
                self.apps_view.state = ListState::Loading(ListLoadingState::FindingPhones);
                self.selected_device = None;
                self.ready = true;
                Command::perform(get_device_list(), Message::InitDevice)
            }
            Message::RebootButtonPressed => {
                self.apps_view.state = ListState::Loading(ListLoadingState::FindingPhones);
                self.selected_device = None;
                self.device_list = vec![];
                self.ready = false;
                Command::perform(
                    perform_commands("reboot".to_string(), 0, "ADB".to_string()),
                    |_| Message::Nothing,
                )
            }
            Message::AppsPress => {
                self.view = View::List;
                Command::none()
            }
            Message::AboutPressed => {
                self.view = View::About;
                self.settings_view.self_update_state = SelfUpdateState::default();
                Command::perform(Self::send_self_update_message(), Message::SettingsAction)
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
            Message::AppsAction(msg) => self
                .apps_view
                .update(
                    &mut self.settings_view,
                    &mut self.selected_device.clone().unwrap_or_default(),
                    msg,
                )
                .map(Message::AppsAction),
            Message::SettingsAction(msg) => {
                self.settings_view
                    .update(&self.selected_device.clone().unwrap_or_default(), msg);
                Command::none()
            }
            Message::AboutAction(msg) => {
                self.about_view.update(msg.clone());

                match msg {
                    AboutMessage::UpdateUadLists => {
                        self.settings_view.list_update_state = UadListState::Downloading;
                        Command::perform(Self::download_uad_list(), |_| Message::InitList)
                    }
                    AboutMessage::DoSelfUpdate => {
                        #[cfg(feature = "self-update")]
                        if self
                            .settings_view
                            .self_update_state
                            .latest_release
                            .is_some()
                        {
                            self.settings_view.self_update_state.status =
                                SelfUpdateStatus::Updating;
                            self.apps_view.state =
                                ListState::Loading(ListLoadingState::_UpdatingUad);
                            let bin_name = bin_name().to_owned();
                            let release = self
                                .settings_view
                                .self_update_state
                                .latest_release
                                .as_ref()
                                .unwrap()
                                .clone();
                            Command::perform(
                                download_update_to_temp_file(bin_name, release),
                                Message::_NewReleaseDownloaded,
                            )
                        } else {
                            Command::none()
                        }
                        #[cfg(not(feature = "self-update"))]
                        Command::none()
                    }
                    _ => Command::none(),
                }
            }
            Message::DeviceSelected(device) => {
                self.selected_device = Some(device);
                self.view = View::List;
                Command::perform(
                    Self::load_phone_packages(self.selected_device.clone().unwrap_or_default()),
                    Message::AppsAction,
                )
            }
            Message::_NewReleaseDownloaded(_res) => {
                debug!("UAD update has been download!");

                #[cfg(feature = "self-update")]
                match _res {
                    Ok((relaunch_path, cleanup_path)) => {
                        // Remove first arg, which is path to binary. We don't use this first
                        // arg as binary path because it's not reliable, per the docs.
                        let mut args = std::env::args();
                        args.next();
                        let mut args: Vec<_> = args.collect();

                        // Remove the `--self-update-temp` arg from args if it exists,
                        // since we need to pass it cleanly. Otherwise new process will
                        // fail during arg parsing.
                        if let Some(idx) = args.iter().position(|a| a == "--self-update-temp") {
                            args.remove(idx);
                            // Remove path passed after this arg
                            args.remove(idx);
                        }

                        match std::process::Command::new(&relaunch_path)
                            .args(args)
                            .arg("--self-update-temp")
                            .arg(&cleanup_path)
                            .spawn()
                        {
                            Ok(_) => {
                                if let Err(e) = remove_file(cleanup_path) {
                                    error!("Could not remove temp update file: {}", e);
                                }
                                std::process::exit(0)
                            }
                            Err(error) => {
                                if let Err(e) = remove_file(cleanup_path) {
                                    error!("Could not remove temp update file: {}", e);
                                }
                                error!("Failed to update UAD: {}", error)
                            }
                        }
                    }
                    Err(()) => error!("Failed to update UAD!"),
                }
                Command::none()
            }
            Message::Nothing => Command::perform(get_device_list(), Message::InitDevice),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let apps_refresh_btn = button(refresh_icon())
            .on_press(Message::RefreshButtonPressed)
            .padding(5)
            .style(style::RefreshButton(self.settings_view.theme.palette));

        let reboot_btn = button("Reboot")
            .on_press(Message::RebootButtonPressed)
            .padding(5)
            .style(style::RefreshButton(self.settings_view.theme.palette));

        let uad_version_text = if let Some(r) = &self.settings_view.self_update_state.latest_release
        {
            if self.settings_view.self_update_state.status == SelfUpdateStatus::Updating {
                Text::new("Updating please wait...")
                    .color(self.settings_view.theme.palette.normal.surface)
            } else {
                Text::new(format!(
                    "New UAD version available {} -> {}",
                    env!("CARGO_PKG_VERSION"),
                    r.tag_name
                ))
                .color(self.settings_view.theme.palette.normal.surface)
            }
        } else {
            Text::new(env!("CARGO_PKG_VERSION"))
        };

        let apps_btn = if self
            .settings_view
            .self_update_state
            .latest_release
            .is_some()
        {
            button("Update")
                .on_press(Message::AboutAction(AboutMessage::DoSelfUpdate))
                .padding(5)
                .style(style::SelfUpdateButton(self.settings_view.theme.palette))
        } else {
            button("Apps")
                .on_press(Message::AppsPress)
                .padding(5)
                .style(style::PrimaryButton(self.settings_view.theme.palette))
        };

        let about_btn = button("About")
            .on_press(Message::AboutPressed)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        let settings_btn = button("Settings")
            .on_press(Message::SettingsPressed)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        let device_picklist = pick_list(
            &self.device_list,
            self.selected_device.clone(),
            Message::DeviceSelected,
        )
        .style(style::PickList(self.settings_view.theme.palette));

        let device_list_text = match self.apps_view.state {
            ListState::Loading(ListLoadingState::FindingPhones) => {
                text("finding connected phone...")
            }
            _ => text("no devices/emulators found"),
        };

        let row = match self.selected_device {
            Some(_) => row()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
                .push(apps_refresh_btn)
                .push(reboot_btn)
                .push(device_picklist)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(uad_version_text)
                .push(apps_btn)
                .push(about_btn)
                .push(settings_btn),
            None => row()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
                .push(reboot_btn)
                .push(apps_refresh_btn)
                .push(device_list_text)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(uad_version_text)
                .push(apps_btn)
                .push(about_btn)
                .push(settings_btn),
        };

        let navigation_container = container(row)
            .width(Length::Fill)
            .padding(10)
            .style(style::NavigationContainer(self.settings_view.theme.palette));

        let main_container = match self.view {
            View::List => self
                .apps_view
                .view(
                    &self.settings_view,
                    &self.selected_device.clone().unwrap_or_default(),
                )
                .map(Message::AppsAction),
            View::About => self
                .about_view
                .view(&self.settings_view)
                .map(Message::AboutAction),
            View::Settings => self
                .settings_view
                .view(&self.selected_device.clone().unwrap_or_default())
                .map(Message::SettingsAction),
        };

        Pure::new(
            &mut self.state,
            column()
                .width(Length::Fill)
                .push(navigation_container)
                .push(main_container),
        )
        .into()
    }
}

impl UadGui {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (1050, 800),
                resizable: true,
                decorations: true,
                ..iced::window::Settings::default()
            },
            default_text_size: 17,
            ..iced::Settings::default()
        };
        Self::run(settings).unwrap_err();
    }

    pub async fn load_phone_packages(phone: Phone) -> AppsMessage {
        env::set_var("ANDROID_SERIAL", phone.adb_id);
        info!("{:-^65}", "-");
        info!(
            "ANDROID_SDK: {} | PHONE: {}",
            phone.android_sdk, phone.model
        );
        AppsMessage::LoadPackages
    }

    pub async fn download_uad_list() -> Message {
        Message::InitList
    }

    pub async fn send_self_update_message() -> SettingsMessage {
        SettingsMessage::GetLatestRelease(get_latest_release())
    }
}

fn refresh_icon() -> Text {
    icon('\u{E900}')
}
