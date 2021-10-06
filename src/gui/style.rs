use iced::{
    button, container, scrollable, Background, Color, checkbox, 
    text_input, pick_list,
};
use crate::core::theme::ColorPalette;

pub struct Content(pub ColorPalette);
impl container::StyleSheet for Content {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.0.base.background)),
            text_color: Some(self.0.bright.surface),
            ..container::Style::default()
        }
    }
}

pub struct NavigationContainer(pub ColorPalette);
impl container::StyleSheet for NavigationContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.surface),
            ..container::Style::default()
        }
    }
}

pub struct PrimaryButton(pub ColorPalette);
impl button::StyleSheet for PrimaryButton {
    fn active(&self) -> button::Style {
        button::Style {
            border_color: Color {
                a: 0.5,
                ..self.0.bright.primary
            },
            border_width: 1.0,
            border_radius: 2.0,
            text_color: self.0.bright.primary,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.surface
            })),
            text_color: self.0.bright.primary,
            ..self.active()
        }
    }
}

pub struct RefreshButton(pub ColorPalette);
impl button::StyleSheet for RefreshButton {
    fn active(&self) -> button::Style {
        button::Style {
            border_color: Color {
                a: 0.5,
                ..self.0.bright.primary
            },
            border_width: 1.0,
            border_radius: 2.0,
            text_color: self.0.bright.primary,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.surface
            })),
            text_color: self.0.bright.primary,
            ..self.active()
        }
    }
}


pub enum PackageButton {
    Uninstall(ColorPalette),
    Restore(ColorPalette),
}

impl button::StyleSheet for PackageButton {
    fn active(&self) -> button::Style {
        match self {
            Self::Uninstall(palette) => button::Style {
                border_color: Color {
                    a: 0.5,
                    ..palette.bright.error
                },
                border_width: 1.0,
                border_radius: 2.0,
                text_color: palette.bright.error,
                ..button::Style::default()
            },
            Self::Restore(palette) => button::Style {
                border_color: Color {
                    a: 0.5,
                    ..palette.bright.secondary
                },
                border_width: 1.0,
                border_radius: 2.0,
                text_color: palette.bright.secondary,
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Self::Restore(palette) => button::Style {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active()
            },
            Self::Uninstall(palette) => button::Style {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active()
            },
        }
    }

    fn disabled(&self) -> button::Style {
        match self {
            Self::Restore(palette) => button::Style {
                background: Some(Background::Color(Color {
                a: 0.05,
                ..palette.normal.primary
                })),
                text_color: Color {
                    a: 0.50,
                    ..palette.bright.primary
                },
                ..self.active()
            },
            Self::Uninstall(palette) => button::Style {
                background: Some(Background::Color(Color {
                a: 0.05,
                ..palette.normal.error
                })),
                text_color: Color {
                    a: 0.50,
                    ..palette.normal.error
                },
                ..self.active()
            },
        }
    }
}

pub struct PackageRow(pub ColorPalette);
impl button::StyleSheet for PackageRow {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: self.0.bright.surface,
            ..button::Style::default()
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            text_color: self.0.bright.primary,
            ..self.active()
        }
    }
    fn pressed(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::from_rgb(0.35, 0.43, 0.46))),
                text_color: self.0.bright.primary,
                ..self.active()
            }
        }
}

pub struct Description(pub ColorPalette);
impl container::StyleSheet for Description {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.surface),
            ..container::Style::default()
        }
    }
}

pub struct Scrollable(pub ColorPalette);
impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.0.base.background)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.0.base.foreground,
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
    Enabled(ColorPalette),
    Disabled(ColorPalette)
}

impl checkbox::StyleSheet for SelectionCheckBox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        match self {
            Self::Enabled(palette) => checkbox::Style {
                background: Background::Color(palette.base.background),
                checkmark_color: palette.bright.primary,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: palette.normal.primary,
            },
            Self::Disabled(palette) => checkbox::Style {
                background: Background::Color(palette.base.foreground),
                checkmark_color: palette.bright.primary,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: palette.normal.primary,
            },
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        match self {
            Self::Enabled(palette) => checkbox::Style {
                background: Background::Color(palette.base.foreground),
                checkmark_color: palette.bright.primary,
                border_radius: 2.0,
                border_width: 2.0,
                border_color: palette.bright.primary,
            },

            Self::Disabled(_) => checkbox::Style {
                ..self.active(is_checked)
            },
        }
    }
}

pub struct SearchInput(pub ColorPalette);
impl text_input::StyleSheet for SearchInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.0.base.foreground),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: self.0.base.foreground,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.0.base.foreground),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }

    fn placeholder_color(&self) -> Color {
        self.0.normal.surface
    }

    fn value_color(&self) -> Color {
        self.0.bright.primary
    }

    fn selection_color(&self) -> Color {
        self.0.bright.secondary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self) -> text_input::Style {
        self.focused()
    }
}

pub struct PickList(pub ColorPalette);
impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            text_color: self.0.bright.surface,
            background: Background::Color(self.0.base.foreground),
            border_width: 1.0,
            border_color: self.0.base.background,
            selected_background: Background::Color(Color {
                a: 0.15,
                ..self.0.normal.primary
            }),
            selected_text_color: self.0.bright.primary,
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: self.0.bright.surface,
            background: self.0.base.background.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: self.0.bright.surface,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        let active = self.active();
        pick_list::Style {
            text_color: self.0.bright.primary,
            ..active
        }
    }
}
