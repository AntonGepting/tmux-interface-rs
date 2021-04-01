use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for showing options
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux show-options [-gsw] [-t target-session | target-window]
/// (alias: show)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-options [-t target-session]
/// (alias: show)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux show-options [-t target-session] option value
/// (alias: show)
/// ```
// XXX: better result type?
#[derive(Debug, Clone)]
pub struct ShowOptions<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowOptions<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_OPTIONS)),
            ..Default::default()
        })
    }
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-A]` - includes options inherited from a parent set of options
    #[cfg(feature = "tmux_3_0")]
    pub fn include_inherited(&mut self) -> &mut Self {
        self.0.push_flag(A_UPPERCASE_KEY);
        self
    }

    /// `[-g]` - global session or window options are listed
    #[cfg(feature = "tmux_1_2")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-H]` - includes hooks (omitted by default)
    #[cfg(feature = "tmux_3_0")]
    pub fn hooks(&mut self) -> &mut Self {
        self.0.push_flag(H_UPPERCASE_KEY);
        self
    }

    /// `[-p]` - show window options
    #[cfg(feature = "tmux_3_0")]
    pub fn pane(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-q]` - no error will be returned if `option` is unset
    #[cfg(feature = "tmux_1_8")]
    pub fn quiet(&mut self) -> &mut Self {
        self.0.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-s]` - show the server options
    #[cfg(feature = "tmux_1_2")]
    pub fn server(&mut self) -> &mut Self {
        self.0.push_flag(S_LOWERCASE_KEY);
        self
    }

    /// `[-v]` - shows only the option value
    #[cfg(feature = "tmux_1_8")]
    pub fn value(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-w]` - show the window options
    #[cfg(feature = "tmux_1_2")]
    pub fn window(&mut self) -> &mut Self {
        self.0.push_flag(W_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - target session or window name
    //#[cfg(feature = "tmux_X_X")]
    pub fn target<S: Into<Cow<'a, str>>>(&mut self, target: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target);
        self
    }

    /// `[option]` - specify option name
    #[cfg(feature = "tmux_1_7")]
    pub fn option<S: Into<Cow<'a, str>>>(&mut self, option: S) -> &mut Self {
        self.0.push_param(option);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ShowOptions<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_OPTIONS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowOptions<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_OPTIONS)),
            ..Default::default()
        })
    }
}
