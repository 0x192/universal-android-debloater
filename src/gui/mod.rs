pub mod style;
pub mod views;
pub mod widgets;

use crate::core::sync::{get_devices_list, perform_adb_commands, CommandType, Phone};
use crate::core::theme::Theme;
use crate::core::uad_lists::UadListState;
use crate::core::update::{get_latest_release, Release, SelfUpdateState, SelfUpdateStatus};
use crate::core::utils::string_to_theme;

use views::about::{About as AboutView, Message as AboutMessage};
use views::list::{List as AppsView, LoadingState as ListLoadingState, Message as AppsMessage};
use views::settings::{Message as SettingsMessage, Settings as SettingsView};
use widgets::navigation_menu::nav_menu;

use iced::widget::column;
use iced::{
    window::Settings as Window, Alignment, Application, Command, Element, Length, Renderer,
    Settings,
};
use std::{env, path::PathBuf};

#[cfg(feature = "self-update")]
use crate::core::update::{bin_name, download_update_to_temp_file, remove_file};

#[derive(Default, Debug, Clone)]
enum View {
    #[default]
    List,
    About,
    Settings,
}

#[derive(Default, Clone)]
pub struct UpdateState {
    self_update: SelfUpdateState,
    uad_list: UadListState,
}

#[derive(Default, Clone)]
pub struct UadGui {
    view: View,
    apps_view: AppsView,
    about_view: AboutView,
    settings_view: SettingsView,
    devices_list: Vec<Phone>,
    selected_device: Option<Phone>, // index of devices_list
    update_state: UpdateState,
    nb_running_async_adb_commands: u32,
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
    LoadDevices(Vec<Phone>),
    _NewReleaseDownloaded(Result<(PathBuf, PathBuf), ()>),
    GetLatestRelease(Result<Option<Release>, ()>),
    Nothing,
}

