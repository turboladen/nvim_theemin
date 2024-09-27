use core::fmt;

use palette::Srgb;

use super::{Name, Nr16, Nr8};

/// Represents a color as defined in `:h cterm-colors`.
///
#[derive(Debug, Clone, Copy)]
pub enum Color {
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

impl fmt::Display for Color {
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

#[cfg(test)]
mod tests {
    use bit_struct::{u3, u4};

    use super::*;

    #[test]
    fn name_to_string_test() {
        assert_eq!(Color::Name(Name::DarkRed).to_string(), "DarkRed");
    }

    #[test]
    fn nr16_to_string_test() {
        assert_eq!(Color::Nr16(u4::new(15).unwrap().into()).to_string(), "15");
    }

    #[test]
    fn nr8_to_string_test() {
        let value = u3::new(7).unwrap();

        assert_eq!(Color::Nr8(value.into()).to_string(), "7");
        assert_eq!(Color::Nr8(Nr8::new(value, true)).to_string(), "7*");
    }

    #[test]
    fn rgb_to_string_test() {
        assert_eq!(Color::Rgb(0x123456.into()).to_string(), "#123456");
    }
}
