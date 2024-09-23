use core::fmt;

use bit_struct::u3;

use super::{Name, Nr16};

/// Color used for 8-color terminals. See `:h cterm-colors`.
///
#[derive(Debug, Clone, Copy)]
pub struct Nr8 {
    value: u3,
    is_bold: bool,
}

impl Nr8 {
    pub fn new(value: u3, is_bold: bool) -> Self {
        Self { value, is_bold }
    }

    pub fn value(&self) -> u3 {
        self.value
    }

    pub fn is_bold(&self) -> bool {
        self.is_bold
    }
}

impl From<Name> for Nr8 {
    fn from(name: Name) -> Self {
        let (value, is_bold) = match name {
            Name::Red | Name::LightRed => (1, true),
            Name::DarkRed => (1, false),
            Name::Green | Name::LightGreen => (2, true),
            Name::DarkGreen => (2, false),
            Name::Blue | Name::LightBlue => (4, true),
            Name::DarkBlue => (4, false),
            Name::Cyan | Name::LightCyan => (6, true),
            Name::DarkCyan => (6, false),
            Name::Magenta | Name::LightMagenta => (5, true),
            Name::DarkMagenta => (5, false),
            Name::Yellow | Name::LightYellow => (3, true),
            Name::Brown | Name::DarkYellow => (3, false),
            Name::Gray | Name::LightGray => (7, false),
            Name::DarkGray => (0, true),
            Name::Black => (0, false),
            Name::White => (7, true),
        };

        Self {
            value: unsafe { u3::new_unchecked(value) },
            is_bold,
        }
    }
}

impl From<Nr16> for Nr8 {
    fn from(nr16: Nr16) -> Self {
        Name::from(nr16).into()
    }
}

impl fmt::Display for Nr8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_bold {
            write!(f, "{}*", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}
