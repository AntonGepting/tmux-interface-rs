use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Select the last (previously selected) window
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux last-window [-t target-session]
/// (alias: last)
/// ```
#[derive(Debug, Clone)]
pub struct LastWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for LastWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LAST_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> LastWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LastWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LAST_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for LastWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LAST_WINDOW)),
            ..Default::default()
        })
    }
}
