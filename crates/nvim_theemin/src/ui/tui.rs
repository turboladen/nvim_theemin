mod name;
mod nr16;
mod nr8;
mod tui_highlight_arg;

use palette::Srgb;

pub use self::{name::Name, nr16::Nr16, nr8::Nr8, tui_highlight_arg::TuiHighlightArg};

use core::fmt;

/// Represents all highlighting options for the TUI. See "1. TUI highlight arguments" in `:h
/// highlight-args`.
///
#[derive(Debug, Clone)]
pub struct Tui {
    /// See `:h highlight-cterm`.
    ///
    cterm: Vec<TuiHighlightArg>,

    /// See `:h highlight-start`.
    ///
    start: Option<String>,

    /// See `:h highlight-stop`.
    ///
    stop: Option<String>,

    /// See `:h ctermfg`.
    ///
    ctermfg: Option<CtermColor>,

    /// See `:h ctermbg`.
    ///
    ctermbg: Option<CtermColor>,
}

impl Tui {
    /// Constructs a new `Tui` with `cterm=NONE`.
    ///
    pub fn new_none() -> Self {
        Self {
            cterm: vec![TuiHighlightArg::None],
            start: None,
            stop: None,
            ctermfg: None,
            ctermbg: None,
        }
    }
}

/// Represents a color as defined in `:h cterm-colors`.
///
#[derive(Debug, Clone, Copy)]
pub enum CtermColor {
    Name(Name),

    /// This is from 0 to the number of `:h tui-colors` available (maxes at 24-bit, but we use 32
    /// to compensate).
    ///
    Nr16(Nr16),

    Nr8(Nr8),

    /// This is from 0 to the number of `:h tui-colors` available. Also see `:h cterm-colors`.
    ///
    Rgb(Srgb<u8>),

    None,
}

impl fmt::Display for CtermColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(scn) => write!(f, "{scn}"),
            Self::Nr16(num) => write!(f, "{num}"),
            Self::Nr8(num) => write!(f, "{num}"),
            Self::Rgb(num) => write!(f, "#{num:x}"),
            Self::None => f.write_str("NONE"),
        }
    }
}
