use iced::{button, container, scrollable, Background, Color, Vector, checkbox};
// 331C12
pub const BORDER_COLOR: Color = Color::from_rgb(0.32, 0.41, 0.05);
pub const BUTTON_COLOR_DEFAULT: Color = Color::from_rgb(0.39, 0.48, 0.1);
pub const BUTTON_COLOR_HOVER: Color = Color::from_rgb(0.47, 0.58, 0.15);
pub const BACKGROUND_COLOR: Color = Color::from_rgb(0.27, 0.16, 0.11);
pub const NAVIGATION_COLOR: Color = Color::from_rgb(0.2, 0.11, 0.07);
pub const ROW_COLOR_PRIMARY: Color = Color::from_rgb(0.55, 0.44, 0.27);
pub const UNINSTALL_BUTTON_COLOR: Color = Color::from_rgb(0.6, 0.1, 0.0);
pub const UNINSTALL_BORDER_COLOR: Color = Color::from_rgb(0.48, 0.01, 0.0);
pub const UNINSTALL_COLOR_HOVER: Color = Color::from_rgb(0.60, 0.05, 0.0);


/// Color for disabled elements
pub const DISABLED_COLOR: Color = Color::from_rgb(
    195 as f32 / 255.0,
    195 as f32 / 255.0,
    195 as f32 / 255.0,
);

pub enum PrimaryButton {
    Enabled,
    //Disabled,
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
            /*Self::Disabled => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.35, 0.43, 0.46))),
                border_color: Color::from_rgb(0.29, 0.19, 0.03),
                border_width: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            },*/
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Self::Enabled => button::Style {
                background: Some(Background::Color(BUTTON_COLOR_HOVER)),
                text_color: Color::WHITE,
                ..self.active()
            },
            /*Self::Disabled => button::Style {
                background: Some(Background::Color(Color::from_rgb8(91, 110, 117))),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..self.active()
            },*/
        }
    }
}


pub enum PackageButton {
    Uninstall,
    Restore,
}

impl button::StyleSheet for PackageButton {
    fn active(&self) -> button::Style {
        match self {
            Self::Uninstall => button::Style {
                background: Some(Background::Color(UNINSTALL_BUTTON_COLOR)),
                border_color: UNINSTALL_BORDER_COLOR,
                border_width: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            },
            Self::Restore => button::Style {
                background: Some(Background::Color(BUTTON_COLOR_DEFAULT)),
                border_color: BORDER_COLOR,
                border_width: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Self::Restore => button::Style {
                background: Some(Background::Color(BUTTON_COLOR_HOVER)),
                text_color: Color::WHITE,
                ..self.active()
            },
            Self::Uninstall => button::Style {
                background: Some(Background::Color(UNINSTALL_COLOR_HOVER)),
                text_color: Color::WHITE,
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

pub struct Description;
impl container::StyleSheet for Description {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ROW_COLOR_PRIMARY)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
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


pub enum SelectionCheckBox {
    Enabled,
    Disabled
}

impl checkbox::StyleSheet for SelectionCheckBox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        match self {
            Self::Enabled => checkbox::Style {
                background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
                checkmark_color: Color::from_rgb(0.3, 0.3, 0.3),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: Color::from_rgb(0.6, 0.6, 0.6),
            },
            Self::Disabled => checkbox::Style {
                    background: Background::Color(DISABLED_COLOR),
                    border_color: Color::TRANSPARENT,
                    checkmark_color: Color::from_rgb(0.3, 0.3, 0.3),
                    border_radius: 5.0,
                    border_width: 1.0,
            },
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        match self {
            Self::Enabled => checkbox::Style {
                background: Background::Color(Color::from_rgb(0.90, 0.90, 0.90)),
                ..self.active(is_checked)
            },

            Self::Disabled => checkbox::Style {
                    background: Background::Color(DISABLED_COLOR),
                    border_color: Color::TRANSPARENT,
                    checkmark_color: Color::from_rgb(0.3, 0.3, 0.3),
                    border_radius: 5.0,
                    border_width: 1.0,
            },
        }
    }
}