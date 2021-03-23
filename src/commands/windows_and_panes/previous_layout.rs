use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Move to the previous layout in the session
///
/// # Manual
///
/// tmux ^1.3:
/// ```text
/// tmux previous-layout [-t target-window]
/// (alias: prevl)
/// ```
#[derive(Debug, Clone)]
pub struct PreviousLayout<'a>(pub TmuxCommand<'a>);

impl<'a> Default for PreviousLayout<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(PREVIOUS_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> PreviousLayout<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(t_KEY, target_window);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for PreviousLayout<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(PREVIOUS_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for PreviousLayout<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(PREVIOUS_LAYOUT)),
            ..Default::default()
        })
    }
}
