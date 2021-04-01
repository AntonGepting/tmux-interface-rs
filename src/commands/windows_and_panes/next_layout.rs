use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Move a window to the next layout and rearrange the panes to fit
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux next-layout [-t target-window]
/// (alias: nextl)
/// ```
#[derive(Debug, Clone)]
pub struct NextLayout<'a>(pub TmuxCommand<'a>);

impl<'a> Default for NextLayout<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(NEXT_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> NextLayout<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for NextLayout<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(NEXT_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for NextLayout<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(NEXT_LAYOUT)),
            ..Default::default()
        })
    }
}
