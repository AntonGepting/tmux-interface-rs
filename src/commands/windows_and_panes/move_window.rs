use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux move-window [-rdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux move-window [-dk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux move-window [-d] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Debug, Clone)]
pub struct MoveWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for MoveWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(MOVE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> MoveWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub fn add(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-r]` - all windows in the session are renumbered in sequential order
    #[cfg(feature = "tmux_1_7")]
    pub fn renumber(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-d]` - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-k]` - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_1_3")]
    pub fn kill(&mut self) -> &mut Self {
        self.0.push_flag(K_LOWERCASE_KEY);
        self
    }

    /// `[-s src-window]` - src-window
    #[cfg(feature = "tmux_0_8")]
    pub fn src_window<S: Into<Cow<'a, str>>>(&mut self, src_window: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, src_window);
        self
    }

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(&mut self, dst_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for MoveWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(MOVE_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for MoveWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(MOVE_WINDOW)),
            ..Default::default()
        })
    }
}
