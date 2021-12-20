pub mod style;
pub mod views;
pub mod widgets;

pub use crate::core::sync::{get_device_list, Phone};
pub use crate::core::uad_lists::Package;
use crate::core::utils::icon;
use std::env;
pub use views::about::About as AboutView;
pub use views::list::{List as AppsView, Message as AppsMessage};
pub use views::settings::{
    Message as SettingsMessage, Phone as SettingsPhone, Settings as SettingsView,
};

use iced::{
    button, pick_list, window::Settings as Window, Alignment, Application, Button, Column, Command,
    Container, Element, Font, Length, PickList, Row, Settings, Space, Text,
};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../resources/assets/icons.ttf"),
};

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

#[derive(Debug, Default)]
pub struct UadGui {
    view: View,
    apps_view: AppsView,
    about_view: AboutView,
    settings_view: SettingsView,
    about_btn: button::State,
    settings_btn: button::State,
    apps_btn: button::State,
    apps_refresh_btn: button::State,
    device_picklist: pick_list::State<Phone>,
    device_list: Vec<Phone>,
    selected_device: Option<Phone>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    LoadDevices(usize),
    AppsPress,

    DeviceSelected(Phone),
    AppsAction(AppsMessage),
    SettingsAction(SettingsMessage),
    RefreshButtonPressed,
    LoadDeviceList(Vec<Phone>),
    Init(Vec<Phone>),
}

impl Application for UadGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(get_device_list(), Message::Init),
        )
    }

    fn title(&self) -> String {
        String::from("Universal Android Debloater")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Init(device_list) => {
                self.device_list = device_list;
                self.selected_device = Some(Phone::default());
                Command::perform(Self::refresh(10), Message::LoadDevices)
            }
            Message::LoadDeviceList(device_list) => {
                self.device_list = device_list;

                // Save the current selected device
                let i = self
                    .device_list
                    .iter()
                    .position(|phone| *phone == self.selected_device.clone().unwrap_or_default())
                    .unwrap_or(0);
                self.selected_device = Some(Phone::default());
                Command::perform(Self::refresh(i), Message::LoadDevices)
            }
            Message::RefreshButtonPressed => {
                self.apps_view.ready = false;
                Command::batch([
                    Command::perform(Self::please_wait(), Message::AppsAction),
                    Command::perform(Self::device_lists(), Message::LoadDeviceList),
                ])
            }
            Message::LoadDevices(last_selected_device) => {
                self.settings_view.phone = SettingsPhone::default();

                // Try to reload last selected phone
                if !self.device_list.is_empty() {
                    self.selected_device = match last_selected_device < self.device_list.len() {
                        true => Some(self.device_list[last_selected_device].clone()),
                        false => match self.device_list.last() {
                            Some(last) => Some(last.clone()),
                            None => Some(Phone::default()),
                        },
                    };
                    self.apps_view = AppsView::default();
                    self.view = View::List;
                    Command::perform(
                        Self::load_phone_packages(self.selected_device.clone().unwrap()),
                        Message::AppsAction,
                    )
                } else {
                    self.selected_device = None;
                    Command::none()
                }
            }
            Message::AppsPress => {
                self.view = View::List;
                Command::none()
            }
            Message::AboutPressed => {
                self.view = View::About;
                Command::none()
            }
            Message::SettingsPressed => {
                self.view = View::Settings;
                Command::none()
            }
            Message::AppsAction(msg) => self
                .apps_view
                .update(
                    &self.settings_view.phone,
                    &mut self.selected_device.clone().unwrap_or_default(),
                    msg,
                )
                .map(Message::AppsAction),
            Message::SettingsAction(msg) => {
                self.settings_view.update(msg);
                Command::none()
            }
            Message::DeviceSelected(device) => {
                self.selected_device = Some(device);
                self.apps_view = AppsView::default();
                self.view = View::List;
                Command::perform(
                    Self::load_phone_packages(self.selected_device.clone().unwrap()),
                    Message::AppsAction,
                )
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let apps_btn = Button::new(&mut self.apps_btn, Text::new("Apps"))
            .on_press(Message::AppsPress)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        let apps_refresh_btn = Button::new(&mut self.apps_refresh_btn, refresh_icon())
            .on_press(Message::RefreshButtonPressed)
            .padding(5)
            .style(style::RefreshButton(self.settings_view.theme.palette));

        let uad_version = Text::new(env!("CARGO_PKG_VERSION"));

        let about_btn = Button::new(&mut self.about_btn, Text::new("About"))
            .on_press(Message::AboutPressed)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        let settings_btn = Button::new(&mut self.settings_btn, Text::new("Settings"))
            .on_press(Message::SettingsPressed)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        let device_picklist = PickList::new(
            &mut self.device_picklist,
            &self.device_list,
            self.selected_device.clone(),
            Message::DeviceSelected,
        )
        .style(style::PickList(self.settings_view.theme.palette));

        let row = match self.selected_device {
            Some(_) => Row::new()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
                .push(apps_refresh_btn)
                .push(device_picklist)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(uad_version)
                .push(apps_btn)
                .push(about_btn)
                .push(settings_btn),
            None => Row::new()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
                .push(apps_refresh_btn)
                .push(Text::new("no devices/emulators found"))
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(uad_version)
                .push(apps_btn)
                .push(about_btn)
                .push(settings_btn),
        };

        let navigation_container = Container::new(row)
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
            View::About => self.about_view.view(&self.settings_view),
            View::Settings => self.settings_view.view().map(Message::SettingsAction),
        };

        Column::new()
            .width(Length::Fill)
            .push(navigation_container)
            .push(main_container)
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

    pub async fn refresh(i: usize) -> usize {
        i
    }
    pub async fn device_lists() -> Vec<Phone> {
        get_device_list().await
    }
    pub async fn please_wait() -> AppsMessage {
        AppsMessage::Nothing
    }
}

fn refresh_icon() -> Text {
    icon('\u{E900}')
}
