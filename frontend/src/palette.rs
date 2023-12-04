use ratatui::style::{Color, Style};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GruvboxColor {
    Neutral(GruvboxNeutral),
    Accent(GruvboxAccent),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GruvboxNeutral {
    Dark(Shade),
    Light(Shade),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shade {
    Darkest,
    Darker,
    Lighter,
    Lightest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GruvboxAccent {
    Red,
    BurntOrange,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Pink,
}

impl GruvboxColor {
    pub const fn color_index(self) -> u8 {
        match self {
            GruvboxColor::Neutral(c) => match c {
                GruvboxNeutral::Dark(s) => match s {
                    Shade::Darkest => 0,
                    Shade::Darker => 1,
                    Shade::Lighter => 2,
                    Shade::Lightest => 3,
                },
                GruvboxNeutral::Light(s) => match s {
                    Shade::Darkest => 4,
                    Shade::Darker => 5,
                    Shade::Lighter => 6,
                    Shade::Lightest => 7,
                },
            },
            GruvboxColor::Accent(c) => match c {
                GruvboxAccent::Red => 8,
                GruvboxAccent::BurntOrange => 9,
                GruvboxAccent::Orange => 10,
                GruvboxAccent::Yellow => 11,
                GruvboxAccent::Green => 12,
                GruvboxAccent::Teal => 13,
                GruvboxAccent::Blue => 14,
                GruvboxAccent::Pink => 15,
            },
        }
    }

    pub const fn to_color(self) -> Color {
        Color::Indexed(self.color_index())
    }

    pub const fn to_color_str(self) -> &'static str {
        indexed_color_str(self.color_index())
    }

    /// Creates a style that uses this color as the fforeground color and the default background
    /// color as the background.
    pub const fn fg_style(self) -> Style {
        Style::new()
            .fg(self.to_color())
            .bg(Self::default_bg().to_color())
    }

    /// Creates a style that uses this color as the background color and the default foreground
    /// color as the foreground.
    pub const fn bg_style(self) -> Style {
        Style::new()
            .fg(Self::default_fg().to_color())
            .bg(self.to_color())
    }

    /// Creates a style that uses this and another color. This color is used as the foreground and
    /// the other as the background color.
    pub const fn full_style(self, other: Self) -> Style {
        Style::new().fg(self.to_color()).bg(other.to_color())
    }

    pub const fn default_style() -> Style {
        Style::new()
            .fg(Self::default_fg().to_color())
            .bg(Self::default_bg().to_color())
    }

    pub const fn default_fg() -> Self {
        Self::light_3()
    }

    pub const fn default_bg() -> Self {
        Self::dark_2()
    }

    pub const fn dark_1() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Darkest))
    }

    pub const fn dark_2() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Darker))
    }

    pub const fn dark_3() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Lighter))
    }

    pub const fn dark_4() -> Self {
        Self::Neutral(GruvboxNeutral::Dark(Shade::Lightest))
    }

    pub const fn light_1() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Darkest))
    }

    pub const fn light_2() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Darker))
    }

    pub const fn light_3() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Lighter))
    }

    pub const fn light_4() -> Self {
        Self::Neutral(GruvboxNeutral::Light(Shade::Lightest))
    }

    pub const fn red() -> Self {
        Self::Accent(GruvboxAccent::Red)
    }

    pub const fn burnt_orange() -> Self {
        Self::Accent(GruvboxAccent::BurntOrange)
    }

    pub const fn orange() -> Self {
        Self::Accent(GruvboxAccent::Orange)
    }

    pub const fn yellow() -> Self {
        Self::Accent(GruvboxAccent::Yellow)
    }

    pub const fn green() -> Self {
        Self::Accent(GruvboxAccent::Green)
    }

    pub const fn teal() -> Self {
        Self::Accent(GruvboxAccent::Teal)
    }

    pub const fn blue() -> Self {
        Self::Accent(GruvboxAccent::Blue)
    }

    pub const fn pink() -> Self {
        Self::Accent(GruvboxAccent::Pink)
    }
}

