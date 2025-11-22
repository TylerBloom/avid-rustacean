use ratatui::style::{Color, Style};

pub use avid_rustacean_model::*;

/// A simple trait to extend the iterface of GruvboxColor.
pub trait GruvboxExt {
    // TODO: Const-ify these when possible.
    fn color_index(self) -> u8;
    fn to_color(self) -> Color;
    fn to_color_str(self) -> &'static str;
    fn fg_style(self) -> Style;
    fn bg_style(self) -> Style;
    fn full_style(self, other: Self) -> Style;
    fn default_style() -> Style;
}

impl GruvboxExt for GruvboxColor {
    fn color_index(self) -> u8 {
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
                GruvboxAccent::Orange => 9,
                GruvboxAccent::Yellow => 10,
                GruvboxAccent::Green => 11,
                GruvboxAccent::Teal => 12,
                GruvboxAccent::Blue => 13,
                GruvboxAccent::Pink => 14,
                GruvboxAccent::BurntOrange => 15,
            },
        }
    }

    fn full_style(self, other: Self) -> Style {
        Style::new().fg(self.to_color()).bg(other.to_color())
    }

    fn default_style() -> Style {
        Style::new()
            .fg(Self::default_fg().to_color())
            .bg(Self::default_bg().to_color())
    }

    fn to_color(self) -> Color {
        indexed_color(self.color_index())
    }

    fn to_color_str(self) -> &'static str {
        indexed_color_str(self.color_index())
    }

    fn fg_style(self) -> Style {
        Style::new()
            .fg(self.to_color())
            .bg(Self::default_bg().to_color())
    }

    fn bg_style(self) -> Style {
        Style::new()
            .fg(Self::default_fg().to_color())
            .bg(self.to_color())
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
    assert!(i <= 15);
    Color::Indexed(i)
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

/*
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
*/
