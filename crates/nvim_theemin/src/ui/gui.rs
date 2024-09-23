mod name;

pub use self::name::Name;

use std::fmt;

use palette::Srgb;

use crate::AsGroupName;

use super::HighlightArg;

#[derive(Debug, Clone)]
pub struct HighlightArguments {
    /// See `:h highlight-gui`.
    ///
    gui: Vec<HighlightArg>,

    /// See `:h highlight-font`.
    ///
    font: Option<String>,

    /// See `:h guifg`.
    ///
    guifg: Option<Color>,

    /// See `:h guibg`.
    ///
    guibg: Option<Color>,

    /// See `:h guisp`.
    ///
    guisp: Option<Color>,

    /// See `:h highlight-blend`.
    ///
    blend: Option<u32>,
}

impl HighlightArguments {
    /// Constructs a new `Tui` with `cterm=NONE`.
    ///
    pub fn new_none() -> Self {
        Self {
            gui: vec![HighlightArg::None],
            font: None,
            guifg: None,
            guibg: None,
            guisp: None,
            blend: None,
        }
    }
}

/// See `:h gui-colors`
///
#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// These are available on most systems. See `:h gui-colors`.
    ///
    Name(Name),

    /// Color name with an embedded space or other special characters (eg. "salmon pink").
    ///
    CustomName(&'static str),

    /// Specify a color by its RGB (red, green, blue) values.
    ///
    Hex(Srgb<u8>),

    /// Use normal background color.
    ///
    Background,

    /// Use normal foreground color.
    ///
    Foreground,

    /// No color; transparent.
    ///
    None,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(scn) => f.write_str(scn.as_group_name()),
            Self::CustomName(name) => f.write_str(name),
            Self::Hex(hex) => write!(f, "#{hex:x}"),
            Self::Background => f.write_str("bg"),
            Self::Foreground => f.write_str("fg"),
            Self::None => f.write_str("NONE"),
        }
    }
}
