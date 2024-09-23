/// See `:help highlight-cterm`.
///
#[derive(Debug, Clone, Copy)]
pub enum TuiHighlightArg {
    Bold,
    Underline,
    Undercurl,
    UnderDouble,
    UnderDotted,
    UnderDashed,
    Strikethrough,
    Reverse,
    Inverse,
    Italic,
    Standout,
    NoCombine,
    None,
}

impl AsRef<str> for TuiHighlightArg {
    fn as_ref(&self) -> &str {
        match self {
            TuiHighlightArg::Bold => "bold",
            TuiHighlightArg::Underline => "underline",
            TuiHighlightArg::Undercurl => "undercurl",
            TuiHighlightArg::UnderDouble => "underdouble",
            TuiHighlightArg::UnderDotted => "underdotted",
            TuiHighlightArg::UnderDashed => "underdashed",
            TuiHighlightArg::Strikethrough => "strikethrough",
            TuiHighlightArg::Reverse => "reverse",
            TuiHighlightArg::Inverse => "inverse",
            TuiHighlightArg::Italic => "italic",
            TuiHighlightArg::Standout => "standout",
            TuiHighlightArg::NoCombine => "nocombine",
            TuiHighlightArg::None => "NONE",
        }
    }
}
