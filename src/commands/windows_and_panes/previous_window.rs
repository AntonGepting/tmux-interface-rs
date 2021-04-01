use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Move to the previous window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// tmux previous-window [-a] [-t target-session]
/// (alias: prev)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux previous-window [-t target-session]
/// (alias: prev)
/// ```
#[derive(Debug, Clone)]
pub struct PreviousWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for PreviousWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(PREVIOUS_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> PreviousWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_0_9")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for PreviousWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(PREVIOUS_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for PreviousWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(PREVIOUS_WINDOW)),
            ..Default::default()
        })
    }
}
