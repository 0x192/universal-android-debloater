pub mod style;
pub mod views;

pub use views::about::About as AboutView;
pub use views::list::{List as ListView, Message as ListMessage};
pub use views::settings::{Settings as SettingsView, Message as SettingsMessage};
pub use crate::core::uad_lists::{ load_debloat_lists, Package };
pub use crate::core::sync::get_phone_brand;
use std::{collections::HashMap};
use static_init::{dynamic};

use iced::{
    button, Align, Application, Button, Clipboard, Column, Command, Space,
    Container, Element, Length, Row, Settings, Text, HorizontalAlignment, 
    VerticalAlignment, window::Settings as Window, Svg,
};

#[dynamic]
static UAD_LISTS: HashMap<String, Package> = load_debloat_lists();

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

#[derive(Debug, Clone)]
pub enum UadGui {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
pub struct State {
    view: View,
    list_view: ListView,
    about_view: AboutView,
    settings_view: SettingsView,
    input_value: String,
    about_btn: button::State,
    settings_btn: button::State,
    catalog_btn: button::State,
    device_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(State),

    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    CatalogRefreshPress,

    CatalogAction(ListMessage),
    SettingsAction(SettingsMessage),
}

impl Default for State {
    fn default() -> Self {
        Self {
            view: View::default(),
            list_view: ListView::default(),
            about_view: AboutView::default(),
            settings_view: SettingsView::default(),
            input_value: "".to_string(),
            device_name: "No phone connected".to_string(),
            about_btn: button::State::default(),
            settings_btn: button::State::default(),
            catalog_btn: button::State::default(),
        }
    }
}

impl Application for UadGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(Self::init_application(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("UadGui")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match self {
            UadGui::Loading => {
                if let Message::Loaded(_state) = message {
                    *self = UadGui::Loaded(State { ..State::default() });
                }
                Command::perform(Self::load_phone_packages(), Message::CatalogAction)
            }

            UadGui::Loaded(state) => match message {
                Message::CatalogRefreshPress => {
                    state.list_view = ListView::default();
                    state.list_view.update(ListMessage::LoadSettings(state.settings_view.clone()));
                    state.view = View::List;
                    Command::perform(Self::load_phone_packages(), Message::CatalogAction)

                },
                Message::AboutPressed => {
                    state.view = View::About;
                    Command::none()
                }
                Message::SettingsPressed => {
                    state.view = View::Settings;
                    Command::none()
                }
                Message::CatalogAction(msg) => {
                    state.device_name = get_phone_brand();
                    state.list_view.update(msg).map(Message::CatalogAction)
                }
                Message::SettingsAction(msg) => {
                    state.settings_view.update(msg);
                    Command::none()
                }
                Message::Loaded(_) => Command::none(),
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            UadGui::Loading => loading_data(),
            UadGui::Loaded(State { 
                view,
                list_view,
                about_view,
                settings_view,
                input_value:_,
                about_btn,
                catalog_btn,
                settings_btn,
                device_name,

            }) => {
                let add_svg_path = format!("{}/ressources/assets/refresh.svg", env!("CARGO_MANIFEST_DIR"));
                let refresh_list_icon = Svg::from_path(add_svg_path);

                let catalog_btn = Button::new(catalog_btn, 
                        Row::new()
                        .push(refresh_list_text)
                        .push(Text::new("Catalog "))
                    )
                    .on_press(Message::CatalogRefreshPress)
                    .padding(5)
                    .style(style::PrimaryButton::Enabled);

                let about_btn = Button::new(about_btn, Text::new("About"))
                    .on_press(Message::AboutPressed)
                    .padding(5)
                    .style(style::PrimaryButton::Enabled);

                let settings_btn = Button::new(settings_btn, Text::new("Settings"))
                    .on_press(Message::SettingsPressed)
                    .padding(5)
                    .style(style::PrimaryButton::Enabled);

                let row = Row::new()
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(Text::new("Device: ".to_string() + &device_name))
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(catalog_btn)
                    .push(about_btn)
                    .push(settings_btn);

                let navigation_container = Container::new(row)
                    .width(Length::Fill)
                    .padding(10)
                    .style(style::NavigationContainer);

                match view {
                    View::List => {
                        let main_container = list_view.view().map(Message::CatalogAction);
                        Column::new()
                            .width(Length::Fill)
                            .push(navigation_container)
                            .push(main_container)
                            .into()
                    }
                    View::About => {
                        let main_container = about_view.view();
                        Column::new()
                            .width(Length::Fill)
                            .push(navigation_container)
                            .push(main_container)
                            .into()
                    }
                    View::Settings => {
                        let main_container = settings_view.view().map(Message::SettingsAction);
                        Column::new()
                            .width(Length::Fill)
                            .push(navigation_container)
                            .push(main_container)
                            .into()
                    }
                }
            }

        }
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
        UadGui::run(settings).unwrap_err();
    }

    pub async fn init_application() -> State {
        State::default()
    }
    pub async fn load_phone_packages() -> ListMessage {
        ListMessage::LoadPackages(&UAD_LISTS)
    }
}

fn loading_data<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Please wait...")
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