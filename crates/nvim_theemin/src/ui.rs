//! This module provides means for defining (neo)vim's different color types.
//!
pub mod gui;
mod highlight_arg;
pub mod tui;

pub use self::highlight_arg::HighlightArg;
