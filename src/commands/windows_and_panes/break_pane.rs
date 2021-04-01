use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// tmux ^2.4:
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux break-pane [-dP] [-F format] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux break-pane [-d] [-t target-window]
/// (alias: breakp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux break-pane [-d] [-p pane-index] [-t target-window]
/// (alias: breakp)
/// ```
// FIXME: phantom conditionals

#[derive(Debug, Clone)]
pub struct BreakPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for BreakPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(BREAK_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> BreakPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - the new window does not become the current window
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-P]` - option prints information about the new window after it has been created
    #[cfg(feature = "tmux_1_7")]
    pub fn print(&mut self) -> &mut Self {
        self.0.push_flag(P_UPPERCASE_KEY);
        self
    }

    /// `[-F format]` - specify format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[-n]` - window-name
    #[cfg(feature = "tmux_2_4")]
    pub fn window_name<S: Into<Cow<'a, str>>>(&mut self, window_name: S) -> &mut Self {
        self.0.push_option(N_LOWERCASE_KEY, window_name);
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_2_1")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, src_pane);
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_pane);
        self
    }

    /// `[-t dst-window]` - dst-window
    #[cfg(feature = "tmux_2_2")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(&mut self, dst_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_window);
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    // FIXME:
    // `[-p pane-index]` - pane-index

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for BreakPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(BREAK_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for BreakPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(BREAK_PANE)),
            ..Default::default()
        })
    }
}