pub const fn indexed_gruvbox(i: u8) -> GruvboxColor {
    match i {
        0 => GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Darkest)),
        1 => GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Darker)),
        2 => GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Lighter)),
        3 => GruvboxColor::Neutral(GruvboxNeutral::Dark(Shade::Lightest)),
        4 => GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Darkest)),
        5 => GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Darker)),
        6 => GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Lighter)),
        7 => GruvboxColor::Neutral(GruvboxNeutral::Light(Shade::Lightest)),
        8 => GruvboxColor::Accent(GruvboxAccent::Red),
        9 => GruvboxColor::Accent(GruvboxAccent::BurntOrange),
        10 => GruvboxColor::Accent(GruvboxAccent::Orange),
        11 => GruvboxColor::Accent(GruvboxAccent::Yellow),
        12 => GruvboxColor::Accent(GruvboxAccent::Green),
        13 => GruvboxColor::Accent(GruvboxAccent::Teal),
        14 => GruvboxColor::Accent(GruvboxAccent::Blue),
        15 => GruvboxColor::Accent(GruvboxAccent::Pink),
        _ => panic!("Unknown color index!!"),
    }
}

pub const fn indexed_color(i: u8) -> Color {
    match i {
        0 => BASE_0_RGB,
        1 => BASE_1_RGB,
        2 => BASE_2_RGB,
        3 => BASE_3_RGB,
        4 => BASE_4_RGB,
        5 => BASE_5_RGB,
        6 => BASE_6_RGB,
        7 => BASE_7_RGB,
        8 => BASE_8_RGB,
        9 => BASE_9_RGB,
        10 => BASE_A_RGB,
        11 => BASE_B_RGB,
        12 => BASE_C_RGB,
        13 => BASE_D_RGB,
        14 => BASE_E_RGB,
        15 => BASE_F_RGB,
        _ => panic!("Unknown color index!!"),
    }
}

pub const fn indexed_color_str(i: u8) -> &'static str {
    match i {
        0 => BASE_0_HEX,
        1 => BASE_1_HEX,
        2 => BASE_2_HEX,
        3 => BASE_3_HEX,
        4 => BASE_4_HEX,
        5 => BASE_5_HEX,
        6 => BASE_6_HEX,
        7 => BASE_7_HEX,
        8 => BASE_8_HEX,
        9 => BASE_9_HEX,
        10 => BASE_A_HEX,
        11 => BASE_B_HEX,
        12 => BASE_C_HEX,
        13 => BASE_D_HEX,
        14 => BASE_E_HEX,
        15 => BASE_F_HEX,
        _ => panic!("Unknown color index!!"),
    }
}

// Darks
const BASE_0_HEX: &str = "#1d2021";
const BASE_1_HEX: &str = "#3c3836";
const BASE_2_HEX: &str = "#504945";
const BASE_3_HEX: &str = "#665c54";

// Lights
const BASE_4_HEX: &str = "#bdae93";
const BASE_5_HEX: &str = "#d5c4a1";
const BASE_6_HEX: &str = "#ebdbb2";
const BASE_7_HEX: &str = "#fbf1c7";

// Accents
const BASE_8_HEX: &str = "#fb4934";
const BASE_9_HEX: &str = "#d65d0e";
const BASE_A_HEX: &str = "#fe8019";
const BASE_B_HEX: &str = "#fabd2f";
const BASE_C_HEX: &str = "#b8bb26";
const BASE_D_HEX: &str = "#8ec07c";
const BASE_E_HEX: &str = "#83a598";
const BASE_F_HEX: &str = "#d3869b";

// Darks
const BASE_0_RGB: Color = Color::Rgb(29, 32, 33);
const BASE_1_RGB: Color = Color::Rgb(60, 56, 54);
const BASE_2_RGB: Color = Color::Rgb(80, 73, 69);
const BASE_3_RGB: Color = Color::Rgb(102, 92, 84);

// Lights
const BASE_4_RGB: Color = Color::Rgb(189, 174, 147);
const BASE_5_RGB: Color = Color::Rgb(213, 196, 161);
const BASE_6_RGB: Color = Color::Rgb(235, 219, 178);
const BASE_7_RGB: Color = Color::Rgb(251, 241, 199);

// Accents
const BASE_8_RGB: Color = Color::Rgb(251, 73, 52);
const BASE_9_RGB: Color = Color::Rgb(214, 93, 14);
const BASE_A_RGB: Color = Color::Rgb(254, 128, 25);
const BASE_B_RGB: Color = Color::Rgb(250, 189, 47);
const BASE_C_RGB: Color = Color::Rgb(184, 187, 38);
const BASE_D_RGB: Color = Color::Rgb(142, 192, 124);
const BASE_E_RGB: Color = Color::Rgb(131, 165, 152);
const BASE_F_RGB: Color = Color::Rgb(211, 134, 155);
