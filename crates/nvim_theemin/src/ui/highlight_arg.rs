/// See `:help highlight-cterm`.
///
#[derive(Debug, Clone, Copy)]
pub enum HighlightArg {
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

impl AsRef<str> for HighlightArg {
    fn as_ref(&self) -> &str {
        match self {
            HighlightArg::Bold => "bold",
            HighlightArg::Underline => "underline",
            HighlightArg::Undercurl => "undercurl",
            HighlightArg::UnderDouble => "underdouble",
            HighlightArg::UnderDotted => "underdotted",
            HighlightArg::UnderDashed => "underdashed",
            HighlightArg::Strikethrough => "strikethrough",
            HighlightArg::Reverse => "reverse",
            HighlightArg::Inverse => "inverse",
            HighlightArg::Italic => "italic",
            HighlightArg::Standout => "standout",
            HighlightArg::NoCombine => "nocombine",
            HighlightArg::None => "NONE",
        }
    }
}
