use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Select the last (previously selected) pane
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux last-pane [-deZ] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux last-pane [-de] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux last-pane [-t target-window]
/// (alias: lastp)
/// ```
// FIXME: versions and function parameters
#[derive(Debug, Clone)]
pub struct LastPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for LastPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LAST_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> LastPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(&mut self) -> &mut Self {
        self.0.push_flag(E_LOWERCASE_KEY);
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_4")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LastPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LAST_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for LastPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LAST_PANE)),
            ..Default::default()
        })
    }
}
