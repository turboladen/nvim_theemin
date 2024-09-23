use core::fmt;

use bit_struct::u4;

use super::{Name, Nr8};

/// This is from 0 to the number of `:h tui-colors` available. Also see `:h cterm-colors`.
///
#[derive(Debug, Clone, Copy)]
pub struct Nr16(u4);

impl Nr16 {
    pub fn new(value: u4) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u4 {
        self.0
    }
}

impl From<Name> for Nr16 {
    fn from(name: Name) -> Self {
        let x = match name {
            Name::Red | Name::LightRed => 12,
            Name::DarkRed => 4,
            Name::Green | Name::LightGreen => 10,
            Name::DarkGreen => 2,
            Name::Blue | Name::LightBlue => 9,
            Name::DarkBlue => 1,
            Name::Cyan | Name::LightCyan => 11,
            Name::DarkCyan => 3,
            Name::Magenta | Name::LightMagenta => 13,
            Name::DarkMagenta => 5,
            Name::Yellow | Name::LightYellow => 14,
            Name::Brown | Name::DarkYellow => 6,
            Name::Gray | Name::LightGray => 7,
            Name::DarkGray => 8,
            Name::Black => 0,
            Name::White => 15,
        };

        Self(unsafe { u4::new_unchecked(x) })
    }
}

impl From<Nr8> for Nr16 {
    fn from(nr8: Nr8) -> Self {
        Name::from(nr8).into()
    }
}

impl fmt::Display for Nr16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
