pub mod style;
pub mod views;
pub mod widgets;

pub use views::about::About as AboutView;
pub use views::list::{List as AppsView, Message as AppsMessage};
pub use views::settings::{Settings as SettingsView, Message as SettingsMessage};
pub use crate::core::uad_lists::Package;
pub use crate::core::sync::{Phone};

use iced::{
    button, Alignment, Application, Button, Column, Command, Space,
    Container, Element, Length, Row, Settings, Text, window::Settings as Window,
};

#[cfg(feature = "wgpu")]
use iced::Svg;

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
    phone: Phone,
    view: View,
    apps_view: AppsView,
    about_view: AboutView,
    settings_view: SettingsView,
    about_btn: button::State,
    settings_btn: button::State,
    apps_btn: button::State,
    apps_refresh_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    AppsRefreshPress,
    AppsPress,

    AppsAction(AppsMessage),
    SettingsAction(SettingsMessage),
    Init(AppsMessage),
}

impl Application for UadGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(Self::load_phone_packages(), Message::Init),
        )
    }

    fn title(&self) -> String {
        String::from("UadGui")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Init(_) => {
                info!("ANDROID_SDK: {} | PHONE: {}", self.phone.android_sdk, self.phone.model);
                Command::perform(Self::load_phone_packages(), Message::AppsAction)
            }
            Message::AppsRefreshPress => {
                self.phone = Phone::default();
                self.settings_view = SettingsView::default();
                info!("{:-^65}", "-");
                info!("ANDROID_SDK: {} | PHONE: {}", self.phone.android_sdk, self.phone.model);
                self.apps_view = AppsView::default();
                self.view = View::List;
                Command::perform(Self::load_phone_packages(), Message::AppsAction)
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
            Message::AppsAction(msg) => {
                self.apps_view.update(&self.settings_view, &mut self.phone, msg).map(Message::AppsAction)
            }
            Message::SettingsAction(msg) => {
                self.settings_view.update(msg);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let apps_btn = Button::new(&mut self.apps_btn, Text::new("Apps"))
            .on_press(Message::AppsPress)
            .padding(5)
            .style(style::PrimaryButton(self.settings_view.theme.palette));

        #[cfg(feature = "wgpu")]
        #[allow(unused_variables)]
        let refresh_btn_display = Svg::from_path(
            format!("{}/ressources/assets/refresh.svg", env!("CARGO_MANIFEST_DIR"))
        )
            .width(Length::Units(17))
            .height(Length::Units(17));

        #[cfg(feature = "glow")] // No svg support
        let refresh_btn_display = Text::new("Refresh");

        let apps_refresh_btn = Button::new(&mut self.apps_refresh_btn, refresh_btn_display)
            .on_press(Message::AppsRefreshPress)
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

        let row = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(apps_refresh_btn)
            .push(Text::new("Device: ".to_string() + &self.phone.model))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(uad_version)
            .push(apps_btn)
            .push(about_btn)
            .push(settings_btn);

        let navigation_container = Container::new(row)
            .width(Length::Fill)
            .padding(10)
            .style(style::NavigationContainer(self.settings_view.theme.palette));

        let main_container = match self.view {
            View::List => self.apps_view.view(&self.settings_view, &self.phone).map(Message::AppsAction),
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

    pub async fn load_phone_packages() -> AppsMessage {
        AppsMessage::LoadPackages
    }
}