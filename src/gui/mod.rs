pub mod style;
pub mod views;

pub use views::about::About as AboutView;
pub use views::list::{List as ListView, Message as ListMessage};
pub use views::settings::{Settings as SettingsView, Message as SettingsMessage};

// use crate::core::sync::list_phone_packages;

use iced::{
    button, Align, Application, Button, Clipboard, Column, Command, Space,
    Container, Element, Length, Row, Settings, Text, HorizontalAlignment, VerticalAlignment
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
    packages_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(State),

    // Navigation Panel
    AboutPressed,
    SettingsPressed,
    PackagesPressed,

    ListAction(ListMessage),
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
            about_btn: button::State::default(),
            settings_btn: button::State::default(),
            packages_btn: button::State::default(),
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
            Command::perform(Self::load_packages(), Message::Loaded),
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
                Command::none()
            }

            UadGui::Loaded(state) => match message {
                Message::PackagesPressed => {
                    state.view = View::List;
                    state.list_view.update(ListMessage::LoadPackages);
                    Command::none()
                }
                Message::AboutPressed => {
                    state.view = View::About;
                    Command::none()
                }
                Message::SettingsPressed => {
                    state.view = View::Settings;
                    Command::none()
                }
                Message::ListAction(msg) => {
                    state.list_view.update(msg).map(Message::ListAction)
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
                packages_btn,
                settings_btn, 

            }) => {
                let packages_btn = Button::new(packages_btn, Text::new("List"))
                    .on_press(Message::PackagesPressed)
                    .padding(5)
                    .style(style::PrimaryButton::Enabled);
                let divider = Space::new(Length::Fill, Length::Shrink);
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
                    .align_items(Align::End)
                    .spacing(10)
                    .push(packages_btn)
                    .push(divider)
                    .push(about_btn)
                    .push(settings_btn);

                let navigation_container = Container::new(row)
                    .width(Length::Fill)
                    .padding(10)
                    .style(style::NavigationContainer);

                match view {
                    View::List => {
                        let main_container = list_view.view().map(Message::ListAction);
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
        UadGui::run(Settings::default()).unwrap_err();
    }

    pub async fn load_packages() -> State {
        State::default()
    }
}

fn loading_data<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Packages loading...")
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