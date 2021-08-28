use iced::{button, container, scrollable, Background, Color, Vector};
// 331C12
pub const BORDER_COLOR: Color = Color::from_rgb(0.32, 0.41, 0.05);
pub const BUTTON_COLOR_DEFAULT: Color = Color::from_rgb(0.39, 0.48, 0.1);
pub const BUTTON_COLOR_HOVER: Color = Color::from_rgb(0.47, 0.58, 0.15);
pub const BACKGROUND_COLOR: Color = Color::from_rgb(0.27, 0.16, 0.11);
pub const NAVIGATION_COLOR: Color = Color::from_rgb(0.2, 0.11, 0.07);
pub const ROW_COLOR_PRIMARY: Color = Color::from_rgb(0.55, 0.44, 0.27);

pub enum PrimaryButton {
    Enabled,
    Disabled,
}

impl button::StyleSheet for PrimaryButton {
    fn active(&self) -> button::Style {
        match self {
            Self::Enabled => button::Style {
                background: Some(Background::Color(BUTTON_COLOR_DEFAULT)),
                border_color: BORDER_COLOR,
                border_width: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            },
            Self::Disabled => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.35, 0.43, 0.46))),
                border_color: Color::from_rgb(0.29, 0.19, 0.03),
                border_width: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Self::Enabled => button::Style {
                background: Some(Background::Color(BUTTON_COLOR_HOVER)),
                text_color: Color::WHITE,
                ..self.active()
            },
            Self::Disabled => button::Style {
                background: Some(Background::Color(Color::from_rgb8(91, 110, 117))),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..self.active()
            },
        }
    }
}

pub struct Content;
impl container::StyleSheet for Content {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BACKGROUND_COLOR)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct NavigationContainer;
impl container::StyleSheet for NavigationContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(NAVIGATION_COLOR)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct PackageRow;
impl button::StyleSheet for PackageRow {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ROW_COLOR_PRIMARY)),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ROW_COLOR_PRIMARY)),
            text_color: Color::WHITE,
            ..self.active()
        }
    }
    fn pressed(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::from_rgb(0.35, 0.43, 0.46))),
                text_color: Color::WHITE,
                ..self.active()
            }
        }
}

pub struct Scrollable;
impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: BUTTON_COLOR_DEFAULT,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..active.scroller },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}