use iced::Color;

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Lupin,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub const ALL: [Self; 3] = [Self::Lupin, Self::Dark, Self::Light];
    pub fn palette(&self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#111111").unwrap(),
                    foreground: hex_to_color("#1C1C1C").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#5E4266").unwrap(),
                    secondary: hex_to_color("#386e50").unwrap(),
                    surface: hex_to_color("#828282").unwrap(),
                    error: hex_to_color("#992B2B").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#BA84FC").unwrap(),
                    secondary: hex_to_color("#49eb7a").unwrap(),
                    surface: hex_to_color("#E0E0E0").unwrap(),
                    error: hex_to_color("#C13047").unwrap(),
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#EEEEEE").unwrap(),
                    foreground: hex_to_color("#E0E0E0").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#230F08").unwrap(),
                    secondary: hex_to_color("#F9D659").unwrap(),
                    surface: hex_to_color("#818181").unwrap(),
                    error: hex_to_color("#992B2B").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#673AB7").unwrap(),
                    secondary: hex_to_color("#3797A4").unwrap(),
                    surface: hex_to_color("#000000").unwrap(),
                    error: hex_to_color("#C13047").unwrap(),
                },
            },
            Self::Lupin => ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#282a36").unwrap(),
                    foreground: hex_to_color("#353746").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#58406F").unwrap(),
                    secondary: hex_to_color("#386e50").unwrap(),
                    surface: hex_to_color("#a2a4a3").unwrap(),
                    error: hex_to_color("#A13034").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#bd94f9").unwrap(),
                    secondary: hex_to_color("#49eb7a").unwrap(),
                    surface: hex_to_color("#f4f8f3").unwrap(),
                    error: hex_to_color("#E63E6D").unwrap(),
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Dark => "Dark",
                Theme::Light => "Light",
                Theme::Lupin => "Lupin",
            }
        )
    }
}

fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}
