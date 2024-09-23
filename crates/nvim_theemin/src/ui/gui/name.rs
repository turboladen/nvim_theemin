use core::fmt;

use crate::AsGroupName;

/// See `:h termguicolors`.
///
#[derive(Debug, Clone, Copy)]
pub enum Name {
    Red,
    LightRed,
    DarkRed,

    Green,
    LightGreen,
    DarkGreen,
    SeaGreen,

    Blue,
    LightBlue,
    DarkBlue,
    SlateBlue,

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

    Orange,
    Purple,
    Violet,

    NvimDarkBlue,
    NvimLightBlue,

    NvimDarkCyan,
    NvimLightCyan,

    NvimDarkGray1,
    NvimLightGray1,

    NvimDarkGray2,
    NvimLightGray2,

    NvimDarkGray3,
    NvimLightGray3,

    NvimDarkGray4,
    NvimLightGray4,

    NvimDarkGreen,
    NvimLightGreen,

    NvimDarkMagenta,
    NvimLightMagenta,

    NvimDarkRed,
    NvimLightRed,

    NvimDarkYellow,
    NvimLightYellow,
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
            Self::SeaGreen => "SeaGreen",
            Self::Blue => "Blue",
            Self::LightBlue => "LightBlue",
            Self::DarkBlue => "DarkBlue",
            Self::SlateBlue => "SlateBlue",
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
            Self::Orange => "Orange",
            Self::Purple => "Purple",
            Self::Violet => "Violet",
            Self::NvimDarkBlue => "NvimDarkBlue",
            Self::NvimLightBlue => "NvimLightBlue",
            Self::NvimDarkCyan => "NvimDarkCyan",
            Self::NvimLightCyan => "NvimLightCyan",
            Self::NvimDarkGray1 => "NvimDarkGray1",
            Self::NvimLightGray1 => "NvimLightGray1",
            Self::NvimDarkGray2 => "NvimDarkGray2",
            Self::NvimLightGray2 => "NvimLightGray2",
            Self::NvimDarkGray3 => "NvimDarkGray3",
            Self::NvimLightGray3 => "NvimLgithGray3",
            Self::NvimDarkGray4 => "NvimDarkGray4",
            Self::NvimLightGray4 => "NvimLightGray4",
            Self::NvimDarkGreen => "NvimDarkGreen",
            Self::NvimLightGreen => "NvimLightGreen",
            Self::NvimDarkMagenta => "NvimDarkMagenta",
            Self::NvimLightMagenta => "NvimLightMagenta",
            Self::NvimDarkRed => "NvimDarkRed",
            Self::NvimLightRed => "NvimLightRed",
            Self::NvimDarkYellow => "NvimDarkYellow",
            Self::NvimLightYellow => "NvimLightYellow",
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_group_name())
    }
}
