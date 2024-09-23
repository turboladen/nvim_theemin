use core::fmt;

use crate::AsGroupName;

use super::{Nr16, Nr8};

#[derive(Debug, Clone, Copy)]
pub enum Name {
    Red,
    LightRed,
    DarkRed,

    Green,
    LightGreen,
    DarkGreen,

    Blue,
    LightBlue,
    DarkBlue,

    Cyan,
    LightCyan,
    DarkCyan,

    Magenta,
    LightMagenta,
    DarkMagenta,

    Yellow,
    LightYellow,
    Brown,
    DarkYellow,

    Gray,
    LightGray,
    DarkGray,

    Black,
    White,
}

impl From<Nr16> for Name {
    fn from(nr16: Nr16) -> Self {
        match nr16.value().value() {
            0 => Self::Black,
            1 => Self::DarkBlue,
            2 => Self::DarkGreen,
            3 => Self::DarkCyan,
            4 => Self::DarkRed,
            5 => Self::DarkMagenta,
            6 => Self::Brown,
            7 => Self::Gray,
            8 => Self::DarkGray,
            9 => Self::Blue,
            10 => Self::Green,
            11 => Self::Cyan,
            12 => Self::Red,
            13 => Self::Magenta,
            14 => Self::Yellow,
            15 => Self::White,
            _ => unreachable!(),
        }
    }
}

impl From<Nr8> for Name {
    fn from(nr8: Nr8) -> Self {
        match (nr8.value().value(), nr8.is_bold()) {
            (0, true) => Self::DarkGray,
            (0, false) => Self::Black,
            (1, true) => Self::Red,
            (1, false) => Self::DarkRed,
            (2, true) => Self::Green,
            (2, false) => Self::DarkGreen,
            (3, true) => Self::Yellow,
            (3, false) => Self::Brown,
            (4, true) => Self::Blue,
            (4, false) => Self::DarkBlue,
            (5, true) => Self::Magenta,
            (5, false) => Self::DarkMagenta,
            (6, true) => Self::Cyan,
            (6, false) => Self::DarkCyan,
            (7, true) => Self::White,
            (7, false) => Self::Gray,
            (t, _) => unreachable!("{t} should not be greater than 7"),
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_group_name())
    }
}

impl AsGroupName for Name {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::LightRed => "LightRed",
            Self::DarkRed => "DarkRed",
            Self::Green => "Green",
            Self::LightGreen => "LightGreen",
            Self::DarkGreen => "DarkGreen",
            Self::Blue => "Blue",
            Self::LightBlue => "LightBlue",
            Self::DarkBlue => "DarkBlue",
            Self::Cyan => "Cyan",
            Self::LightCyan => "LightCyan",
            Self::DarkCyan => "DarkCyan",
            Self::Magenta => "Magenta",
            Self::LightMagenta => "LightMagenta",
            Self::DarkMagenta => "DarkMagenta",
            Self::Yellow => "Yellow",
            Self::LightYellow => "LightYellow",
            Self::Brown => "Brown",
            Self::DarkYellow => "DarkYellow",
            Self::Gray => "Gray",
            Self::LightGray => "LightGray",
            Self::DarkGray => "DarkGray",
            Self::Black => "Black",
            Self::White => "White",
        }
    }
}
