use crate::ui::HighlightArg;

use super::{Color, HighlightArguments};

#[derive(Debug, Clone, Default)]
pub struct Builder {
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

impl Builder {
    pub fn cterm(&mut self, cterm: Vec<HighlightArg>) -> &mut Self {
        self.cterm = cterm;
        self
    }

    pub fn start(&mut self, start: String) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn stop(&mut self, stop: String) -> &mut Self {
        self.stop = Some(stop);
        self
    }

    pub fn ctermfg(&mut self, ctermfg: Color) -> &mut Self {
        self.ctermfg = Some(ctermfg);
        self
    }

    pub fn ctermbg(&mut self, ctermbg: Color) -> &mut Self {
        self.ctermbg = Some(ctermbg);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.cterm.is_empty()
            && self.start.is_none()
            && self.stop.is_none()
            && self.ctermfg.is_none()
            && self.ctermbg.is_none()
    }

    /// Returns `None` if no argument is set (there's no value in building a `HighlightArguments`
    /// with no arguments).
    ///
    pub fn build(&self) -> Option<HighlightArguments> {
        if self.is_empty() {
            None
        } else {
            Some(HighlightArguments {
                cterm: self.cterm.clone(),
                start: self.start.clone(),
                stop: self.stop.clone(),
                ctermfg: self.ctermfg,
                ctermbg: self.ctermbg,
            })
        }
    }
}