impl Application for UadGui {
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::batch([
                Command::perform(get_devices_list(), Message::LoadDevices),
                Command::perform(
                    async move { get_latest_release() },
                    Message::GetLatestRelease,
                ),
            ]),
        )
    }

    fn theme(&self) -> Theme {
        string_to_theme(&self.settings_view.general.theme)
    }

    fn title(&self) -> String {
        String::from("Universal Android Debloater")
    }
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            #[allow(clippy::option_if_let_else)]
            Message::LoadDevices(devices_list) => {
                self.selected_device = match &self.selected_device {
                    Some(s_device) => {
                        // Try to reload last selected phone
                        devices_list
                            .iter()
                            .find(|phone| phone.adb_id == s_device.adb_id)
                            .cloned()
                    }
                    None => devices_list.first().cloned(),
                };
                self.devices_list = devices_list;

                #[allow(unused_must_use)]
                {
                    self.update(Message::SettingsAction(SettingsMessage::LoadDeviceSettings));
                }

                self.update(Message::AppsAction(AppsMessage::LoadUadList(true)))
            }
            Message::AppsPress => {
                self.view = View::List;
                Command::none()
            }
            Message::AboutPressed => {
                self.view = View::About;
                self.update_state.self_update = SelfUpdateState::default();
                Command::perform(
                    async move { get_latest_release() },
                    Message::GetLatestRelease,
                )
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
            Message::RefreshButtonPressed => {
                self.apps_view = AppsView::default();
                Command::perform(get_devices_list(), Message::LoadDevices)
            }
            Message::RebootButtonPressed => {
                self.apps_view = AppsView::default();
                self.selected_device = None;
                self.devices_list = vec![];
                Command::perform(
                    perform_adb_commands("reboot".to_string(), CommandType::Shell),
                    |_| Message::Nothing,
                )
            }
            Message::AppsAction(msg) => self
                .apps_view
                .update(
                    &mut self.settings_view,
                    &mut self.selected_device.clone().unwrap_or_default(),
                    &mut self.update_state.uad_list,
                    msg,
                )
                .map(Message::AppsAction),
            Message::SettingsAction(msg) => {
                match msg {
                    SettingsMessage::RestoringDevice(ref output) => {
                        self.nb_running_async_adb_commands -= 1;
                        self.view = View::List;

                        #[allow(unused_must_use)]
                        {
                            self.apps_view.update(
                                &mut self.settings_view,
                                &mut self.selected_device.clone().unwrap_or_default(),
                                &mut self.update_state.uad_list,
                                AppsMessage::RestoringDevice(output.clone()),
                            );
                        }
                        if self.nb_running_async_adb_commands == 0 {
                            return self.update(Message::RefreshButtonPressed);
                        }
                    }
                    SettingsMessage::MultiUserMode(toggled) => {
                        if toggled {
                            for user in self.apps_view.phone_packages.clone() {
                                for (i, _) in
                                    user.iter().enumerate().filter(|&(_, pkg)| pkg.selected)
                                {
                                    for u in self
                                        .selected_device
                                        .as_ref()
                                        .unwrap()
                                        .user_list
                                        .iter()
                                        .filter(|&u| !u.protected)
                                    {
                                        self.apps_view.phone_packages[u.index][i].selected = true;
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }
                self.settings_view
                    .update(
                        &self.selected_device.clone().unwrap_or_default(),
                        &self.apps_view.phone_packages,
                        &mut self.nb_running_async_adb_commands,
                        msg,
                    )
                    .map(Message::SettingsAction)
            }
            Message::AboutAction(msg) => {
                self.about_view.update(msg.clone());

                match msg {
                    AboutMessage::UpdateUadLists => {
                        self.update_state.uad_list = UadListState::Downloading;
                        self.apps_view.loading_state =
                            ListLoadingState::DownloadingList(String::new());
                        self.update(Message::AppsAction(AppsMessage::LoadUadList(true)))
                    }
                    AboutMessage::DoSelfUpdate => {
                        #[cfg(feature = "self-update")]
                        if self.update_state.self_update.latest_release.is_some() {
                            self.update_state.self_update.status = SelfUpdateStatus::Updating;
                            self.apps_view.loading_state =
                                ListLoadingState::_UpdatingUad(String::new());
                            let bin_name = bin_name().to_owned();
                            let release = self
                                .update_state
                                .self_update
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
                    AboutMessage::UrlPressed(_) => Command::none(),
                }
            }
            Message::DeviceSelected(s_device) => {
                self.selected_device = Some(s_device.clone());
                self.view = View::List;
                env::set_var("ANDROID_SERIAL", s_device.adb_id);
                info!("{:-^65}", "-");
                info!(
                    "ANDROID_SDK: {} | DEVICE: {}",
                    s_device.android_sdk, s_device.model
                );
                info!("{:-^65}", "-");
                self.apps_view.loading_state = ListLoadingState::FindingPhones(String::new());

                #[allow(unused_must_use)]
                {
                    self.update(Message::SettingsAction(SettingsMessage::LoadDeviceSettings));
                }
                self.update(Message::AppsAction(AppsMessage::LoadPhonePackages((
                    self.apps_view.uad_lists.clone(),
                    UadListState::Done,
                ))))
            }
            Message::_NewReleaseDownloaded(res) => {
                debug!("UAD update has been download!");

                #[cfg(feature = "self-update")]
                if let Ok((relaunch_path, cleanup_path)) = res {
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

                    match std::process::Command::new(relaunch_path)
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
                            error!("Failed to update UAD: {}", error);
                        }
                    }
                } else {
                    error!("Failed to update UAD!");
                }
                Command::none()
            }
            Message::GetLatestRelease(release) => {
                match release {
                    Ok(r) => {
                        self.update_state.self_update.status = SelfUpdateStatus::Done;
                        self.update_state.self_update.latest_release = r;
                    }
                    Err(_) => self.update_state.self_update.status = SelfUpdateStatus::Failed,
                };
                Command::none()
            }
            Message::Nothing => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        let navigation_container = nav_menu(
            &self.devices_list,
            self.selected_device.clone(),
            &self.apps_view,
            &self.update_state.self_update,
        );

        let selected_device = self.selected_device.clone().unwrap_or_default();
        let main_container = match self.view {
            View::List => self
                .apps_view
                .view(&self.settings_view, &selected_device)
                .map(Message::AppsAction),
            View::About => self
                .about_view
                .view(&self.update_state)
                .map(Message::AboutAction),
            View::Settings => self
                .settings_view
                .view(&selected_device)
                .map(Message::SettingsAction),
        };

        column![navigation_container, main_container]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}

impl UadGui {
    pub fn start() -> iced::Result {
        Self::run(Settings {
            window: Window {
                size: (1050, 800),
                resizable: true,
                decorations: true,
                ..iced::window::Settings::default()
            },
            default_text_size: 17.0,
            ..Settings::default()
        })
    }
}
