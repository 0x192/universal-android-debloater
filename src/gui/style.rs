use crate::core::theme::Theme;
use iced::overlay::menu;
use iced::widget::{button, checkbox, container, pick_list, scrollable, text, text_input};
use iced::{application, Background, Color};

#[derive(Debug, Clone, Copy)]
pub enum Application {
    Default,
}

impl Default for Application {
    fn default() -> Self {
        Self::Default
    }
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette.base.background,
            text_color: self.palette.base.foreground,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Container {
    Content,
    Navigation,
    Description,
}

impl Default for Container {
    fn default() -> Self {
        Self::Content
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: Self::Style) -> container::Appearance {
        let from_appearance = |c: Color| container::Appearance {
            background: Some(Background::Color(c)),
            text_color: Some(self.palette.bright.surface),
            ..container::Appearance::default()
        };

        match style {
            Container::Content => from_appearance(self.palette.base.background),
            Container::Navigation => from_appearance(self.palette.base.background),
            Container::Description => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.bright.surface),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: self.palette.base.background,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Primary,
    Unavailable,
    SelfUpdate,
    Refresh,
    UninstallPackage,
    RestorePackage,
    NormalPackage,
    SelectedPackage,
}

impl Default for Button {
    fn default() -> Self {
        Self::Primary
    }
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: Self::Style) -> button::Appearance {
        let p = self.palette;

        let appearance = button::Appearance {
            border_width: 1.0,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border_color: Color { a: 0.5, ..mc },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary => active_appearance(None, p.bright.primary),
            Button::Unavailable => active_appearance(None, p.bright.error),
            Button::Refresh => active_appearance(None, p.bright.primary),
            Button::SelfUpdate => active_appearance(None, p.bright.primary),
            Button::UninstallPackage => active_appearance(None, p.bright.error),
            Button::RestorePackage => active_appearance(None, p.bright.secondary),
            Button::NormalPackage => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: p.base.background,
                ..appearance
            },
            Button::SelectedPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..p.normal.primary
                })),
                text_color: p.bright.primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: p.normal.primary,
                ..appearance
            },
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette;

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary => hover_appearance(p.bright.primary, None),
            Button::Unavailable => hover_appearance(p.bright.error, None),
            Button::Refresh => hover_appearance(p.bright.primary, None),
            Button::SelfUpdate => hover_appearance(p.bright.primary, None),
            Button::UninstallPackage => hover_appearance(p.bright.error, None),
            Button::RestorePackage => hover_appearance(p.bright.secondary, None),
            Button::NormalPackage => hover_appearance(p.normal.primary, Some(p.bright.surface)),
            Button::SelectedPackage => hover_appearance(p.normal.primary, None),
        }
    }

    fn disabled(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette;

        let disabled_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.05, ..bg })),
            text_color: Color {
                a: 0.50,
                ..tc.unwrap_or(bg)
            },
            ..active
        };

        match style {
            Button::RestorePackage => disabled_appearance(p.normal.primary, Some(p.bright.primary)),
            Button::UninstallPackage => disabled_appearance(p.bright.error, None),
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

impl Default for Scrollable {
    fn default() -> Self {
        Self::Description
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Scrollable {
    Description,
    Packages,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: Self::Style) -> scrollable::Scrollbar {
        let from_appearance = |c: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette.base.foreground,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        };

        match style {
            Scrollable::Description => from_appearance(self.palette.base.foreground),
            Scrollable::Packages => from_appearance(self.palette.base.background),
        }
    }

    fn hovered(&self, style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                ..self.active(style).scroller
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CheckBox {
    PackageEnabled,
    PackageDisabled,
    SettingsEnabled,
    SettingsDisabled,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self::PackageEnabled
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: Self::Style, _is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckBox::PackageEnabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.background),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.base.background,
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::PackageDisabled => checkbox::Appearance {
                background: Background::Color(Color {
                    a: 0.55,
                    ..self.palette.base.background
                }),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.normal.primary,
                text_color: Some(self.palette.normal.primary),
            },
            CheckBox::SettingsEnabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.background),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.bright.primary,
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::SettingsDisabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.foreground),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.normal.primary,
                text_color: Some(self.palette.normal.primary),
            },
        }
    }

    fn hovered(&self, style: Self::Style, is_checked: bool) -> checkbox::Appearance {
        let from_appearance = || checkbox::Appearance {
            background: Background::Color(self.palette.base.foreground),
            checkmark_color: self.palette.bright.primary,
            border_radius: 5.0,
            border_width: 2.0,
            border_color: self.palette.bright.primary,
            text_color: Some(self.palette.bright.surface),
        };

        match style {
            CheckBox::PackageEnabled => from_appearance(),
            CheckBox::SettingsEnabled => from_appearance(),
            CheckBox::PackageDisabled => self.active(style, is_checked),
            CheckBox::SettingsDisabled => self.active(style, is_checked),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextInput {
    Default,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::Default
    }
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette.base.foreground),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: self.palette.base.foreground,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette.base.foreground),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette.normal.primary
            },
        }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        self.palette.normal.surface
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        self.palette.bright.primary
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        self.palette.normal.primary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PickList {
    Default,
}

impl Default for PickList {
    fn default() -> Self {
        Self::Default
    }
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> menu::Appearance {
        let p = self.palette;

        menu::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border_width: 1.0,
            border_radius: 2.0,
            border_color: p.base.background,
            selected_text_color: p.bright.surface,
            selected_background: p.normal.primary.into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: ()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette.bright.surface,
            background: self.palette.base.background.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette.normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: self.palette.bright.surface,
        }
    }

    fn hovered(&self, style: ()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            text_color: self.palette.bright.primary,
            ..active
        }
    }
}

#[derive(Clone, Copy)]
pub enum Text {
    Default,
    Color(Color),
}

impl Default for Text {
    fn default() -> Self {
        Self::Default
    }
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}
