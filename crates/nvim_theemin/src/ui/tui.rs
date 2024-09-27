mod builder;
mod color;
mod name;
mod nr16;
mod nr8;

use crate::highlight::{ToHighlightCommand, VimL};

use super::HighlightArg;

pub use self::{builder::Builder, color::Color, name::Name, nr16::Nr16, nr8::Nr8};

/// Represents all highlighting options for the TUI. See "1. TUI highlight arguments" in `:h
/// highlight-args`.
///
#[derive(Debug, Clone)]
pub struct HighlightArguments {
    /// See `:h highlight-cterm`.
    ///
    cterm: Vec<HighlightArg>,

    /// See `:h highlight-start`.
    ///
    start: Option<String>,

    /// See `:h highlight-stop`.
    ///
    stop: Option<String>,

    /// See `:h ctermfg`.
    ///
    ctermfg: Option<Color>,

    /// See `:h ctermbg`.
    ///
    ctermbg: Option<Color>,
}

impl HighlightArguments {
    /// Constructs a new `Tui` with `cterm=NONE`.
    ///
    pub fn new_none() -> Self {
        Self {
            cterm: vec![HighlightArg::None],
            start: None,
            stop: None,
            ctermfg: None,
            ctermbg: None,
        }
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl ToHighlightCommand<VimL> for HighlightArguments {
    fn to_highlight_command(&self) -> VimL {
        let mut output = Vec::new();

        if !self.cterm.is_empty() {
            let cterm_args: String = self
                .cterm
                .iter()
                .map(|arg| arg.as_ref())
                .collect::<Vec<&str>>()
                .join(",");
            output.push(format!("cterm={cterm_args}"));
        }

        if let Some(start) = self.start.as_ref() {
            output.push(format!("start={start}"));
        }

        if let Some(stop) = self.stop.as_ref() {
            output.push(format!("stop={stop}"));
        }

        if let Some(ctermfg) = self.ctermfg {
            output.push(format!("ctermfg={ctermfg}"));
        }

        if let Some(ctermbg) = self.ctermbg {
            output.push(format!("ctermbg={ctermbg}"));
        }

        VimL::new(output.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_highlight_command_viml {
        use crate::ui::HighlightArg;

        use super::*;

        #[test]
        fn no_args_test() {
            assert!(HighlightArguments::builder().build().is_none())
        }

        #[test]
        fn new_none_test() {
            assert_eq!(
                HighlightArguments::new_none().to_highlight_command(),
                VimL::new("cterm=NONE")
            );
        }

        #[test]
        fn all_args_test() {
            let args = HighlightArguments::builder()
                .cterm(vec![HighlightArg::Italic, HighlightArg::Bold])
                .start("<Esc>[27h;".to_string())
                .stop("[<Space>r;".to_string())
                .ctermfg(Color::Rgb(0xffaa33.into()))
                .ctermbg(Color::Name(Name::Black))
                .build()
                .unwrap();
            assert_eq!(
                args.to_highlight_command(),
                VimL::new(
                    "cterm=italic,bold start=<Esc>[27h; stop=[<Space>r; ctermfg=#ffaa33 ctermbg=Black"
                )
            );
        }
    }
}
