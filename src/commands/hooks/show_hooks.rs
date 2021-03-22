use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^2.2:
/// ```text
/// tmux show-hooks [-g] [-t target-session]
/// ```
#[derive(Debug, Clone)]
pub struct ShowHooks<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowHooks<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_HOOKS)),
            ..Default::default()
        })
    }
}

impl<'a> ShowHooks<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(g_KEY);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ShowHooks<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_HOOKS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowHooks<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_HOOKS)),
            ..Default::default()
        })
    }
}
